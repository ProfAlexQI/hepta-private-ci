# Checks the three machine-readable browser probes without altering their
# historical fail-closed jq contract.
hepta_browser_validate_results() {
if [[ "$progressive_qa_status" != "0" ]] || ! jq -e '
  .status == "ready"
  and .registry_route_count == 21
  and .successful_route_count == 21
  and .snapshot_request_count == 1
  and .copy_interaction_ready == true
  and .chat_search_ready == true
  and .command_palette_search_ready == true
  and .unavailable_controls_ready == true
  and .unavailable_click_noop_ready == true
  and .seeded_conversations_ready == true
  and .local_json_inspector_ready == true
  and .composer_picker_search_ready == true
  and .local_draft_insertion_ready == true
  and .local_route_navigation_ready == true
  and .allowed_button_count == 21
  and .unsafe_enabled_button_count == 0
  and .cross_origin_request_count == 0
  and .non_get_request_count == 0
  and .unexpected_api_request_count == 0
  and (.missing_or_duplicate_api_paths | length) == 0
  and .console_error_count == 0
  and .live_adapter_bound == false
  and .mutation_endpoint_called == false
' <<<"$progressive_qa_json" >/dev/null; then
  echo "Control UI progressive-enhancement browser QA failed" >&2
  jq '.' <<<"$progressive_qa_json" >&2 || true
  exit 1
fi

progressive_adversarial_qa_status=0
progressive_adversarial_qa_json="$(node "$HEPTA_BROWSER_SMOKE_LIB_DIR/progressive-enhancement-adversarial-qa.cjs" "$CHROME_BIN" "$BASE_URL")" \
  || progressive_adversarial_qa_status="$?"
printf '%s\n' "$progressive_adversarial_qa_json" \
  >"$OUT_DIR/progressive-enhancement-adversarial-qa.json"

if [[ "$progressive_adversarial_qa_status" != "0" ]] || ! jq -e '
  .status == "ready"
  and .race.latest_result_retained == true
  and .race.old_card_state == "superseded"
  and .stale_timeout.stale_timeout_suppressed == true
  and .oversized_utf8.rejected == true
  and .oversized_content_length.rejected == true
  and .no_content_length_stream.rejected_at_bound == true
  and .jsonp.rejected == true
  and .structured_json.accepted == true
  and .redirect.blocked == true
  and .redirect.external_request_count == 0
  and .xss.blocked == true
  and .no_script_product_truth.ready == true
  and .no_script_product_truth.unavailable_control_count == 99
  and .no_script_product_truth.disabled_click_count == 0
  and .no_script_product_truth.api_request_count == 0
  and .no_script_product_truth.non_get_request_count == 0
  and (.failures | length) == 0
' <<<"$progressive_adversarial_qa_json" >/dev/null; then
  echo "Control UI progressive-enhancement adversarial QA failed" >&2
  jq '.' <<<"$progressive_adversarial_qa_json" >&2 || true
  exit 1
fi

if [[ "$density_qa_status" != "0" ]] || ! jq -e '
	  (
	    .status == "ready"
	    and .control_ui_visual_density_qa_ready == true
	    and .viewport_count == 4
	    and .phone320_ready == true
	    and .default_submenus_closed_ready == true
	    and .single_submenu_audit_ready == true
	    and .unavailable_submenu_items_ready == true
	    and .disabled_submenu_item_count >= 8
	    and .engineering_session_chips_suppressed_ready == true
	    and .shallow_light_glass_ready == true
	    and .light_theme_semantics_ready == true
	    and .stable_content_surface_ready == true
	    and .native_popover_interaction_ready == true
	    and .shallow_floating_surface_ready == true
	    and .restrained_optics_ready == true
	    and .restrained_mobile_metadata_ready == true
	    and .key_touch_controls_ready == true
	    and .horizontal_overflow_free == true
	    and .browser_error_page_absent == true
	    and (.results | length) == 4
	    and (.results | all(.status == "ready"))
	  )
	  or (
	  false
	  and .rail_action_icon_ready == true
  and .icon_button_ready == true
  and .icon_prismatic_control_light_glass_ready == true
	  and .topbar_action_light_glass_ready == true
	  and .primary_shell_light_glass_ready == true
	  and .translucent_shell_light_glass_ready == true
	  and .refractive_depth_light_glass_ready == true
	  and .optical_clarity_light_glass_ready == true
	  and .surface_clear_alpha_light_glass_ready == true
	  and .substrate_caustic_field_light_glass_ready == true
	  and .specular_edge_light_glass_ready == true
	  and .prismatic_dispersion_light_glass_ready == true
	  and .caustic_highlight_light_glass_ready == true
	  and .caustic_depth_shift_light_glass_ready == true
		  and .optical_thickness_tiers_light_glass_ready == true
		  and .faceted_reflection_light_glass_ready == true
		  and .beveled_rim_light_glass_ready == true
		  and .micro_refraction_light_glass_ready == true
		  and .sparkle_glint_light_glass_ready == true
		  and .lens_bloom_light_glass_ready == true
		  and .spectral_fusion_light_glass_ready == true
		  and .optical_magnification_light_glass_ready == true
		  and .biaxial_magnification_light_glass_ready == true
		  and .anisotropic_magnification_light_glass_ready == true
		  and .phase_separated_refraction_light_glass_ready == true
		  and .two_axis_phase_refraction_light_glass_ready == true
		  and .surface_phase_drift_light_glass_ready == true
		  and .surface_lens_scale_drift_light_glass_ready == true
		  and .layer_scale_parallax_light_glass_ready == true
		  and .surface_spectral_angle_drift_light_glass_ready == true
		  and .surface_glint_focal_drift_light_glass_ready == true
		  and .composer_glint_focal_decoupling_light_glass_ready == true
		  and .composer_spectral_angle_decoupling_light_glass_ready == true
		  and .composer_phase_decoupling_light_glass_ready == true
		  and .composer_layer_scale_decoupling_light_glass_ready == true
		  and .chrome_bar_translucency_light_glass_ready == true
		  and .chrome_refractive_skin_light_glass_ready == true
		  and .clear_white_balance_light_glass_ready == true
		  and .chamfer_cut_edge_light_glass_ready == true
		  and .prismatic_cut_edge_light_glass_ready == true
		  and .pane_prismatic_perimeter_light_glass_ready == true
		  and .composer_prismatic_control_light_glass_ready == true
		  and .menu_trigger_ready == true
	  and .folder_chip_touch_ready == true
	  and .folder_chip_label_prismatic_etch_light_glass_ready == true
	  and .row_menu_touch_ready == true
		  and .row_menu_all_rows_ready == true
		  and .row_menu_light_glass_ready == true
	  and .command_palette_ready == true
	  and .command_palette_surface_light_glass_ready == true
	  and .command_palette_surface_prismatic_perimeter_light_glass_ready == true
	  and .command_palette_backdrop_caustic_veil_light_glass_ready == true
	  and .command_palette_trigger_light_glass_ready == true
	  and .command_palette_close_light_glass_ready == true
	  and .command_palette_close_prismatic_icon_light_glass_ready == true
	  and .command_palette_input_light_glass_ready == true
		  and .command_palette_input_text_prismatic_etch_light_glass_ready == true
		  and .command_palette_input_placeholder_prismatic_etch_light_glass_ready == true
			  and .command_palette_input_row_prismatic_separator_light_glass_ready == true
				  and .command_palette_results_well_light_glass_ready == true
				  and .command_palette_results_well_prismatic_rim_light_glass_ready == true
				  and .command_palette_input_icon_light_glass_ready == true
				  and .command_palette_input_icon_prismatic_light_glass_ready == true
				  and .command_palette_item_light_glass_ready == true
				  and .command_palette_item_prismatic_rim_light_glass_ready == true
			  and .command_palette_kind_chip_light_glass_ready == true
			  and .command_palette_item_hover_prismatic_light_glass_ready == true
			  and .command_palette_item_label_prismatic_etch_light_glass_ready == true
		  and .control_form_control_title_touch_ready == true
		  and .chat_row_option_semantic_touch_ready == true
		  and .rail_chat_row_prismatic_slab_light_glass_ready == true
		  and .menu_item_icon_ready == true
  and .menu_surface_ready == true
	  and .thread_tools_menu_ready == true
	  and .composer_tools_menu_ready == true
	  and .composer_popover_ready == true
	  and .composer_popover_item_label_prismatic_etch_light_glass_ready == true
	  and .composer_popover_header_prismatic_etch_light_glass_ready == true
	  and .composer_popover_search_light_glass_ready == true
	  and .composer_popover_search_placeholder_prismatic_etch_light_glass_ready == true
	  and .rail_search_light_glass_ready == true
	  and .rail_search_placeholder_prismatic_etch_light_glass_ready == true
	  and .rail_prismatic_filter_light_glass_ready == true
	  and .micro_surface_light_glass_ready == true
	  and .micro_prismatic_badge_light_glass_ready == true
	  and .micro_badge_label_prismatic_etch_light_glass_ready == true
	  and .message_metadata_prismatic_light_glass_ready == true
	  and .thread_subtitle_prismatic_light_glass_ready == true
	  and .composer_shortcut_hint_prismatic_light_glass_ready == true
		  and .rail_metadata_chip_prismatic_light_glass_ready == true
	  and .rail_status_count_prismatic_light_glass_ready == true
	  and .rail_preview_prismatic_etch_light_glass_ready == true
	  and .rail_chat_title_prismatic_etch_light_glass_ready == true
	  and .message_body_prismatic_etch_light_glass_ready == true
		  and .message_speaker_prismatic_chip_light_glass_ready == true
		  and .composer_placeholder_prismatic_etch_light_glass_ready == true
		  and .header_title_prismatic_etch_light_glass_ready == true
	  and .message_routing_badge_light_glass_ready == true
	  and .thread_intro_badge_light_glass_ready == true
	  and .status_trust_strip_light_glass_ready == true
	  and .nav_icon_ready == true
		  and .scroll_edge_ready == true
		  and .microcopy_word_split_guard_ready == true
		  and .logo_clip_guard_ready == true
		  and .avatar_prismatic_rim_light_glass_ready == true
		  and .active_chat_readability_ready == true
		  and .placeholder_readability_ready == true
		  and .small_control_readability_ready == true
		  and .visible_text_integrity_ready == true
		  and .horizontal_overflow_free == true
	  and .browser_error_page_absent == true
  and (.results | length) == 4
	  and (.results | all(.status == "ready"))
	  )
	' <<<"$density_qa_json" >/dev/null; then
  echo "control UI density QA failed" >&2
  jq '{
    status,
    failures,
    viewport_count,
    phone320_ready,
    default_submenus_closed_ready,
    single_submenu_audit_ready,
    engineering_session_chips_suppressed_ready,
    preferred_touch_targets_ready,
    control_glass_action_ready,
    harsh_referee_ready,
    rail_action_icon_ready,
    icon_button_ready,
    icon_prismatic_control_light_glass_ready,
    topbar_action_light_glass_ready,
    primary_shell_light_glass_ready,
	    translucent_shell_light_glass_ready,
	    refractive_depth_light_glass_ready,
	    optical_clarity_light_glass_ready,
	    surface_clear_alpha_light_glass_ready,
	    substrate_caustic_field_light_glass_ready,
	    specular_edge_light_glass_ready,
		    prismatic_dispersion_light_glass_ready,
		    caustic_highlight_light_glass_ready,
		    caustic_depth_shift_light_glass_ready,
		    menu_trigger_ready,
    folder_chip_touch_ready,
    folder_chip_label_prismatic_etch_light_glass_ready,
	    row_menu_touch_ready,
		    row_menu_all_rows_ready,
		    row_menu_light_glass_ready,
		    command_palette_ready,
		    command_palette_surface_prismatic_perimeter_light_glass_ready,
		    command_palette_trigger_light_glass_ready,
	    command_palette_close_light_glass_ready,
		        command_palette_input_light_glass_ready,
		        command_palette_input_text_prismatic_etch_light_glass_ready,
		        command_palette_input_placeholder_prismatic_etch_light_glass_ready,
		        command_palette_input_row_prismatic_separator_light_glass_ready,
	    command_palette_item_light_glass_ready,
	    control_form_control_title_touch_ready,
	    chat_row_option_semantic_touch_ready,
	    rail_chat_row_prismatic_slab_light_glass_ready,
	    menu_item_icon_ready,
    menu_surface_ready,
    thread_tools_menu_ready,
    composer_tools_menu_ready,
    composer_popover_ready,
    rail_search_light_glass_ready,
    rail_prismatic_filter_light_glass_ready,
    micro_surface_light_glass_ready,
    micro_prismatic_badge_light_glass_ready,
    micro_badge_label_prismatic_etch_light_glass_ready,
    message_metadata_prismatic_light_glass_ready,
    thread_subtitle_prismatic_light_glass_ready,
    composer_shortcut_hint_prismatic_light_glass_ready,
    rail_metadata_chip_prismatic_light_glass_ready,
    rail_chat_title_prismatic_etch_light_glass_ready,
    message_routing_badge_light_glass_ready,
    thread_intro_badge_light_glass_ready,
	    status_trust_strip_light_glass_ready,
	    faceted_reflection_light_glass_ready,
	    beveled_rim_light_glass_ready,
	    micro_refraction_light_glass_ready,
	    sparkle_glint_light_glass_ready,
	    lens_bloom_light_glass_ready,
	    spectral_fusion_light_glass_ready,
	    optical_magnification_light_glass_ready,
	    biaxial_magnification_light_glass_ready,
	    anisotropic_magnification_light_glass_ready,
	    phase_separated_refraction_light_glass_ready,
	    two_axis_phase_refraction_light_glass_ready,
	    surface_phase_drift_light_glass_ready,
	    surface_lens_scale_drift_light_glass_ready,
	    layer_scale_parallax_light_glass_ready,
	    surface_spectral_angle_drift_light_glass_ready,
	    surface_glint_focal_drift_light_glass_ready,
	    composer_glint_focal_decoupling_light_glass_ready,
	    composer_spectral_angle_decoupling_light_glass_ready,
	    composer_phase_decoupling_light_glass_ready,
	    composer_layer_scale_decoupling_light_glass_ready,
	    chrome_bar_translucency_light_glass_ready,
	    chrome_refractive_skin_light_glass_ready,
	    clear_white_balance_light_glass_ready,
	    nav_icon_ready,
	    scroll_edge_ready,
	    microcopy_word_split_guard_ready,
	    logo_clip_guard_ready,
	    avatar_prismatic_rim_light_glass_ready,
	    active_chat_readability_ready,
		    placeholder_readability_ready,
		    small_control_readability_ready,
		    visible_text_integrity_ready,
		    horizontal_overflow_free,
    browser_error_page_absent,
    bad_viewports: [
      .results[] | select(.status != "ready" or .harsh_referee_ready != true) | {
        name,
        status,
        errors,
        default_submenus_closed_ready,
        default_submenus_closed_details,
        single_submenu_audit_ready,
        single_submenu_audit_target_count,
        single_submenu_audit_details,
        engineering_session_chips_suppressed_ready,
        engineering_session_chip_details,
        icon_button_ready,
        topbar_action_light_glass_ready,
        primary_shell_light_glass_ready,
	        translucent_shell_light_glass_ready,
	        refractive_depth_light_glass_ready,
	        optical_clarity_light_glass_ready,
	        surface_clear_alpha_light_glass_ready,
	        substrate_caustic_field_light_glass_ready,
	        chrome_refractive_skin_light_glass_ready,
	        clear_white_balance_light_glass_ready,
		        specular_edge_light_glass_ready,
		        prismatic_dispersion_light_glass_ready,
		        caustic_highlight_light_glass_ready,
		        caustic_depth_shift_light_glass_ready,
			        caustic_depth_shift_key_count,
			        faceted_reflection_light_glass_ready,
			        beveled_rim_light_glass_ready,
			        micro_refraction_light_glass_ready,
			        sparkle_glint_light_glass_ready,
			        lens_bloom_light_glass_ready,
			        spectral_fusion_light_glass_ready,
			        optical_magnification_light_glass_ready,
			        biaxial_magnification_light_glass_ready,
			        anisotropic_magnification_light_glass_ready,
			        phase_separated_refraction_light_glass_ready,
			        two_axis_phase_refraction_light_glass_ready,
			        surface_phase_drift_light_glass_ready,
			        surface_lens_scale_drift_light_glass_ready,
			        layer_scale_parallax_light_glass_ready,
			        surface_spectral_angle_drift_light_glass_ready,
			        surface_glint_focal_drift_light_glass_ready,
			        composer_glint_focal_decoupling_light_glass_ready,
			        refractive_depth_details,
			        surface_clear_alpha_details,
			        substrate_caustic_field_details,
		        specular_edge_details,
		        prismatic_dispersion_details,
		        caustic_highlight_details,
			        caustic_depth_shift_details,
			        faceted_reflection_details,
			        beveled_rim_details,
			        micro_refraction_details,
			        sparkle_glint_details,
			        lens_bloom_details,
			        spectral_fusion_details,
			        optical_magnification_details,
			        biaxial_magnification_details,
			        anisotropic_magnification_details,
			        phase_separated_refraction_details,
			        two_axis_phase_refraction_details,
			        surface_phase_drift_position_count,
			        surface_phase_drift_details,
			        surface_lens_scale_drift_size_count,
			        surface_lens_scale_drift_details,
			        layer_scale_parallax_details,
			        surface_spectral_angle_drift_details,
			        surface_glint_focal_drift_details,
			        composer_glint_focal_decoupling_details,
			        chrome_refractive_skin_details,
			        clear_white_balance_details,
			        chamfer_cut_edge_details,
			        menu_trigger_ready,
	        folder_chip_touch_ready,
	        folder_chip_label_prismatic_etch_light_glass_ready,
	        folder_chip_details,
	        row_menu_touch_ready,
	        row_menu_light_glass_ready,
	        command_palette_ready,
	        command_palette_surface_prismatic_perimeter_light_glass_ready,
	        command_palette_trigger_light_glass_ready,
	        command_palette_close_light_glass_ready,
	        command_palette_close_prismatic_icon_light_glass_ready,
	        command_palette_input_light_glass_ready,
	        command_palette_input_text_prismatic_etch_light_glass_ready,
	        command_palette_input_row_prismatic_separator_light_glass_ready,
	        command_palette_input_icon_light_glass_ready,
		        command_palette_item_light_glass_ready,
		        command_palette_item_label_prismatic_etch_light_glass_ready,
		        control_form_control_title_touch_ready,
		        chat_row_option_semantic_touch_ready,
		        rail_chat_row_prismatic_slab_light_glass_ready,
		        menu_item_icon_ready,
        thread_tools_menu_ready,
        composer_popover_item_label_prismatic_etch_light_glass_ready,
        composer_popover_search_light_glass_ready,
        composer_popover_search_placeholder_prismatic_etch_light_glass_ready,
        rail_search_light_glass_ready,
        rail_search_placeholder_prismatic_etch_light_glass_ready,
        rail_search_placeholder_prismatic_etch_details,
        rail_prismatic_filter_light_glass_ready,
        message_metadata_prismatic_light_glass_ready,
        message_metadata_prismatic_details,
        thread_subtitle_prismatic_light_glass_ready,
        thread_subtitle_prismatic_details,
        composer_shortcut_hint_prismatic_light_glass_ready,
        composer_shortcut_hint_prismatic_details,
        rail_metadata_chip_prismatic_light_glass_ready,
	        rail_metadata_chip_prismatic_details,
	        rail_status_count_prismatic_light_glass_ready,
	        rail_status_count_prismatic_details,
	        rail_preview_prismatic_etch_light_glass_ready,
	        rail_preview_prismatic_etch_details,
	        rail_chat_title_prismatic_etch_light_glass_ready,
	        rail_chat_title_prismatic_etch_details,
	        message_body_prismatic_etch_light_glass_ready,
	        message_body_prismatic_etch_details,
	        message_speaker_prismatic_chip_light_glass_ready,
	        message_speaker_prismatic_chip_details,
	        composer_placeholder_prismatic_etch_light_glass_ready,
	        composer_placeholder_prismatic_etch_details,
	        header_title_prismatic_etch_light_glass_ready,
        header_title_prismatic_etch_details,
        status_trust_strip_light_glass_ready,
        icon_button_title_match_ready,
        menu_trigger_title_match_ready,
        menu_surface_ready,
        nav_icon_ready,
        scroll_edge_ready,
        bad_icon_buttons: [
          (.icon_button_details // [])[] | select(.visible != true or .width < 44 or .height < 44 or ((.aria_label // "") | length) == 0 or ((.title // "") | length) == 0 or .title_matches_aria_label != true or .svg_icon_present != true or .visible_icon_text_absent != true)
        ],
        bad_icon_prismatic_controls: [
          (.icon_prismatic_control_details // [])[] | select(.icon_prismatic_control_ready != true or (.icon_prismatic_control_drop_shadow_count // 0) < 2 or .box_shadow == "none" or ((.backdrop_filter // "") | contains("blur(") | not))
        ],
        bad_translucent_glass: [
          (.translucent_glass_details // [])[] | select(.translucent_ready != true or .background_alpha < 0.35 or .background_alpha > 0.88 or ((.backdrop_filter // "") | contains("blur(") | not) or .box_shadow == "none")
        ],
        bad_refractive_depth: (
          .refractive_depth_details // {} | select(.body_background_image != "present" or .before_background_image != "present" or .before_opacity < 0.12 or .primary_shell_gradient_count < 3 or .primary_shell_low_alpha_count < 3)
        ),
	        bad_optical_clarity: (
	          .refractive_depth_details // {} | select(.body_background_translucent_layer != true or .body_background_layer_count < 3 or .before_opacity < 0.2 or .primary_shell_clear_alpha_count < 3)
	        ),
	        bad_surface_clear_alpha: [
	          (.surface_clear_alpha_details // [])[] | select(.clear_alpha_ready != true or (.surface_alpha_max // 1) > 0.49 or (.surface_alpha_average // 1) > 0.44 or (.surface_alpha_min // 1) > 0.4 or (.surface_alpha_below_045_count // 0) < ((.surface_count // 1) - 1) or .readable != true)
	        ],
	        bad_substrate_caustic_field: (
	          .substrate_caustic_field_details // {} | select(.body_background_layer_count < 4 or .body_background_repeating_layer_count < 2 or .body_background_angle_count < 4 or .body_background_translucent_layer != true or .before_opacity < 0.2)
	        ),
	        bad_chrome_refractive_skin: [
	          (.chrome_refractive_skin_details // [])[] | select(.refractive_chrome_ready != true or (.chrome_refraction_layer_count // 0) < 2 or (.chrome_refraction_repeating_layer_count // 0) < 1 or (.specular_layer_count // 0) < 2)
	        ],
	        bad_clear_white_balance: (
	          .clear_white_balance_details // {} | select(.body_clear_white_ready != true or .primary_clear_white_ready != true or .chrome_clear_white_ready != true or (.body_background_channel_spread // 255) > 10 or (.primary_surface_channel_spread_max // 255) > 10 or (.chrome_channel_spread_max // 255) > 10)
	        ),
	        bad_chamfer_cut_edge: [
	          (.chamfer_cut_edge_details // [])[] | select(.polygon_clip_ready != true or .box_shadow == "none")
	        ],
	        bad_prismatic_cut_edge: [
	          (.prismatic_cut_edge_details // [])[] | select(.prismatic_cut_edge_ready != true or (.cut_edge_drop_shadow_count // 0) < 2)
	        ],
	        bad_pane_prismatic_perimeter: [
	          (.pane_prismatic_perimeter_details // [])[] | select(.pane_prismatic_perimeter_ready != true or (.perimeter_drop_shadow_count // 0) < 2 or .box_shadow == "none")
	        ],
	        bad_composer_prismatic_control: [
	          (.composer_prismatic_control_details // [])[] | select(.composer_prismatic_control_ready != true or (.control_drop_shadow_count // 0) < 2 or .box_shadow == "none" or ((.backdrop_filter // "") | contains("blur(") | not))
	        ],
	        bad_specular_edge: [
	          (.specular_edge_details // [])[] | select(.specular_edge_ready != true)
	        ],
	        bad_prismatic_dispersion: [
	          (.prismatic_dispersion_details // [])[] | select(.prismatic_dispersion_ready != true)
	        ],
		        bad_caustic_highlight: [
		          (.caustic_highlight_details // [])[] | select(.caustic_highlight_ready != true)
		        ],
		        bad_caustic_depth_shift: (
		          if (.caustic_depth_shift_key_count // 0) < 2
		          then (.caustic_depth_shift_details // [])
		          else [(.caustic_depth_shift_details // [])[] | select(.caustic_highlight_ready != true)]
		          end
		        ),
		        bad_faceted_reflection: [
		          (.faceted_reflection_details // [])[] | select(.faceted_reflection_ready != true or .faceted_reflection_angle_count < 3)
		        ],
			        bad_beveled_rim: [
			          (.beveled_rim_details // [])[] | select(.beveled_rim_ready != true or .beveled_rim_layer_count < 5)
			        ],
			        bad_micro_refraction: [
			          (.micro_refraction_details // [])[] | select(.micro_refraction_ready != true or .micro_refraction_line_count < 1)
			        ],
			        bad_sparkle_glint: [
			          (.sparkle_glint_details // [])[] | select(.sparkle_glint_ready != true or .sparkle_glint_count < 1)
			        ],
			        bad_lens_bloom: [
			          (.lens_bloom_details // [])[] | select(.lens_bloom_ready != true or .lens_bloom_count < 2)
			        ],
			        bad_spectral_fusion: [
			          (.spectral_fusion_details // [])[] | select(.spectral_fusion_ready != true or .spectral_fusion_layer_count < 6 or ((.spectral_fusion_blend_mode // "") | contains("screen") | not))
			        ],
			        bad_optical_magnification: [
			          (.optical_magnification_details // [])[] | select(.optical_magnification_ready != true or ((.optical_magnification_size // "") | contains("%") | not))
			        ],
			        bad_biaxial_magnification: [
			          (.biaxial_magnification_details // [])[] | select(.biaxial_magnification_ready != true or ((.biaxial_magnification_size // "") | test("[0-9]+% [0-9]+%") | not))
			        ],
			        bad_anisotropic_magnification: [
			          (.anisotropic_magnification_details // [])[] | select(.anisotropic_magnification_ready != true or (((.anisotropic_magnification_size // "") | contains("128% 132%")) or ((.anisotropic_magnification_size // "") | contains("126% 134%")) | not))
			        ],
			        bad_phase_separated_refraction: [
			          (.phase_separated_refraction_details // [])[] | select(.phase_separated_refraction_ready != true or (.phase_position_count // 0) < 6)
			        ],
			        bad_two_axis_phase_refraction: [
			          (.two_axis_phase_refraction_details // [])[] | select(.two_axis_phase_refraction_ready != true or (.phase_position_count // 0) < 6 or (.phase_y_axis_count // 0) < 3)
			        ],
			        bad_surface_phase_drift: [
			          (.surface_phase_drift_details // [])[] | select(.two_axis_phase_refraction_ready != true or (.surface_phase_drift_position_count // 0) < 2)
			        ],
			        bad_surface_lens_scale_drift: [
			          (.surface_lens_scale_drift_details // [])[] | select(.anisotropic_magnification_ready != true or (.surface_lens_scale_drift_size_count // 0) < 2)
			        ],
			        bad_layer_scale_parallax: [
			          (.layer_scale_parallax_details // [])[] | select(.layer_scale_parallax_ready != true or (.lens_scale_layer_count // 0) < 2 or (.lens_scale_parallax_size_count // 0) < 2)
			        ],
			        bad_surface_spectral_angle_drift: [
			          (.surface_spectral_angle_drift_details // [])[] | select(.layer_scale_parallax_ready != true or (.surface_spectral_angle_drift_signature_count // 0) < 2 or (.spectral_angle_layer_count // 0) < 4 or (.spectral_angle_count // 0) < 4)
			        ],
			        bad_surface_glint_focal_drift: [
			          (.surface_glint_focal_drift_details // [])[] | select(.surface_spectral_angle_drift_ready != true or (.surface_glint_focal_drift_signature_count // 0) < 2 or (.radial_focal_layer_count // 0) < 2 or (.radial_focal_count // 0) < 2)
			        ],
			        bad_composer_glint_focal_decoupling: [
			          (.composer_glint_focal_decoupling_details // [])[] | select(.composer_focal_decoupled != true or (.surface_glint_focal_drift_signature_count // 0) < 3 or (.radial_focal_layer_count // 0) < 2 or (.radial_focal_count // 0) < 2)
			        ],
			        bad_composer_spectral_angle_decoupling: [
			          (.composer_spectral_angle_decoupling_details // [])[] | select(.composer_spectral_angle_decoupled != true or (.surface_spectral_angle_drift_signature_count // 0) < 3 or (.spectral_angle_layer_count // 0) < 4 or (.spectral_angle_count // 0) < 4)
			        ],
			        bad_composer_phase_decoupling: [
			          (.composer_phase_decoupling_details // [])[] | select(.composer_phase_decoupled != true or (.surface_phase_drift_position_count // 0) < 3 or (.phase_position_count // 0) < 6 or (.phase_y_axis_count // 0) < 3)
			        ],
			        bad_composer_layer_scale_decoupling: [
			          (.composer_layer_scale_decoupling_details // [])[] | select(.composer_layer_scale_decoupled != true or (.surface_lens_scale_drift_size_count // 0) < 3 or (.lens_scale_layer_count // 0) < 2 or (.lens_scale_parallax_size_count // 0) < 2)
			        ],
			        bad_chrome_bar_translucency: [
			          (.chrome_bar_translucency_details // [])[] | select(.translucent_chrome_ready != true or (.background_alpha // 1) > 0.72 or (.backdrop_blur_px // 0) < 20)
			        ],
			        bad_menu_triggers: [
          (.menu_trigger_details // [])[] | select(.visible != true or .width < 44 or .height < 44 or ((.aria_label // "") | length) == 0 or ((.title // "") | length) == 0 or .title_matches_aria_label != true or .svg_icon_present != true or .visible_icon_text_absent != true)
        ],
        bad_folder_chips: [
          (.folder_chip_details // [])[] | select(.visible != true or .width < 44 or .height < 44 or ((.aria_label // "") | length) == 0 or ((.title // "") | length) == 0 or .title_matches_aria_label != true or .active_state_matches_aria_pressed != true or .box_shadow == "none")
        ],
        bad_folder_chip_label_prismatic_etch: [
          (.folder_chip_details // [])[] | select(.visible != true or ((.text // "") | length) == 0 or .width < 44 or .height < 44 or .text_shadow != "present" or .folder_chip_label_prismatic_etch_ready != true or (.folder_chip_label_text_shadow_count // 0) < 2 or .readable != true or .contrast_ratio < 4.5)
        ],
        bad_row_menu_toggles: [
          (.row_menu_toggle_details // [])[] | select(.marker != "light-glass" or .width < 44 or .height < 44 or ((.aria_label // "") | length) == 0 or ((.title // "") | length) == 0 or .title_matches_aria_label != true or .svg_icon_present != true or .visible_icon_text_absent != true or .box_shadow == "none")
        ],
	        bad_row_menu_panels: [
	          (.row_menu_panel_details // [])[] | select(.visible != true or .item_count < 3 or .width < 180 or .height < 132 or .border_radius < 16 or (.backdrop_filter | contains("blur(") | not) or .box_shadow == "none" or .in_viewport != true or .light_glass_ready != true or .effective_luminance < 0.72 or .effective_luminance > 0.98)
	        ],
	        bad_row_menu_items: [
	          (.row_menu_item_details // [])[] | select(.visible != true or .height < 44 or ((.aria_label // "") | length) == 0 or ((.title // "") | length) == 0 or .title_matches_aria_label != true or .icon_svg_present != true or .label_nowrap_ready != true or .readable != true or .contrast_ratio < 4.5)
	        ],
	        bad_command_palette: {
	          trigger: [(.command_palette_trigger_details // [])[] | select(.visible != true or .marker != "light-glass" or .href != "#command-palette" or .width < 44 or .height < 44 or .border_radius < 20 or .light_glass_ready != true or .effective_luminance < 0.72 or .effective_luminance > 0.98 or ((.backdrop_filter // "") | contains("blur(") | not) or .box_shadow == "none" or ((.aria_label // "") | length) == 0 or ((.title // "") | length) == 0 or .title_matches_aria_label != true or .svg_icon_present != true or .visible_icon_text_absent != true or .readable != true or .contrast_ratio < 4.5)],
	          panel: (.command_palette_panel_details // {} | select(.visible != true or .marker != "light-glass" or .role != "dialog" or .aria_modal != "true" or .aria_label != "Command palette" or .light_glass_ready != true or .effective_luminance < 0.72 or .effective_luminance > 0.98 or .border_radius < 18 or (.backdrop_filter | contains("blur(") | not) or .box_shadow == "none" or .in_viewport != true)),
	          backdrop: (.command_palette_backdrop_details // {} | select(.visible != true or .background_alpha < 0.2 or .background_alpha > 0.6 or .background_image != "present" or (.command_palette_backdrop_repeating_layer_count // 0) < 1 or .command_palette_backdrop_caustic_veil_ready != true or (.backdrop_blur_px // 0) < 10 or .covers_viewport != true)),
	          close: (.command_palette_close_details // {} | select(.visible != true or .marker != "light-glass" or .href != "#commands" or .width < 44 or .height < 44 or .border_radius < 20 or .light_glass_ready != true or .effective_luminance < 0.72 or .effective_luminance > 0.98 or ((.aria_label // "") != "Close command palette") or ((.title // "") | length) == 0 or .title_matches_aria_label != true or .svg_icon_present != true or .visible_icon_text_absent != true or (.backdrop_filter | contains("blur(") | not) or .box_shadow == "none" or .readable != true or .contrast_ratio < 4.5)),
	          input: (.command_palette_input_details // {} | select(.visible != true or .marker != "light-glass" or .type != "search" or ((.placeholder // "") | length) == 0 or .height < 44 or .border_radius < 10 or .light_glass_ready != true or .effective_luminance < 0.72 or .effective_luminance > 0.98 or (.backdrop_filter | contains("blur(") | not) or .box_shadow == "none" or ((.aria_label // "") | length) == 0 or ((.title // "") | length) == 0 or .title_matches_aria_label != true or .readable != true or .contrast_ratio < 4.5)),
	          items: [(.command_palette_item_details // [])[] | select(.visible != true or .marker != "light-glass" or .width < 180 or .height < 44 or .border_radius < 8 or .light_glass_ready != true or .effective_luminance < 0.72 or .effective_luminance > 0.98 or (.backdrop_filter | contains("blur(") | not) or .box_shadow == "none" or ((.key // "") | length) == 0 or ((.kind // "") | length) == 0 or ((.label // "") | length) == 0 or ((.detail // "") | length) == 0 or ((.aria_label // "") | length) == 0 or ((.title // "") | length) == 0 or .title_matches_aria_label != true or .readable != true or .contrast_ratio < 4.5)]
	        },
	        bad_command_palette_surface_prismatic_perimeter: (.command_palette_panel_details // {} | select(.filter != "present" or .command_palette_surface_prismatic_perimeter_ready != true or (.command_palette_surface_drop_shadow_count // 0) < 2 or .box_shadow == "none")),
	        bad_command_palette_item_label_prismatic_etch: [
	          (.command_palette_item_details // [])[] | select(.kind_text_shadow != "present" or .label_text_shadow != "present" or .detail_text_shadow != "present" or .command_palette_item_label_prismatic_etch_ready != true or (.command_palette_item_kind_text_shadow_count // 0) < 2 or (.command_palette_item_label_text_shadow_count // 0) < 2 or (.command_palette_item_detail_text_shadow_count // 0) < 2 or .kind_readable != true or .readable != true or .detail_readable != true or .kind_contrast_ratio < 4.5 or .contrast_ratio < 4.5 or .detail_contrast_ratio < 4.5)
	        ],
	        bad_command_palette_kind_chip: [
	          (.command_palette_item_details // [])[] | select((.kind_width // 0) < 44 or (.kind_height // 0) < 22 or (.kind_background_alpha // 0) < 0.25 or (.kind_background_alpha // 0) > 0.75 or (.kind_effective_luminance // 0) < 0.72 or (.kind_effective_luminance // 0) > 0.98 or (.kind_border_alpha // 0) < 0.25 or (.kind_border_radius // 0) < 20 or ((.kind_backdrop_filter // "") | contains("blur(") | not) or .kind_box_shadow == "none" or (.command_palette_kind_chip_shadow_count // 0) < 2 or .command_palette_kind_chip_light_glass_ready != true or .kind_readable != true or .kind_contrast_ratio < 4.5)
	        ],
	        bad_command_palette_item_hover_prismatic: [
	          (.command_palette_item_details // [])[] | select(.audit_hover == true and ((.command_palette_item_hover_prismatic_ready != true) or ((.command_palette_item_hover_shadow_count // 0) < 2) or ((.border_alpha // 0) < 0.25) or .box_shadow == "none"))
	        ],
	        bad_command_palette_item_prismatic_rim: [
	          (.command_palette_item_details // [])[] | select(.command_palette_item_prismatic_rim_ready != true or (.command_palette_item_rim_shadow_count // 0) < 2 or (.border_alpha // 0) < 0.25 or .box_shadow == "none")
	        ],
		        bad_command_palette_close_prismatic_icon: (.command_palette_close_details // {} | select(.filter != "present" or .command_palette_close_prismatic_icon_ready != true or (.command_palette_close_drop_shadow_count // 0) < 2 or .svg_icon_present != true or .visible_icon_text_absent != true)),
			        bad_command_palette_input_text_prismatic_etch: (.command_palette_input_details // {} | select(.text_shadow != "present" or .command_palette_input_prismatic_etch_ready != true or (.command_palette_input_text_shadow_count // 0) < 2 or .readable != true or .contrast_ratio < 4.5)),
			        bad_command_palette_input_placeholder_prismatic_etch: (.command_palette_input_details // {} | select(((.placeholder // "") | length) == 0 or .placeholder_text_shadow != "present" or .command_palette_input_placeholder_prismatic_etch_ready != true or (.command_palette_input_placeholder_text_shadow_count // 0) < 2 or (.command_palette_input_placeholder_font_weight // 0) < 600 or .placeholder_readable != true or .placeholder_contrast_ratio < 4.5)),
			        bad_command_palette_input_row_prismatic_separator: (.command_palette_input_row_details // {} | select(.visible != true or .width < 274 or .height < 60 or (.border_bottom_alpha // 0) < 0.25 or .box_shadow == "none" or (.command_palette_input_row_separator_shadow_count // 0) < 2 or .command_palette_input_row_prismatic_separator_ready != true)),
		        bad_command_palette_results_well: (.command_palette_results_well_details // {} | select(.visible != true or .width < 274 or .height < 58 or (.background_alpha // 0) < 0.1 or (.background_alpha // 0) > 0.4 or .light_glass_ready != true or ((.backdrop_filter // "") | contains("blur(") | not) or (.backdrop_blur_px // 0) < 10 or .command_palette_results_well_light_glass_ready != true)),
		        bad_command_palette_results_well_prismatic_rim: (.command_palette_results_well_details // {} | select((.border_alpha // 0) < 0.25 or (.border_radius // 0) < 12 or .box_shadow == "none" or (.command_palette_results_well_rim_shadow_count // 0) < 2 or .command_palette_results_well_prismatic_rim_ready != true)),
		        bad_command_palette_input_icon: (.command_palette_input_icon_details // {} | select(.visible != true or .width < 44 or .height < 44 or .border_radius < 20 or .light_glass_ready != true or .effective_luminance < 0.72 or .effective_luminance > 0.98 or ((.backdrop_filter // "") | contains("blur(") | not) or .box_shadow == "none" or .svg_icon_present != true or .visible_icon_text_absent != true or .readable != true or .contrast_ratio < 4.5)),
		        bad_command_palette_input_icon_prismatic: (.command_palette_input_icon_details // {} | select(.filter != "present" or .command_palette_input_icon_prismatic_ready != true or (.command_palette_input_icon_drop_shadow_count // 0) < 2 or .svg_icon_present != true or .visible_icon_text_absent != true)),
		        bad_form_controls: [
		          (.control_form_control_details // [])[] | select(.height < 44 or ((.aria_label // "") | length) == 0 or ((.title // "") | length) == 0 or .title_matches_aria_label != true or .readable != true or .contrast_ratio < 4.5)
		        ],
		        bad_chat_row_options: [
		          (.chat_row_option_details // [])[] | select(.role != "listitem" or .width < 44 or .height < 64 or ((.aria_label // "") | length) == 0 or ((.title // "") | length) == 0 or .title_matches_aria_label != true or .tabindex != "0" or .active_state_matches_aria_current != true or .border_radius < 18)
		        ],
		        bad_rail_chat_row_prismatic_slabs: [
		          (.rail_chat_row_prismatic_slab_details // [])[] | select(.visible != true or .width < 44 or .height < 64 or .border_radius < 18 or .chat_row_prismatic_slab_ready != true or (.chat_row_drop_shadow_count // 0) < 2 or .box_shadow == "none" or ((.backdrop_filter // "") | contains("blur(") | not))
		        ],
        bad_menu_items: [
          (.menu_item_details // [])[] | select(.icon_present != true or .icon_svg_present != true or .label_ready != true or .visible != true or .height < 36 or .label_nowrap_ready != true)
        ],
        bad_menu_surfaces: [
	          (.menu_surface_details // [])[] | select(.visible != true or .item_count < 1 or .width < 180 or .height < 44 or .border_radius < 16 or (.backdrop_filter | contains("blur(") | not) or .box_shadow == "none" or .in_viewport != true)
        ],
		        bad_thread_tools_trigger: (
		          .thread_tools_trigger_details // {} | select(.exists != true or .marker != "light-glass" or .visible != true or .width < 44 or .height < 44 or .border_radius < 20 or .light_glass_ready != true or .effective_luminance < 0.72 or .effective_luminance > 0.98 or (.backdrop_filter | contains("blur(") | not) or .box_shadow == "none" or .title_matches_aria_label != true or .svg_icon_present != true or .visible_icon_text_absent != true or .readable != true or .contrast_ratio < 4.5)
		        ),
		        bad_thread_tools_panel: (
		          .thread_tools_panel_details // {} | select(.exists != true or .marker != "light-glass" or .visible != true or .role != "menu" or .aria_label != "Thread tools" or .item_count != 3 or .width < 180 or .height < 44 or .border_radius < 16 or .light_glass_ready != true or .effective_luminance < 0.72 or .effective_luminance > 0.98 or (.backdrop_filter | contains("blur(") | not) or .box_shadow == "none" or .in_viewport != true or .top_clipped != false or .bottom_clipped != false)
		        ),
		        bad_thread_tools_items: [
		          (.thread_tools_item_details // [])[] | select(.visible != true or .role != "menuitem" or .height < 44 or ((.aria_label // "") | length) == 0 or ((.title // "") | length) == 0 or .title_matches_aria_label != true or .icon_svg_present != true or .label_nowrap_ready != true or .readable != true or .contrast_ratio < 4.5)
		        ],
		        bad_composer_tools_panel: (
		          .composer_tools_panel_details // {} | select(.exists != true or .visible != true or .role != "menu" or .aria_label != "Composer tools" or .item_count != 2 or .width < 180 or .height < 44 or .border_radius < 16 or .marker != "light-glass" or .light_glass_ready != true or .effective_luminance < 0.72 or .effective_luminance > 0.98 or (.backdrop_filter | contains("blur(") | not) or .box_shadow == "none" or .in_viewport != true or .top_clipped != false or .bottom_clipped != false)
		        ),
		        bad_composer_tools_items: [
		          (.composer_tools_item_details // [])[] | select(.visible != true or .role != "menuitem" or .height < 44 or ((.aria_label // "") | length) == 0 or ((.title // "") | length) == 0 or .title_matches_aria_label != true or .icon_svg_present != true or .label_nowrap_ready != true or .select_present != true or .select_visible != true or .select_height < 44 or ((.select_aria_label // "") | length) == 0 or ((.select_title // "") | length) == 0 or .select_title_matches_aria_label != true or .select_readable != true or .select_contrast_ratio < 4.5 or .readable != true or .contrast_ratio < 4.5)
		        ],
		        bad_composer_popover_toggles: [
		          (.composer_popover_toggle_details // [])[] | select(.visible != true or .width < 44 or .height < 44 or ((.aria_label // "") | length) == 0 or ((.title // "") | length) == 0 or .title_matches_aria_label != true or .aria_haspopup != "menu" or ((.aria_controls // "") | length) == 0 or .svg_icon_present != true or .visible_icon_text_absent != true)
		        ],
		        bad_composer_popover_panels: [
		          (.composer_popover_panel_details // [])[] | select(.visible != true or .role != "menu" or ((.aria_label // "") | length) == 0 or .search_count != 1 or .item_count != 2 or .width < 180 or .height < 132 or .border_radius < 16 or .light_glass_ready != true or .effective_luminance < 0.72 or .effective_luminance > 0.98 or (.backdrop_filter | contains("blur(") | not) or .box_shadow == "none" or .in_viewport != true or .top_clipped != false or .bottom_clipped != false)
		        ],
		        bad_composer_popover_search: [
		          (.composer_popover_search_details // [])[] | select(.visible != true or .marker != "light-glass" or .height < 44 or .border_radius < 10 or .light_glass_ready != true or .effective_luminance < 0.72 or .effective_luminance > 0.98 or (.backdrop_filter | contains("blur(") | not) or .box_shadow == "none" or ((.aria_label // "") | length) == 0 or ((.title // "") | length) == 0 or .title_matches_aria_label != true or .readable != true or .contrast_ratio < 4.5)
		        ],
		        bad_composer_popover_search_placeholder_prismatic_etch: [
		          (.composer_popover_search_details // [])[] | select(.visible != true or ((.placeholder // "") | length) == 0 or .height < 44 or .placeholder_text_shadow != "present" or .composer_popover_search_placeholder_prismatic_etch_ready != true or (.composer_popover_search_placeholder_text_shadow_count // 0) < 2 or .placeholder_readable != true or .placeholder_contrast_ratio < 4.5)
		        ],
        bad_rail_search: [
          (.rail_search_details // [])[] | select(.visible == true and (.marker != "light-glass" or .type != "search" or ((.placeholder // "") | length) == 0 or .width < 180 or .height < 44 or .border_radius < 12 or .light_glass_ready != true or .effective_luminance < 0.72 or .effective_luminance > 0.98 or (.backdrop_filter | contains("blur(") | not) or .box_shadow == "none" or ((.aria_label // "") | length) == 0 or ((.title // "") | length) == 0 or .title_matches_aria_label != true or .readable != true or .contrast_ratio < 4.5 or .placeholder_readable != true or .placeholder_contrast_ratio < 4.5))
        ],
        bad_rail_search_placeholder_prismatic_etch: [
          (.rail_search_placeholder_prismatic_etch_details // [])[] | select(.visible != true or ((.placeholder // "") | length) == 0 or .width < 180 or .height < 44 or .placeholder_text_shadow != "present" or .rail_search_placeholder_prismatic_etch_ready != true or (.rail_search_placeholder_text_shadow_count // 0) < 2 or .placeholder_readable != true or .placeholder_contrast_ratio < 4.5)
        ],
	        bad_rail_prismatic_filters: [
	          (.rail_prismatic_filter_details // [])[] | select(.visible != true or .width < 44 or .height < 44 or .border_radius < 12 or .rail_prismatic_filter_ready != true or (.rail_filter_drop_shadow_count // 0) < 2 or .box_shadow == "none" or ((.backdrop_filter // "") | contains("blur(") | not))
	        ],
		        bad_composer_popover_items: [
		          (.composer_popover_item_details // [])[] | select(.visible != true or .role != "menuitem" or .width < 120 or .height < 44 or ((.aria_label // "") | length) == 0 or ((.title // "") | length) == 0 or .title_matches_aria_label != true or ((.label // "") | length) == 0 or ((.detail // "") | length) == 0 or .icon_svg_present != true or .label_nowrap_ready != true or .readable != true or .contrast_ratio < 4.5)
		        ],
		        bad_composer_popover_item_label_prismatic_etch: [
		          (.composer_popover_item_details // [])[] | select(.visible != true or ((.label // "") | length) == 0 or ((.detail // "") | length) == 0 or .label_text_shadow != "present" or .detail_text_shadow != "present" or .composer_popover_item_label_prismatic_etch_ready != true or (.composer_popover_item_label_text_shadow_count // 0) < 2 or (.composer_popover_item_detail_text_shadow_count // 0) < 2 or .readable != true or .detail_readable != true or .contrast_ratio < 4.5 or .detail_contrast_ratio < 4.5 or .label_nowrap_ready != true or .detail_nowrap_ready != true)
		        ],
		        bad_composer_popover_header_prismatic_etch: [
		          (.composer_popover_header_prismatic_etch_details // [])[] | select(.visible != true or .label_visible != true or .status_visible != true or ((.label // "") | length) == 0 or ((.status // "") | length) == 0 or .label_text_shadow != "present" or .status_text_shadow != "present" or .composer_popover_header_prismatic_etch_ready != true or (.composer_popover_header_label_text_shadow_count // 0) < 2 or (.composer_popover_header_status_text_shadow_count // 0) < 2 or .label_readable != true or .status_readable != true or .label_contrast_ratio < 4.5 or .status_contrast_ratio < 4.5)
		        ],
		        bad_micro_surfaces: [
		          (.micro_surface_details // [])[] | select(.visible != true or ((.key // "") | length) == 0 or ((.text // "") | length) == 0 or .height < 22 or .border_radius < 10 or .light_glass_ready != true or .effective_luminance < 0.72 or .effective_luminance > 0.98 or (.backdrop_filter | contains("blur(") | not) or .box_shadow == "none" or .readable != true or .contrast_ratio < 4.5 or .label_nowrap_ready != true)
		        ],
		        bad_micro_prismatic_badges: [
		          (.micro_surface_details // [])[] | select(.micro_prismatic_badge_ready != true or (.micro_prismatic_badge_drop_shadow_count // 0) < 2 or .box_shadow == "none" or ((.backdrop_filter // "") | contains("blur(") | not))
		        ],
		        bad_micro_badge_label_prismatic_etch: [
		          (.micro_surface_details // [])[] | select(.text_shadow != "present" or .micro_badge_label_prismatic_etch_ready != true or (.micro_badge_label_text_shadow_count // 0) < 2 or .readable != true or .contrast_ratio < 4.5)
		        ],
		        bad_message_metadata_prismatic: [
		          (.message_metadata_prismatic_details // [])[] | select(.visible != true or ((.text // "") | length) == 0 or .height < 22 or .border_radius < 10 or .light_glass_ready != true or .effective_luminance < 0.72 or .effective_luminance > 0.98 or ((.backdrop_filter // "") | contains("blur(") | not) or .box_shadow == "none" or .message_metadata_prismatic_ready != true or (.message_metadata_drop_shadow_count // 0) < 2 or .readable != true or .contrast_ratio < 4.5 or .label_nowrap_ready != true)
		        ],
		        bad_thread_subtitle_prismatic: [
		          (.thread_subtitle_prismatic_details // [])[] | select(.visible != true or ((.text // "") | length) == 0 or .height < 22 or .border_radius < 10 or .light_glass_ready != true or .effective_luminance < 0.72 or .effective_luminance > 0.98 or ((.backdrop_filter // "") | contains("blur(") | not) or .box_shadow == "none" or .thread_subtitle_prismatic_ready != true or (.thread_subtitle_drop_shadow_count // 0) < 2 or .readable != true or .contrast_ratio < 4.5 or .label_nowrap_ready != true)
		        ],
		        bad_composer_shortcut_hint_prismatic: [
		          (.composer_shortcut_hint_prismatic_details // [])[] | select(.visible != true or ((.text // "") | length) == 0 or .height < 22 or .border_radius < 10 or .light_glass_ready != true or .effective_luminance < 0.72 or .effective_luminance > 0.98 or ((.backdrop_filter // "") | contains("blur(") | not) or .box_shadow == "none" or .composer_shortcut_hint_prismatic_ready != true or (.composer_shortcut_hint_drop_shadow_count // 0) < 2 or .readable != true or .contrast_ratio < 4.5 or .label_nowrap_ready != true)
		        ],
		        bad_rail_metadata_chip_prismatic: [
		          (.rail_metadata_chip_prismatic_details // [])[] | select(.visible != true or ((.text // "") | length) == 0 or .height < 22 or .border_radius < 10 or .light_glass_ready != true or .effective_luminance < 0.72 or .effective_luminance > 0.98 or ((.backdrop_filter // "") | contains("blur(") | not) or .box_shadow == "none" or .rail_metadata_chip_prismatic_ready != true or (.rail_metadata_chip_drop_shadow_count // 0) < 2 or .readable != true or .contrast_ratio < 4.5 or .label_nowrap_ready != true)
		        ],
			        bad_rail_status_count_prismatic: [
			          (.rail_status_count_prismatic_details // [])[] | select(.visible != true or ((.text // "") | length) == 0 or .height < 22 or .border_radius < 10 or .light_glass_ready != true or .effective_luminance < 0.72 or .effective_luminance > 0.98 or ((.backdrop_filter // "") | contains("blur(") | not) or .box_shadow == "none" or .rail_status_count_prismatic_ready != true or (.rail_status_count_drop_shadow_count // 0) < 2 or .readable != true or .contrast_ratio < 4.5 or .label_nowrap_ready != true)
			        ],
			        bad_rail_preview_prismatic_etch: [
			          (.rail_preview_prismatic_etch_details // [])[] | select(.visible != true or ((.text // "") | length) == 0 or .width <= 20 or .height < 14 or .filter != "present" or .rail_preview_prismatic_etch_ready != true or (.rail_preview_drop_shadow_count // 0) < 2 or .readable != true or .contrast_ratio < 4.5)
			        ],
			        bad_rail_chat_title_prismatic_etch: [
			          (.rail_chat_title_prismatic_etch_details // [])[] | select(.visible != true or ((.text // "") | length) == 0 or .width <= 20 or .height < 14 or .filter != "present" or .rail_chat_title_prismatic_etch_ready != true or (.rail_chat_title_drop_shadow_count // 0) < 2 or .readable != true or .contrast_ratio < 4.5)
			        ],
			        bad_message_body_prismatic_etch: [
			          (.message_body_prismatic_etch_details // [])[] | select(.visible != true or ((.text // "") | length) == 0 or .width <= 20 or .height < 16 or .filter != "present" or .message_body_prismatic_etch_ready != true or (.message_body_drop_shadow_count // 0) < 2 or .readable != true or .contrast_ratio < 4.5)
			        ],
			        bad_message_speaker_prismatic_chip: [
			          (.message_speaker_prismatic_chip_details // [])[] | select(.visible != true or ((.text // "") | length) == 0 or .height < 22 or .border_radius < 10 or .light_glass_ready != true or .effective_luminance < 0.72 or .effective_luminance > 0.98 or ((.backdrop_filter // "") | contains("blur(") | not) or .box_shadow == "none" or .filter != "present" or .message_speaker_prismatic_chip_ready != true or (.message_speaker_chip_drop_shadow_count // 0) < 2 or .readable != true or .contrast_ratio < 4.5 or .label_nowrap_ready != true)
			        ],
			        bad_composer_placeholder_prismatic_etch: [
			          (.composer_placeholder_prismatic_etch_details // [])[] | select(.visible != true or ((.placeholder // "") | length) == 0 or .width < 100 or .height < 44 or .placeholder_text_shadow != "present" or .composer_placeholder_prismatic_etch_ready != true or (.composer_placeholder_text_shadow_count // 0) < 2 or .readable != true or .contrast_ratio < 4.5)
			        ],
			        bad_header_title_prismatic_etch: [
		          (.header_title_prismatic_etch_details // [])[] | select(.visible != true or ((.text // "") | length) == 0 or .width <= 20 or .height < 16 or .filter != "present" or .header_title_prismatic_etch_ready != true or (.header_title_drop_shadow_count // 0) < 2 or .readable != true or .contrast_ratio < 4.5)
		        ],
		        bad_message_routing_badges: [
		          (.message_routing_badge_details // [])[] | select(.visible != true or ((.key // "") | length) == 0 or ((.text // "") | length) == 0 or .height < 22 or .border_radius < 10 or .light_glass_ready != true or .effective_luminance < 0.72 or .effective_luminance > 0.98 or (.backdrop_filter | contains("blur(") | not) or .box_shadow == "none" or .readable != true or .contrast_ratio < 4.5 or .label_nowrap_ready != true)
		        ],
		        bad_thread_intro_badges: [
		          (.thread_intro_badge_details // [])[] | select(.visible != true or ((.key // "") | length) == 0 or ((.text // "") | length) == 0 or .height < 22 or .border_radius < 10 or .light_glass_ready != true or .effective_luminance < 0.72 or .effective_luminance > 0.98 or (.backdrop_filter | contains("blur(") | not) or .box_shadow == "none" or .readable != true or .contrast_ratio < 4.5 or .label_nowrap_ready != true)
		        ],
		        bad_status_trust_badges: [
		          (.status_trust_badge_details // [])[] | select(.visible != true or ((.key // "") | length) == 0 or ((.text // "") | length) == 0 or .height < 22 or .border_radius < 10 or .light_glass_ready != true or .effective_luminance < 0.72 or .effective_luminance > 0.98 or (.backdrop_filter | contains("blur(") | not) or .box_shadow == "none" or .readable != true or .contrast_ratio < 4.5 or .label_nowrap_ready != true or ((.aria_label // "") | length) == 0 or ((.title // "") | length) == 0 or .title_matches_aria_label != true)
		        ],
	        bad_microcopy_wrap: [
	          (.microcopy_wrap_details // [])[] | select(.overflow_wrap == "anywhere" or .word_break == "break-word" or .word_break == "break-all")
	        ],
		        bad_logo_clip: [
		          (.logo_clip_details // [])[] | select(.visible != true or .image_present != true or .width < 32 or .height < 32 or .image_fills_container != true)
		        ],
		        bad_avatar_prismatic_rims: [
		          (.avatar_prismatic_rim_details // [])[] | select(.visible != true or .width < 40 or .height < 40 or .border_radius < 16 or .avatar_prismatic_rim_ready != true or (.avatar_rim_drop_shadow_count // 0) < 2 or .box_shadow == "none")
		        ],
		        bad_active_chat_readability: [
		          (.active_chat_readability_details // [])[] | select(.readable != true)
		        ],
		        bad_placeholder_readability: [
		          (.placeholder_readability_details // [])[] | select(.readable != true)
		        ],
		        bad_small_control_readability: [
		          (.small_control_readability_details // [])[] | select(.readable != true)
		        ],
		        visible_text_integrity_probe
	      }
    ]
  }' <<<"$density_qa_json" >&2 || true
  exit 1
fi
}
