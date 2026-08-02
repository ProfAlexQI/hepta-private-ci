module.exports = `
		  if (!harshRefereeReady) {
		    errors.push("control_ui_harsh_2026_referee_not_ready");
		  }
		  if (!lightThemeSemanticsReady) errors.push("light_theme_semantics_not_ready");
		  if (!stableContentSurfaceReady) errors.push("stable_content_surface_not_ready");
		  if (!nativePopoverInteractionReady) errors.push("native_popover_interaction_not_ready");
		  if (!shallowFloatingSurfaceReady) errors.push("shallow_floating_surface_not_ready");
		  if (!restrainedOpticsReady) errors.push("restrained_optics_not_ready");
		  if (!restrainedMobileMetadataReady) errors.push("restrained_mobile_metadata_not_ready");
		  if (!keyTouchControlsReady) errors.push("key_touch_controls_not_ready");
		  if (!mobilePaneNavigationReady) errors.push("mobile_pane_navigation_not_ready");
		  /* Keep the historical extreme-optics probes as non-gating diagnostics.
		     They intentionally fail for the restrained 2026 shallow-glass system. */
		  if (legacyExtremeOpticsReady) {
		  if (!iconPrismaticControlLightGlassReady) {
	    errors.push("icon_prismatic_control_light_glass_guard_not_ready");
	  }
	  if (!commandPaletteReady) {
	    errors.push("command_palette_touch_guard_not_ready");
	  }
	  if (!commandPaletteTriggerLightGlassReady) {
	    errors.push("command_palette_trigger_light_glass_guard_not_ready");
	  }
	  if (!commandPaletteCloseLightGlassReady) {
	    errors.push("command_palette_close_light_glass_guard_not_ready");
	  }
	  if (!commandPaletteClosePrismaticIconLightGlassReady) {
	    errors.push("command_palette_close_prismatic_icon_light_glass_guard_not_ready");
	  }
	  if (!commandPaletteSurfaceLightGlassReady) {
	    errors.push("command_palette_surface_light_glass_guard_not_ready");
	  }
	  if (!commandPaletteSurfacePrismaticPerimeterLightGlassReady) {
	    errors.push("command_palette_surface_prismatic_perimeter_light_glass_guard_not_ready");
	  }
	  if (!commandPaletteBackdropCausticVeilLightGlassReady) {
	    errors.push("command_palette_backdrop_caustic_veil_light_glass_guard_not_ready");
	  }
	  if (!commandPaletteInputLightGlassReady) {
	    errors.push("command_palette_input_light_glass_guard_not_ready");
	  }
	  if (!commandPaletteInputTextPrismaticEtchLightGlassReady) {
	    errors.push("command_palette_input_text_prismatic_etch_light_glass_guard_not_ready");
	  }
	  if (!commandPaletteInputPlaceholderPrismaticEtchLightGlassReady) {
	    errors.push("command_palette_input_placeholder_prismatic_etch_light_glass_guard_not_ready");
	  }
		  if (!commandPaletteInputRowPrismaticSeparatorLightGlassReady) {
		    errors.push("command_palette_input_row_prismatic_separator_light_glass_guard_not_ready");
		  }
		  if (!commandPaletteResultsWellLightGlassReady) {
		    errors.push("command_palette_results_well_light_glass_guard_not_ready");
		  }
		  if (!commandPaletteResultsWellPrismaticRimLightGlassReady) {
		    errors.push("command_palette_results_well_prismatic_rim_light_glass_guard_not_ready");
		  }
		  if (!commandPaletteInputIconLightGlassReady) {
		    errors.push("command_palette_input_icon_light_glass_guard_not_ready");
	  }
		  if (!commandPaletteInputIconPrismaticLightGlassReady) {
		    errors.push("command_palette_input_icon_prismatic_light_glass_guard_not_ready");
	  }
		  if (!commandPaletteItemLightGlassReady) {
		    errors.push("command_palette_item_light_glass_guard_not_ready");
		  }
		  if (!commandPaletteItemPrismaticRimLightGlassReady) {
		    errors.push("command_palette_item_prismatic_rim_light_glass_guard_not_ready");
		  }
		  if (!commandPaletteKindChipLightGlassReady) {
		    errors.push("command_palette_kind_chip_light_glass_guard_not_ready");
		  }
		  if (!commandPaletteItemHoverPrismaticLightGlassReady) {
		    errors.push("command_palette_item_hover_prismatic_light_glass_guard_not_ready");
		  }
		  if (!commandPaletteItemLabelPrismaticEtchLightGlassReady) {
		    errors.push("command_palette_item_label_prismatic_etch_light_glass_guard_not_ready");
		  }
		  if (!controlFormControlReady) {
		    errors.push("control_form_control_title_touch_guard_not_ready");
		  }
		  if (!chatRowOptionSemanticTouchReady) {
		    errors.push("chat_row_option_semantic_touch_guard_not_ready");
		  }
		  if (!railChatRowPrismaticSlabLightGlassReady) {
		    errors.push("rail_chat_row_prismatic_slab_light_glass_guard_not_ready");
		  }
		  if (!composerPopoverSearchLightGlassReady) {
		    errors.push("composer_popover_search_light_glass_guard_not_ready");
		  }
		  if (!composerPopoverSearchPlaceholderPrismaticEtchLightGlassReady) {
		    errors.push("composer_popover_search_placeholder_prismatic_etch_light_glass_guard_not_ready");
		  }
		  if (!railSearchLightGlassReady) {
		    errors.push("rail_search_light_glass_guard_not_ready");
		  }
		  if (!railSearchPlaceholderPrismaticEtchLightGlassReady) {
		    errors.push("rail_search_placeholder_prismatic_etch_light_glass_guard_not_ready");
		  }
		  if (!railPrismaticFilterLightGlassReady) {
		    errors.push("rail_prismatic_filter_light_glass_guard_not_ready");
		  }
		  if (!folderChipLabelPrismaticEtchLightGlassReady) {
		    errors.push("folder_chip_label_prismatic_etch_light_glass_guard_not_ready");
		  }
		  if (!rowMenuAllRowsReady) {
		    errors.push("row_menu_all_rows_guard_not_ready");
		  }
		  if (!threadToolsMenuReady) {
		    errors.push("thread_tools_menu_light_glass_guard_not_ready");
		  }
		  if (!composerToolsMenuReady) {
		    errors.push("composer_tools_menu_light_glass_guard_not_ready");
		  }
		  if (!composerPopoverReady) {
		    errors.push("composer_popover_light_glass_guard_not_ready");
		  }
		  if (!composerPopoverItemLabelPrismaticEtchLightGlassReady) {
		    errors.push("composer_popover_item_label_prismatic_etch_light_glass_guard_not_ready");
		  }
		  if (!composerPopoverHeaderPrismaticEtchLightGlassReady) {
		    errors.push("composer_popover_header_prismatic_etch_light_glass_guard_not_ready");
		  }
		  if (!microSurfaceLightGlassReady) {
		    errors.push("micro_surface_light_glass_guard_not_ready");
		  }
		  if (!microPrismaticBadgeLightGlassReady) {
		    errors.push("micro_prismatic_badge_light_glass_guard_not_ready");
		  }
		  if (!microBadgeLabelPrismaticEtchLightGlassReady) {
		    errors.push("micro_badge_label_prismatic_etch_light_glass_guard_not_ready");
		  }
		  if (!messageMetadataPrismaticLightGlassReady) {
		    errors.push("message_metadata_prismatic_light_glass_guard_not_ready");
		  }
		  if (!threadSubtitlePrismaticLightGlassReady) {
		    errors.push("thread_subtitle_prismatic_light_glass_guard_not_ready");
		  }
		  if (!composerShortcutHintPrismaticLightGlassReady) {
		    errors.push("composer_shortcut_hint_prismatic_light_glass_guard_not_ready");
		  }
		  if (!railMetadataChipPrismaticLightGlassReady) {
		    errors.push("rail_metadata_chip_prismatic_light_glass_guard_not_ready");
		  }
			  if (!railStatusCountPrismaticLightGlassReady) {
			    errors.push("rail_status_count_prismatic_light_glass_guard_not_ready");
			  }
			  if (!railPreviewPrismaticEtchLightGlassReady) {
			    errors.push("rail_preview_prismatic_etch_light_glass_guard_not_ready");
			  }
			  if (!railChatTitlePrismaticEtchLightGlassReady) {
			    errors.push("rail_chat_title_prismatic_etch_light_glass_guard_not_ready");
			  }
			  if (!messageBodyPrismaticEtchLightGlassReady) {
			    errors.push("message_body_prismatic_etch_light_glass_guard_not_ready");
			  }
			  if (!messageSpeakerPrismaticChipLightGlassReady) {
			    errors.push("message_speaker_prismatic_chip_light_glass_guard_not_ready");
			  }
			  if (!composerPlaceholderPrismaticEtchLightGlassReady) {
			    errors.push("composer_placeholder_prismatic_etch_light_glass_guard_not_ready");
			  }
			  if (!headerTitlePrismaticEtchLightGlassReady) {
			    errors.push("header_title_prismatic_etch_light_glass_guard_not_ready");
			  }
		  if (!messageRoutingBadgeLightGlassReady) {
		    errors.push("message_routing_badge_light_glass_guard_not_ready");
		  }
		  if (!threadIntroBadgeLightGlassReady) {
		    errors.push("thread_intro_badge_light_glass_guard_not_ready");
		  }
		  if (!statusTrustStripLightGlassReady) {
		    errors.push("status_trust_strip_light_glass_guard_not_ready");
		  }
		  if (!avatarPrismaticRimLightGlassReady) {
		    errors.push("avatar_prismatic_rim_light_glass_guard_not_ready");
		  }
		  if (!topbarActionLightGlassReady) {
		    errors.push("topbar_action_light_glass_guard_not_ready");
		  }
	  if (!primaryShellLightGlassReady) {
	    errors.push("primary_shell_light_glass_guard_not_ready");
	  }
	  if (!translucentShellLightGlassReady) {
	    errors.push("translucent_shell_light_glass_guard_not_ready");
	  }
	  if (!refractiveDepthLightGlassReady) {
	    errors.push("refractive_depth_light_glass_guard_not_ready");
	  }
		  if (!opticalClarityLightGlassReady) {
		    errors.push("optical_clarity_light_glass_guard_not_ready");
		  }
			  if (!specularEdgeLightGlassReady) {
			    errors.push("specular_edge_light_glass_guard_not_ready");
			  }
			  if (!prismaticDispersionLightGlassReady) {
			    errors.push("prismatic_dispersion_light_glass_guard_not_ready");
			  }
				  if (!causticHighlightLightGlassReady) {
				    errors.push("caustic_highlight_light_glass_guard_not_ready");
				  }
				  if (!causticDepthShiftLightGlassReady) {
				    errors.push("caustic_depth_shift_light_glass_guard_not_ready");
				  }
				  if (!opticalThicknessTiersLightGlassReady) {
				    errors.push("optical_thickness_tiers_light_glass_guard_not_ready");
				  }
				  if (!facetedReflectionLightGlassReady) {
				    errors.push("faceted_reflection_light_glass_guard_not_ready");
				  }
					  if (!beveledRimLightGlassReady) {
					    errors.push("beveled_rim_light_glass_guard_not_ready");
					  }
					  if (!microRefractionLightGlassReady) {
					    errors.push("micro_refraction_light_glass_guard_not_ready");
					  }
					  if (!sparkleGlintLightGlassReady) {
					    errors.push("sparkle_glint_light_glass_guard_not_ready");
					  }
					  if (!lensBloomLightGlassReady) {
					    errors.push("lens_bloom_light_glass_guard_not_ready");
					  }
					  if (!spectralFusionLightGlassReady) {
					    errors.push("spectral_fusion_light_glass_guard_not_ready");
					  }
					  if (!opticalMagnificationLightGlassReady) {
					    errors.push("optical_magnification_light_glass_guard_not_ready");
					  }
					  if (!biaxialMagnificationLightGlassReady) {
					    errors.push("biaxial_magnification_light_glass_guard_not_ready");
					  }
					  if (!anisotropicMagnificationLightGlassReady) {
					    errors.push("anisotropic_magnification_light_glass_guard_not_ready");
					  }
					  if (!phaseSeparatedRefractionLightGlassReady) {
					    errors.push("phase_separated_refraction_light_glass_guard_not_ready");
					  }
					  if (!twoAxisPhaseRefractionLightGlassReady) {
					    errors.push("two_axis_phase_refraction_light_glass_guard_not_ready");
					  }
					  if (!surfacePhaseDriftLightGlassReady) {
					    errors.push("surface_phase_drift_light_glass_guard_not_ready");
					  }
					  if (!surfaceLensScaleDriftLightGlassReady) {
					    errors.push("surface_lens_scale_drift_light_glass_guard_not_ready");
					  }
					  if (!layerScaleParallaxLightGlassReady) {
					    errors.push("layer_scale_parallax_light_glass_guard_not_ready");
					  }
					  if (!surfaceSpectralAngleDriftLightGlassReady) {
					    errors.push("surface_spectral_angle_drift_light_glass_guard_not_ready");
					  }
					  if (!surfaceGlintFocalDriftLightGlassReady) {
					    errors.push("surface_glint_focal_drift_light_glass_guard_not_ready");
					  }
					  if (!composerGlintFocalDecouplingLightGlassReady) {
					    errors.push("composer_glint_focal_decoupling_light_glass_guard_not_ready");
					  }
					  if (!composerSpectralAngleDecouplingLightGlassReady) {
					    errors.push("composer_spectral_angle_decoupling_light_glass_guard_not_ready");
					  }
					  if (!composerPhaseDecouplingLightGlassReady) {
					    errors.push("composer_phase_decoupling_light_glass_guard_not_ready");
					  }
					  if (!composerLayerScaleDecouplingLightGlassReady) {
					    errors.push("composer_layer_scale_decoupling_light_glass_guard_not_ready");
					  }
					  if (!chromeBarTranslucencyLightGlassReady) {
					    errors.push("chrome_bar_translucency_light_glass_guard_not_ready");
					  }
					  if (!chromeRefractiveSkinLightGlassReady) {
					    errors.push("chrome_refractive_skin_light_glass_guard_not_ready");
					  }
					  if (!clearWhiteBalanceLightGlassReady) {
					    errors.push("clear_white_balance_light_glass_guard_not_ready");
					  }
					  if (!chamferCutEdgeLightGlassReady) {
					    errors.push("chamfer_cut_edge_light_glass_guard_not_ready");
					  }
					  if (!prismaticCutEdgeLightGlassReady) {
					    errors.push("prismatic_cut_edge_light_glass_guard_not_ready");
					  }
					  if (!panePrismaticPerimeterLightGlassReady) {
					    errors.push("pane_prismatic_perimeter_light_glass_guard_not_ready");
					  }
					  if (!composerPrismaticControlLightGlassReady) {
					    errors.push("composer_prismatic_control_light_glass_guard_not_ready");
					  }
					  if (!surfaceClearAlphaLightGlassReady) {
					    errors.push("surface_clear_alpha_light_glass_guard_not_ready");
					  }
					  if (!substrateCausticFieldLightGlassReady) {
					    errors.push("substrate_caustic_field_light_glass_guard_not_ready");
					  }
		  }
			  if (!microcopyWrapReady) {
	    errors.push("microcopy_word_split_guard_not_ready");
	  }
		  if (!logoClipReady) {
		    errors.push("logo_clip_guard_not_ready");
		  }
		  if (!activeChatReadabilityReady) {
		    errors.push("active_chat_readability_guard_not_ready");
		  }
	  if (!visibleTextIntegrityReady) {
	    errors.push("visible_text_integrity_guard_not_ready");
	  }
	  if (!defaultSubmenusClosedReady) {
	    errors.push("default_submenus_closed_guard_not_ready");
	  }
	  if (!singleSubmenuAuditReady) {
	    errors.push("single_submenu_audit_guard_not_ready");
	  }
	  if (!engineeringSessionChipsSuppressedReady) {
	    errors.push("engineering_session_chips_suppressed_guard_not_ready");
	  }
	  return {
	    errors,
	    title: document.title,
	    marker,
    viewport: { width: window.innerWidth, height: window.innerHeight },
    document_scroll_width: document.documentElement.scrollWidth,
    body_scroll_width: document.body.scrollWidth,
    horizontal_overflow_free: htmlOverflow <= 1 && bodyOverflow <= 1,
    default_submenus_closed_ready: defaultSubmenusClosedReady,
    default_submenus_closed_details: defaultSubmenuDetails,
    single_submenu_audit_ready: singleSubmenuAuditReady,
    unavailable_submenu_items_ready: unavailableSubmenuItemsReady,
    disabled_submenu_item_count: disabledSubmenuItemCount,
    row_menu_distinct_positions_ready: rowMenuDistinctPositionsReady,
    mobile_pane_navigation_ready: mobilePaneNavigationReady,
    mobile_pane_route_details: mobilePaneRouteDetails,
    mobile_pane_row_menu_ready: mobilePaneRowMenuReady,
    single_submenu_audit_target_count: singleSubmenuAuditDetails.length,
    single_submenu_audit_details: singleSubmenuAuditDetails,
    engineering_session_chips_suppressed_ready: engineeringSessionChipsSuppressedReady,
    engineering_session_chip_details: engineeringSessionChipDetails,
    narrow_composer_non_overlap_ready: narrowComposerNonOverlapReady,
    narrow_composer_non_overlap_details: narrowComposerNonOverlapDetails,
    preferred_touch_target_ready: preferredTouchTargetReady,
	    control_glass_action_ready: controlGlassActionReady,
	    harsh_referee_ready: harshRefereeReady,
	    shallow_light_glass_ready: shallowLightGlassReady,
	    light_theme_semantics_ready: lightThemeSemanticsReady,
	    light_theme_semantics_details: lightThemeSemanticsDetails,
	    stable_content_surface_ready: stableContentSurfaceReady,
	    stable_content_surface_details: stableContentSurfaceDetails,
	    native_popover_interaction_ready: nativePopoverInteractionReady,
	    native_popover_compatibility_source: "native_actual_click_single_submenu_audit",
	    legacy_menu_compatibility_uses_actual_click: actualClickMenuCompatibilityReady,
	    shallow_floating_surface_ready: shallowFloatingSurfaceReady,
	    floating_surface_details: floatingSurfaceDetails,
	    restrained_optics_ready: restrainedOpticsReady,
	    restrained_mobile_metadata_ready: restrainedMobileMetadataReady,
	    restrained_mobile_metadata_details: mobileSecondaryMetadataDetails,
	    visible_mobile_status_count: visibleMobileStatusCount,
	    key_touch_controls_ready: keyTouchControlsReady,
	    key_touch_control_details: keyTouchControlDetails,
	    legacy_extreme_optics_diagnostic_ready: legacyExtremeOpticsReady,
    rail_visible: railVisible,
    rail_action_icon_ready: railActionIconReady,
    icon_button_ready: iconButtonReady,
    icon_button_details: iconButtonDetails,
    icon_prismatic_control_light_glass_ready: iconPrismaticControlLightGlassReady,
    icon_prismatic_control_details: iconPrismaticControlDetails,
    topbar_action_light_glass_ready: topbarActionLightGlassReady,
    topbar_action_details: visibleTopbarActionDetails,
    chrome_bar_translucency_light_glass_ready: chromeBarTranslucencyLightGlassReady,
    chrome_bar_translucency_details: chromeBarTranslucencyDetails,
    chrome_refractive_skin_light_glass_ready: chromeRefractiveSkinLightGlassReady,
    chrome_refractive_skin_details: chromeBarTranslucencyDetails,
    clear_white_balance_light_glass_ready: clearWhiteBalanceLightGlassReady,
    clear_white_balance_details: clearWhiteBalanceDetails,
    chamfer_cut_edge_light_glass_ready: chamferCutEdgeLightGlassReady,
    chamfer_cut_edge_details: chamferCutEdgeSurfaceDetails,
    prismatic_cut_edge_light_glass_ready: prismaticCutEdgeLightGlassReady,
    prismatic_cut_edge_details: chamferCutEdgeSurfaceDetails,
    pane_prismatic_perimeter_light_glass_ready: panePrismaticPerimeterLightGlassReady,
    pane_prismatic_perimeter_details: panePrismaticPerimeterDetails,
    composer_prismatic_control_light_glass_ready: composerPrismaticControlLightGlassReady,
    composer_prismatic_control_details: composerPrismaticControlDetails,
    primary_shell_light_glass_ready: primaryShellLightGlassReady,
    primary_shell_surface_details: primaryShellSurfaceDetails,
    translucent_shell_light_glass_ready: translucentShellLightGlassReady,
    translucent_glass_details: translucentGlassDetails,
	    refractive_depth_light_glass_ready: refractiveDepthLightGlassReady,
	    optical_clarity_light_glass_ready: opticalClarityLightGlassReady,
	    surface_clear_alpha_light_glass_ready: surfaceClearAlphaLightGlassReady,
	    substrate_caustic_field_light_glass_ready: substrateCausticFieldLightGlassReady,
		    specular_edge_light_glass_ready: specularEdgeLightGlassReady,
		    prismatic_dispersion_light_glass_ready: prismaticDispersionLightGlassReady,
		    caustic_highlight_light_glass_ready: causticHighlightLightGlassReady,
		    caustic_depth_shift_light_glass_ready: causticDepthShiftLightGlassReady,
		    caustic_depth_shift_key_count: causticDepthShiftKeyCount,
		    optical_thickness_tiers_light_glass_ready: opticalThicknessTiersLightGlassReady,
		    optical_thickness_blur_tier_count: opticalThicknessBlurTierCount,
		    optical_thickness_alpha_tier_count: opticalThicknessAlphaTierCount,
		    faceted_reflection_light_glass_ready: facetedReflectionLightGlassReady,
		    beveled_rim_light_glass_ready: beveledRimLightGlassReady,
		    refractive_depth_details: refractiveDepthDetails,
		    substrate_caustic_field_details: substrateCausticFieldDetails,
		    specular_edge_details: specularEdgeDetails,
		    prismatic_dispersion_details: prismaticDispersionDetails,
		    caustic_highlight_details: causticHighlightDetails,
		    caustic_depth_shift_details: causticDepthShiftDetails,
		    optical_thickness_tier_details: opticalThicknessTierDetails,
		    faceted_reflection_details: facetedReflectionDetails,
		    beveled_rim_details: beveledRimDetails,
		    surface_clear_alpha_details: surfaceClearAlphaDetails,
		    micro_refraction_light_glass_ready: microRefractionLightGlassReady,
		    micro_refraction_details: microRefractionDetails,
		    sparkle_glint_light_glass_ready: sparkleGlintLightGlassReady,
		    sparkle_glint_details: sparkleGlintDetails,
		    lens_bloom_light_glass_ready: lensBloomLightGlassReady,
		    lens_bloom_details: lensBloomDetails,
		    spectral_fusion_light_glass_ready: spectralFusionLightGlassReady,
		    spectral_fusion_details: spectralFusionDetails,
		    optical_magnification_light_glass_ready: opticalMagnificationLightGlassReady,
		    optical_magnification_details: opticalMagnificationDetails,
		    biaxial_magnification_light_glass_ready: biaxialMagnificationLightGlassReady,
		    biaxial_magnification_details: biaxialMagnificationDetails,
		    anisotropic_magnification_light_glass_ready: anisotropicMagnificationLightGlassReady,
		    anisotropic_magnification_details: anisotropicMagnificationDetails,
		    phase_separated_refraction_light_glass_ready: phaseSeparatedRefractionLightGlassReady,
		    phase_separated_refraction_details: phaseSeparatedRefractionDetails,
		    two_axis_phase_refraction_light_glass_ready: twoAxisPhaseRefractionLightGlassReady,
		    two_axis_phase_refraction_details: twoAxisPhaseRefractionDetails,
		    surface_phase_drift_light_glass_ready: surfacePhaseDriftLightGlassReady,
		    surface_phase_drift_position_count: surfacePhaseDriftPositionCount,
		    surface_phase_drift_details: surfacePhaseDriftDetails,
		    surface_lens_scale_drift_light_glass_ready: surfaceLensScaleDriftLightGlassReady,
		    surface_lens_scale_drift_size_count: surfaceLensScaleDriftSizeCount,
		    surface_lens_scale_drift_details: surfaceLensScaleDriftDetails,
		    layer_scale_parallax_light_glass_ready: layerScaleParallaxLightGlassReady,
		    layer_scale_parallax_details: layerScaleParallaxDetails,
		    surface_spectral_angle_drift_light_glass_ready: surfaceSpectralAngleDriftLightGlassReady,
		    surface_spectral_angle_drift_signature_count: surfaceSpectralAngleDriftSignatureCount,
		    surface_spectral_angle_drift_details: surfaceSpectralAngleDriftDetails,
		    surface_glint_focal_drift_light_glass_ready: surfaceGlintFocalDriftLightGlassReady,
		    surface_glint_focal_drift_signature_count: surfaceGlintFocalDriftSignatureCount,
		    surface_glint_focal_drift_details: surfaceGlintFocalDriftDetails,
		    composer_glint_focal_decoupling_light_glass_ready: composerGlintFocalDecouplingLightGlassReady,
		    composer_glint_focal_decoupling_details: composerGlintFocalDecouplingDetails,
		    composer_spectral_angle_decoupling_light_glass_ready: composerSpectralAngleDecouplingLightGlassReady,
		    composer_spectral_angle_decoupling_details: composerSpectralAngleDecouplingDetails,
		    composer_phase_decoupling_light_glass_ready: composerPhaseDecouplingLightGlassReady,
		    composer_phase_decoupling_details: composerPhaseDecouplingDetails,
		    composer_layer_scale_decoupling_light_glass_ready: composerLayerScaleDecouplingLightGlassReady,
		    composer_layer_scale_decoupling_details: composerLayerScaleDecouplingDetails,
	      menu_trigger_ready: menuTriggerReady,
    menu_trigger_details: menuTriggerDetails,
	    folder_chip_touch_ready: folderChipTouchReady,
	    folder_chip_label_prismatic_etch_light_glass_ready: folderChipLabelPrismaticEtchLightGlassReady,
	    folder_chip_details: folderChipDetails,
	    row_menu_touch_ready: actualClickRowMenuCompatibilityReady,
	    row_menu_all_rows_ready: actualClickRowMenuCompatibilityReady,
	    row_menu_light_glass_ready: actualClickRowMenuCompatibilityReady,
	    row_menu_toggle_details: railVisible ? rowMenuToggleDetails : [],
    row_menu_panel_details: railVisible ? visibleRowMenuPanelDetails : [],
    row_menu_visible_item_count: visibleRowMenuItemDetails.length,
    row_menu_item_details: railVisible ? visibleRowMenuItemDetails : [],
	    command_palette_ready: actualClickCommandPaletteCompatibilityReady,
	    command_palette_surface_light_glass_ready: actualClickCommandPaletteCompatibilityReady,
	    command_palette_surface_prismatic_perimeter_light_glass_ready: commandPaletteSurfacePrismaticPerimeterLightGlassReady,
	    command_palette_backdrop_caustic_veil_light_glass_ready: commandPaletteBackdropCausticVeilLightGlassReady,
	    command_palette_trigger_light_glass_ready: commandPaletteTriggerLightGlassReady,
	    command_palette_close_light_glass_ready: actualClickCommandPaletteCompatibilityReady,
	    command_palette_close_prismatic_icon_light_glass_ready: commandPaletteClosePrismaticIconLightGlassReady,
			    command_palette_input_light_glass_ready: actualClickCommandPaletteCompatibilityReady,
			    command_palette_input_text_prismatic_etch_light_glass_ready: commandPaletteInputTextPrismaticEtchLightGlassReady,
			    command_palette_input_placeholder_prismatic_etch_light_glass_ready: commandPaletteInputPlaceholderPrismaticEtchLightGlassReady,
			    command_palette_input_row_prismatic_separator_light_glass_ready: commandPaletteInputRowPrismaticSeparatorLightGlassReady,
			    command_palette_results_well_light_glass_ready: commandPaletteResultsWellLightGlassReady,
			    command_palette_results_well_prismatic_rim_light_glass_ready: commandPaletteResultsWellPrismaticRimLightGlassReady,
				    command_palette_input_icon_light_glass_ready: commandPaletteInputIconLightGlassReady,
				    command_palette_input_icon_prismatic_light_glass_ready: commandPaletteInputIconPrismaticLightGlassReady,
			    command_palette_item_light_glass_ready: actualClickCommandPaletteCompatibilityReady,
			    command_palette_item_prismatic_rim_light_glass_ready: commandPaletteItemPrismaticRimLightGlassReady,
		    command_palette_kind_chip_light_glass_ready: commandPaletteKindChipLightGlassReady,
		    command_palette_item_hover_prismatic_light_glass_ready: commandPaletteItemHoverPrismaticLightGlassReady,
		    command_palette_item_label_prismatic_etch_light_glass_ready: commandPaletteItemLabelPrismaticEtchLightGlassReady,
	    command_palette_panel_details: commandPalettePanelDetails,
	    command_palette_backdrop_details: commandPaletteBackdropDetails,
	    command_palette_close_details: commandPaletteCloseDetails,
	    command_palette_trigger_details: visibleCommandPaletteTriggerDetails,
		    command_palette_input_details: commandPaletteInputDetails,
		    command_palette_input_row_details: commandPaletteInputRowDetails,
		    command_palette_results_well_details: commandPaletteResultsWellDetails,
		    command_palette_input_icon_details: commandPaletteInputIconDetails,
		    command_palette_item_details: commandPaletteItemDetails.slice(0, 2),
		    control_form_control_title_touch_ready: controlFormControlReady,
		    control_form_control_details: controlFormControlDetails,
		    chat_row_option_semantic_touch_ready: chatRowOptionSemanticTouchReady,
		    chat_row_option_details: chatRowOptionDetails,
		    rail_chat_row_prismatic_slab_light_glass_ready: railChatRowPrismaticSlabLightGlassReady,
		    rail_chat_row_prismatic_slab_details: chatRowOptionDetails.map((item) => ({
		      key: item.key,
		      active: item.active,
		      visible: item.visible,
		      width: item.width,
		      height: item.height,
		      border_radius: item.border_radius,
		      box_shadow: item.box_shadow,
		      backdrop_filter: item.backdrop_filter,
		      filter: item.filter,
		      filter_sample: item.filter_sample,
		      chat_row_drop_shadow_count: item.chat_row_drop_shadow_count,
		      chat_row_prismatic_slab_ready: item.chat_row_prismatic_slab_ready,
		    })),
	    menu_item_icon_ready: actualClickMenuCompatibilityReady,
    menu_item_details: menuItemDetails,
    menu_surface_ready: actualClickMenuCompatibilityReady,
    menu_surface_details: menuSurfaceDetails,
    thread_tools_menu_ready: actualClickThreadToolsCompatibilityReady,
    thread_tools_trigger_details: threadToolsTriggerDetails,
    thread_tools_panel_details: threadToolsPanelDetails,
    thread_tools_item_details: threadToolsItemDetails,
    composer_tools_menu_ready: actualClickComposerToolsCompatibilityReady,
    composer_tools_trigger_light_glass_ready: composerToolsTriggerLightGlassReady,
    composer_tools_trigger_details: composerToolsTriggerDetails,
    composer_tools_panel_details: composerToolsPanelDetails,
    composer_tools_item_details: composerToolsItemDetails,
    composer_popover_ready: actualClickComposerPopoverCompatibilityReady,
    composer_popover_item_label_prismatic_etch_light_glass_ready: composerPopoverItemLabelPrismaticEtchLightGlassReady,
    composer_popover_header_prismatic_etch_light_glass_ready: composerPopoverHeaderPrismaticEtchLightGlassReady,
    composer_popover_header_prismatic_etch_details: composerPopoverHeaderDetails,
    composer_popover_search_light_glass_ready: actualClickComposerPopoverCompatibilityReady,
    composer_popover_search_placeholder_prismatic_etch_light_glass_ready: composerPopoverSearchPlaceholderPrismaticEtchLightGlassReady,
    rail_search_light_glass_ready: railSearchLightGlassReady,
    rail_search_placeholder_prismatic_etch_light_glass_ready: railSearchPlaceholderPrismaticEtchLightGlassReady,
    rail_search_placeholder_prismatic_etch_details: railSearchPlaceholderPrismaticEtchDetails,
    rail_prismatic_filter_light_glass_ready: railPrismaticFilterLightGlassReady,
    rail_prismatic_filter_details: railPrismaticFilterDetails.map((item) => ({
      kind: item.kind,
      key: item.key || item.marker || "",
      text: item.text || item.placeholder || "",
      visible: item.visible,
      width: item.width,
      height: item.height,
      border_radius: item.border_radius,
      backdrop_filter: item.backdrop_filter,
      box_shadow: item.box_shadow,
      filter: item.filter,
      filter_sample: item.filter_sample,
      rail_filter_drop_shadow_count: item.rail_filter_drop_shadow_count,
      rail_prismatic_filter_ready: item.rail_prismatic_filter_ready,
    })),
    rail_search_visible_count: visibleRailSearchDetails.length,
    rail_search_details: railSearchDetails.map((item) => ({
      marker: item.marker,
      visible: item.visible,
      type: item.type,
      placeholder: item.placeholder,
      width: item.width,
      height: item.height,
      border_radius: item.border_radius,
      effective_luminance: item.effective_luminance,
      light_glass_ready: item.light_glass_ready,
      backdrop_filter: item.backdrop_filter,
      box_shadow: item.box_shadow,
      aria_label: item.aria_label,
      title: item.title,
      title_matches_aria_label: item.title_matches_aria_label,
      readable: item.readable,
      contrast_ratio: item.contrast_ratio,
      placeholder_readable: item.placeholder_readable,
      placeholder_contrast_ratio: item.placeholder_contrast_ratio,
      placeholder_text_shadow: item.placeholder_text_shadow,
      rail_search_placeholder_text_shadow_count: item.rail_search_placeholder_text_shadow_count,
      rail_search_placeholder_prismatic_etch_ready: item.rail_search_placeholder_prismatic_etch_ready,
    })),
    composer_popover_toggle_details: composerPopoverToggleDetails.map((item) => ({
      key: item.key,
      visible: item.visible,
      width: item.width,
      height: item.height,
      aria_label: item.aria_label,
      title: item.title,
      title_matches_aria_label: item.title_matches_aria_label,
      aria_haspopup: item.aria_haspopup,
      aria_controls: item.aria_controls,
      svg_icon_present: item.svg_icon_present,
      visible_icon_text_absent: item.visible_icon_text_absent,
    })),
    composer_popover_panel_details: composerPopoverPanelDetails.map((item) => ({
      key: item.key,
      role: item.role,
      aria_label: item.aria_label,
      visible: item.visible,
      search_count: item.search_count,
      item_count: item.item_count,
      width: item.width,
      height: item.height,
      border_radius: item.border_radius,
      effective_luminance: item.effective_luminance,
      light_glass_ready: item.light_glass_ready,
      backdrop_filter: item.backdrop_filter,
      box_shadow: item.box_shadow,
      in_viewport: item.in_viewport,
      top_clipped: item.top_clipped,
      bottom_clipped: item.bottom_clipped,
    })),
    composer_popover_search_details: composerPopoverSearchDetails.map((item) => ({
      key: item.key,
      marker: item.marker,
      placeholder: item.placeholder,
      visible: item.visible,
      height: item.height,
      border_radius: item.border_radius,
      effective_luminance: item.effective_luminance,
      light_glass_ready: item.light_glass_ready,
      backdrop_filter: item.backdrop_filter,
      box_shadow: item.box_shadow,
      aria_label: item.aria_label,
      title: item.title,
      title_matches_aria_label: item.title_matches_aria_label,
      readable: item.readable,
      contrast_ratio: item.contrast_ratio,
      placeholder_readable: item.placeholder_readable,
      placeholder_contrast_ratio: item.placeholder_contrast_ratio,
      placeholder_text_shadow: item.placeholder_text_shadow,
      placeholder_text_shadow_sample: item.placeholder_text_shadow_sample,
      composer_popover_search_placeholder_text_shadow_count: item.composer_popover_search_placeholder_text_shadow_count,
      composer_popover_search_placeholder_prismatic_etch_ready: item.composer_popover_search_placeholder_prismatic_etch_ready,
    })),
    composer_popover_item_details: composerPopoverItemDetails.map((item) => ({
      key: item.key,
      role: item.role,
      visible: item.visible,
      width: item.width,
      height: item.height,
      aria_label: item.aria_label,
      title: item.title,
      title_matches_aria_label: item.title_matches_aria_label,
      label: item.label,
      detail: item.detail,
      icon_svg_present: item.icon_svg_present,
      label_nowrap_ready: item.label_nowrap_ready,
      detail_nowrap_ready: item.detail_nowrap_ready,
      background_alpha: item.background_alpha,
      translucent_ready: item.translucent_ready,
      effective_luminance: item.effective_luminance,
      light_glass_ready: item.light_glass_ready,
      backdrop_filter: item.backdrop_filter,
      box_shadow: item.box_shadow,
      readable: item.readable,
      contrast_ratio: item.contrast_ratio,
      detail_readable: item.detail_readable,
      detail_contrast_ratio: item.detail_contrast_ratio,
      label_text_shadow: item.label_text_shadow,
      label_text_shadow_sample: item.label_text_shadow_sample,
      detail_text_shadow: item.detail_text_shadow,
      detail_text_shadow_sample: item.detail_text_shadow_sample,
      composer_popover_item_label_text_shadow_count: item.composer_popover_item_label_text_shadow_count,
      composer_popover_item_detail_text_shadow_count: item.composer_popover_item_detail_text_shadow_count,
      composer_popover_item_label_prismatic_etch_ready: item.composer_popover_item_label_prismatic_etch_ready,
    })),
    composer_popover_header_prismatic_etch_light_glass_ready: composerPopoverHeaderPrismaticEtchLightGlassReady,
    composer_popover_header_prismatic_etch_details: composerPopoverHeaderDetails.map((item) => ({
      key: item.key,
      label: item.label,
      status: item.status,
      visible: item.visible,
      label_visible: item.label_visible,
      status_visible: item.status_visible,
      label_text_shadow: item.label_text_shadow,
      label_text_shadow_sample: item.label_text_shadow_sample,
      status_text_shadow: item.status_text_shadow,
      status_text_shadow_sample: item.status_text_shadow_sample,
      composer_popover_header_label_text_shadow_count: item.composer_popover_header_label_text_shadow_count,
      composer_popover_header_status_text_shadow_count: item.composer_popover_header_status_text_shadow_count,
      composer_popover_header_prismatic_etch_ready: item.composer_popover_header_prismatic_etch_ready,
      label_readable: item.label_readable,
      status_readable: item.status_readable,
      label_contrast_ratio: item.label_contrast_ratio,
      status_contrast_ratio: item.status_contrast_ratio,
      width: item.width,
      height: item.height,
    })),
    micro_surface_light_glass_ready: microSurfaceLightGlassReady,
    micro_prismatic_badge_light_glass_ready: microPrismaticBadgeLightGlassReady,
    micro_badge_label_prismatic_etch_light_glass_ready: microBadgeLabelPrismaticEtchLightGlassReady,
    message_metadata_prismatic_light_glass_ready: messageMetadataPrismaticLightGlassReady,
    message_metadata_prismatic_details: messageMetadataPrismaticDetails,
    thread_subtitle_prismatic_light_glass_ready: threadSubtitlePrismaticLightGlassReady,
    thread_subtitle_prismatic_details: threadSubtitlePrismaticDetails,
    composer_shortcut_hint_prismatic_light_glass_ready: composerShortcutHintPrismaticLightGlassReady,
    composer_shortcut_hint_expected_visible: composerShortcutHintExpectedVisible,
    composer_shortcut_hint_prismatic_details: composerShortcutHintPrismaticDetails,
    rail_metadata_chip_prismatic_light_glass_ready: railMetadataChipPrismaticLightGlassReady,
    rail_metadata_chip_expected_visible: railMetadataChipExpectedVisible,
    rail_metadata_chip_prismatic_details: railMetadataChipPrismaticDetails,
	    rail_status_count_prismatic_light_glass_ready: railStatusCountPrismaticLightGlassReady,
	    rail_status_count_expected_visible: railStatusCountExpectedVisible,
	      rail_status_count_prismatic_details: railStatusCountPrismaticDetails,
	      rail_preview_prismatic_etch_light_glass_ready: railPreviewPrismaticEtchLightGlassReady,
	      rail_preview_expected_visible: railPreviewExpectedVisible,
	      rail_preview_prismatic_etch_details: railPreviewPrismaticEtchDetails,
	      rail_chat_title_prismatic_etch_light_glass_ready: railChatTitlePrismaticEtchLightGlassReady,
	      rail_chat_title_expected_visible: railChatTitleExpectedVisible,
	      rail_chat_title_prismatic_etch_details: railChatTitlePrismaticEtchDetails,
	      message_body_prismatic_etch_light_glass_ready: messageBodyPrismaticEtchLightGlassReady,
	    message_body_prismatic_etch_details: messageBodyPrismaticEtchDetails,
	    message_speaker_prismatic_chip_light_glass_ready: messageSpeakerPrismaticChipLightGlassReady,
	    message_speaker_prismatic_chip_details: messageSpeakerPrismaticChipDetails,
	    composer_placeholder_prismatic_etch_light_glass_ready: composerPlaceholderPrismaticEtchLightGlassReady,
	    composer_placeholder_prismatic_etch_details: composerPlaceholderPrismaticEtchDetails,
	    header_title_prismatic_etch_light_glass_ready: headerTitlePrismaticEtchLightGlassReady,
    header_title_expected_count: headerTitleExpectedCount,
    header_title_prismatic_etch_details: headerTitlePrismaticEtchDetails,
    micro_surface_details: microSurfaceDetails.map((item) => ({
      key: item.key,
      text: item.text,
      visible: item.visible,
      width: item.width,
      height: item.height,
      border_radius: item.border_radius,
      effective_luminance: item.effective_luminance,
      light_glass_ready: item.light_glass_ready,
      backdrop_filter: item.backdrop_filter,
      box_shadow: item.box_shadow,
      filter: item.filter,
      filter_sample: item.filter_sample,
      micro_prismatic_badge_drop_shadow_count: item.micro_prismatic_badge_drop_shadow_count,
      micro_prismatic_badge_ready: item.micro_prismatic_badge_ready,
      text_shadow: item.text_shadow,
      text_shadow_sample: item.text_shadow_sample,
      micro_badge_label_text_shadow_count: item.micro_badge_label_text_shadow_count,
      micro_badge_label_prismatic_etch_ready: item.micro_badge_label_prismatic_etch_ready,
      readable: item.readable,
      contrast_ratio: item.contrast_ratio,
      label_nowrap_ready: item.label_nowrap_ready,
    })),
    message_routing_badge_light_glass_ready: messageRoutingBadgeLightGlassReady,
    message_routing_badge_details: routingBadgeDetails.map((item) => ({
      key: item.key,
      text: item.text,
      visible: item.visible,
      width: item.width,
      height: item.height,
      border_radius: item.border_radius,
      effective_luminance: item.effective_luminance,
      light_glass_ready: item.light_glass_ready,
      backdrop_filter: item.backdrop_filter,
      box_shadow: item.box_shadow,
      readable: item.readable,
      contrast_ratio: item.contrast_ratio,
      label_nowrap_ready: item.label_nowrap_ready,
    })),
    thread_intro_badge_light_glass_ready: threadIntroBadgeLightGlassReady,
    thread_intro_badge_visible: threadIntroVisible,
    thread_intro_badge_details: threadIntroBadgeDetails.map((item) => ({
      key: item.key,
      text: item.text,
      visible: item.visible,
      width: item.width,
      height: item.height,
      border_radius: item.border_radius,
      effective_luminance: item.effective_luminance,
      light_glass_ready: item.light_glass_ready,
      backdrop_filter: item.backdrop_filter,
      box_shadow: item.box_shadow,
      readable: item.readable,
      contrast_ratio: item.contrast_ratio,
      label_nowrap_ready: item.label_nowrap_ready,
    })),
    status_trust_strip_light_glass_ready: statusTrustStripLightGlassReady,
    status_trust_strip_visible: statusTrustStripVisible,
    status_trust_badge_details: statusTrustBadgeDetails.map((item) => ({
      key: item.key,
      micro_surface_key: item.micro_surface_key,
      text: item.text,
      visible: item.visible,
      width: item.width,
      height: item.height,
      border_radius: item.border_radius,
      effective_luminance: item.effective_luminance,
      light_glass_ready: item.light_glass_ready,
      backdrop_filter: item.backdrop_filter,
      box_shadow: item.box_shadow,
      aria_label: item.aria_label,
      title: item.title,
      title_matches_aria_label: item.title_matches_aria_label,
      readable: item.readable,
      contrast_ratio: item.contrast_ratio,
      label_nowrap_ready: item.label_nowrap_ready,
    })),
    nav_icon_ready: navIconReady,
	    scroll_edge_ready: scrollEdgeReady,
	    microcopy_word_split_guard_ready: microcopyWrapReady,
	    microcopy_wrap_details: microcopyWrapDetails.slice(0, 8),
		    logo_clip_guard_ready: logoClipReady,
		    logo_clip_details: logoClipDetails,
		    avatar_prismatic_rim_light_glass_ready: avatarPrismaticRimLightGlassReady,
		    avatar_prismatic_rim_details: avatarPrismaticRimDetails,
		    active_chat_readability_ready: activeChatReadabilityReady,
			    active_chat_readability_details: activeChatReadabilityDetails.slice(0, 4),
		    placeholder_readability_ready: placeholderReadabilityReady,
			    placeholder_readability_details: placeholderReadabilityDetails.slice(0, 2),
		    small_control_readability_ready: smallControlReadabilityReady,
			    small_control_readability_details: smallControlReadabilityDetails.slice(0, 4),
		    visible_text_integrity_ready: visibleTextIntegrityReady,
		    visible_text_integrity_probe: {
		      expected: visibleTextIntegrityExpected,
		      actual: visibleTextIntegritySample,
		    },
		    message_speaker_prismatic_chip_light_glass_ready: messageSpeakerPrismaticChipLightGlassReady,
		    message_speaker_prismatic_chip_details: messageSpeakerPrismaticChipDetails.map((item) => ({
		      text: item.text,
		      visible: item.visible,
		      width: item.width,
		      height: item.height,
		      border_radius: item.border_radius,
		      background_alpha: item.background_alpha,
		      effective_luminance: item.effective_luminance,
		      backdrop_filter: item.backdrop_filter,
		      box_shadow: item.box_shadow,
		      filter: item.filter,
		      message_speaker_chip_drop_shadow_count: item.message_speaker_chip_drop_shadow_count,
		      message_speaker_prismatic_chip_ready: item.message_speaker_prismatic_chip_ready,
		      readable: item.readable,
		      contrast_ratio: item.contrast_ratio,
		      label_nowrap_ready: item.label_nowrap_ready,
		    })),
		    composer_glass_ready: composerGlassReady,
    send_glass_ready: sendGlassReady,
    selectors: selectors.filter((item) => [
      "[data-agent-chat-send]",
      "[data-chat-composer-input]",
      ".tg-compose-bar",
    ].includes(item.selector)),
    errors,
  };
})()

`;
