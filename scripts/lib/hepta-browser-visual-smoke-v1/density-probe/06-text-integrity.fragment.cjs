module.exports = `
  const microcopySelectors = [
    ".badge",
    ".tg-bubble p",
    ".tg-agent-reply-group",
    ".tg-agent-reply-card",
    ".tile",
    ".mini-card",
    ".row-card",
    ".timeline-item",
    ".empty-state",
    ".panel",
    ".card",
    ".tg-room-section",
    ".tg-room-panel__header",
    ".tg-thread-details__grid article",
    ".tg-menu-item__label",
    ".tg-chat-item__topline strong",
  ];
  const microcopyWrapDetails = microcopySelectors.flatMap((selector) =>
    Array.from(document.querySelectorAll(selector))
      .filter(elementVisible)
      .slice(0, 20)
      .map((node) => {
        const style = getComputedStyle(node);
        return {
          selector,
          text: visibleText(node).slice(0, 80),
          overflow_wrap: style.overflowWrap,
          word_break: style.wordBreak,
          white_space: style.whiteSpace,
          ...richRect(node),
        };
      })
  );
	  const microcopyWrapReady = microcopyWrapDetails.length >= 6
	    && microcopyWrapDetails.every((item) => (
	      item.overflow_wrap !== "anywhere"
	      && item.word_break !== "break-word"
	      && item.word_break !== "break-all"
	    ));
	  const logoClipDetails = Array.from(document.querySelectorAll('[data-hepta-agent-logo="true"]'))
	    .filter(elementVisible)
	    .map((node) => {
	      const img = node.querySelector("img");
	      const style = getComputedStyle(node);
	      const rect = richRect(node);
	      const imgRect = img ? richRect(img) : null;
	      return {
	        selector: node.className || node.tagName.toLowerCase(),
	        flex_shrink: style.flexShrink,
	        min_width: style.minWidth,
	        visible: elementVisible(node),
	        image_present: Boolean(img),
	        image_width: imgRect?.width || 0,
	        image_height: imgRect?.height || 0,
	        image_fills_container: Boolean(
	          imgRect
	          && imgRect.width >= Math.min(rect.width, rect.height) * 0.9
	          && imgRect.height >= Math.min(rect.width, rect.height) * 0.9
	        ),
	        ...rect,
	      };
	    });
		  const logoClipReady = logoClipDetails.length >= 1
		    && logoClipDetails.every((item) => (
		      item.visible
	      && item.image_present
	      && item.width >= 32
	      && item.height >= 32
		      && item.image_fills_container
		    ));
	  const avatarPrismaticRimDetails = Array.from(document.querySelectorAll(".tg-chat-item__avatar,.tg-thread-avatar"))
	    .filter(elementVisible)
	    .map((node) => {
	      const style = getComputedStyle(node);
	      const filterText = style.filter || "";
	      const dropShadowCount = (filterText.match(/drop-shadow/g) || []).length;
	      const img = node.querySelector("img");
	      return {
	        selector: node.className || node.tagName.toLowerCase(),
	        text: visibleText(node),
	        visible: true,
	        image_present: Boolean(img),
	        border_radius: Number.parseFloat(style.borderTopLeftRadius || "0"),
	        box_shadow: compactShadow(style.boxShadow),
	        filter: filterText && filterText !== "none" ? "present" : "none",
	        filter_sample: filterText.slice(0, 180),
	        avatar_rim_drop_shadow_count: dropShadowCount,
	        avatar_prismatic_rim_ready: dropShadowCount >= 2,
	        ...richRect(node),
	      };
	    });
	  const avatarPrismaticRimLightGlassReady = avatarPrismaticRimDetails.length >= (railVisible ? 4 : 1)
	    && avatarPrismaticRimDetails.every((item) => (
	      item.visible
	      && item.width >= 40
	      && item.height >= 40
	      && item.border_radius >= 16
	      && (item.image_present || item.text.length > 0)
	      && item.box_shadow !== "none"
	      && item.avatar_prismatic_rim_ready === true
	      && item.avatar_rim_drop_shadow_count >= 2
	    ));
		  const readabilityHost = (node) => node.matches("[data-control-ui-rail-search-input]") ? node : (node.closest(".tg-chat-item,.tg-thread-header,.tg-search-shell,.tg-compose-bar,.tg-thread-hepta-controls,.tg-folder-chip,.tg-menu-item,.tg-compose-footer") || node);
		  const readabilityDetail = (node) => {
		      const style = getComputedStyle(node);
		      const textColor = parseCssColor(style.color);
		      const bgColor = effectiveBackground(readabilityHost(node));
		      const ratio = textColor ? contrastRatio(textColor, bgColor) : 0;
		      return {
		        selector: node.tagName.toLowerCase() + (node.className ? "." + String(node.className).replace(/\\s+/g, ".") : ""),
		        text: visibleText(node).slice(0, 80),
		        color: style.color,
		        effective_background: "rgb(" + Math.round(bgColor.r) + ", " + Math.round(bgColor.g) + ", " + Math.round(bgColor.b) + ")",
		        contrast_ratio: Number(ratio.toFixed(2)),
		        readable: ratio >= 4.5,
		        ...richRect(node),
		      };
		    };
		  const placeholderReadabilityDetail = (node) => {
		    const style = getComputedStyle(node, "::placeholder");
		    const textColor = parseCssColor(style.color);
		    const bgColor = effectiveBackground(readabilityHost(node));
		    const ratio = textColor ? contrastRatio(textColor, bgColor) : 0;
		    return {
		      selector: node.tagName.toLowerCase() + "[placeholder]::placeholder",
		      text: (node.getAttribute("placeholder") || "").slice(0, 80),
		      color: style.color,
		      effective_background: "rgb(" + Math.round(bgColor.r) + ", " + Math.round(bgColor.g) + ", " + Math.round(bgColor.b) + ")",
		      contrast_ratio: Number(ratio.toFixed(2)),
		      readable: ratio >= 4.5,
		      ...richRect(node),
		    };
		  };
		  const chatRowReadabilityDetails = Array.from(document.querySelectorAll(".tg-chat-item :is(.tg-chat-item__topline strong,.tg-chat-item__topline span,.tg-chat-item__body p)"))
		    .filter(elementVisible)
		    .map(readabilityDetail);
		  const threadHeaderReadabilityDetails = Array.from(document.querySelectorAll(".tg-thread-header__main p"))
		    .filter(elementVisible)
		    .map(readabilityDetail);
		  const composeFooterReadabilityDetails = Array.from(document.querySelectorAll(".tg-compose-footer [data-chat-shortcut-hint]"))
		    .filter(elementVisible)
		    .map(readabilityDetail);
		  const messageMetaReadabilityDetails = Array.from(document.querySelectorAll(".tg-message small"))
		    .filter(elementVisible)
		    .map(readabilityDetail);
		  const placeholderReadabilityDetails = Array.from(document.querySelectorAll(".tg-search-shell input[placeholder],.tg-compose-bar textarea[placeholder]"))
		    .filter(elementVisible)
		    .map(placeholderReadabilityDetail);
		  const smallControlReadabilityDetails = Array.from(document.querySelectorAll(".tg-folder-chip,.tg-folder-chip small,.tg-thread-hepta-controls span,.tg-thread-hepta-controls select,.tg-autoscroll-select,.tg-autoscroll-select select,.tg-menu-item__label"))
		    .filter(elementVisible)
		    .map(readabilityDetail);
		  const activeChatReadabilityDetails = chatRowReadabilityDetails.concat(threadHeaderReadabilityDetails, composeFooterReadabilityDetails, messageMetaReadabilityDetails, placeholderReadabilityDetails, smallControlReadabilityDetails);
		  const placeholderReadabilityReady = placeholderReadabilityDetails.length >= 1 && placeholderReadabilityDetails.every((item) => item.readable);
		  const smallControlReadabilityReady = smallControlReadabilityDetails.every((item) => item.readable);
			  const activeChatReadabilityReady = activeChatReadabilityDetails.length >= 4
			    && placeholderReadabilityReady
		    && smallControlReadabilityReady
		    && activeChatReadabilityDetails.every((item) => item.readable);
		  const translucentGlassDetails = [
		    ...primaryShellSurfaceDetails.map((item) => ({ group: "primary-shell", ...item })),
		    ...visibleTopbarActionDetails.map((item) => ({ group: "topbar-action", ...item })),
		    ...visibleRailSearchDetails.map((item) => ({ group: "rail-search", ...item })),
		    ...microSurfaceDetails.map((item) => ({ group: "micro-surface", ...item })),
		    ...(commandPalettePanelDetails.exists ? [{ group: "command-palette-panel", ...commandPalettePanelDetails }] : []),
		    ...(commandPaletteCloseDetails.exists ? [{ group: "command-palette-close", ...commandPaletteCloseDetails }] : []),
		    ...(commandPaletteInputDetails.exists ? [{ group: "command-palette-input", ...commandPaletteInputDetails }] : []),
		    ...commandPaletteItemDetails.map((item) => ({ group: "command-palette-item", ...item })),
		    ...composerPopoverPanelDetails.map((item) => ({ group: "composer-popover-panel", ...item })),
		    ...composerPopoverSearchDetails.map((item) => ({ group: "composer-popover-search", ...item })),
		    ...composerPopoverItemDetails.map((item) => ({ group: "composer-popover-item", ...item })),
		  ].filter((item) => item.visible === true);
			  const translucentShellLightGlassReady = translucentGlassDetails.length >= 18
			    && translucentGlassDetails.every((item) => (
			      item.translucent_ready === true
			      && item.background_alpha >= 0.35
			      && item.background_alpha <= 0.88
			      && (item.backdrop_filter || "").includes("blur(")
			      && item.box_shadow !== "none"
			    ));
			  const bodyStyle = getComputedStyle(document.body);
			  const bodyBeforeStyle = getComputedStyle(document.body, "::before");
			  const bodyBackgroundImage = bodyStyle.backgroundImage || "";
			  const bodyBeforeBackgroundImage = bodyBeforeStyle.backgroundImage || "";
			  const bodyBeforeOpacity = Number.parseFloat(bodyBeforeStyle.opacity || "0");
			  const bodyBackgroundLayerCount = bodyBackgroundImage.split("gradient(").length - 1;
			  const bodyBackgroundRepeatingLayerCount = (bodyBackgroundImage.match(/repeating-linear-gradient/g) || []).length;
			  const bodyBackgroundAngles = Array.from(bodyBackgroundImage.matchAll(/(?:repeating-)?linear-gradient\\(([-\\d.]+)deg/g))
			    .map((match) => Number(match[1]))
			    .filter(Number.isFinite);
			  const bodyBackgroundAngleCount = new Set(bodyBackgroundAngles.map((angle) => Math.round(angle))).size;
			  const refractiveDepthDetails = {
			    body_background_image: bodyBackgroundImage && bodyBackgroundImage !== "none" ? "present" : "none",
			    body_background_translucent_layer: bodyBackgroundImage.includes("rgba("),
			    body_background_layer_count: bodyBackgroundLayerCount,
			    body_background_repeating_layer_count: bodyBackgroundRepeatingLayerCount,
			    body_background_angles: bodyBackgroundAngles.map((angle) => Math.round(angle)),
			    body_background_angle_count: bodyBackgroundAngleCount,
			    before_background_image: bodyBeforeBackgroundImage && bodyBeforeBackgroundImage !== "none" ? "present" : "none",
			    before_opacity: Number((Number.isFinite(bodyBeforeOpacity) ? bodyBeforeOpacity : 0).toFixed(2)),
			    primary_shell_gradient_count: primaryShellSurfaceDetails.filter((item) => item.background_image === "present").length,
			    primary_shell_low_alpha_count: primaryShellSurfaceDetails.filter((item) => item.background_alpha >= 0.38 && item.background_alpha <= 0.58).length,
			    primary_shell_clear_alpha_count: primaryShellSurfaceDetails.filter((item) => item.background_alpha >= 0.38 && item.background_alpha <= 0.49).length,
			  };
			  const primaryShellAlphaValues = primaryShellSurfaceDetails.map((item) => item.background_alpha).filter(Number.isFinite);
			  const primaryShellAlphaMax = primaryShellAlphaValues.length > 0 ? Math.max(...primaryShellAlphaValues) : 1;
			  const primaryShellAlphaMin = primaryShellAlphaValues.length > 0 ? Math.min(...primaryShellAlphaValues) : 1;
			  const primaryShellAlphaAverage = primaryShellAlphaValues.length > 0
			    ? primaryShellAlphaValues.reduce((sum, alpha) => sum + alpha, 0) / primaryShellAlphaValues.length
			    : 1;
			  const primaryShellAlphaBelow045Count = primaryShellAlphaValues.filter((alpha) => alpha < 0.45).length;
			  const substrateCausticFieldDetails = {
			    body_background_image: refractiveDepthDetails.body_background_image,
			    body_background_translucent_layer: refractiveDepthDetails.body_background_translucent_layer,
			    body_background_layer_count: bodyBackgroundLayerCount,
			    body_background_repeating_layer_count: bodyBackgroundRepeatingLayerCount,
			    body_background_angle_count: bodyBackgroundAngleCount,
			    body_background_angles: bodyBackgroundAngles.map((angle) => Math.round(angle)),
			    before_background_image: refractiveDepthDetails.before_background_image,
			    before_opacity: refractiveDepthDetails.before_opacity,
			  };
			  const refractiveDepthLightGlassReady = refractiveDepthDetails.body_background_image === "present"
			    && refractiveDepthDetails.before_background_image === "present"
			    && refractiveDepthDetails.before_opacity >= 0.12
			    && refractiveDepthDetails.primary_shell_gradient_count >= 3
			    && refractiveDepthDetails.primary_shell_low_alpha_count >= 3;
			  const opticalClarityLightGlassReady = refractiveDepthLightGlassReady
			    && refractiveDepthDetails.body_background_translucent_layer === true
			    && refractiveDepthDetails.body_background_layer_count >= 3
			    && refractiveDepthDetails.before_opacity >= 0.2
			    && refractiveDepthDetails.primary_shell_clear_alpha_count >= 3;
			  const surfaceClearAlphaDetails = primaryShellSurfaceDetails.map((item) => ({
			    selector: item.selector,
			    background_alpha: item.background_alpha,
			    effective_luminance: item.effective_luminance,
			    backdrop_blur_px: item.backdrop_blur_px,
			    surface_alpha_max: Number(primaryShellAlphaMax.toFixed(2)),
			    surface_alpha_min: Number(primaryShellAlphaMin.toFixed(2)),
			    surface_alpha_average: Number(primaryShellAlphaAverage.toFixed(3)),
			    surface_alpha_below_045_count: primaryShellAlphaBelow045Count,
			    surface_count: primaryShellAlphaValues.length,
			    clear_alpha_ready: item.background_alpha >= 0.38 && item.background_alpha <= 0.49,
			    readable: item.readable,
			    width: item.width,
			    height: item.height,
			  }));
			  const surfaceClearAlphaLightGlassReady = opticalClarityLightGlassReady
			    && primaryShellAlphaValues.length >= 3
			    && primaryShellAlphaMax <= 0.49
			    && primaryShellAlphaAverage <= 0.44
			    && primaryShellAlphaMin <= 0.4
			    && primaryShellAlphaBelow045Count >= Math.max(1, primaryShellAlphaValues.length - 1)
			    && primaryShellSurfaceDetails.every((item) => item.background_alpha >= 0.38 && item.background_alpha <= 0.49 && item.readable === true);
			  const substrateCausticFieldLightGlassReady = opticalClarityLightGlassReady
			    && bodyBackgroundLayerCount >= 4
			    && bodyBackgroundRepeatingLayerCount >= 2
			    && bodyBackgroundAngleCount >= 4
			    && refractiveDepthDetails.body_background_translucent_layer === true
			    && refractiveDepthDetails.before_opacity >= 0.2;
	  const legacyExtremeOpticsReady = iconButtonReady
	    && iconPrismaticControlLightGlassReady
	    && defaultSubmenusClosedReady
	    && engineeringSessionChipsSuppressedReady
		    && translucentShellLightGlassReady
		    && refractiveDepthLightGlassReady
		    && opticalClarityLightGlassReady
		    && surfaceClearAlphaLightGlassReady
		    && substrateCausticFieldLightGlassReady
		    && chromeRefractiveSkinLightGlassReady
		    && clearWhiteBalanceLightGlassReady
		    && chamferCutEdgeLightGlassReady
			    && specularEdgeLightGlassReady
			    && prismaticDispersionLightGlassReady
			    && causticHighlightLightGlassReady
			    && causticDepthShiftLightGlassReady
				    && opticalThicknessTiersLightGlassReady
				    && facetedReflectionLightGlassReady
				    && beveledRimLightGlassReady
				    && microRefractionLightGlassReady
				    && sparkleGlintLightGlassReady
				    && lensBloomLightGlassReady
				    && spectralFusionLightGlassReady
				    && opticalMagnificationLightGlassReady
				    && biaxialMagnificationLightGlassReady
				    && anisotropicMagnificationLightGlassReady
				    && phaseSeparatedRefractionLightGlassReady
				    && twoAxisPhaseRefractionLightGlassReady
				    && surfacePhaseDriftLightGlassReady
				    && surfaceLensScaleDriftLightGlassReady
				    && layerScaleParallaxLightGlassReady
				    && surfaceSpectralAngleDriftLightGlassReady
				    && surfaceGlintFocalDriftLightGlassReady
				    && composerGlintFocalDecouplingLightGlassReady
				    && topbarActionLightGlassReady
	    && primaryShellLightGlassReady
	    && menuTriggerReady
	    && folderChipTouchReady
	    && folderChipLabelPrismaticEtchLightGlassReady
	    && rowMenuTouchReady
	    && rowMenuAllRowsReady
		    && rowMenuLightGlassReady
		    && commandPaletteReady
		    && commandPaletteTriggerLightGlassReady
			    && commandPaletteCloseLightGlassReady
				    && commandPaletteInputLightGlassReady
				    && commandPaletteInputPlaceholderPrismaticEtchLightGlassReady
					    && commandPaletteInputRowPrismaticSeparatorLightGlassReady
				    && commandPaletteResultsWellLightGlassReady
				    && commandPaletteResultsWellPrismaticRimLightGlassReady
					    && commandPaletteInputIconPrismaticLightGlassReady
					    && commandPaletteItemLightGlassReady
					    && commandPaletteItemPrismaticRimLightGlassReady
				    && commandPaletteKindChipLightGlassReady
			    && commandPaletteItemHoverPrismaticLightGlassReady
			    && commandPaletteItemLabelPrismaticEtchLightGlassReady
		    && controlFormControlReady
		    && chatRowOptionSemanticTouchReady
		    && railChatRowPrismaticSlabLightGlassReady
		    && menuItemIconReady
    && menuSurfaceReady
	    && threadToolsMenuReady
	    && composerToolsMenuReady
	    && composerPopoverReady
	    && composerPopoverItemLabelPrismaticEtchLightGlassReady
	    && composerPopoverSearchLightGlassReady
	    && composerPopoverSearchPlaceholderPrismaticEtchLightGlassReady
	    && railSearchLightGlassReady
	    && railSearchPlaceholderPrismaticEtchLightGlassReady
	    && railPrismaticFilterLightGlassReady
	    && microSurfaceLightGlassReady
	    && microPrismaticBadgeLightGlassReady
	    && microBadgeLabelPrismaticEtchLightGlassReady
	    && messageMetadataPrismaticLightGlassReady
	    && threadSubtitlePrismaticLightGlassReady
	    && composerShortcutHintPrismaticLightGlassReady
		    && railMetadataChipPrismaticLightGlassReady
		    && railStatusCountPrismaticLightGlassReady
		    && railPreviewPrismaticEtchLightGlassReady
		    && railChatTitlePrismaticEtchLightGlassReady
		    && messageBodyPrismaticEtchLightGlassReady
		    && messageSpeakerPrismaticChipLightGlassReady
		    && composerPlaceholderPrismaticEtchLightGlassReady
		    && messageRoutingBadgeLightGlassReady
	    && threadIntroBadgeLightGlassReady
	    && statusTrustStripLightGlassReady
		    && navIconReady
		    && scrollEdgeReady
		    && microcopyWrapReady
		    && logoClipReady
		    && avatarPrismaticRimLightGlassReady
		    && activeChatReadabilityReady
			    && visibleTextIntegrityReady;
		  const rootStyle = getComputedStyle(document.documentElement);
		  const bodySurfaceStyle = getComputedStyle(document.body);
		  const rootBackground = parseCssColor(rootStyle.backgroundColor);
		  const bodyBackground = parseCssColor(bodySurfaceStyle.backgroundColor);
		  const lightThemeSemanticsDetails = {
		    theme_mode: document.documentElement.getAttribute("data-theme-mode") || "",
		    direction: document.documentElement.getAttribute("dir") || "",
		    color_scheme: rootStyle.colorScheme || "",
		    root_background: rootStyle.backgroundColor,
		    body_background: bodySurfaceStyle.backgroundColor,
		    text_token: rootStyle.getPropertyValue("--text").trim(),
		    panel_token: rootStyle.getPropertyValue("--panel").trim(),
		    input_token: rootStyle.getPropertyValue("--input").trim(),
		  };
		  const lightThemeSemanticsReady = lightThemeSemanticsDetails.theme_mode === "light"
		    && lightThemeSemanticsDetails.direction === "auto"
		    && lightThemeSemanticsDetails.color_scheme.includes("light")
		    && Boolean(rootBackground && relativeLuminance(rootBackground) >= 0.75)
		    && Boolean(bodyBackground && relativeLuminance(bodyBackground) >= 0.75)
		    && lightThemeSemanticsDetails.text_token.length > 0
		    && lightThemeSemanticsDetails.panel_token.length > 0
		    && lightThemeSemanticsDetails.input_token.length > 0;
		  const stableContentSurfaceDetails = primaryShellSurfaceDetails.map((item) => ({
		    selector: item.selector,
		    effective_luminance: item.effective_luminance,
		    contrast_ratio: item.contrast_ratio,
		    backdrop_blur_px: item.backdrop_blur_px,
		    background_image_sample: item.background_image_sample,
		    box_shadow: item.box_shadow,
		    readable: item.readable,
		    restrained_optics: !item.background_image_sample.includes("radial-gradient")
		      && !item.background_image_sample.includes("repeating-")
		      && item.micro_refraction_line_count === 0
		      && item.sparkle_glint_count === 0
		      && item.lens_bloom_count === 0,
		  }));
		  const stableContentSurfaceReady = stableContentSurfaceDetails.length >= 3
		    && stableContentSurfaceDetails.every((item) => item.effective_luminance >= 0.72
		      && item.effective_luminance <= 0.99
		      && item.contrast_ratio >= 4.5
		      && item.backdrop_blur_px >= 8
		      && item.backdrop_blur_px <= 18
		      && item.box_shadow !== "none"
		      && item.restrained_optics === true
		      && item.readable === true);
		  const expectedNativePopoverCount = railVisible ? 8 : 5;
		  const nativePopoverInteractionReady = singleSubmenuAuditDetails.length === expectedNativePopoverCount
		    && rowMenuDistinctPositionsReady
		    && singleSubmenuAuditDetails.every((item) => item.ready === true
		      && item.native_trigger === true
		      && item.trigger_target.length > 0
		      && item.popover_open === true
		      && item.focus_policy_ready === true
		      && item.unavailable_items_ready === true
		      && item.unexpected_visible_count === 0);
		  const floatingSurfaceDetails = singleSubmenuAuditDetails.flatMap((item) => item.surface_details || []);
		  const shallowFloatingSurfaceReady = floatingSurfaceDetails.length >= expectedNativePopoverCount
		    && floatingSurfaceDetails.every((item) => item.effective_luminance >= 0.72
		      && item.effective_luminance <= 0.99
		      && (item.backdrop_filter || "").includes("blur(")
		      && item.box_shadow !== "none"
		      && item.border_radius >= 14
		      && item.in_viewport === true);
		  const keyTouchControlDetails = Array.from(document.querySelectorAll(
		    "[data-control-ui-icon-button],[data-control-ui-menu-trigger='icon'],[data-chat-row-menu-toggle],[data-open-command-palette]",
		  )).filter(elementVisible).map((node) => ({
		    selector: node.getAttribute("data-control-ui-icon-button") || node.getAttribute("data-control-ui-menu-trigger") || node.getAttribute("data-chat-row-menu-toggle") || "command-palette",
		    ...richRect(node),
		  }));
		  const keyTouchControlsReady = keyTouchControlDetails.length >= 4
		    && keyTouchControlDetails.every((item) => item.width >= 44 && item.height >= 44);
		  const mobileSecondaryMetadataDetails = window.innerWidth > 360 ? [] : Array.from(document.querySelectorAll(
		    ".tg-message small,.tg-bubble>span,.tg-routing-badges .badge[data-control-ui-micro-surface]",
		  )).filter(elementVisible).map((node) => {
		    const style = getComputedStyle(node);
		    return {
		      text: visibleText(node),
		      background_color: style.backgroundColor,
		      background_alpha: directBackgroundAlpha(style),
		      box_shadow: compactShadow(style.boxShadow),
		      filter: style.filter || "none",
		      text_shadow: style.textShadow || "none",
		    };
		  });
		  const visibleMobileStatusCount = window.innerWidth > 360 ? 1 : Array.from(document.querySelectorAll(
		    "[data-control-ui-status-trust-badge]",
		  )).filter(elementVisible).length;
		  const restrainedMobileMetadataReady = window.innerWidth > 360 || (
		    visibleMobileStatusCount === 1
		    && mobileSecondaryMetadataDetails.length >= 3
		    && mobileSecondaryMetadataDetails.every((item) => item.background_alpha <= 0.05
		      && item.box_shadow === "none"
		      && (item.filter === "none" || item.filter === "")
		      && (item.text_shadow === "none" || item.text_shadow === ""))
		  );
		  const restrainedOpticsReady = stableContentSurfaceDetails.every((item) => item.restrained_optics === true)
		    && bodyBackgroundLayerCount <= 2
		    && bodyBackgroundRepeatingLayerCount === 0;
		  const shallowLightGlassReady = lightThemeSemanticsReady
		    && stableContentSurfaceReady
		    && nativePopoverInteractionReady
		    && shallowFloatingSurfaceReady
		    && restrainedOpticsReady
		    && restrainedMobileMetadataReady
		    && keyTouchControlsReady
		    && mobilePaneNavigationReady
		    && defaultSubmenusClosedReady
		    && engineeringSessionChipsSuppressedReady
		    && htmlOverflow <= 1
		    && bodyOverflow <= 1
		    && microcopyWrapReady
		    && logoClipReady
		    && activeChatReadabilityReady
		    && visibleTextIntegrityReady;
		  const harshRefereeReady = shallowLightGlassReady;

`;
