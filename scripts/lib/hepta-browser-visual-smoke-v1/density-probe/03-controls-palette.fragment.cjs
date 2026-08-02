module.exports = `
			  const menuTriggers = Array.from(document.querySelectorAll("summary[data-control-ui-menu-trigger='icon']"));
  const menuTriggerDetails = menuTriggers.map((node) => {
    const icon = node.querySelector(".tg-menu-icon");
    const label = node.querySelector(".sr-only");
    const style = getComputedStyle(node);
    const visible = elementVisible(node);
    const backdrop = style.backdropFilter || style.webkitBackdropFilter || "";
    const filterText = style.filter || "";
    const dropShadowCount = (filterText.match(/drop-shadow/g) || []).length;
    const visibleIconText = visibleText(node);
    const ariaLabel = node.getAttribute("aria-label") || "";
    const title = node.getAttribute("title") || "";
    return {
      aria_label: ariaLabel,
      title,
      title_matches_aria_label: title === ariaLabel,
      icon_present: Boolean(icon),
      svg_icon_present: hasSvgIcon(node),
      sr_label_present: Boolean(label && (label.textContent || "").trim().length > 0),
      visible,
      visible_icon_text: visibleIconText,
      visible_icon_text_absent: visibleIconText.length === 0,
      border_radius: Number.parseFloat(style.borderTopLeftRadius || "0"),
      backdrop_filter: backdrop,
      box_shadow: compactShadow(style.boxShadow),
      filter: filterText && filterText !== "none" ? "present" : "none",
      filter_sample: filterText.slice(0, 180),
      icon_prismatic_control_drop_shadow_count: dropShadowCount,
      icon_prismatic_control_ready: dropShadowCount >= 2,
      ...richRect(node),
    };
  });
  const menuTriggerReady = menuTriggerDetails.length >= 2
    && menuTriggerDetails.every((item) => (
      item.aria_label.length > 0
      && item.title.length > 0
      && item.title_matches_aria_label
      && item.icon_present
      && item.sr_label_present
    ))
    && menuTriggerDetails.every((item) => (
      item.visible
      && item.width >= 44
      && item.height >= 44
      && item.border_radius >= 20
      && /blur\\(/.test(item.backdrop_filter)
      && item.box_shadow !== "none"
      && item.svg_icon_present
      && item.visible_icon_text_absent
    ));
  const folderChips = Array.from(document.querySelectorAll(".tg-folder-chip")).filter(elementVisible);
  const folderChipDetails = folderChips.map((node) => {
    const style = getComputedStyle(node);
    const ariaLabel = node.getAttribute("aria-label") || "";
    const title = node.getAttribute("title") || "";
    const ariaPressed = node.getAttribute("aria-pressed") || "";
    const active = node.classList.contains("active");
    const backdrop = style.backdropFilter || style.webkitBackdropFilter || "";
    const filterText = style.filter || "";
    const dropShadowCount = (filterText.match(/drop-shadow/g) || []).length;
    const textShadow = style.textShadow || "";
    const textShadowCount = textShadow && textShadow !== "none" ? ((textShadow.match(/rgb/g) || []).length || 1) : 0;
    const textColor = parseCssColor(style.color);
    const bgColor = effectiveBackground(node);
    const ratio = textColor ? contrastRatio(textColor, bgColor) : 0;
    return {
      key: node.getAttribute("data-chat-folder") || "",
      text: visibleText(node),
      aria_label: ariaLabel,
      title,
      title_matches_aria_label: title === ariaLabel,
      aria_pressed: ariaPressed,
      active,
      active_state_matches_aria_pressed: active ? ariaPressed === "true" : ariaPressed === "false",
      visible: true,
      border_radius: Number.parseFloat(style.borderTopLeftRadius || "0"),
      backdrop_filter: backdrop,
      box_shadow: compactShadow(style.boxShadow),
      filter: filterText && filterText !== "none" ? "present" : "none",
      filter_sample: filterText.slice(0, 180),
      rail_filter_drop_shadow_count: dropShadowCount,
      rail_prismatic_filter_ready: dropShadowCount >= 2,
      text_shadow: textShadow && textShadow !== "none" ? "present" : "none",
      text_shadow_sample: textShadow.slice(0, 180),
      folder_chip_label_text_shadow_count: textShadowCount,
      folder_chip_label_prismatic_etch_ready: textShadowCount >= 2,
      contrast_ratio: Number(ratio.toFixed(2)),
      readable: ratio >= 4.5,
      ...richRect(node),
    };
  });
  const folderChipTouchReady = folderChipDetails.length === 0 || (
    folderChipDetails.length >= 3
    && folderChipDetails.every((item) => (
      item.key.length > 0
      && item.text.length > 0
      && item.aria_label.length > 0
      && item.title.length > 0
      && item.title_matches_aria_label
      && item.active_state_matches_aria_pressed
      && item.width >= 44
      && item.height >= 44
      && item.border_radius >= 20
      && /blur\\(/.test(item.backdrop_filter)
      && item.box_shadow !== "none"
    ))
  );
  const folderChipLabelPrismaticEtchLightGlassReady = railVisible
    ? folderChipDetails.length >= 3
      && folderChipDetails.every((item) => (
        item.visible
        && item.text.length > 0
        && item.width >= 44
        && item.height >= 44
        && item.text_shadow === "present"
        && item.folder_chip_label_prismatic_etch_ready === true
        && item.folder_chip_label_text_shadow_count >= 2
        && item.readable === true
        && item.contrast_ratio >= 4.5
      ))
    : folderChipDetails.length === 0;
  const rowMenuToggles = Array.from(document.querySelectorAll("[data-chat-row-menu-toggle]"));
  const rowMenuToggleDetails = rowMenuToggles.map((node) => {
    const style = getComputedStyle(node);
    const ariaLabel = node.getAttribute("aria-label") || "";
    const title = node.getAttribute("title") || "";
    const backdrop = style.backdropFilter || style.webkitBackdropFilter || "";
    const visibleIconText = visibleText(node);
    const row = node.closest("[data-chat-conversation]");
    return {
      owner_key: row?.getAttribute("data-chat-conversation") || "",
      toggle_key: node.getAttribute("data-chat-row-menu-toggle") || "",
      marker: node.getAttribute("data-control-ui-row-menu-trigger") || "",
      aria_label: ariaLabel,
      title,
      title_matches_aria_label: title === ariaLabel,
      visible: elementVisible(node),
      svg_icon_present: hasSvgIcon(node),
      visible_icon_text: visibleIconText,
      visible_icon_text_absent: visibleIconText.length === 0,
      border_radius: Number.parseFloat(style.borderTopLeftRadius || "0"),
      backdrop_filter: backdrop,
      box_shadow: compactShadow(style.boxShadow),
      ...richRect(node),
    };
  });
  const rowMenuPanels = Array.from(document.querySelectorAll("[data-chat-row-menu-panel]"));
  const rowMenuPanelDetails = rowMenuPanels.map((node) => {
    const style = getComputedStyle(node);
    const backdrop = style.backdropFilter || style.webkitBackdropFilter || "";
    const rect = richRect(node);
    const bgColor = effectiveBackground(node);
    const bgLuminance = relativeLuminance(bgColor);
    const horizontalInViewport = rect.left >= -1 && rect.right <= window.innerWidth + 1 && rect.width <= window.innerWidth - 16;
    const verticalInViewport = rect.top >= -1 && rect.bottom <= window.innerHeight + 1 && rect.height <= window.innerHeight - 16;
    const row = node.closest("[data-chat-conversation]");
    return {
      owner_key: row?.getAttribute("data-chat-conversation") || "",
      panel_key: node.getAttribute("data-chat-row-menu-panel") || "",
      marker: node.getAttribute("data-control-ui-row-menu-panel") || "",
      visible: elementVisible(node),
      item_count: node.querySelectorAll("[data-chat-row-menu-item]").length,
      background_color: style.backgroundColor,
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
  const rowMenuItemDetails = Array.from(document.querySelectorAll("[data-chat-row-menu-item]")).map((node) => {
    const labelNode = node.querySelector(".tg-row-action__label");
    const iconNode = node.querySelector(".tg-row-action__icon");
    const style = getComputedStyle(node);
    const labelStyle = labelNode ? getComputedStyle(labelNode) : null;
    const ariaLabel = node.getAttribute("aria-label") || "";
    const title = node.getAttribute("title") || "";
    const textColor = parseCssColor(style.color);
    const bgColor = effectiveBackground(node);
    const ratio = textColor ? contrastRatio(textColor, bgColor) : 0;
    const row = node.closest("[data-chat-conversation]");
    return {
      owner_key: row?.getAttribute("data-chat-conversation") || "",
      key: node.getAttribute("data-chat-row-menu-item") || "",
      aria_label: ariaLabel,
      title,
      title_matches_aria_label: title === ariaLabel,
      color: style.color,
      effective_background: "rgb(" + Math.round(bgColor.r) + ", " + Math.round(bgColor.g) + ", " + Math.round(bgColor.b) + ")",
      contrast_ratio: Number(ratio.toFixed(2)),
      readable: ratio >= 4.5,
      icon_present: Boolean(iconNode),
      icon_svg_present: hasSvgIcon(iconNode),
      label: (labelNode?.textContent || "").replace(/\\s+/g, " ").trim(),
      visible: elementVisible(node),
      height: Math.round(node.getBoundingClientRect().height),
      label_white_space: labelStyle?.whiteSpace || "",
      label_nowrap_ready: labelStyle?.whiteSpace === "nowrap",
      ...richRect(node),
    };
  });
  const expectedVisibleRowMenuPanelCount = railVisible ? 3 : 0;
  const visibleRowMenuToggleDetails = rowMenuToggleDetails.filter((item) => item.visible);
  const visibleRowMenuPanelDetails = rowMenuPanelDetails.filter((item) => item.visible);
  const visibleRowMenuItemDetails = rowMenuItemDetails.filter((item) => item.visible);
  const rowMenuPanelKeys = new Set(visibleRowMenuPanelDetails.map((item) => item.owner_key));
  const rowMenuAllRowsReady = !railVisible || (
    visibleRowMenuToggleDetails.length === expectedVisibleRowMenuPanelCount
    && visibleRowMenuPanelDetails.length === expectedVisibleRowMenuPanelCount
    && visibleRowMenuItemDetails.length === expectedVisibleRowMenuPanelCount * 3
    && visibleRowMenuToggleDetails.every((item) => (
      item.owner_key.length > 0
      && item.toggle_key === item.owner_key
      && item.marker === "light-glass"
      && rowMenuPanelKeys.has(item.owner_key)
    ))
    && visibleRowMenuPanelDetails.every((item) => (
      item.owner_key.length > 0
      && item.panel_key === item.owner_key
      && item.marker === "light-glass"
      && item.item_count === 3
    ))
  );
	  const rowMenuTouchReady = !railVisible || (
    rowMenuAllRowsReady
    && visibleRowMenuToggleDetails.every((item) => (
      item.aria_label.length > 0
      && item.title.length > 0
      && item.title_matches_aria_label
      && item.width >= 44
      && item.height >= 44
      && item.border_radius >= 20
      && /blur\\(/.test(item.backdrop_filter)
      && item.box_shadow !== "none"
      && item.svg_icon_present
      && item.visible_icon_text_absent
    ))
    && visibleRowMenuPanelDetails.every((item) => (
      item.visible
      && item.item_count >= 3
      && item.width >= 180
      && item.height >= 132
      && item.border_radius >= 16
      && /blur\\(/.test(item.backdrop_filter)
      && item.box_shadow !== "none"
      && item.in_viewport
    ))
    && visibleRowMenuItemDetails.every((item) => (
      item.owner_key.length > 0
      && item.key.length > 0
      && item.aria_label.length > 0
      && item.title.length > 0
      && item.title_matches_aria_label
      && item.icon_present
      && item.icon_svg_present
      && item.label.length > 0
      && item.visible
      && item.height >= 44
      && item.label_nowrap_ready
	    ))
	  );
	  const rowMenuLightGlassReady = !railVisible || (
	    rowMenuAllRowsReady
	    && visibleRowMenuPanelDetails.every((item) => (
	      item.visible
	      && item.light_glass_ready
	      && item.effective_luminance >= 0.72
	      && item.effective_luminance <= 0.98
	      && /blur\\(/.test(item.backdrop_filter)
	      && item.box_shadow !== "none"
	    ))
	    && visibleRowMenuItemDetails.every((item) => (
	      item.visible
	      && item.readable
	      && item.contrast_ratio >= 4.5
	    ))
	  );
	  const commandPalettePanel = document.querySelector("[data-control-ui-command-palette-surface]");
	  const commandPalettePanelDetails = (() => {
	    if (!commandPalettePanel) {
	      return { exists: false, visible: false };
	    }
	    const style = getComputedStyle(commandPalettePanel);
	    const backdrop = style.backdropFilter || style.webkitBackdropFilter || "";
	    const filter = style.filter || "";
	    const dropShadowCount = (filter.match(/drop-shadow/g) || []).length;
	    const rect = richRect(commandPalettePanel);
	    const bgColor = effectiveBackground(commandPalettePanel);
	    const bgLuminance = relativeLuminance(bgColor);
	    const horizontalInViewport = rect.left >= -1 && rect.right <= window.innerWidth + 1 && rect.width <= window.innerWidth - 16;
	    const verticalInViewport = rect.top >= -1 && rect.bottom <= window.innerHeight + 1 && rect.height <= window.innerHeight - 16;
	    return {
	      exists: true,
	      visible: elementVisible(commandPalettePanel),
	      marker: commandPalettePanel.getAttribute("data-control-ui-command-palette-surface") || "",
	      role: commandPalettePanel.getAttribute("role") || "",
	      aria_modal: commandPalettePanel.getAttribute("aria-modal") || "",
	      aria_label: commandPalettePanel.getAttribute("aria-label") || "",
	      background_color: style.backgroundColor,
	      background_alpha: Number(directBackgroundAlpha(style).toFixed(2)),
	      translucent_ready: translucentGlassReady(style),
	      effective_background: "rgb(" + Math.round(bgColor.r) + ", " + Math.round(bgColor.g) + ", " + Math.round(bgColor.b) + ")",
	      effective_luminance: Number(bgLuminance.toFixed(3)),
	      light_glass_ready: bgLuminance >= 0.72 && bgLuminance <= 0.98,
	      border_radius: Number.parseFloat(style.borderTopLeftRadius || "0"),
	      backdrop_filter: backdrop,
	      box_shadow: compactShadow(style.boxShadow),
	      filter: filter && filter !== "none" ? "present" : "none",
	      filter_sample: filter.slice(0, 180),
	      command_palette_surface_drop_shadow_count: dropShadowCount,
	      command_palette_surface_prismatic_perimeter_ready: dropShadowCount >= 2,
	      horizontal_in_viewport: horizontalInViewport,
	      vertical_in_viewport: verticalInViewport,
	      in_viewport: horizontalInViewport && verticalInViewport,
	      ...rect,
	    };
	  })();
	  const commandPaletteBackdrop = document.querySelector(".command-palette-backdrop");
	  const commandPaletteBackdropDetails = (() => {
	    if (!commandPaletteBackdrop) {
	      return { exists: false, visible: false };
	    }
	    const style = getComputedStyle(commandPaletteBackdrop);
	    const backdrop = style.backdropFilter || style.webkitBackdropFilter || "";
	    const blurPx = Number.parseFloat((backdrop.split("blur(")[1] || "").split("px")[0] || "0");
	    const backgroundImage = style.backgroundImage || "";
	    const repeatingLayerCount = (backgroundImage.match(/repeating-linear-gradient/g) || []).length;
	    const rect = richRect(commandPaletteBackdrop);
	    return {
	      exists: true,
	      visible: elementVisible(commandPaletteBackdrop),
	      background_color: style.backgroundColor,
	      background_alpha: Number(directBackgroundAlpha(style).toFixed(2)),
	      background_image: backgroundImage && backgroundImage !== "none" ? "present" : "none",
	      background_image_sample: backgroundImage.slice(0, 180),
	      backdrop_filter: backdrop,
	      backdrop_blur_px: Number((Number.isFinite(blurPx) ? blurPx : 0).toFixed(2)),
	      command_palette_backdrop_repeating_layer_count: repeatingLayerCount,
	      command_palette_backdrop_caustic_veil_ready: repeatingLayerCount >= 1 && blurPx >= 10,
	      covers_viewport: rect.width >= window.innerWidth - 1 && rect.height >= window.innerHeight - 1,
	      ...rect,
	    };
	  })();
	  const commandPaletteInputRow = document.querySelector("[data-control-ui-command-palette-surface] .command-palette__input-row");
	  const commandPaletteInputRowDetails = (() => {
	    if (!commandPaletteInputRow) {
	      return { exists: false, visible: false };
	    }
	    const style = getComputedStyle(commandPaletteInputRow);
	    const borderColor = parseCssColor(style.borderBottomColor);
	    const boxShadow = style.boxShadow || "";
	    const shadowCount = boxShadow && boxShadow !== "none" ? ((boxShadow.match(/rgb/g) || []).length || 1) : 0;
	    return {
	      exists: true,
	      visible: elementVisible(commandPaletteInputRow),
	      border_bottom_color: style.borderBottomColor,
	      border_bottom_alpha: borderColor ? Number(borderColor.a.toFixed(2)) : 0,
	      box_shadow: compactShadow(boxShadow),
	      box_shadow_sample: boxShadow.slice(0, 180),
	      command_palette_input_row_separator_shadow_count: shadowCount,
	      command_palette_input_row_prismatic_separator_ready: shadowCount >= 2 && !!borderColor && borderColor.a >= 0.25,
	      ...richRect(commandPaletteInputRow),
	    };
	  })();
	  const commandPaletteResultsWell = document.querySelector("[data-control-ui-command-palette-surface] .command-palette__results");
	  const commandPaletteResultsWellDetails = (() => {
	    if (!commandPaletteResultsWell) {
	      return { exists: false, visible: false };
	    }
	    const style = getComputedStyle(commandPaletteResultsWell);
	    const backdrop = style.backdropFilter || style.webkitBackdropFilter || "";
	    const blurPx = Number.parseFloat((backdrop.split("blur(")[1] || "").split("px")[0] || "0");
	    const bgColor = effectiveBackground(commandPaletteResultsWell);
	    const bgLuminance = relativeLuminance(bgColor);
	    const alpha = directBackgroundAlpha(style);
	    const borderColor = parseCssColor(style.borderTopColor);
	    const boxShadow = style.boxShadow || "";
	    const shadowCount = boxShadow && boxShadow !== "none" ? ((boxShadow.match(/rgb/g) || []).length || 1) : 0;
	    const radius = Number.parseFloat(style.borderTopLeftRadius || "0");
	    return {
	      exists: true,
	      visible: elementVisible(commandPaletteResultsWell),
	      background_color: style.backgroundColor,
	      background_alpha: Number(alpha.toFixed(2)),
	      effective_background: "rgb(" + Math.round(bgColor.r) + ", " + Math.round(bgColor.g) + ", " + Math.round(bgColor.b) + ")",
	      effective_luminance: Number(bgLuminance.toFixed(3)),
	      light_glass_ready: bgLuminance >= 0.72 && bgLuminance <= 0.98,
	      border_color: style.borderTopColor,
	      border_alpha: borderColor ? Number(borderColor.a.toFixed(2)) : 0,
	      border_radius: radius,
	      box_shadow: compactShadow(boxShadow),
	      box_shadow_sample: boxShadow.slice(0, 180),
	      backdrop_filter: backdrop,
	      backdrop_blur_px: Number((Number.isFinite(blurPx) ? blurPx : 0).toFixed(2)),
	      command_palette_results_well_rim_shadow_count: shadowCount,
	      command_palette_results_well_light_glass_ready: alpha >= 0.1 && alpha <= 0.4 && blurPx >= 10,
	      command_palette_results_well_prismatic_rim_ready: shadowCount >= 2 && !!borderColor && borderColor.a >= 0.25 && radius >= 12,
	      ...richRect(commandPaletteResultsWell),
	    };
	  })();
	  const commandPaletteClose = document.querySelector("[data-control-ui-command-palette-close]");
	  const commandPaletteCloseDetails = (() => {
	    if (!commandPaletteClose) {
	      return { exists: false, visible: false };
	    }
	    const style = getComputedStyle(commandPaletteClose);
	    const backdrop = style.backdropFilter || style.webkitBackdropFilter || "";
	    const textColor = parseCssColor(style.color);
	    const bgColor = effectiveBackground(commandPaletteClose);
	    const bgLuminance = relativeLuminance(bgColor);
	    const ratio = textColor ? contrastRatio(textColor, bgColor) : 0;
	    const filter = style.filter || "";
	    const dropShadowCount = (filter.match(/drop-shadow/g) || []).length;
	    const ariaLabel = commandPaletteClose.getAttribute("aria-label") || "";
	    const title = commandPaletteClose.getAttribute("title") || "";
	    return {
	      exists: true,
	      marker: commandPaletteClose.getAttribute("data-control-ui-command-palette-close") || "",
	      href: commandPaletteClose.getAttribute("href") || "",
	      visible: elementVisible(commandPaletteClose),
	      aria_label: ariaLabel,
	      title,
	      title_matches_aria_label: title === ariaLabel,
	      svg_icon_present: hasSvgIcon(commandPaletteClose),
	      visible_icon_text: visibleText(commandPaletteClose),
	      visible_icon_text_absent: visibleText(commandPaletteClose).length === 0,
	      background_color: style.backgroundColor,
	      background_alpha: Number(directBackgroundAlpha(style).toFixed(2)),
	      translucent_ready: translucentGlassReady(style),
	      effective_background: "rgb(" + Math.round(bgColor.r) + ", " + Math.round(bgColor.g) + ", " + Math.round(bgColor.b) + ")",
	      effective_luminance: Number(bgLuminance.toFixed(3)),
	      light_glass_ready: bgLuminance >= 0.72 && bgLuminance <= 0.98,
	      border_radius: Number.parseFloat(style.borderTopLeftRadius || "0"),
	      backdrop_filter: backdrop,
	      box_shadow: compactShadow(style.boxShadow),
	      filter: filter && filter !== "none" ? "present" : "none",
	      filter_sample: filter.slice(0, 180),
	      command_palette_close_drop_shadow_count: dropShadowCount,
	      command_palette_close_prismatic_icon_ready: dropShadowCount >= 2,
	      contrast_ratio: Number(ratio.toFixed(2)),
	      readable: ratio >= 4.5,
	      ...richRect(commandPaletteClose),
	    };
	  })();
	  const commandPaletteInput = document.querySelector("[data-control-ui-command-palette-input]");
	  const commandPaletteInputDetails = (() => {
	    if (!commandPaletteInput) {
	      return { exists: false, visible: false };
	    }
	    const style = getComputedStyle(commandPaletteInput);
	    const backdrop = style.backdropFilter || style.webkitBackdropFilter || "";
	    const textColor = parseCssColor(style.color);
	    const bgColor = effectiveBackground(commandPaletteInput);
	    const bgLuminance = relativeLuminance(bgColor);
	    const ratio = textColor ? contrastRatio(textColor, bgColor) : 0;
	    const ariaLabel = commandPaletteInput.getAttribute("aria-label") || "";
	    const title = commandPaletteInput.getAttribute("title") || "";
	    const marker = commandPaletteInput.getAttribute("data-control-ui-command-palette-input") || "";
	    const textShadow = style.textShadow || "";
	    const textShadowCount = textShadow && textShadow !== "none" ? ((textShadow.match(/rgb/g) || []).length || 1) : 0;
	    const placeholderStyle = getComputedStyle(commandPaletteInput, "::placeholder");
	    const placeholderColor = parseCssColor(placeholderStyle.color);
	    const placeholderRatio = placeholderColor ? contrastRatio(placeholderColor, bgColor) : 0;
	    const placeholderTextShadow = placeholderStyle.textShadow || "";
	    const placeholderTextShadowCount = placeholderTextShadow && placeholderTextShadow !== "none" ? ((placeholderTextShadow.match(/rgb/g) || []).length || 1) : 0;
	    const placeholderFontWeight = Number.parseFloat(placeholderStyle.fontWeight || "0") || 0;
	    return {
	      exists: true,
	      visible: elementVisible(commandPaletteInput),
	      marker,
	      aria_label: ariaLabel,
	      title,
	      title_matches_aria_label: title === ariaLabel,
	      type: commandPaletteInput.getAttribute("type") || "",
	      placeholder: commandPaletteInput.getAttribute("placeholder") || "",
	      background_color: style.backgroundColor,
	      background_alpha: Number(directBackgroundAlpha(style).toFixed(2)),
	      translucent_ready: translucentGlassReady(style),
	      effective_background: "rgb(" + Math.round(bgColor.r) + ", " + Math.round(bgColor.g) + ", " + Math.round(bgColor.b) + ")",
	      effective_luminance: Number(bgLuminance.toFixed(3)),
	      light_glass_ready: marker === "light-glass" && bgLuminance >= 0.72 && bgLuminance <= 0.98,
	      border_radius: Number.parseFloat(style.borderTopLeftRadius || "0"),
	      backdrop_filter: backdrop,
	      box_shadow: compactShadow(style.boxShadow),
	      text_shadow: textShadow && textShadow !== "none" ? "present" : "none",
	      text_shadow_sample: textShadow.slice(0, 180),
	      command_palette_input_text_shadow_count: textShadowCount,
	      command_palette_input_prismatic_etch_ready: textShadowCount >= 2,
	      placeholder_color: placeholderStyle.color,
	      placeholder_text_shadow: placeholderTextShadow && placeholderTextShadow !== "none" ? "present" : "none",
	      placeholder_text_shadow_sample: placeholderTextShadow.slice(0, 180),
	      command_palette_input_placeholder_text_shadow_count: placeholderTextShadowCount,
	      command_palette_input_placeholder_font_weight: placeholderFontWeight,
	      command_palette_input_placeholder_prismatic_etch_ready: placeholderTextShadowCount >= 2 && placeholderFontWeight >= 600,
	      placeholder_contrast_ratio: Number(placeholderRatio.toFixed(2)),
	      placeholder_readable: placeholderRatio >= 4.5,
	      contrast_ratio: Number(ratio.toFixed(2)),
	      readable: ratio >= 4.5,
	      ...richRect(commandPaletteInput),
	    };
	  })();
	  const commandPaletteInputIcon = document.querySelector(".command-palette__input-row > span");
	  const commandPaletteInputIconDetails = (() => {
	    if (!commandPaletteInputIcon) {
	      return { exists: false, visible: false };
	    }
	    const style = getComputedStyle(commandPaletteInputIcon);
	    const backdrop = style.backdropFilter || style.webkitBackdropFilter || "";
	    const filter = style.filter || "";
	    const dropShadowCount = (filter.match(/drop-shadow/g) || []).length;
	    const textColor = parseCssColor(style.color);
	    const bgColor = effectiveBackground(commandPaletteInputIcon);
	    const bgLuminance = relativeLuminance(bgColor);
	    const ratio = textColor ? contrastRatio(textColor, bgColor) : 0;
	    return {
	      exists: true,
	      visible: elementVisible(commandPaletteInputIcon),
	      svg_icon_present: hasSvgIcon(commandPaletteInputIcon),
	      visible_icon_text: visibleText(commandPaletteInputIcon),
	      visible_icon_text_absent: visibleText(commandPaletteInputIcon).length === 0,
	      background_color: style.backgroundColor,
	      background_alpha: Number(directBackgroundAlpha(style).toFixed(2)),
	      translucent_ready: translucentGlassReady(style),
	      effective_background: "rgb(" + Math.round(bgColor.r) + ", " + Math.round(bgColor.g) + ", " + Math.round(bgColor.b) + ")",
	      effective_luminance: Number(bgLuminance.toFixed(3)),
	      light_glass_ready: bgLuminance >= 0.72 && bgLuminance <= 0.98,
	      border_radius: Number.parseFloat(style.borderTopLeftRadius || "0"),
	      backdrop_filter: backdrop,
	      box_shadow: compactShadow(style.boxShadow),
	      filter: filter && filter !== "none" ? "present" : "none",
	      filter_sample: filter.slice(0, 180),
	      command_palette_input_icon_drop_shadow_count: dropShadowCount,
	      command_palette_input_icon_prismatic_ready: dropShadowCount >= 2,
	      contrast_ratio: Number(ratio.toFixed(2)),
	      readable: ratio >= 4.5,
	      ...richRect(commandPaletteInputIcon),
	    };
	  })();
	  const commandPaletteItemDetails = Array.from(document.querySelectorAll("[data-control-ui-command-palette-item]")).map((node) => {
	    const style = getComputedStyle(node);
	    const backdrop = style.backdropFilter || style.webkitBackdropFilter || "";
	    const borderColor = parseCssColor(style.borderTopColor);
	    const itemShadowCount = style.boxShadow && style.boxShadow !== "none" ? ((style.boxShadow.match(/rgb/g) || []).length || 1) : 0;
	    const kindNode = node.querySelector(".command-palette__kind");
	    const strong = node.querySelector("strong");
	    const detailNode = node.querySelector("small");
	    const kindStyle = kindNode ? getComputedStyle(kindNode) : style;
	    const strongStyle = strong ? getComputedStyle(strong) : style;
	    const detailStyle = detailNode ? getComputedStyle(detailNode) : style;
	    const kindColor = parseCssColor(kindStyle.color);
	    const textColor = parseCssColor(strongStyle.color);
	    const detailColor = parseCssColor(detailStyle.color);
	    const bgColor = effectiveBackground(node);
	    const kindBgColor = kindNode ? effectiveBackground(kindNode) : bgColor;
	    const bgLuminance = relativeLuminance(bgColor);
	    const kindBgLuminance = relativeLuminance(kindBgColor);
	    const kindRatio = kindColor ? contrastRatio(kindColor, kindBgColor) : 0;
	    const ratio = textColor ? contrastRatio(textColor, bgColor) : 0;
	    const detailRatio = detailColor ? contrastRatio(detailColor, bgColor) : 0;
	    const kindBackdrop = kindStyle.backdropFilter || kindStyle.webkitBackdropFilter || "";
	    const kindBorderColor = parseCssColor(kindStyle.borderTopColor);
	    const kindShadow = kindStyle.boxShadow || "";
	    const kindShadowCount = kindShadow && kindShadow !== "none" ? ((kindShadow.match(/rgb/g) || []).length || 1) : 0;
	    const kindRect = kindNode ? richRect(kindNode) : { width: 0, height: 0 };
	    const kindTextShadow = kindStyle.textShadow || "";
	    const labelTextShadow = strongStyle.textShadow || "";
	    const detailTextShadow = detailStyle.textShadow || "";
	    const kindTextShadowCount = kindTextShadow && kindTextShadow !== "none" ? ((kindTextShadow.match(/rgb/g) || []).length || 1) : 0;
	    const labelTextShadowCount = labelTextShadow && labelTextShadow !== "none" ? ((labelTextShadow.match(/rgb/g) || []).length || 1) : 0;
	    const detailTextShadowCount = detailTextShadow && detailTextShadow !== "none" ? ((detailTextShadow.match(/rgb/g) || []).length || 1) : 0;
	    const ariaLabel = node.getAttribute("aria-label") || "";
	    const title = node.getAttribute("title") || "";
	    return {
	      key: node.getAttribute("data-control-ui-command-palette-item") || "",
	      marker: node.getAttribute("data-control-ui-command-palette-result") || "",
	      kind: visibleText(kindNode),
	      label: visibleText(strong),
	      detail: visibleText(detailNode),
	      text: visibleText(node),
	      aria_label: ariaLabel,
	      title,
	      title_matches_aria_label: title === ariaLabel,
	      visible: elementVisible(node),
	      audit_hover: node.classList.contains("command-palette__item--audit-hover"),
	      background_color: style.backgroundColor,
	      background_alpha: Number(directBackgroundAlpha(style).toFixed(2)),
	      translucent_ready: translucentGlassReady(style),
	      effective_background: "rgb(" + Math.round(bgColor.r) + ", " + Math.round(bgColor.g) + ", " + Math.round(bgColor.b) + ")",
	      effective_luminance: Number(bgLuminance.toFixed(3)),
	      light_glass_ready: bgLuminance >= 0.72 && bgLuminance <= 0.98,
	      border_radius: Number.parseFloat(style.borderTopLeftRadius || "0"),
	      border_color: style.borderTopColor,
	      border_alpha: borderColor ? Number(borderColor.a.toFixed(2)) : 0,
	      backdrop_filter: backdrop,
	      box_shadow: compactShadow(style.boxShadow),
	      command_palette_item_hover_shadow_count: itemShadowCount,
	      command_palette_item_hover_prismatic_ready: node.classList.contains("command-palette__item--audit-hover") && itemShadowCount >= 2 && !!borderColor && borderColor.a >= 0.25,
	      command_palette_item_rim_shadow_count: itemShadowCount,
	      command_palette_item_prismatic_rim_ready: itemShadowCount >= 2 && !!borderColor && borderColor.a >= 0.25,
	      kind_background_color: kindStyle.backgroundColor,
	      kind_background_alpha: Number(directBackgroundAlpha(kindStyle).toFixed(2)),
	      kind_effective_luminance: Number(kindBgLuminance.toFixed(3)),
	      kind_border_alpha: kindBorderColor ? Number(kindBorderColor.a.toFixed(2)) : 0,
	      kind_border_radius: Number.parseFloat(kindStyle.borderTopLeftRadius || "0"),
	      kind_width: kindRect.width || 0,
	      kind_height: kindRect.height || 0,
	      kind_backdrop_filter: kindBackdrop,
	      kind_box_shadow: compactShadow(kindShadow),
	      command_palette_kind_chip_shadow_count: kindShadowCount,
	      command_palette_kind_chip_light_glass_ready: Boolean(kindNode) && kindRect.width >= 44 && kindRect.height >= 22 && directBackgroundAlpha(kindStyle) >= 0.25 && directBackgroundAlpha(kindStyle) <= 0.75 && kindBgLuminance >= 0.72 && kindBgLuminance <= 0.98 && kindShadowCount >= 2 && !!kindBorderColor && kindBorderColor.a >= 0.25 && Number.parseFloat(kindStyle.borderTopLeftRadius || "0") >= 20 && kindBackdrop.includes("blur("),
	      kind_text_shadow: kindTextShadow && kindTextShadow !== "none" ? "present" : "none",
	      kind_text_shadow_sample: kindTextShadow.slice(0, 180),
	      label_text_shadow: labelTextShadow && labelTextShadow !== "none" ? "present" : "none",
	      label_text_shadow_sample: labelTextShadow.slice(0, 180),
	      detail_text_shadow: detailTextShadow && detailTextShadow !== "none" ? "present" : "none",
	      detail_text_shadow_sample: detailTextShadow.slice(0, 180),
	      command_palette_item_kind_text_shadow_count: kindTextShadowCount,
	      command_palette_item_label_text_shadow_count: labelTextShadowCount,
	      command_palette_item_detail_text_shadow_count: detailTextShadowCount,
	      command_palette_item_label_prismatic_etch_ready: kindTextShadowCount >= 2 && labelTextShadowCount >= 2 && detailTextShadowCount >= 2,
	      kind_contrast_ratio: Number(kindRatio.toFixed(2)),
	      kind_readable: kindRatio >= 4.5,
	      contrast_ratio: Number(ratio.toFixed(2)),
	      readable: ratio >= 4.5,
	      detail_contrast_ratio: Number(detailRatio.toFixed(2)),
	      detail_readable: detailRatio >= 4.5,
	      ...richRect(node),
	    };
	  });
	  const commandPaletteTriggerDetails = Array.from(document.querySelectorAll("[data-control-ui-command-palette-trigger]")).map((node) => {
	    const style = getComputedStyle(node);
	    const backdrop = style.backdropFilter || style.webkitBackdropFilter || "";
	    const filterText = style.filter || "";
	    const dropShadowCount = (filterText.match(/drop-shadow/g) || []).length;
	    const textColor = parseCssColor(style.color);
	    const bgColor = effectiveBackground(node);
	    const bgLuminance = relativeLuminance(bgColor);
	    const ratio = textColor ? contrastRatio(textColor, bgColor) : 0;
	    const ariaLabel = node.getAttribute("aria-label") || "";
	    const title = node.getAttribute("title") || "";
	    const visibleTextValue = visibleText(node);
	    return {
	      marker: node.getAttribute("data-control-ui-command-palette-trigger") || "",
	      href: node.getAttribute("href") || "",
	      aria_label: ariaLabel,
	      title,
	      title_matches_aria_label: title === ariaLabel,
	      svg_icon_present: hasSvgIcon(node),
	      visible_icon_text: visibleTextValue,
	      visible_icon_text_absent: visibleTextValue.length === 0,
	      visible: elementVisible(node),
	      background_color: style.backgroundColor,
	      background_alpha: Number(directBackgroundAlpha(style).toFixed(2)),
	      translucent_ready: translucentGlassReady(style),
	      effective_background: "rgb(" + Math.round(bgColor.r) + ", " + Math.round(bgColor.g) + ", " + Math.round(bgColor.b) + ")",
	      effective_luminance: Number(bgLuminance.toFixed(3)),
	      light_glass_ready: bgLuminance >= 0.72 && bgLuminance <= 0.98,
	      border_radius: Number.parseFloat(style.borderTopLeftRadius || "0"),
	      backdrop_filter: backdrop,
	      box_shadow: compactShadow(style.boxShadow),
	      filter: filterText && filterText !== "none" ? "present" : "none",
	      filter_sample: filterText.slice(0, 180),
	      icon_prismatic_control_drop_shadow_count: dropShadowCount,
	      icon_prismatic_control_ready: dropShadowCount >= 2,
	      contrast_ratio: Number(ratio.toFixed(2)),
	      readable: ratio >= 4.5,
	      ...richRect(node),
	    };
	  });
	  const visibleCommandPaletteTriggerDetails = commandPaletteTriggerDetails.filter((item) => item.visible);
	  const commandPaletteTriggerLightGlassReady = visibleCommandPaletteTriggerDetails.length === 1
	    && visibleCommandPaletteTriggerDetails.every((item) => (
	      item.marker === "light-glass"
	      && item.href === "#command-palette"
	      && item.width >= 44
	      && item.height >= 44
	      && item.border_radius >= 20
	      && item.light_glass_ready === true
	      && item.translucent_ready === true
	      && item.effective_luminance >= 0.72
	      && item.effective_luminance <= 0.98
	      && (item.backdrop_filter || "").includes("blur(")
	      && item.box_shadow !== "none"
	      && item.aria_label.length > 0
	      && item.title.length > 0
	      && item.title_matches_aria_label === true
	      && item.svg_icon_present === true
	      && item.visible_icon_text_absent === true
	      && item.readable === true
	      && item.contrast_ratio >= 4.5
	    ));
	  const iconPrismaticControlDetails = [
	    ...iconButtonDetails.map((item) => ({ group: "icon-button", ...item })),
	    ...menuTriggerDetails.map((item) => ({ group: "menu-trigger", ...item })),
	    ...visibleCommandPaletteTriggerDetails.map((item) => ({ group: "command-palette-trigger", ...item })),
	  ];
	  const iconPrismaticControlLightGlassReady = iconButtonReady
	    && menuTriggerReady
	    && commandPaletteTriggerLightGlassReady
	    && iconPrismaticControlDetails.length >= (railVisible ? 8 : 7)
	    && iconPrismaticControlDetails.every((item) => (
	      item.visible === true
	      && item.width >= 44
	      && item.height >= 44
	      && item.border_radius >= 20
	      && (item.backdrop_filter || "").includes("blur(")
	      && item.box_shadow !== "none"
	      && item.icon_prismatic_control_ready === true
	      && item.icon_prismatic_control_drop_shadow_count >= 2
	    ));
	  const commandPaletteReady = commandPalettePanelDetails.exists === true
	    && commandPalettePanelDetails.visible === true
	    && commandPalettePanelDetails.marker === "light-glass"
	    && commandPalettePanelDetails.role === "dialog"
	    && commandPalettePanelDetails.aria_modal === "true"
	    && commandPalettePanelDetails.aria_label === "Command palette"
	    && commandPalettePanelDetails.light_glass_ready === true
	    && commandPalettePanelDetails.translucent_ready === true
	    && commandPalettePanelDetails.effective_luminance >= 0.72
	    && commandPalettePanelDetails.effective_luminance <= 0.98
	    && commandPalettePanelDetails.border_radius >= 18
	    && /blur\\(/.test(commandPalettePanelDetails.backdrop_filter || "")
	    && commandPalettePanelDetails.box_shadow !== "none"
	    && commandPalettePanelDetails.in_viewport === true
	    && commandPaletteCloseDetails.exists === true
	    && commandPaletteCloseDetails.visible === true
	    && commandPaletteCloseDetails.width >= 44
	    && commandPaletteCloseDetails.height >= 44
	    && commandPaletteCloseDetails.title_matches_aria_label === true
	    && commandPaletteCloseDetails.svg_icon_present === true
	    && commandPaletteCloseDetails.visible_icon_text_absent === true
	    && /blur\\(/.test(commandPaletteCloseDetails.backdrop_filter || "")
	    && commandPaletteCloseDetails.box_shadow !== "none"
	    && commandPaletteInputDetails.exists === true
	    && commandPaletteInputDetails.visible === true
	    && commandPaletteInputDetails.height >= 44
	    && commandPaletteInputDetails.title_matches_aria_label === true
	    && commandPaletteInputDetails.readable === true
	    && commandPaletteItemDetails.length >= 1
	    && commandPaletteItemDetails.every((item) => (
	      item.visible
	      && item.width >= 180
	      && item.height >= 44
	      && item.text.length > 0
	      && item.aria_label.length > 0
	      && item.title.length > 0
	      && item.title_matches_aria_label
	      && item.readable
	      && item.contrast_ratio >= 4.5
	    ));
	  const commandPaletteSurfaceLightGlassReady = commandPalettePanelDetails.exists === true
	    && commandPalettePanelDetails.visible === true
	    && commandPalettePanelDetails.marker === "light-glass"
	    && commandPalettePanelDetails.role === "dialog"
	    && commandPalettePanelDetails.aria_modal === "true"
	    && commandPalettePanelDetails.aria_label === "Command palette"
	    && commandPalettePanelDetails.width >= 274
	    && commandPalettePanelDetails.height >= 132
	    && commandPalettePanelDetails.border_radius >= 18
	    && commandPalettePanelDetails.light_glass_ready === true
	    && commandPalettePanelDetails.translucent_ready === true
	    && commandPalettePanelDetails.effective_luminance >= 0.72
	    && commandPalettePanelDetails.effective_luminance <= 0.98
	    && /blur\\(/.test(commandPalettePanelDetails.backdrop_filter || "")
	    && commandPalettePanelDetails.box_shadow !== "none"
	    && commandPalettePanelDetails.in_viewport === true;
	  const commandPaletteSurfacePrismaticPerimeterLightGlassReady = commandPaletteSurfaceLightGlassReady
	    && commandPalettePanelDetails.filter === "present"
	    && commandPalettePanelDetails.command_palette_surface_prismatic_perimeter_ready === true
	    && commandPalettePanelDetails.command_palette_surface_drop_shadow_count >= 2;
	  const commandPaletteBackdropCausticVeilLightGlassReady = commandPaletteBackdropDetails.exists === true
	    && commandPaletteBackdropDetails.visible === true
	    && commandPaletteBackdropDetails.background_alpha >= 0.2
	    && commandPaletteBackdropDetails.background_alpha <= 0.6
	    && commandPaletteBackdropDetails.background_image === "present"
	    && commandPaletteBackdropDetails.command_palette_backdrop_caustic_veil_ready === true
	    && commandPaletteBackdropDetails.command_palette_backdrop_repeating_layer_count >= 1
	    && commandPaletteBackdropDetails.backdrop_blur_px >= 10
	    && commandPaletteBackdropDetails.covers_viewport === true;
	  const commandPaletteInputRowPrismaticSeparatorLightGlassReady = commandPaletteInputRowDetails.exists === true
	    && commandPaletteInputRowDetails.visible === true
	    && commandPaletteInputRowDetails.width >= 274
	    && commandPaletteInputRowDetails.height >= 60
	    && commandPaletteInputRowDetails.border_bottom_alpha >= 0.25
	    && commandPaletteInputRowDetails.box_shadow !== "none"
	    && commandPaletteInputRowDetails.command_palette_input_row_separator_shadow_count >= 2
	    && commandPaletteInputRowDetails.command_palette_input_row_prismatic_separator_ready === true;
	  const commandPaletteResultsWellLightGlassReady = commandPaletteResultsWellDetails.exists === true
	    && commandPaletteResultsWellDetails.visible === true
	    && commandPaletteResultsWellDetails.width >= 274
	    && commandPaletteResultsWellDetails.height >= 58
	    && commandPaletteResultsWellDetails.background_alpha >= 0.1
	    && commandPaletteResultsWellDetails.background_alpha <= 0.4
	    && commandPaletteResultsWellDetails.light_glass_ready === true
	    && String(commandPaletteResultsWellDetails.backdrop_filter || "").includes("blur(")
	    && commandPaletteResultsWellDetails.backdrop_blur_px >= 10
	    && commandPaletteResultsWellDetails.command_palette_results_well_light_glass_ready === true;
	  const commandPaletteResultsWellPrismaticRimLightGlassReady = commandPaletteResultsWellLightGlassReady
	    && commandPaletteResultsWellDetails.border_alpha >= 0.25
	    && commandPaletteResultsWellDetails.border_radius >= 12
	    && commandPaletteResultsWellDetails.box_shadow !== "none"
	    && commandPaletteResultsWellDetails.command_palette_results_well_rim_shadow_count >= 2
	    && commandPaletteResultsWellDetails.command_palette_results_well_prismatic_rim_ready === true;
	  const commandPaletteCloseLightGlassReady = commandPaletteCloseDetails.exists === true
	    && commandPaletteCloseDetails.visible === true
	    && commandPaletteCloseDetails.marker === "light-glass"
	    && commandPaletteCloseDetails.href === "#commands"
	    && commandPaletteCloseDetails.width >= 44
	    && commandPaletteCloseDetails.height >= 44
	    && commandPaletteCloseDetails.border_radius >= 20
	    && commandPaletteCloseDetails.light_glass_ready === true
	    && commandPaletteCloseDetails.translucent_ready === true
	    && commandPaletteCloseDetails.effective_luminance >= 0.72
	    && commandPaletteCloseDetails.effective_luminance <= 0.98
	    && /blur\\(/.test(commandPaletteCloseDetails.backdrop_filter || "")
	    && commandPaletteCloseDetails.box_shadow !== "none"
	    && commandPaletteCloseDetails.aria_label === "Close command palette"
	    && commandPaletteCloseDetails.title_matches_aria_label === true
	    && commandPaletteCloseDetails.svg_icon_present === true
	    && commandPaletteCloseDetails.visible_icon_text_absent === true
	    && commandPaletteCloseDetails.readable === true
	    && commandPaletteCloseDetails.contrast_ratio >= 4.5;
	  const commandPaletteClosePrismaticIconLightGlassReady = commandPaletteCloseLightGlassReady
	    && commandPaletteCloseDetails.filter === "present"
	    && commandPaletteCloseDetails.command_palette_close_prismatic_icon_ready === true
	    && commandPaletteCloseDetails.command_palette_close_drop_shadow_count >= 2;
	  const commandPaletteItemLightGlassReady = commandPaletteItemDetails.length >= 1
	    && commandPaletteItemDetails.every((item) => (
	      item.visible
	      && item.marker === "light-glass"
	      && item.key.length > 0
	      && item.kind.length > 0
	      && item.label.length > 0
	      && item.detail.length > 0
	      && item.width >= 180
	      && item.height >= 44
	      && item.border_radius >= 8
	      && item.light_glass_ready === true
	      && item.translucent_ready === true
	      && item.effective_luminance >= 0.72
	      && item.effective_luminance <= 0.98
	      && /blur\\(/.test(item.backdrop_filter || "")
	      && item.box_shadow !== "none"
	      && item.aria_label.length > 0
	      && item.title.length > 0
	      && item.title_matches_aria_label === true
	      && item.readable === true
	      && item.contrast_ratio >= 4.5
	    ));
	  const commandPaletteItemPrismaticRimLightGlassReady = commandPaletteItemLightGlassReady
	    && commandPaletteItemDetails.every((item) => (
	      item.border_alpha >= 0.25
	      && item.box_shadow !== "none"
	      && item.command_palette_item_rim_shadow_count >= 2
	      && item.command_palette_item_prismatic_rim_ready === true
	    ));
	  const commandPaletteKindChipLightGlassReady = commandPaletteItemLightGlassReady
	    && commandPaletteItemDetails.every((item) => (
	      item.kind_width >= 44
	      && item.kind_height >= 22
	      && item.kind_background_alpha >= 0.25
	      && item.kind_background_alpha <= 0.75
	      && item.kind_effective_luminance >= 0.72
	      && item.kind_effective_luminance <= 0.98
	      && item.kind_border_alpha >= 0.25
	      && item.kind_border_radius >= 20
	      && (item.kind_backdrop_filter || "").includes("blur(")
	      && item.kind_box_shadow !== "none"
	      && item.command_palette_kind_chip_shadow_count >= 2
	      && item.command_palette_kind_chip_light_glass_ready === true
	      && item.kind_readable === true
	      && item.kind_contrast_ratio >= 4.5
	    ));
	  const commandPaletteItemHoverDetails = commandPaletteItemDetails.filter((item) => item.audit_hover === true);
	  const commandPaletteItemHoverPrismaticLightGlassReady = commandPaletteItemLightGlassReady
	    && commandPaletteItemHoverDetails.length >= 1
	    && commandPaletteItemHoverDetails.every((item) => (
	      item.command_palette_item_hover_prismatic_ready === true
	      && item.command_palette_item_hover_shadow_count >= 2
	      && item.border_alpha >= 0.25
	      && item.box_shadow !== "none"
	    ));
	  const commandPaletteItemLabelPrismaticEtchLightGlassReady = commandPaletteItemLightGlassReady
	    && commandPaletteItemDetails.every((item) => (
	      item.kind_text_shadow === "present"
	      && item.label_text_shadow === "present"
	      && item.detail_text_shadow === "present"
	      && item.command_palette_item_label_prismatic_etch_ready === true
	      && item.command_palette_item_kind_text_shadow_count >= 2
	      && item.command_palette_item_label_text_shadow_count >= 2
	      && item.command_palette_item_detail_text_shadow_count >= 2
	      && item.kind_readable
	      && item.readable
	      && item.detail_readable
	      && item.kind_contrast_ratio >= 4.5
	      && item.contrast_ratio >= 4.5
	      && item.detail_contrast_ratio >= 4.5
	    ));
	  const commandPaletteInputLightGlassReady = commandPaletteInputDetails.exists === true
	    && commandPaletteInputDetails.visible === true
	    && commandPaletteInputDetails.marker === "light-glass"
	    && commandPaletteInputDetails.type === "search"
	    && commandPaletteInputDetails.placeholder.length > 0
	    && commandPaletteInputDetails.height >= 44
	    && commandPaletteInputDetails.border_radius >= 10
	    && commandPaletteInputDetails.light_glass_ready === true
	    && commandPaletteInputDetails.translucent_ready === true
	    && commandPaletteInputDetails.effective_luminance >= 0.72
	    && commandPaletteInputDetails.effective_luminance <= 0.98
	    && /blur\\(/.test(commandPaletteInputDetails.backdrop_filter || "")
	    && commandPaletteInputDetails.box_shadow !== "none"
	    && commandPaletteInputDetails.aria_label.length > 0
	    && commandPaletteInputDetails.title.length > 0
	    && commandPaletteInputDetails.title_matches_aria_label === true
	    && commandPaletteInputDetails.readable === true
	    && commandPaletteInputDetails.contrast_ratio >= 4.5;
	  const commandPaletteInputTextPrismaticEtchLightGlassReady = commandPaletteInputLightGlassReady
	    && commandPaletteInputDetails.text_shadow === "present"
	    && commandPaletteInputDetails.command_palette_input_prismatic_etch_ready === true
	    && commandPaletteInputDetails.command_palette_input_text_shadow_count >= 2
	    && commandPaletteInputDetails.readable === true
	    && commandPaletteInputDetails.contrast_ratio >= 4.5;
	  const commandPaletteInputPlaceholderPrismaticEtchLightGlassReady = commandPaletteInputLightGlassReady
	    && commandPaletteInputDetails.placeholder.length > 0
	    && commandPaletteInputDetails.placeholder_text_shadow === "present"
	    && commandPaletteInputDetails.command_palette_input_placeholder_prismatic_etch_ready === true
	    && commandPaletteInputDetails.command_palette_input_placeholder_text_shadow_count >= 2
	    && commandPaletteInputDetails.command_palette_input_placeholder_font_weight >= 600
	    && commandPaletteInputDetails.placeholder_readable === true
	    && commandPaletteInputDetails.placeholder_contrast_ratio >= 4.5;
	  const commandPaletteInputIconLightGlassReady = commandPaletteInputIconDetails.exists === true
	    && commandPaletteInputIconDetails.visible === true
	    && commandPaletteInputIconDetails.width >= 44
	    && commandPaletteInputIconDetails.height >= 44
	    && commandPaletteInputIconDetails.border_radius >= 20
	    && commandPaletteInputIconDetails.light_glass_ready === true
	    && commandPaletteInputIconDetails.translucent_ready === true
	    && commandPaletteInputIconDetails.effective_luminance >= 0.72
	    && commandPaletteInputIconDetails.effective_luminance <= 0.98
	    && /blur\\(/.test(commandPaletteInputIconDetails.backdrop_filter || "")
	    && commandPaletteInputIconDetails.box_shadow !== "none"
	    && commandPaletteInputIconDetails.svg_icon_present === true
	    && commandPaletteInputIconDetails.visible_icon_text_absent === true
	    && commandPaletteInputIconDetails.readable === true
	    && commandPaletteInputIconDetails.contrast_ratio >= 4.5;
	  const commandPaletteInputIconPrismaticLightGlassReady = commandPaletteInputIconLightGlassReady
	    && commandPaletteInputIconDetails.filter === "present"
	    && commandPaletteInputIconDetails.command_palette_input_icon_prismatic_ready === true
	    && commandPaletteInputIconDetails.command_palette_input_icon_drop_shadow_count >= 2;
	  const controlFormControlDetails = Array.from(document.querySelectorAll("[data-chat-search],[data-chat-composer-input],[data-chat-routing-mode],[data-chat-autoscroll-mode]"))
	    .filter(elementVisible)
	    .map((node) => {
	      const style = getComputedStyle(node);
	      const textColor = parseCssColor(style.color);
	      const formBackgroundHost = node.matches("[data-control-ui-rail-search-input]") ? node : (node.closest(".tg-search-shell,.tg-compose-bar,.tg-menu-item,.command-palette") || node);
	      const bgColor = effectiveBackground(formBackgroundHost);
	      const ratio = textColor ? contrastRatio(textColor, bgColor) : 0;
	      const ariaLabel = node.getAttribute("aria-label") || "";
	      const title = node.getAttribute("title") || "";
	      return {
	        role: node.getAttribute("data-chat-search") !== null ? "chat-search"
	          : node.getAttribute("data-chat-composer-input") !== null ? "chat-composer-input"
	          : node.getAttribute("data-chat-routing-mode") !== null ? "chat-routing-mode"
	          : node.getAttribute("data-chat-autoscroll-mode") !== null ? "chat-autoscroll-mode"
	          : node.tagName.toLowerCase(),
	        tag: node.tagName.toLowerCase(),
	        aria_label: ariaLabel,
	        title,
	        title_matches_aria_label: title === ariaLabel,
	        placeholder: node.getAttribute("placeholder") || "",
	        color: style.color,
	        effective_background: "rgb(" + Math.round(bgColor.r) + ", " + Math.round(bgColor.g) + ", " + Math.round(bgColor.b) + ")",
	        contrast_ratio: Number(ratio.toFixed(2)),
	        readable: ratio >= 4.5,
	        ...richRect(node),
	      };
	    });
	  const expectedVisibleFormControlCount = railVisible ? 4 : 1;
		  const controlFormControlReady = controlFormControlDetails.length >= expectedVisibleFormControlCount
		    && controlFormControlDetails.every((item) => (
		      item.aria_label.length > 0
		      && item.title.length > 0
		      && item.title_matches_aria_label
		      && item.height >= 44
		      && item.readable
		      && item.contrast_ratio >= 4.5
		    ));
		  const chatRowOptionDetails = Array.from(document.querySelectorAll("[data-chat-conversation]"))
		    .filter(elementVisible)
		    .map((node) => {
		      const style = getComputedStyle(node);
		      const ariaLabel = node.getAttribute("aria-label") || "";
		      const title = node.getAttribute("title") || "";
		      const active = node.classList.contains("active");
		      const ariaCurrent = node.getAttribute("aria-current") || "";
		      const backdrop = style.backdropFilter || style.webkitBackdropFilter || "";
		      const filterText = style.filter || "";
		      const dropShadowCount = (filterText.match(/drop-shadow/g) || []).length;
		      return {
		        key: node.getAttribute("data-chat-conversation") || "",
		        role: node.getAttribute("role") || "",
		        aria_current: ariaCurrent,
		        aria_label: ariaLabel,
		        title,
		        title_matches_aria_label: title === ariaLabel,
		        tabindex: node.getAttribute("tabindex") || "",
		        active,
		        visible: elementVisible(node),
		        active_state_matches_aria_current: (active ? "true" : "") === ariaCurrent,
		        border_radius: styleNumber(style, "borderTopLeftRadius"),
		        box_shadow: compactShadow(style.boxShadow),
		        backdrop_filter: backdrop,
		        filter: filterText && filterText !== "none" ? "present" : "none",
		        filter_sample: filterText.slice(0, 180),
		        chat_row_drop_shadow_count: dropShadowCount,
		        chat_row_prismatic_slab_ready: dropShadowCount >= 2,
		        ...richRect(node),
		      };
		    });
		  const expectedVisibleChatRowOptionCount = railVisible ? 3 : 0;
		  const chatRowOptionSemanticTouchReady = chatRowOptionDetails.length === expectedVisibleChatRowOptionCount
		    && chatRowOptionDetails.every((item) => (
		      item.key.length > 0
		      && item.role === "listitem"
		      && item.width >= 44
		      && item.height >= 64
		      && item.aria_label.length > 0
		      && item.title.length > 0
		      && item.title_matches_aria_label
		      && item.tabindex === "0"
		      && item.active_state_matches_aria_current
		      && item.border_radius >= 18
		    ));
		  const railChatRowPrismaticSlabLightGlassReady = railVisible
		    ? (
		      chatRowOptionDetails.length >= 3
		      && chatRowOptionDetails.every((item) => (
		        item.visible !== false
		        && item.width >= 44
		        && item.height >= 64
		        && item.border_radius >= 18
		        && item.box_shadow !== "none"
		        && (item.backdrop_filter || "").includes("blur(")
		        && item.chat_row_prismatic_slab_ready === true
		        && item.chat_row_drop_shadow_count >= 2
		      ))
		    )
		    : chatRowOptionDetails.length === 0;

`;
