module.exports = `
  const iconButtons = Array.from(document.querySelectorAll("[data-control-ui-icon-button]")).filter(elementVisible);
  const iconButtonDetails = iconButtons.map((node) => {
    const style = getComputedStyle(node);
    const visible = elementVisible(node);
    const backdrop = style.backdropFilter || style.webkitBackdropFilter || "";
    const filterText = style.filter || "";
    const dropShadowCount = (filterText.match(/drop-shadow/g) || []).length;
    const visibleIconText = visibleText(node);
    const ariaLabel = node.getAttribute("aria-label") || "";
    const title = node.getAttribute("title") || "";
    return {
      role: node.getAttribute("data-control-ui-icon-button"),
      aria_label: ariaLabel,
      title,
      title_matches_aria_label: title === ariaLabel,
      visible,
      svg_icon_present: hasSvgIcon(node),
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
  const railActionIconReady = !railVisible || iconButtonDetails.some((item) => (
    item.role === "new-conversation"
    && item.aria_label === "New conversation"
    && item.title_matches_aria_label
    && item.visible
    && item.width >= 44
    && item.height >= 44
    && item.svg_icon_present
    && item.visible_icon_text_absent
  ));
  const iconButtonReady = iconButtonDetails.length >= (railVisible ? 5 : 4)
    && railActionIconReady
    && iconButtonDetails.every((item) => item.aria_label.length > 0 && item.title.length > 0 && item.title_matches_aria_label)
    && iconButtonDetails.every((item) => (
      item.visible
      && item.width >= 44
      && item.height >= 44
      && item.border_radius >= 20
      && /blur\\(/.test(item.backdrop_filter)
      && item.box_shadow !== "none"
      && item.svg_icon_present
      && item.visible_icon_text_absent
    ));
  const topbarActionDetails = Array.from(document.querySelectorAll("[data-control-ui-topbar-action]")).map((node) => {
    const style = getComputedStyle(node);
    const bgColor = effectiveBackground(node);
    const textColor = parseCssColor(style.color);
    const bgLuminance = relativeLuminance(bgColor);
    const ratio = textColor ? contrastRatio(textColor, bgColor) : 0;
    const ariaLabel = node.getAttribute("aria-label") || "";
    const title = node.getAttribute("title") || "";
    const backdrop = style.backdropFilter || style.webkitBackdropFilter || "";
    return {
      marker: node.getAttribute("data-control-ui-topbar-action") || "",
      href: node.getAttribute("href") || "",
      text: visibleText(node),
      aria_label: ariaLabel,
      title,
      title_matches_aria_label: title === ariaLabel,
      visible: elementVisible(node),
      svg_icon_present: hasSvgIcon(node),
      effective_luminance: Number(bgLuminance.toFixed(3)),
      light_glass_ready: bgLuminance >= 0.72 && bgLuminance <= 0.98,
      background_color: style.backgroundColor,
      background_alpha: Number(directBackgroundAlpha(style).toFixed(2)),
      translucent_ready: translucentGlassReady(style),
      border_radius: Number.parseFloat(style.borderTopLeftRadius || "0"),
      backdrop_filter: backdrop,
      box_shadow: compactShadow(style.boxShadow),
      contrast_ratio: Number(ratio.toFixed(2)),
      readable: ratio >= 4.5,
      label_nowrap_ready: style.whiteSpace === "nowrap" || node.scrollWidth <= node.clientWidth + 1,
      ...richRect(node),
    };
  });
  const visibleTopbarActionDetails = topbarActionDetails.filter((item) => item.visible);
  const topbarActionLightGlassReady = visibleTopbarActionDetails.length === 0 || visibleTopbarActionDetails.length >= 2
    && visibleTopbarActionDetails.every((item) => (
      item.marker === "light-glass"
      && item.width >= 44
      && item.height >= 44
      && item.border_radius >= 20
      && item.light_glass_ready === true
      && item.translucent_ready === true
      && (item.backdrop_filter || "").includes("blur(")
      && item.box_shadow !== "none"
      && item.aria_label.length > 0
      && item.title.length > 0
      && item.title_matches_aria_label === true
      && item.svg_icon_present === true
      && item.readable === true
      && item.contrast_ratio >= 4.5
      && item.label_nowrap_ready === true
    ));
  const chromeBarTranslucencyDetails = Array.from(document.querySelectorAll(".tg-thread-header,.tg-compose-wrap"))
    .filter(elementVisible)
    .map((node) => {
      const style = getComputedStyle(node);
      const bgColor = effectiveBackground(node);
      const bgLuminance = relativeLuminance(bgColor);
      const backdrop = style.backdropFilter || style.webkitBackdropFilter || "";
      const backgroundImageText = style.backgroundImage || "";
      const backgroundLayerCount = (backgroundImageText.match(/gradient\\(/g) || []).length;
      const backgroundRepeatingLayerCount = (backgroundImageText.match(/repeating-linear-gradient/g) || []).length;
      const specularLayerCount = (style.boxShadow.match(/\\binset\\b/g) || []).length;
      const directAlpha = directBackgroundAlpha(style);
      const directColor = parseCssColor(style.backgroundColor);
      const directChannelSpread = colorChannelSpread(directColor);
      const blurPx = Number.parseFloat((backdrop.split("blur(")[1] || "").split("px")[0] || "0");
      const translucentChromeReady = directAlpha >= 0.42 && directAlpha <= 0.72
        && bgLuminance >= 0.72 && bgLuminance <= 0.98
        && backdrop.includes("blur(")
        && blurPx >= 20;
      return {
        selector: node.className || node.tagName.toLowerCase(),
        background_color: style.backgroundColor,
        background_image: backgroundImageText && backgroundImageText !== "none" ? "present" : "none",
        background_image_sample: backgroundImageText.slice(0, 180),
        chrome_refraction_layer_count: backgroundLayerCount,
        chrome_refraction_repeating_layer_count: backgroundRepeatingLayerCount,
        background_alpha: Number(directAlpha.toFixed(2)),
        background_channel_spread: Number(directChannelSpread.toFixed(1)),
        clear_white_balance_ready: directChannelSpread <= 10,
        effective_luminance: Number(bgLuminance.toFixed(3)),
        backdrop_filter: backdrop,
        backdrop_blur_px: Number((Number.isFinite(blurPx) ? blurPx : 0).toFixed(2)),
        specular_layer_count: specularLayerCount,
        box_shadow: compactShadow(style.boxShadow),
        border_color: style.borderTopColor,
        translucent_chrome_ready: translucentChromeReady,
        refractive_chrome_ready: translucentChromeReady
          && backgroundLayerCount >= 2
          && backgroundRepeatingLayerCount >= 1
          && backgroundImageText.includes("255, 255, 255")
          && specularLayerCount >= 2,
        ...richRect(node),
      };
    });
  const chromeBarTranslucencyLightGlassReady = chromeBarTranslucencyDetails.length >= 2
    && chromeBarTranslucencyDetails.every((item) => item.translucent_chrome_ready === true);
  const chromeRefractiveSkinLightGlassReady = chromeBarTranslucencyLightGlassReady
    && chromeBarTranslucencyDetails.every((item) => item.refractive_chrome_ready === true);
  const primaryShellSurfaceDetails = Array.from(document.querySelectorAll(".tg-conversation-rail,.tg-thread-panel,.tg-compose-bar,.tg-bubble"))
    .filter(elementVisible)
    .map((node) => {
	      const style = getComputedStyle(node);
	      const bgColor = effectiveBackground(node);
	      const textColor = parseCssColor(style.color);
	      const bgLuminance = relativeLuminance(bgColor);
	      const ratio = textColor ? contrastRatio(textColor, bgColor) : 0;
	      const backdrop = style.backdropFilter || style.webkitBackdropFilter || "";
	      const directColor = parseCssColor(style.backgroundColor);
	      const directChannelSpread = colorChannelSpread(directColor);
	      const backgroundImageText = style.backgroundImage || "";
	      const backgroundBlendModeText = style.backgroundBlendMode || "";
	      const backgroundSizeText = style.backgroundSize || "";
	      const backgroundPositionText = style.backgroundPosition || "";
	      const gradientLayerCount = (backgroundImageText.match(/gradient\\(/g) || []).length;
	      const lensScaleLayers = backgroundSizeText.split(",").map((item) => item.trim()).filter(Boolean);
	      const layerScaleParallaxSizeCount = new Set(lensScaleLayers).size;
	      const layerScaleParallaxReady = gradientLayerCount >= 6 && lensScaleLayers.length >= 2 && layerScaleParallaxSizeCount >= 2;
	      const phasePositionLayers = backgroundPositionText.split(",").map((item) => item.trim()).filter(Boolean);
	      const phasePositionCount = new Set(phasePositionLayers).size;
	      const phaseYAxisCount = new Set(phasePositionLayers.map((item) => {
	        const parts = item.split(/\\s+/).filter(Boolean);
	        return parts.length > 1 ? parts.slice(1).join(" ") : "50%";
	      })).size;
	      const phaseSeparatedRefractionReady = gradientLayerCount >= 6 && phasePositionCount >= 6;
	      const twoAxisPhaseRefractionReady = phaseSeparatedRefractionReady && phaseYAxisCount >= 3;
	      const biaxialMagnificationReady = gradientLayerCount >= 6 && /\\d+% \\d+%/.test(backgroundSizeText);
	      const anisotropicMagnificationReady = gradientLayerCount >= 6 && (
	        backgroundSizeText.includes("128% 132%") || backgroundSizeText.includes("126% 134%")
	      );
	      const microRefractionLineCount = (backgroundImageText.match(/repeating-linear-gradient/g) || []).length;
	      const microRefractionAngles = Array.from(backgroundImageText.matchAll(/repeating-linear-gradient\\(([-\\d.]+)deg/g))
	        .map((match) => Number(match[1]))
	        .filter(Number.isFinite);
	      const microRefractionReady = microRefractionLineCount >= 1
	        && microRefractionAngles.some((angle) => angle >= 90 && angle <= 110);
	      const sparkleGlintCount = (backgroundImageText.match(/radial-gradient/g) || []).length;
	      const sparkleGlintReady = sparkleGlintCount >= 1 && backgroundImageText.includes("255, 255, 255");
	      const lensBloomReady = sparkleGlintCount >= 2 && (
	        backgroundImageText.includes("223, 255, 233") || backgroundImageText.includes("223 255 233")
	      );
	      const radialFocalSignatures = Array.from(backgroundImageText.matchAll(/radial-gradient\\(at\\s+([^,]+),/g))
	        .map((match) => match[1].trim().replace(/\\s+/g, " "));
	      const radialFocalSignature = radialFocalSignatures.join("|");
	      const radialFocalCount = new Set(radialFocalSignatures).size;
	      const blurPx = Number.parseFloat((backdrop.split("blur(")[1] || "").split("px")[0] || "0");
	      const causticLayerCount = (backgroundImageText.match(/linear-gradient/g) || []).length;
	      const causticHighlightPresent = backgroundImageText.includes("255, 255, 255");
	      const prismaticPinkPresent = backgroundImageText.includes("255, 223, 244") || backgroundImageText.includes("255 223 244");
	      const prismaticMintPresent = backgroundImageText.includes("223, 255, 233") || backgroundImageText.includes("223 255 233");
	      const facetedReflectionAngles = Array.from(backgroundImageText.matchAll(/linear-gradient\\(([-\\d.]+)deg/g))
	        .map((match) => Number(match[1]))
	        .filter(Number.isFinite);
	      const facetedReflectionAngleCount = new Set(facetedReflectionAngles.map((angle) => Math.round(angle))).size;
	      const spectralAngleSignature = Array.from(new Set(facetedReflectionAngles.map((angle) => Math.round(angle)))).join("/");
	      const specularLayerCount = (style.boxShadow.match(/\\binset\\b/g) || []).length;
	      const shadowColorLayerCount = (style.boxShadow.match(/rgba?\\(/g) || []).length;
	      const specularOutlineColor = parseCssColor(style.outlineColor);
	      const specularOutlineWidth = Number.parseFloat(style.outlineWidth || "0");
	      const specularOutlineOffset = Number.parseFloat(style.outlineOffset || "0");
	      const specularOutlineReady = style.outlineStyle === "solid"
	        && specularOutlineWidth >= 1
	        && specularOutlineOffset <= -1
	        && !!specularOutlineColor
	        && specularOutlineColor.a >= 0.35;
	      return {
	        selector: node.className || node.tagName.toLowerCase(),
	        text: visibleText(node).slice(0, 80),
        visible: elementVisible(node),
		        background_color: style.backgroundColor,
		        background_image: backgroundImageText && backgroundImageText !== "none" ? "present" : "none",
		        background_image_sample: backgroundImageText.slice(0, 220),
		        background_position: backgroundPositionText,
		        phase_position_count: phasePositionCount,
		        phase_y_axis_count: phaseYAxisCount,
		        phase_separated_refraction_ready: phaseSeparatedRefractionReady,
		        two_axis_phase_refraction_ready: twoAxisPhaseRefractionReady,
		        background_size: backgroundSizeText,
	        micro_refraction_line_count: microRefractionLineCount,
	        micro_refraction_angles: microRefractionAngles.map((angle) => Math.round(angle)),
	        micro_refraction_ready: microRefractionReady,
	        sparkle_glint_count: sparkleGlintCount,
	        sparkle_glint_ready: sparkleGlintReady,
	        lens_bloom_count: sparkleGlintCount,
	        lens_bloom_ready: lensBloomReady,
	        radial_focal_signature: radialFocalSignature,
	        radial_focal_layer_count: radialFocalSignatures.length,
	        radial_focal_count: radialFocalCount,
	        spectral_fusion_layer_count: gradientLayerCount,
	        spectral_fusion_blend_mode: backgroundBlendModeText,
	        spectral_fusion_ready: gradientLayerCount >= 6 && backgroundBlendModeText.includes("screen"),
	        optical_magnification_size: backgroundSizeText,
	        optical_magnification_ready: gradientLayerCount >= 6 && backgroundSizeText.includes("%"),
	        biaxial_magnification_size: backgroundSizeText,
	        biaxial_magnification_ready: biaxialMagnificationReady,
	        anisotropic_magnification_size: backgroundSizeText,
	        anisotropic_magnification_ready: anisotropicMagnificationReady,
	        lens_scale_layer_count: lensScaleLayers.length,
	        lens_scale_parallax_size_count: layerScaleParallaxSizeCount,
	        layer_scale_parallax_ready: layerScaleParallaxReady,
		        spectral_angle_signature: spectralAngleSignature,
		        spectral_angle_layer_count: facetedReflectionAngles.length,
		        spectral_angle_count: facetedReflectionAngleCount,
		        caustic_layer_count: causticLayerCount,
		        caustic_highlight_present: causticHighlightPresent,
		        caustic_highlight_ready: causticLayerCount >= 2 && causticHighlightPresent,
		        faceted_reflection_angles: facetedReflectionAngles.map((angle) => Math.round(angle)),
		        faceted_reflection_angle_count: facetedReflectionAngleCount,
		        faceted_reflection_ready: facetedReflectionAngleCount >= 3 && causticHighlightPresent && prismaticPinkPresent && prismaticMintPresent,
	        prismatic_pink_present: prismaticPinkPresent,
	        prismatic_mint_present: prismaticMintPresent,
	        prismatic_dispersion_ready: prismaticPinkPresent && prismaticMintPresent,
	        background_alpha: Number(directBackgroundAlpha(style).toFixed(2)),
	        background_channel_spread: Number(directChannelSpread.toFixed(1)),
	        clear_white_balance_ready: directChannelSpread <= 10,
        translucent_ready: translucentGlassReady(style),
        effective_luminance: Number(bgLuminance.toFixed(3)),
        light_glass_ready: bgLuminance >= 0.72 && bgLuminance <= 0.98,
	        border_radius: Number.parseFloat(style.borderTopLeftRadius || "0"),
	        backdrop_filter: backdrop,
	        backdrop_blur_px: Number((Number.isFinite(blurPx) ? blurPx : 0).toFixed(2)),
	        box_shadow: compactShadow(style.boxShadow),
	        specular_layer_count: specularLayerCount,
	        beveled_rim_layer_count: shadowColorLayerCount,
	        beveled_rim_ready: shadowColorLayerCount >= 5 && specularOutlineReady,
	        specular_outline_width: Number((Number.isFinite(specularOutlineWidth) ? specularOutlineWidth : 0).toFixed(2)),
	        specular_outline_offset: Number((Number.isFinite(specularOutlineOffset) ? specularOutlineOffset : 0).toFixed(2)),
	        specular_outline_alpha: specularOutlineColor ? Number(specularOutlineColor.a.toFixed(2)) : 0,
	        specular_edge_ready: specularLayerCount >= 2 || specularOutlineReady,
	        contrast_ratio: Number(ratio.toFixed(2)),
	        readable: ratio >= 4.5 || visibleText(node).length === 0,
	        ...richRect(node),
      };
    });
  const primaryShellLightGlassReady = primaryShellSurfaceDetails.length >= 3
    && primaryShellSurfaceDetails.every((item) => (
      item.visible
      && item.light_glass_ready === true
      && item.translucent_ready === true
      && item.effective_luminance >= 0.72
      && item.effective_luminance <= 0.98
      && item.border_radius >= 10
      && (item.backdrop_filter || "").includes("blur(")
	      && item.box_shadow !== "none"
	      && item.readable === true
	    ));
	  const bodyDirectColor = parseCssColor(getComputedStyle(document.body).backgroundColor);
	  const bodyChannelSpread = colorChannelSpread(bodyDirectColor);
	  const primaryClearWhiteBalanceReady = primaryShellSurfaceDetails.length >= 3
	    && primaryShellSurfaceDetails.every((item) => item.clear_white_balance_ready === true);
	  const chromeClearWhiteBalanceReady = chromeBarTranslucencyDetails.length >= 2
	    && chromeBarTranslucencyDetails.every((item) => item.clear_white_balance_ready === true);
	  const clearWhiteBalanceLightGlassReady = bodyChannelSpread <= 10
	    && primaryClearWhiteBalanceReady
	    && chromeClearWhiteBalanceReady;
	  const clearWhiteBalanceDetails = {
	    body_background_color: getComputedStyle(document.body).backgroundColor,
	    body_background_channel_spread: Number(bodyChannelSpread.toFixed(1)),
	    primary_surface_channel_spread_max: Math.max(...primaryShellSurfaceDetails.map((item) => item.background_channel_spread ?? 255)),
	    chrome_channel_spread_max: Math.max(...chromeBarTranslucencyDetails.map((item) => item.background_channel_spread ?? 255)),
	    body_clear_white_ready: bodyChannelSpread <= 10,
	    primary_clear_white_ready: primaryClearWhiteBalanceReady,
	    chrome_clear_white_ready: chromeClearWhiteBalanceReady,
	  };
	  const chamferCutEdgeSurfaceDetails = Array.from(document.querySelectorAll(".tg-bubble,.tg-chat-item.active"))
	    .filter(elementVisible)
	    .map((node) => {
	      const style = getComputedStyle(node);
	      const clipPath = style.clipPath || style.webkitClipPath || "";
	      const filterText = style.filter || "";
	      const polygonReady = clipPath.includes("polygon(") && clipPath.includes("9px") && clipPath.includes("calc(");
	      const dropShadowCount = (filterText.match(/drop-shadow/g) || []).length;
	      const prismaticCutEdgeReady = polygonReady && dropShadowCount >= 2;
	      return {
	        selector: node.className || node.tagName.toLowerCase(),
	        text: visibleText(node).slice(0, 80),
	        clip_path: clipPath && clipPath !== "none" ? "present" : "none",
	        clip_path_sample: clipPath.slice(0, 160),
	        polygon_clip_ready: polygonReady,
	        filter: filterText && filterText !== "none" ? "present" : "none",
	        filter_sample: filterText.slice(0, 180),
	        cut_edge_drop_shadow_count: dropShadowCount,
	        prismatic_cut_edge_ready: prismaticCutEdgeReady,
	        border_radius: Number.parseFloat(style.borderTopLeftRadius || "0"),
	        box_shadow: compactShadow(style.boxShadow),
	        ...richRect(node),
	      };
	    });
	  const chamferBubbleCount = chamferCutEdgeSurfaceDetails.filter((item) => item.selector.includes("tg-bubble")).length;
	  const chamferActiveCardCount = chamferCutEdgeSurfaceDetails.filter((item) => item.selector.includes("tg-chat-item") && item.selector.includes("active")).length;
	  const chamferCutEdgeLightGlassReady = chamferBubbleCount >= 3
	    && (!railVisible || chamferActiveCardCount >= 1)
	    && chamferCutEdgeSurfaceDetails.every((item) => item.polygon_clip_ready === true && item.box_shadow !== "none");
	  const prismaticCutEdgeLightGlassReady = chamferCutEdgeLightGlassReady
	    && chamferCutEdgeSurfaceDetails.every((item) => item.prismatic_cut_edge_ready === true);
	  const panePrismaticPerimeterDetails = Array.from(document.querySelectorAll(".tg-conversation-rail,.tg-thread-panel,.tg-room-panel"))
	    .filter(elementVisible)
	    .map((node) => {
	      const style = getComputedStyle(node);
	      const filterText = style.filter || "";
	      const dropShadowCount = (filterText.match(/drop-shadow/g) || []).length;
	      return {
	        selector: node.className || node.tagName.toLowerCase(),
	        filter: filterText && filterText !== "none" ? "present" : "none",
	        filter_sample: filterText.slice(0, 180),
	        perimeter_drop_shadow_count: dropShadowCount,
	        pane_prismatic_perimeter_ready: dropShadowCount >= 2,
	        border_radius: Number.parseFloat(style.borderTopLeftRadius || "0"),
	        box_shadow: compactShadow(style.boxShadow),
	        ...richRect(node),
	      };
	    });
	  const panePrismaticPerimeterLightGlassReady = panePrismaticPerimeterDetails.length >= 1
	    && panePrismaticPerimeterDetails.every((item) => item.pane_prismatic_perimeter_ready === true && item.box_shadow !== "none");
	  const composerPrismaticControlDetails = Array.from(document.querySelectorAll(".tg-compose-bar,[data-agent-chat-send]"))
	    .filter(elementVisible)
	    .map((node) => {
	      const style = getComputedStyle(node);
	      const filterText = style.filter || "";
	      const backdrop = style.backdropFilter || style.webkitBackdropFilter || "";
	      const dropShadowCount = (filterText.match(/drop-shadow/g) || []).length;
	      return {
	        selector: node.className || node.getAttribute("data-agent-chat-send") || node.tagName.toLowerCase(),
	        filter: filterText && filterText !== "none" ? "present" : "none",
	        filter_sample: filterText.slice(0, 180),
	        control_drop_shadow_count: dropShadowCount,
	        composer_prismatic_control_ready: dropShadowCount >= 2,
	        backdrop_filter: backdrop,
	        border_radius: Number.parseFloat(style.borderTopLeftRadius || "0"),
	        box_shadow: compactShadow(style.boxShadow),
	        ...richRect(node),
	      };
	    });
	  const composerPrismaticControlLightGlassReady = composerPrismaticControlDetails.length >= 2
	    && composerPrismaticControlDetails.every((item) => (
	      item.composer_prismatic_control_ready === true
	      && item.box_shadow !== "none"
	      && (item.backdrop_filter || "").includes("blur(")
	    ));
	  const specularEdgeDetails = primaryShellSurfaceDetails.map((item) => ({
	    selector: item.selector,
	    specular_layer_count: item.specular_layer_count,
	    specular_outline_width: item.specular_outline_width,
	    specular_outline_offset: item.specular_outline_offset,
	    specular_outline_alpha: item.specular_outline_alpha,
	    specular_edge_ready: item.specular_edge_ready,
	    box_shadow: item.box_shadow,
	    width: item.width,
	    height: item.height,
	  }));
	  const specularEdgeLightGlassReady = primaryShellSurfaceDetails.length >= 3
	    && primaryShellSurfaceDetails.every((item) => item.specular_edge_ready === true);
	  const prismaticDispersionDetails = primaryShellSurfaceDetails.map((item) => ({
	    selector: item.selector,
	    prismatic_pink_present: item.prismatic_pink_present,
	    prismatic_mint_present: item.prismatic_mint_present,
	    prismatic_dispersion_ready: item.prismatic_dispersion_ready,
	    background_image: item.background_image,
	    background_image_sample: item.background_image_sample,
	    width: item.width,
	    height: item.height,
	  }));
	  const prismaticDispersionLightGlassReady = primaryShellSurfaceDetails.length >= 3
	    && primaryShellSurfaceDetails.every((item) => item.prismatic_dispersion_ready === true);
	  const causticHighlightDetails = primaryShellSurfaceDetails.map((item) => ({
	    selector: item.selector,
	    caustic_layer_count: item.caustic_layer_count,
	    caustic_highlight_present: item.caustic_highlight_present,
	    caustic_highlight_ready: item.caustic_highlight_ready,
	    background_image: item.background_image,
	    background_image_sample: item.background_image_sample,
	    width: item.width,
	    height: item.height,
	  }));
		  const causticHighlightLightGlassReady = primaryShellSurfaceDetails.length >= 3
		    && primaryShellSurfaceDetails.every((item) => item.caustic_highlight_ready === true);
		  const causticDepthShiftDetails = primaryShellSurfaceDetails.map((item) => ({
		    selector: item.selector,
		    caustic_highlight_ready: item.caustic_highlight_ready,
		    background_position: item.background_position,
		    phase_position_count: item.phase_position_count,
		    phase_y_axis_count: item.phase_y_axis_count,
		    phase_separated_refraction_ready: item.phase_separated_refraction_ready,
		    two_axis_phase_refraction_ready: item.two_axis_phase_refraction_ready,
		    width: item.width,
		    height: item.height,
		  }));
		  const causticDepthShiftKeyCount = new Set(causticDepthShiftDetails.map((item) => item.background_position)).size;
		  const phaseSeparatedRefractionLightGlassReady = primaryShellSurfaceDetails.length >= 3
		    && primaryShellSurfaceDetails.every((item) => item.phase_separated_refraction_ready === true);
		  const causticDepthShiftLightGlassReady = primaryShellSurfaceDetails.length >= 3
		    && (causticDepthShiftKeyCount >= 2 || phaseSeparatedRefractionLightGlassReady)
		    && primaryShellSurfaceDetails.every((item) => item.caustic_highlight_ready === true);
		  const opticalThicknessTierDetails = primaryShellSurfaceDetails.map((item) => ({
		    selector: item.selector,
		    background_alpha: item.background_alpha,
		    backdrop_blur_px: item.backdrop_blur_px,
		    background_position: item.background_position,
		    background_size: item.background_size,
		    caustic_highlight_ready: item.caustic_highlight_ready,
		    width: item.width,
		    height: item.height,
		  }));
		  const opticalThicknessBlurTierCount = new Set(opticalThicknessTierDetails.map((item) => item.backdrop_blur_px)).size;
		  const opticalThicknessAlphaTierCount = new Set(opticalThicknessTierDetails.map((item) => item.background_alpha)).size;
		  const opticalThicknessTiersLightGlassReady = primaryShellSurfaceDetails.length >= 3
		    && opticalThicknessBlurTierCount >= 3
		    && opticalThicknessAlphaTierCount >= 3
		    && primaryShellSurfaceDetails.every((item) => item.caustic_highlight_ready === true && item.background_alpha >= 0.38 && item.background_alpha <= 0.49);
		  const facetedReflectionDetails = primaryShellSurfaceDetails.map((item) => ({
		    selector: item.selector,
		    faceted_reflection_angles: item.faceted_reflection_angles,
		    faceted_reflection_angle_count: item.faceted_reflection_angle_count,
		    faceted_reflection_ready: item.faceted_reflection_ready,
		    background_image: item.background_image,
		    background_image_sample: item.background_image_sample,
		    width: item.width,
		    height: item.height,
		  }));
		  const facetedReflectionLightGlassReady = primaryShellSurfaceDetails.length >= 3
		    && primaryShellSurfaceDetails.every((item) => item.faceted_reflection_ready === true);
		  const beveledRimDetails = primaryShellSurfaceDetails.map((item) => ({
		    selector: item.selector,
		    beveled_rim_layer_count: item.beveled_rim_layer_count,
		    beveled_rim_ready: item.beveled_rim_ready,
		    box_shadow: item.box_shadow,
		    specular_outline_width: item.specular_outline_width,
		    specular_outline_offset: item.specular_outline_offset,
		    width: item.width,
		    height: item.height,
		  }));
			  const beveledRimLightGlassReady = primaryShellSurfaceDetails.length >= 3
			    && primaryShellSurfaceDetails.every((item) => item.beveled_rim_ready === true);
			  const microRefractionDetails = primaryShellSurfaceDetails.map((item) => ({
			    selector: item.selector,
			    micro_refraction_line_count: item.micro_refraction_line_count,
			    micro_refraction_ready: item.micro_refraction_ready,
			    background_image: item.background_image,
			    background_image_sample: item.background_image_sample,
			    width: item.width,
			    height: item.height,
			  }));
			  const microRefractionLightGlassReady = primaryShellSurfaceDetails.length >= 3
			    && primaryShellSurfaceDetails.every((item) => item.micro_refraction_ready === true);
			  const sparkleGlintDetails = primaryShellSurfaceDetails.map((item) => ({
			    selector: item.selector,
			    sparkle_glint_count: item.sparkle_glint_count,
			    sparkle_glint_ready: item.sparkle_glint_ready,
			    background_image: item.background_image,
			    background_image_sample: item.background_image_sample,
			    width: item.width,
			    height: item.height,
			  }));
			  const sparkleGlintLightGlassReady = primaryShellSurfaceDetails.length >= 3
			    && primaryShellSurfaceDetails.every((item) => item.sparkle_glint_ready === true);
			  const lensBloomDetails = primaryShellSurfaceDetails.map((item) => ({
			    selector: item.selector,
			    lens_bloom_count: item.lens_bloom_count,
			    lens_bloom_ready: item.lens_bloom_ready,
			    background_image: item.background_image,
			    background_image_sample: item.background_image_sample,
			    width: item.width,
			    height: item.height,
			  }));
			  const lensBloomLightGlassReady = primaryShellSurfaceDetails.length >= 3
			    && primaryShellSurfaceDetails.every((item) => item.lens_bloom_ready === true);
			  const spectralFusionDetails = primaryShellSurfaceDetails.map((item) => ({
			    selector: item.selector,
			    spectral_fusion_layer_count: item.spectral_fusion_layer_count,
			    spectral_fusion_blend_mode: item.spectral_fusion_blend_mode,
			    spectral_fusion_ready: item.spectral_fusion_ready,
			    background_image: item.background_image,
			    background_image_sample: item.background_image_sample,
			    width: item.width,
			    height: item.height,
			  }));
			  const spectralFusionLightGlassReady = primaryShellSurfaceDetails.length >= 3
			    && primaryShellSurfaceDetails.every((item) => item.spectral_fusion_ready === true);
			  const opticalMagnificationDetails = primaryShellSurfaceDetails.map((item) => ({
			    selector: item.selector,
			    optical_magnification_size: item.optical_magnification_size,
			    optical_magnification_ready: item.optical_magnification_ready,
			    spectral_fusion_layer_count: item.spectral_fusion_layer_count,
			    background_image: item.background_image,
			    background_image_sample: item.background_image_sample,
			    width: item.width,
			    height: item.height,
			  }));
			  const opticalMagnificationLightGlassReady = primaryShellSurfaceDetails.length >= 3
			    && primaryShellSurfaceDetails.every((item) => item.optical_magnification_ready === true);
			  const biaxialMagnificationDetails = primaryShellSurfaceDetails.map((item) => ({
			    selector: item.selector,
			    biaxial_magnification_size: item.biaxial_magnification_size,
			    biaxial_magnification_ready: item.biaxial_magnification_ready,
			    spectral_fusion_layer_count: item.spectral_fusion_layer_count,
			    background_image: item.background_image,
			    background_image_sample: item.background_image_sample,
			    width: item.width,
			    height: item.height,
			  }));
			  const biaxialMagnificationLightGlassReady = primaryShellSurfaceDetails.length >= 3
			    && primaryShellSurfaceDetails.every((item) => item.biaxial_magnification_ready === true);
			  const anisotropicMagnificationDetails = primaryShellSurfaceDetails.map((item) => ({
			    selector: item.selector,
			    anisotropic_magnification_size: item.anisotropic_magnification_size,
			    anisotropic_magnification_ready: item.anisotropic_magnification_ready,
			    spectral_fusion_layer_count: item.spectral_fusion_layer_count,
			    background_image: item.background_image,
			    background_image_sample: item.background_image_sample,
			    width: item.width,
			    height: item.height,
			  }));
			  const anisotropicMagnificationLightGlassReady = primaryShellSurfaceDetails.length >= 3
			    && primaryShellSurfaceDetails.every((item) => item.anisotropic_magnification_ready === true);
			  const phaseSeparatedRefractionDetails = primaryShellSurfaceDetails.map((item) => ({
			    selector: item.selector,
			    phase_position_count: item.phase_position_count,
			    phase_y_axis_count: item.phase_y_axis_count,
			    phase_separated_refraction_ready: item.phase_separated_refraction_ready,
			    background_position: item.background_position,
			    spectral_fusion_layer_count: item.spectral_fusion_layer_count,
			    width: item.width,
			    height: item.height,
			  }));
			  const twoAxisPhaseRefractionDetails = primaryShellSurfaceDetails.map((item) => ({
			    selector: item.selector,
			    phase_position_count: item.phase_position_count,
			    phase_y_axis_count: item.phase_y_axis_count,
			    two_axis_phase_refraction_ready: item.two_axis_phase_refraction_ready,
			    background_position: item.background_position,
			    spectral_fusion_layer_count: item.spectral_fusion_layer_count,
			    width: item.width,
			    height: item.height,
			  }));
			  const twoAxisPhaseRefractionLightGlassReady = primaryShellSurfaceDetails.length >= 3
			    && primaryShellSurfaceDetails.every((item) => item.two_axis_phase_refraction_ready === true);
			  const surfacePhaseDriftPositionCount = new Set(primaryShellSurfaceDetails.map((item) => item.background_position)).size;
			  const surfacePhaseDriftDetails = primaryShellSurfaceDetails.map((item) => ({
			    selector: item.selector,
			    background_position: item.background_position,
			    surface_phase_drift_position_count: surfacePhaseDriftPositionCount,
			    two_axis_phase_refraction_ready: item.two_axis_phase_refraction_ready,
			    width: item.width,
			    height: item.height,
			  }));
			  const surfacePhaseDriftLightGlassReady = twoAxisPhaseRefractionLightGlassReady
			    && surfacePhaseDriftPositionCount >= 2;
			  const surfaceLensScaleDriftSizeCount = new Set(primaryShellSurfaceDetails.map((item) => item.background_size)).size;
			  const surfaceLensScaleDriftDetails = primaryShellSurfaceDetails.map((item) => ({
			    selector: item.selector,
			    background_size: item.background_size,
			    surface_lens_scale_drift_size_count: surfaceLensScaleDriftSizeCount,
			    anisotropic_magnification_ready: item.anisotropic_magnification_ready,
			    two_axis_phase_refraction_ready: item.two_axis_phase_refraction_ready,
			    width: item.width,
			    height: item.height,
			  }));
			  const surfaceLensScaleDriftLightGlassReady = surfacePhaseDriftLightGlassReady
			    && surfaceLensScaleDriftSizeCount >= 2
			    && primaryShellSurfaceDetails.every((item) => item.anisotropic_magnification_ready === true);
			  const layerScaleParallaxDetails = primaryShellSurfaceDetails.map((item) => ({
			    selector: item.selector,
			    background_size: item.background_size,
			    lens_scale_layer_count: item.lens_scale_layer_count,
			    lens_scale_parallax_size_count: item.lens_scale_parallax_size_count,
			    layer_scale_parallax_ready: item.layer_scale_parallax_ready,
			    surface_lens_scale_drift_size_count: surfaceLensScaleDriftSizeCount,
			    width: item.width,
			    height: item.height,
			  }));
			  const layerScaleParallaxLightGlassReady = surfaceLensScaleDriftLightGlassReady
			    && primaryShellSurfaceDetails.every((item) => item.layer_scale_parallax_ready === true);
			  const surfaceSpectralAngleDriftSignatureCount = new Set(primaryShellSurfaceDetails.map((item) => item.spectral_angle_signature)).size;
			  const surfaceSpectralAngleDriftDetails = primaryShellSurfaceDetails.map((item) => ({
			    selector: item.selector,
			    spectral_angle_signature: item.spectral_angle_signature,
			    spectral_angle_layer_count: item.spectral_angle_layer_count,
			    spectral_angle_count: item.spectral_angle_count,
			    surface_spectral_angle_drift_signature_count: surfaceSpectralAngleDriftSignatureCount,
			    layer_scale_parallax_ready: item.layer_scale_parallax_ready,
			    width: item.width,
			    height: item.height,
			  }));
			  const surfaceSpectralAngleDriftLightGlassReady = layerScaleParallaxLightGlassReady
			    && surfaceSpectralAngleDriftSignatureCount >= 2
			    && primaryShellSurfaceDetails.every((item) => item.spectral_angle_layer_count >= 4 && item.spectral_angle_count >= 4);
			  const surfaceGlintFocalDriftSignatureCount = new Set(primaryShellSurfaceDetails.map((item) => item.radial_focal_signature)).size;
			  const surfaceGlintFocalDriftDetails = primaryShellSurfaceDetails.map((item) => ({
			    selector: item.selector,
			    radial_focal_signature: item.radial_focal_signature,
			    radial_focal_layer_count: item.radial_focal_layer_count,
			    radial_focal_count: item.radial_focal_count,
			    surface_glint_focal_drift_signature_count: surfaceGlintFocalDriftSignatureCount,
			    surface_spectral_angle_drift_ready: surfaceSpectralAngleDriftLightGlassReady,
			    width: item.width,
			    height: item.height,
			  }));
			  const surfaceGlintFocalDriftLightGlassReady = surfaceSpectralAngleDriftLightGlassReady
			    && surfaceGlintFocalDriftSignatureCount >= 2
			    && primaryShellSurfaceDetails.every((item) => item.radial_focal_layer_count >= 2 && item.radial_focal_count >= 2);
			  const threadGlintFocalSignatures = new Set(primaryShellSurfaceDetails
			    .filter((item) => item.selector.includes("tg-thread-panel"))
			    .map((item) => item.radial_focal_signature));
			  const composerGlintFocalDecouplingDetails = primaryShellSurfaceDetails
			    .filter((item) => item.selector.includes("tg-compose-bar"))
			    .map((item) => ({
			      selector: item.selector,
			      radial_focal_signature: item.radial_focal_signature,
			      radial_focal_layer_count: item.radial_focal_layer_count,
			      radial_focal_count: item.radial_focal_count,
			      thread_radial_focal_signatures: Array.from(threadGlintFocalSignatures),
			      surface_glint_focal_drift_signature_count: surfaceGlintFocalDriftSignatureCount,
			      composer_focal_decoupled: !threadGlintFocalSignatures.has(item.radial_focal_signature),
			      width: item.width,
			      height: item.height,
			    }));
			  const composerGlintFocalDecouplingLightGlassReady = surfaceGlintFocalDriftLightGlassReady
			    && surfaceGlintFocalDriftSignatureCount >= 3
			    && composerGlintFocalDecouplingDetails.length >= 1
			    && composerGlintFocalDecouplingDetails.every((item) => item.composer_focal_decoupled === true
			      && item.radial_focal_layer_count >= 2
			      && item.radial_focal_count >= 2);
			  const threadSpectralAngleSignatures = new Set(primaryShellSurfaceDetails
			    .filter((item) => item.selector.includes("tg-thread-panel"))
			    .map((item) => item.spectral_angle_signature));
			  const composerSpectralAngleDecouplingDetails = primaryShellSurfaceDetails
			    .filter((item) => item.selector.includes("tg-compose-bar"))
			    .map((item) => ({
			      selector: item.selector,
			      spectral_angle_signature: item.spectral_angle_signature,
			      spectral_angle_layer_count: item.spectral_angle_layer_count,
			      spectral_angle_count: item.spectral_angle_count,
			      thread_spectral_angle_signatures: Array.from(threadSpectralAngleSignatures),
			      surface_spectral_angle_drift_signature_count: surfaceSpectralAngleDriftSignatureCount,
			      composer_spectral_angle_decoupled: !threadSpectralAngleSignatures.has(item.spectral_angle_signature),
			      width: item.width,
			      height: item.height,
			    }));
			  const composerSpectralAngleDecouplingLightGlassReady = composerGlintFocalDecouplingLightGlassReady
			    && surfaceSpectralAngleDriftSignatureCount >= 3
			    && composerSpectralAngleDecouplingDetails.length >= 1
			    && composerSpectralAngleDecouplingDetails.every((item) => item.composer_spectral_angle_decoupled === true
			      && item.spectral_angle_layer_count >= 4
			      && item.spectral_angle_count >= 4);
			  const threadPhaseSignatures = new Set(primaryShellSurfaceDetails
			    .filter((item) => item.selector.includes("tg-thread-panel"))
			    .map((item) => item.background_position));
			  const composerPhaseDecouplingDetails = primaryShellSurfaceDetails
			    .filter((item) => item.selector.includes("tg-compose-bar"))
			    .map((item) => ({
			      selector: item.selector,
			      background_position: item.background_position,
			      phase_position_count: item.phase_position_count,
			      phase_y_axis_count: item.phase_y_axis_count,
			      thread_phase_signatures: Array.from(threadPhaseSignatures),
			      surface_phase_drift_position_count: surfacePhaseDriftPositionCount,
			      composer_phase_decoupled: !threadPhaseSignatures.has(item.background_position),
			      width: item.width,
			      height: item.height,
			    }));
			  const composerPhaseDecouplingLightGlassReady = composerSpectralAngleDecouplingLightGlassReady
			    && surfacePhaseDriftPositionCount >= 3
			    && composerPhaseDecouplingDetails.length >= 1
			    && composerPhaseDecouplingDetails.every((item) => item.composer_phase_decoupled === true
			      && item.phase_position_count >= 6
			      && item.phase_y_axis_count >= 3);
			  const threadLayerScaleSignatures = new Set(primaryShellSurfaceDetails
			    .filter((item) => item.selector.includes("tg-thread-panel"))
			    .map((item) => item.background_size));
			  const composerLayerScaleDecouplingDetails = primaryShellSurfaceDetails
			    .filter((item) => item.selector.includes("tg-compose-bar"))
			    .map((item) => ({
			      selector: item.selector,
			      background_size: item.background_size,
			      lens_scale_layer_count: item.lens_scale_layer_count,
			      lens_scale_parallax_size_count: item.lens_scale_parallax_size_count,
			      thread_layer_scale_signatures: Array.from(threadLayerScaleSignatures),
			      surface_lens_scale_drift_size_count: surfaceLensScaleDriftSizeCount,
			      composer_layer_scale_decoupled: !threadLayerScaleSignatures.has(item.background_size),
			      width: item.width,
			      height: item.height,
			    }));
			  const composerLayerScaleDecouplingLightGlassReady = composerPhaseDecouplingLightGlassReady
			    && surfaceLensScaleDriftSizeCount >= 3
			    && composerLayerScaleDecouplingDetails.length >= 1
			    && composerLayerScaleDecouplingDetails.every((item) => item.composer_layer_scale_decoupled === true
			      && item.lens_scale_layer_count >= 2
			      && item.lens_scale_parallax_size_count >= 2);

`;
