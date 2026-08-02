module.exports = `
  const microSurfaceDetails = Array.from(document.querySelectorAll("[data-control-ui-micro-surface]"))
    .filter(elementVisible)
    .map((node) => {
      const style = getComputedStyle(node);
      const textColor = parseCssColor(style.color);
      const bgColor = effectiveBackground(node);
      const ratio = textColor ? contrastRatio(textColor, bgColor) : 0;
      const rect = richRect(node);
      const backdrop = style.backdropFilter || style.webkitBackdropFilter || "";
      const filterText = style.filter || "";
      const dropShadowCount = (filterText.match(/drop-shadow/g) || []).length;
      const textShadow = style.textShadow || "";
      const textShadowCount = textShadow && textShadow !== "none" ? ((textShadow.match(/rgb/g) || []).length || 1) : 0;
      const background = parseCssColor(style.backgroundColor);
      const effectiveLuminance = relativeLuminance(bgColor);
      const label = visibleText(node);
      return {
        key: node.getAttribute("data-control-ui-micro-surface") || "",
        text: label,
        visible: true,
        min_height: styleNumber(style, "minHeight"),
        border_radius: styleNumber(style, "borderTopLeftRadius"),
        background_color: style.backgroundColor,
        background_alpha: background ? background.a : 0,
        translucent_ready: translucentGlassReady(style),
        effective_luminance: Number(effectiveLuminance.toFixed(3)),
        light_glass_ready: effectiveLuminance >= 0.72 && effectiveLuminance <= 0.98,
        backdrop_filter: backdrop,
        box_shadow: compactShadow(style.boxShadow),
        filter: filterText && filterText !== "none" ? "present" : "none",
        filter_sample: filterText.slice(0, 180),
        micro_prismatic_badge_drop_shadow_count: dropShadowCount,
        micro_prismatic_badge_ready: dropShadowCount >= 2,
        text_shadow: textShadow && textShadow !== "none" ? "present" : "none",
        text_shadow_sample: textShadow.slice(0, 180),
        micro_badge_label_text_shadow_count: textShadowCount,
        micro_badge_label_prismatic_etch_ready: textShadowCount >= 2,
        color: style.color,
        contrast_ratio: Number(ratio.toFixed(2)),
        readable: ratio >= 4.5,
        label_nowrap_ready: style.whiteSpace === "nowrap",
        ...rect,
      };
    });
  const engineeringSessionChipDetails = Array.from(document.querySelectorAll(".tg-session-state")).map((node) => ({
    text: visibleText(node),
    visible: elementVisible(node),
    display: getComputedStyle(node).display,
    ...richRect(node),
  }));
  const engineeringSessionChipsSuppressedReady = engineeringSessionChipDetails.every((item) => item.visible === false);
  const expectedMicroSurfaceKeys = railVisible
    ? ["unread-count", "thread-status-local", "thread-status-safe-review", "date-divider", "routing-safe-preview", "routing-local-only", "composer-status-ready"]
    : ["thread-status-local", "thread-status-safe-review", "date-divider", "routing-safe-preview", "routing-local-only"];
  const threadIntroBadgeDetails = microSurfaceDetails.filter((item) => item.key.startsWith("thread-intro-"));
  const threadIntroStrip = document.querySelector(".tg-thread-intro");
  const threadIntroVisible = Boolean(threadIntroStrip && elementVisible(threadIntroStrip));
  const expectedThreadIntroBadgeKeys = [
    "thread-intro-telegram-shell",
    "thread-intro-message-workflow",
    "thread-intro-evidence-inline",
    "thread-intro-approval-chat",
  ];
  const threadIntroBadgeNodes = Array.from(document.querySelectorAll("[data-control-ui-thread-intro-badge]"));
  const threadIntroBadgeLightGlassReady = threadIntroVisible
    ? (
      threadIntroBadgeDetails.length === 4
      && expectedThreadIntroBadgeKeys.every((key) => threadIntroBadgeDetails.some((item) => item.key === key))
      && threadIntroBadgeNodes.length === 4
      && threadIntroBadgeNodes.every((node) => {
        const key = node.getAttribute("data-control-ui-thread-intro-badge") || "";
        const ariaLabel = node.getAttribute("aria-label") || "";
        const title = node.getAttribute("title") || "";
        return key.length > 0 && ariaLabel.length > 0 && title.length > 0 && title === ariaLabel;
      })
      && threadIntroBadgeDetails.every((item) => (
        item.visible
        && item.text.length > 0
        && item.height >= 22
        && item.border_radius >= 10
        && item.light_glass_ready
        && item.translucent_ready === true
        && item.effective_luminance >= 0.72 && item.effective_luminance <= 0.98
        && (item.backdrop_filter || "").includes("blur(")
        && item.box_shadow !== "none"
        && item.readable
        && item.contrast_ratio >= 4.5
        && item.label_nowrap_ready
      ))
    )
    : threadIntroBadgeDetails.length === 0;
  const statusTrustStrip = document.querySelector("[data-control-ui-status-trust-strip]");
  const statusTrustStripVisible = Boolean(statusTrustStrip && elementVisible(statusTrustStrip));
  const expectedStatusTrustBadgeKeys = ["local", "safe-review"];
  const statusTrustBadgeNodes = Array.from(document.querySelectorAll("[data-control-ui-status-trust-badge]"));
  const statusTrustBadgeDetails = statusTrustBadgeNodes
    .filter(elementVisible)
    .map((node) => {
      const style = getComputedStyle(node);
      const textColor = parseCssColor(style.color);
      const bgColor = effectiveBackground(node);
      const ratio = textColor ? contrastRatio(textColor, bgColor) : 0;
      const rect = richRect(node);
      const backdrop = style.backdropFilter || style.webkitBackdropFilter || "";
      const background = parseCssColor(style.backgroundColor);
      const effectiveLuminance = relativeLuminance(bgColor);
      const ariaLabel = node.getAttribute("aria-label") || "";
      const title = node.getAttribute("title") || "";
      return {
        key: node.getAttribute("data-control-ui-status-trust-badge") || "",
        micro_surface_key: node.getAttribute("data-control-ui-micro-surface") || "",
        text: visibleText(node),
        visible: true,
        min_height: styleNumber(style, "minHeight"),
        border_radius: styleNumber(style, "borderTopLeftRadius"),
        background_color: style.backgroundColor,
        background_alpha: background ? background.a : 0,
        translucent_ready: translucentGlassReady(style),
        effective_luminance: Number(effectiveLuminance.toFixed(3)),
        light_glass_ready: effectiveLuminance >= 0.72 && effectiveLuminance <= 0.98,
        backdrop_filter: backdrop,
        box_shadow: compactShadow(style.boxShadow),
        color: style.color,
        contrast_ratio: Number(ratio.toFixed(2)),
        readable: ratio >= 4.5,
        label_nowrap_ready: style.whiteSpace === "nowrap",
        aria_label: ariaLabel,
        title,
        title_matches_aria_label: title === ariaLabel,
        ...rect,
      };
    });
  const statusTrustStripLightGlassReady = statusTrustStripVisible
    && statusTrustStrip.getAttribute("data-control-ui-status-trust-strip") === "local-safe-review"
    && statusTrustStrip.getAttribute("role") === "group"
    && statusTrustStrip.getAttribute("aria-label") === "Thread status trust"
    && statusTrustBadgeNodes.length === 2
    && statusTrustBadgeDetails.length === 2
    && expectedStatusTrustBadgeKeys.every((key) => statusTrustBadgeDetails.some((item) => item.key === key))
    && statusTrustBadgeDetails.every((item) => (
      item.visible
      && item.text.length > 0
      && item.height >= 22
      && item.border_radius >= 10
      && item.light_glass_ready
      && item.translucent_ready === true
      && item.effective_luminance >= 0.72 && item.effective_luminance <= 0.98
      && (item.backdrop_filter || "").includes("blur(")
      && item.box_shadow !== "none"
      && item.readable
      && item.contrast_ratio >= 4.5
      && item.label_nowrap_ready
      && item.title.length > 0
      && item.aria_label.length > 0
      && item.title_matches_aria_label
    ));
  const routingBadgeDetails = microSurfaceDetails.filter((item) => item.key === "routing-safe-preview" || item.key === "routing-local-only");
  const messageRoutingBadgeLightGlassReady = routingBadgeDetails.length === 2
    && ["routing-safe-preview", "routing-local-only"].every((key) => routingBadgeDetails.some((item) => item.key === key))
    && routingBadgeDetails.every((item) => (
      item.visible
      && item.text.length > 0
      && item.height >= 22
      && item.border_radius >= 10
      && item.light_glass_ready
      && item.translucent_ready === true
      && item.effective_luminance >= 0.72 && item.effective_luminance <= 0.98
      && (item.backdrop_filter || "").includes("blur(")
      && item.box_shadow !== "none"
      && item.readable
      && item.contrast_ratio >= 4.5
      && item.label_nowrap_ready
    ));
  const microSurfaceLightGlassReady = microSurfaceDetails.length >= expectedMicroSurfaceKeys.length
    && expectedMicroSurfaceKeys.every((key) => microSurfaceDetails.some((item) => item.key === key))
    && microSurfaceDetails.every((item) => (
      item.key.length > 0
      && item.text.length > 0
      && item.height >= 22
      && item.border_radius >= 10
      && item.light_glass_ready
      && item.translucent_ready === true
      && item.effective_luminance >= 0.72 && item.effective_luminance <= 0.98
      && (item.backdrop_filter || "").includes("blur(")
      && item.box_shadow !== "none"
      && item.readable
      && item.contrast_ratio >= 4.5
      && item.label_nowrap_ready
    ));
  const microPrismaticBadgeLightGlassReady = microSurfaceLightGlassReady
    && microSurfaceDetails.every((item) => (
      item.micro_prismatic_badge_ready === true
      && item.micro_prismatic_badge_drop_shadow_count >= 2
      && item.box_shadow !== "none"
      && (item.backdrop_filter || "").includes("blur(")
    ));
  const microBadgeLabelPrismaticEtchLightGlassReady = microSurfaceLightGlassReady
    && microSurfaceDetails.every((item) => (
      item.text_shadow === "present"
      && item.micro_badge_label_prismatic_etch_ready === true
      && item.micro_badge_label_text_shadow_count >= 2
      && item.readable === true
      && item.contrast_ratio >= 4.5
    ));
  const messageMetadataPrismaticDetails = Array.from(document.querySelectorAll(".tg-message small"))
    .filter(elementVisible)
    .map((node) => {
      const style = getComputedStyle(node);
      const textColor = parseCssColor(style.color);
      const bgColor = effectiveBackground(node);
      const ratio = textColor ? contrastRatio(textColor, bgColor) : 0;
      const backdrop = style.backdropFilter || style.webkitBackdropFilter || "";
      const filterText = style.filter || "";
      const dropShadowCount = (filterText.match(/drop-shadow/g) || []).length;
      const background = parseCssColor(style.backgroundColor);
      const effectiveLuminance = relativeLuminance(bgColor);
      return {
        text: visibleText(node),
        visible: true,
        height: richRect(node).height,
        width: richRect(node).width,
        border_radius: styleNumber(style, "borderTopLeftRadius"),
        background_color: style.backgroundColor,
        background_alpha: background ? background.a : 0,
        translucent_ready: translucentGlassReady(style),
        effective_luminance: Number(effectiveLuminance.toFixed(3)),
        light_glass_ready: effectiveLuminance >= 0.72 && effectiveLuminance <= 0.98,
        backdrop_filter: backdrop,
        box_shadow: compactShadow(style.boxShadow),
        filter: filterText && filterText !== "none" ? "present" : "none",
        filter_sample: filterText.slice(0, 180),
        message_metadata_drop_shadow_count: dropShadowCount,
        message_metadata_prismatic_ready: dropShadowCount >= 2,
        color: style.color,
        contrast_ratio: Number(ratio.toFixed(2)),
        readable: ratio >= 4.5,
        label_nowrap_ready: style.whiteSpace === "nowrap",
      };
    });
  const messageMetadataPrismaticLightGlassReady = messageMetadataPrismaticDetails.length >= 3
    && messageMetadataPrismaticDetails.every((item) => (
      item.visible
      && item.text.length > 0
      && item.height >= 22
      && item.border_radius >= 10
      && item.light_glass_ready
      && item.translucent_ready === true
      && item.effective_luminance >= 0.72 && item.effective_luminance <= 0.98
      && (item.backdrop_filter || "").includes("blur(")
      && item.box_shadow !== "none"
      && item.message_metadata_prismatic_ready === true
      && item.message_metadata_drop_shadow_count >= 2
      && item.readable
      && item.contrast_ratio >= 4.5
      && item.label_nowrap_ready
    ));
  const threadSubtitlePrismaticDetails = Array.from(document.querySelectorAll(".tg-thread-header__main p"))
    .filter(elementVisible)
    .map((node) => {
      const style = getComputedStyle(node);
      const textColor = parseCssColor(style.color);
      const bgColor = effectiveBackground(node);
      const ratio = textColor ? contrastRatio(textColor, bgColor) : 0;
      const backdrop = style.backdropFilter || style.webkitBackdropFilter || "";
      const filterText = style.filter || "";
      const dropShadowCount = (filterText.match(/drop-shadow/g) || []).length;
      const background = parseCssColor(style.backgroundColor);
      const effectiveLuminance = relativeLuminance(bgColor);
      return {
        text: visibleText(node),
        visible: true,
        border_radius: styleNumber(style, "borderTopLeftRadius"),
        background_color: style.backgroundColor,
        background_alpha: background ? background.a : 0,
        translucent_ready: translucentGlassReady(style),
        effective_luminance: Number(effectiveLuminance.toFixed(3)),
        light_glass_ready: effectiveLuminance >= 0.72 && effectiveLuminance <= 0.98,
        backdrop_filter: backdrop,
        box_shadow: compactShadow(style.boxShadow),
        filter: filterText && filterText !== "none" ? "present" : "none",
        filter_sample: filterText.slice(0, 180),
        thread_subtitle_drop_shadow_count: dropShadowCount,
        thread_subtitle_prismatic_ready: dropShadowCount >= 2,
        color: style.color,
        contrast_ratio: Number(ratio.toFixed(2)),
        readable: ratio >= 4.5,
        label_nowrap_ready: style.whiteSpace === "nowrap",
        ...richRect(node),
      };
    });
  const threadSubtitlePrismaticLightGlassReady = threadSubtitlePrismaticDetails.length >= 1
    && threadSubtitlePrismaticDetails.every((item) => (
      item.visible
      && item.text.length > 0
      && item.height >= 22
      && item.border_radius >= 10
      && item.light_glass_ready
      && item.translucent_ready === true
      && item.effective_luminance >= 0.72 && item.effective_luminance <= 0.98
      && (item.backdrop_filter || "").includes("blur(")
      && item.box_shadow !== "none"
      && item.thread_subtitle_prismatic_ready === true
      && item.thread_subtitle_drop_shadow_count >= 2
      && item.readable
      && item.contrast_ratio >= 4.5
      && item.label_nowrap_ready
    ));
  const composerShortcutHintPrismaticDetails = Array.from(document.querySelectorAll("[data-chat-shortcut-hint]"))
    .filter(elementVisible)
    .map((node) => {
      const style = getComputedStyle(node);
      const textColor = parseCssColor(style.color);
      const bgColor = effectiveBackground(node);
      const ratio = textColor ? contrastRatio(textColor, bgColor) : 0;
      const backdrop = style.backdropFilter || style.webkitBackdropFilter || "";
      const filterText = style.filter || "";
      const dropShadowCount = (filterText.match(/drop-shadow/g) || []).length;
      const background = parseCssColor(style.backgroundColor);
      const effectiveLuminance = relativeLuminance(bgColor);
      return {
        text: visibleText(node),
        visible: true,
        border_radius: styleNumber(style, "borderTopLeftRadius"),
        background_color: style.backgroundColor,
        background_alpha: background ? background.a : 0,
        translucent_ready: translucentGlassReady(style),
        effective_luminance: Number(effectiveLuminance.toFixed(3)),
        light_glass_ready: effectiveLuminance >= 0.72 && effectiveLuminance <= 0.98,
        backdrop_filter: backdrop,
        box_shadow: compactShadow(style.boxShadow),
        filter: filterText && filterText !== "none" ? "present" : "none",
        filter_sample: filterText.slice(0, 180),
        composer_shortcut_hint_drop_shadow_count: dropShadowCount,
        composer_shortcut_hint_prismatic_ready: dropShadowCount >= 2,
        color: style.color,
        contrast_ratio: Number(ratio.toFixed(2)),
        readable: ratio >= 4.5,
        label_nowrap_ready: style.whiteSpace === "nowrap",
        ...richRect(node),
      };
    });
  const composerShortcutHintExpectedVisible = window.innerWidth > 700;
  const composerShortcutHintPrismaticLightGlassReady = composerShortcutHintExpectedVisible
    ? composerShortcutHintPrismaticDetails.length >= 1
      && composerShortcutHintPrismaticDetails.every((item) => (
        item.visible
        && item.text.length > 0
        && item.height >= 22
        && item.border_radius >= 10
        && item.light_glass_ready
        && item.translucent_ready === true
        && item.effective_luminance >= 0.72 && item.effective_luminance <= 0.98
        && (item.backdrop_filter || "").includes("blur(")
        && item.box_shadow !== "none"
        && item.composer_shortcut_hint_prismatic_ready === true
        && item.composer_shortcut_hint_drop_shadow_count >= 2
        && item.readable
        && item.contrast_ratio >= 4.5
        && item.label_nowrap_ready
      ))
    : composerShortcutHintPrismaticDetails.length === 0;
  const railMetadataChipPrismaticDetails = Array.from(document.querySelectorAll(".tg-conversation-rail .tg-chat-item__topline span"))
    .filter(elementVisible)
    .map((node) => {
      const style = getComputedStyle(node);
      const textColor = parseCssColor(style.color);
      const bgColor = effectiveBackground(node);
      const ratio = textColor ? contrastRatio(textColor, bgColor) : 0;
      const backdrop = style.backdropFilter || style.webkitBackdropFilter || "";
      const filterText = style.filter || "";
      const dropShadowCount = (filterText.match(/drop-shadow/g) || []).length;
      const background = parseCssColor(style.backgroundColor);
      const effectiveLuminance = relativeLuminance(bgColor);
      return {
        text: visibleText(node),
        visible: true,
        border_radius: styleNumber(style, "borderTopLeftRadius"),
        background_color: style.backgroundColor,
        background_alpha: background ? background.a : 0,
        translucent_ready: translucentGlassReady(style),
        effective_luminance: Number(effectiveLuminance.toFixed(3)),
        light_glass_ready: effectiveLuminance >= 0.72 && effectiveLuminance <= 0.98,
        backdrop_filter: backdrop,
        box_shadow: compactShadow(style.boxShadow),
        filter: filterText && filterText !== "none" ? "present" : "none",
        filter_sample: filterText.slice(0, 180),
        rail_metadata_chip_drop_shadow_count: dropShadowCount,
        rail_metadata_chip_prismatic_ready: dropShadowCount >= 2,
        color: style.color,
        contrast_ratio: Number(ratio.toFixed(2)),
        readable: ratio >= 4.5,
        label_nowrap_ready: style.whiteSpace === "nowrap",
        ...richRect(node),
      };
    });
  const railMetadataChipExpectedVisible = window.innerWidth > 700;
  const railMetadataChipPrismaticLightGlassReady = railMetadataChipExpectedVisible
    ? railMetadataChipPrismaticDetails.length >= 3
      && railMetadataChipPrismaticDetails.every((item) => (
        item.visible
        && item.text.length > 0
        && item.height >= 22
        && item.border_radius >= 10
        && item.light_glass_ready
        && item.translucent_ready === true
        && item.effective_luminance >= 0.72 && item.effective_luminance <= 0.98
        && (item.backdrop_filter || "").includes("blur(")
        && item.box_shadow !== "none"
        && item.rail_metadata_chip_prismatic_ready === true
        && item.rail_metadata_chip_drop_shadow_count >= 2
        && item.readable
        && item.contrast_ratio >= 4.5
        && item.label_nowrap_ready
      ))
    : railMetadataChipPrismaticDetails.length === 0;
  const railStatusCountPrismaticDetails = Array.from(document.querySelectorAll(".tg-conversation-rail .tg-rail-status__item"))
    .filter(elementVisible)
    .map((node) => {
      const style = getComputedStyle(node);
      const textColor = parseCssColor(style.color);
      const bgColor = effectiveBackground(node);
      const ratio = textColor ? contrastRatio(textColor, bgColor) : 0;
      const backdrop = style.backdropFilter || style.webkitBackdropFilter || "";
      const filterText = style.filter || "";
      const dropShadowCount = (filterText.match(/drop-shadow/g) || []).length;
      const background = parseCssColor(style.backgroundColor);
      const effectiveLuminance = relativeLuminance(bgColor);
      return {
        text: visibleText(node),
        visible: true,
        border_radius: styleNumber(style, "borderTopLeftRadius"),
        background_color: style.backgroundColor,
        background_alpha: background ? background.a : 0,
        translucent_ready: translucentGlassReady(style),
        effective_luminance: Number(effectiveLuminance.toFixed(3)),
        light_glass_ready: effectiveLuminance >= 0.72 && effectiveLuminance <= 0.98,
        backdrop_filter: backdrop,
        box_shadow: compactShadow(style.boxShadow),
        filter: filterText && filterText !== "none" ? "present" : "none",
        filter_sample: filterText.slice(0, 180),
        rail_status_count_drop_shadow_count: dropShadowCount,
        rail_status_count_prismatic_ready: dropShadowCount >= 2,
        color: style.color,
        contrast_ratio: Number(ratio.toFixed(2)),
        readable: ratio >= 4.5,
        label_nowrap_ready: style.whiteSpace === "nowrap",
        ...richRect(node),
      };
    });
	  const railStatusCountExpectedVisible = window.innerWidth > 700;
	  const railStatusCountPrismaticLightGlassReady = railStatusCountExpectedVisible
	    ? railStatusCountPrismaticDetails.length >= 1
      && railStatusCountPrismaticDetails.every((item) => (
        item.visible
        && item.text.length > 0
        && item.height >= 22
        && item.border_radius >= 10
        && item.light_glass_ready
        && item.translucent_ready === true
        && item.effective_luminance >= 0.72 && item.effective_luminance <= 0.98
        && (item.backdrop_filter || "").includes("blur(")
        && item.box_shadow !== "none"
        && item.rail_status_count_prismatic_ready === true
        && item.rail_status_count_drop_shadow_count >= 2
        && item.readable
        && item.contrast_ratio >= 4.5
        && item.label_nowrap_ready
	      ))
	    : railStatusCountPrismaticDetails.length === 0;
	  const railPreviewPrismaticEtchDetails = Array.from(document.querySelectorAll(".tg-conversation-rail .tg-chat-item__body p"))
	    .filter(elementVisible)
	    .map((node) => {
	      const style = getComputedStyle(node);
	      const textColor = parseCssColor(style.color);
	      const bgColor = effectiveBackground(node);
	      const ratio = textColor ? contrastRatio(textColor, bgColor) : 0;
	      const filterText = style.filter || "";
	      const dropShadowCount = (filterText.match(/drop-shadow/g) || []).length;
	      return {
	        text: visibleText(node),
	        visible: true,
	        filter: filterText && filterText !== "none" ? "present" : "none",
	        filter_sample: filterText.slice(0, 180),
	        rail_preview_drop_shadow_count: dropShadowCount,
	        rail_preview_prismatic_etch_ready: dropShadowCount >= 2,
	        color: style.color,
	        contrast_ratio: Number(ratio.toFixed(2)),
	        readable: ratio >= 4.5,
	        font_weight: style.fontWeight,
	        ...richRect(node),
	      };
	    });
	  const railPreviewExpectedVisible = window.innerWidth > 700;
	  const railPreviewPrismaticEtchLightGlassReady = railPreviewExpectedVisible
	    ? railPreviewPrismaticEtchDetails.length >= 3
	      && railPreviewPrismaticEtchDetails.every((item) => (
	        item.visible
	        && item.text.length > 0
	        && item.width > 20
	        && item.height >= 14
	        && item.filter === "present"
	        && item.rail_preview_prismatic_etch_ready === true
	        && item.rail_preview_drop_shadow_count >= 2
	        && item.readable
	        && item.contrast_ratio >= 4.5
	      ))
	    : railPreviewPrismaticEtchDetails.length === 0;
	  const railChatTitlePrismaticEtchDetails = Array.from(document.querySelectorAll(".tg-conversation-rail .tg-chat-item__topline strong"))
	    .filter(elementVisible)
	    .map((node) => {
	      const style = getComputedStyle(node);
	      const textColor = parseCssColor(style.color);
	      const bgColor = effectiveBackground(node);
	      const ratio = textColor ? contrastRatio(textColor, bgColor) : 0;
	      const filterText = style.filter || "";
	      const dropShadowCount = (filterText.match(/drop-shadow/g) || []).length;
	      return {
	        text: visibleText(node),
	        visible: true,
	        filter: filterText && filterText !== "none" ? "present" : "none",
	        filter_sample: filterText.slice(0, 180),
	        rail_chat_title_drop_shadow_count: dropShadowCount,
	        rail_chat_title_prismatic_etch_ready: dropShadowCount >= 2,
	        color: style.color,
	        contrast_ratio: Number(ratio.toFixed(2)),
	        readable: ratio >= 4.5,
	        font_weight: style.fontWeight,
	        ...richRect(node),
	      };
	    });
	  const railChatTitleExpectedVisible = window.innerWidth > 700;
	  const railChatTitlePrismaticEtchLightGlassReady = railChatTitleExpectedVisible
	    ? railChatTitlePrismaticEtchDetails.length >= 3
	      && railChatTitlePrismaticEtchDetails.every((item) => (
	        item.visible
	        && item.text.length > 0
	        && item.width > 20
	        && item.height >= 14
	        && item.filter === "present"
	        && item.rail_chat_title_prismatic_etch_ready === true
	        && item.rail_chat_title_drop_shadow_count >= 2
	        && item.readable
	        && item.contrast_ratio >= 4.5
	      ))
	    : railChatTitlePrismaticEtchDetails.length === 0;
	  const messageBodyPrismaticEtchDetails = Array.from(document.querySelectorAll(".tg-thread .tg-bubble p"))
	    .filter(elementVisible)
	    .map((node) => {
	      const style = getComputedStyle(node);
	      const textColor = parseCssColor(style.color);
	      const bgColor = effectiveBackground(node);
	      const ratio = textColor ? contrastRatio(textColor, bgColor) : 0;
	      const filterText = style.filter || "";
	      const dropShadowCount = (filterText.match(/drop-shadow/g) || []).length;
	      return {
	        text: visibleText(node),
	        visible: true,
	        filter: filterText && filterText !== "none" ? "present" : "none",
	        filter_sample: filterText.slice(0, 180),
	        message_body_drop_shadow_count: dropShadowCount,
	        message_body_prismatic_etch_ready: dropShadowCount >= 2,
	        color: style.color,
	        contrast_ratio: Number(ratio.toFixed(2)),
	        readable: ratio >= 4.5,
	        font_weight: style.fontWeight,
	        ...richRect(node),
	      };
	    });
	  const messageBodyPrismaticEtchLightGlassReady = messageBodyPrismaticEtchDetails.length >= 3
	    && messageBodyPrismaticEtchDetails.every((item) => (
	      item.visible
	      && item.text.length > 0
	      && item.width > 20
	      && item.height >= 16
	      && item.filter === "present"
	      && item.message_body_prismatic_etch_ready === true
	      && item.message_body_drop_shadow_count >= 2
	      && item.readable
	      && item.contrast_ratio >= 4.5
	    ));
	  const messageSpeakerPrismaticChipDetails = Array.from(document.querySelectorAll(".tg-thread .tg-bubble>span"))
	    .filter(elementVisible)
	    .map((node) => {
	      const style = getComputedStyle(node);
	      const textColor = parseCssColor(style.color);
	      const bgColor = effectiveBackground(node);
	      const ratio = textColor ? contrastRatio(textColor, bgColor) : 0;
	      const backdrop = style.backdropFilter || style.webkitBackdropFilter || "";
	      const filterText = style.filter || "";
	      const dropShadowCount = (filterText.match(/drop-shadow/g) || []).length;
	      const background = parseCssColor(style.backgroundColor);
	      const effectiveLuminance = relativeLuminance(bgColor);
	      return {
	        text: visibleText(node),
	        visible: true,
	        border_radius: styleNumber(style, "borderTopLeftRadius"),
	        background_color: style.backgroundColor,
	        background_alpha: background ? background.a : 0,
	        translucent_ready: translucentGlassReady(style),
	        effective_luminance: Number(effectiveLuminance.toFixed(3)),
	        light_glass_ready: effectiveLuminance >= 0.72 && effectiveLuminance <= 0.98,
	        backdrop_filter: backdrop,
	        box_shadow: compactShadow(style.boxShadow),
	        filter: filterText && filterText !== "none" ? "present" : "none",
	        filter_sample: filterText.slice(0, 180),
	        message_speaker_chip_drop_shadow_count: dropShadowCount,
	        message_speaker_prismatic_chip_ready: dropShadowCount >= 2,
	        color: style.color,
	        contrast_ratio: Number(ratio.toFixed(2)),
	        readable: ratio >= 4.5,
	        label_nowrap_ready: style.whiteSpace === "nowrap",
	        ...richRect(node),
	      };
	    });
	  const messageSpeakerPrismaticChipLightGlassReady = messageSpeakerPrismaticChipDetails.length >= 3
	    && messageSpeakerPrismaticChipDetails.every((item) => (
	      item.visible
	      && item.text.length > 0
	      && item.height >= 22
	      && item.border_radius >= 10
	      && item.light_glass_ready
	      && item.translucent_ready === true
	      && item.effective_luminance >= 0.72 && item.effective_luminance <= 0.98
	      && (item.backdrop_filter || "").includes("blur(")
	      && item.box_shadow !== "none"
	      && item.filter === "present"
	      && item.message_speaker_prismatic_chip_ready === true
	      && item.message_speaker_chip_drop_shadow_count >= 2
	      && item.readable
	      && item.contrast_ratio >= 4.5
	      && item.label_nowrap_ready
	    ));
	  const composerPlaceholderPrismaticEtchDetails = Array.from(document.querySelectorAll(".tg-compose-bar textarea[placeholder]"))
	    .filter(elementVisible)
	    .map((node) => {
	      const style = getComputedStyle(node);
	      const placeholderStyle = getComputedStyle(node, "::placeholder");
	      const placeholderColor = parseCssColor(placeholderStyle.color);
	      const bgColor = effectiveBackground(node);
	      const ratio = placeholderColor ? contrastRatio(placeholderColor, bgColor) : 0;
	      const textShadow = placeholderStyle.textShadow || "";
	      const textShadowCount = textShadow && textShadow !== "none" ? ((textShadow.split("rgba(").length - 1) + (textShadow.split("rgb(").length - 1) || 1) : 0;
	      return {
	        placeholder: node.getAttribute("placeholder") || "",
	        visible: true,
	        placeholder_text_shadow: textShadow && textShadow !== "none" ? "present" : "none",
	        placeholder_text_shadow_sample: textShadow.slice(0, 180),
	        composer_placeholder_text_shadow_count: textShadowCount,
	        composer_placeholder_prismatic_etch_ready: textShadowCount >= 2,
	        color: placeholderStyle.color,
	        contrast_ratio: Number(ratio.toFixed(2)),
	        readable: ratio >= 4.5,
	        font_weight: style.fontWeight,
	        ...richRect(node),
	      };
	    });
	  const composerPlaceholderPrismaticEtchLightGlassReady = composerPlaceholderPrismaticEtchDetails.length >= 1
	    && composerPlaceholderPrismaticEtchDetails.every((item) => (
	      item.visible
	      && item.placeholder.length > 0
	      && item.width >= 100
	      && item.height >= 44
	      && item.placeholder_text_shadow === "present"
	      && item.composer_placeholder_prismatic_etch_ready === true
	      && item.composer_placeholder_text_shadow_count >= 2
	      && item.readable
	      && item.contrast_ratio >= 4.5
	    ));
	  const headerTitlePrismaticEtchDetails = Array.from(document.querySelectorAll(".tg-rail-header h2,.tg-thread-header h2"))
	    .filter(elementVisible)
	    .map((node) => {
      const style = getComputedStyle(node);
      const textColor = parseCssColor(style.color);
      const bgColor = effectiveBackground(node);
      const ratio = textColor ? contrastRatio(textColor, bgColor) : 0;
      const filterText = style.filter || "";
      const dropShadowCount = (filterText.match(/drop-shadow/g) || []).length;
      return {
        text: visibleText(node),
        visible: true,
        filter: filterText && filterText !== "none" ? "present" : "none",
        filter_sample: filterText.slice(0, 180),
        header_title_drop_shadow_count: dropShadowCount,
        header_title_prismatic_etch_ready: dropShadowCount >= 2,
        color: style.color,
        contrast_ratio: Number(ratio.toFixed(2)),
        readable: ratio >= 4.5,
        font_weight: style.fontWeight,
        ...richRect(node),
      };
    });
  const headerTitleExpectedCount = window.innerWidth > 700 ? 2 : 1;
  const headerTitlePrismaticEtchLightGlassReady = headerTitlePrismaticEtchDetails.length >= headerTitleExpectedCount
    && headerTitlePrismaticEtchDetails.every((item) => (
      item.visible
      && item.text.length > 0
      && item.width > 20
      && item.height >= 16
      && item.filter === "present"
      && item.header_title_prismatic_etch_ready === true
      && item.header_title_drop_shadow_count >= 2
      && item.readable
      && item.contrast_ratio >= 4.5
    ));
  const navIconReady = Array.from(document.querySelectorAll(".nav .hepta-ui-icon")).length >= 4
    && Array.from(document.querySelectorAll(".nav a")).every((node) => hasSvgIcon(node.querySelector(".hepta-ui-icon")));
  const threadPanel = document.querySelector(".tg-thread-panel");
  const threadPanelAfter = threadPanel ? getComputedStyle(threadPanel, "::after") : null;
  const threadStyle = document.querySelector(".tg-thread") ? getComputedStyle(document.querySelector(".tg-thread")) : null;
  const scrollEdgeReady = Boolean(
    document.body.getAttribute("data-control-ui-harsh-referee")
    && threadPanelAfter
    && threadPanelAfter.content !== "none"
    && threadStyle
    && threadStyle.overscrollBehaviorY === "contain"
  );

`;
