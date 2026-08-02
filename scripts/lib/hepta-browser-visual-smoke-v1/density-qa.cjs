(async () => {
  const { spawn } = require("node:child_process");
  const fs = require("node:fs");
  const fsPromises = require("node:fs/promises");
  const os = require("node:os");
  const path = require("node:path");

  const [chromeBin, baseUrl] = process.argv.slice(2);
  const viewports = [
    {
      name: "desktop",
      width: 1365,
      height: 900,
      expectedVisible: [
        ".tg-conversation-rail",
        ".tg-thread-panel",
        ".tg-compose-wrap",
        ".tg-compose-bar",
        "[data-chat-composer-input]",
        "[data-agent-chat-send]",
      ],
      expectedHidden: [".tg-room-panel"],
    },
    {
      name: "narrow",
      width: 768,
      height: 900,
      expectedVisible: [
        ".tg-conversation-rail",
        ".tg-thread-panel",
        ".tg-compose-wrap",
        ".tg-compose-bar",
        "[data-chat-composer-input]",
        "[data-agent-chat-send]",
      ],
      expectedHidden: [".tg-room-panel"],
    },
    {
      name: "mobile",
      width: 500,
      height: 844,
      expectedVisible: [
        ".tg-thread-panel",
        ".tg-compose-wrap",
        ".tg-compose-bar",
        "[data-chat-composer-input]",
        "[data-agent-chat-send]",
      ],
      expectedHidden: [".tg-conversation-rail", ".tg-room-panel"],
    },
    {
      name: "phone320",
      width: 320,
      height: 844,
      expectedVisible: [
        ".tg-thread-panel",
        ".tg-compose-wrap",
        ".tg-compose-bar",
        "[data-chat-composer-input]",
        "[data-agent-chat-send]",
      ],
      expectedHidden: [".tg-conversation-rail", ".tg-room-panel"],
    },
  ];

  // The in-page audit is kept as ordered, responsibility-scoped fragments so
  // the Chrome/CDP runner stays reviewable without changing its lexical scope.
  const densityProbeSource = [
    "01-foundation.fragment.cjs",
    "02-shell-optics.fragment.cjs",
    "03-controls-palette.fragment.cjs",
    "04-menus-popovers.fragment.cjs",
    "05-micro-surfaces.fragment.cjs",
    "06-text-integrity.fragment.cjs",
    "07-verdict.fragment.cjs",
  ].map((name) => require(path.join(__dirname, "density-probe", name)).slice(1, -1))
    .join("")
    .replace(/\n$/, "");

  const sleep = (ms) => new Promise((resolve) => setTimeout(resolve, ms));

  async function waitFor(condition, timeoutMs, label) {
    const deadline = Date.now() + timeoutMs;
    while (Date.now() < deadline) {
      const value = condition();
      if (value) {
        return value;
      }
      await sleep(50);
    }
    throw new Error(`Timed out waiting for ${label}`);
  }

  async function inspectViewport(viewport) {
    const profileDir = await fsPromises.mkdtemp(path.join(os.tmpdir(), `hepta-control-density-${viewport.name}-`));
    const chrome = spawn(
      chromeBin,
      [
        "--headless=new",
        "--disable-gpu",
        "--force-device-scale-factor=1",
        "--disable-background-networking",
        "--disable-component-update",
        "--disable-default-apps",
        "--disable-extensions",
        "--disable-sync",
        "--no-first-run",
        "--no-default-browser-check",
        "--hide-scrollbars",
        "--remote-debugging-port=0",
        `--user-data-dir=${path.join(profileDir, "profile")}`,
        `--window-size=${viewport.width},${viewport.height}`,
        "about:blank",
      ],
      { stdio: ["ignore", "ignore", "pipe"] },
    );

    let browserWsUrl = "";
    let stderr = "";
    chrome.stderr.setEncoding("utf8");
    chrome.stderr.on("data", (chunk) => {
      stderr += chunk;
      const match = chunk.match(/DevTools listening on (ws:\/\/[^\s]+)/);
      if (match) {
        browserWsUrl = match[1];
      }
    });

    try {
      browserWsUrl = await waitFor(() => browserWsUrl, 10000, "Chrome DevTools endpoint");
      const browserWs = new URL(browserWsUrl);
      const targets = await (await fetch(`http://${browserWs.host}/json/list`)).json();
      const pageTarget = targets.find((target) => target.type === "page");
      if (!pageTarget?.webSocketDebuggerUrl) {
        throw new Error(`Chrome page target not available for ${viewport.name}`);
      }

      const ws = new WebSocket(pageTarget.webSocketDebuggerUrl);
      await new Promise((resolve, reject) => {
        ws.onopen = resolve;
        ws.onerror = reject;
      });

      let id = 0;
      const pending = new Map();
      ws.onmessage = (event) => {
        const message = JSON.parse(event.data);
        if (message.id && pending.has(message.id)) {
          const { resolve, reject } = pending.get(message.id);
          pending.delete(message.id);
          if (message.error) {
            reject(new Error(JSON.stringify(message.error)));
          } else {
            resolve(message.result);
          }
        }
      };

      function send(method, params = {}) {
        const requestId = ++id;
        ws.send(JSON.stringify({ id: requestId, method, params }));
        return new Promise((resolve, reject) => pending.set(requestId, { resolve, reject }));
      }

      await send("Page.enable");
      await send("Runtime.enable");
      await send("Emulation.setDeviceMetricsOverride", {
        width: viewport.width,
        height: viewport.height,
        deviceScaleFactor: 1,
        mobile: false,
      });
      const navigateResult = await send("Page.navigate", { url: baseUrl });
      await sleep(900);

      const expression = `
(() => {
  const viewportName = ${JSON.stringify(viewport.name)};
  const expectedVisible = ${JSON.stringify(viewport.expectedVisible)};
  const expectedHidden = ${JSON.stringify(viewport.expectedHidden)};
${densityProbeSource}
      `;

      const evaluation = await send("Runtime.evaluate", {
        expression,
        returnByValue: true,
        awaitPromise: false,
      });
      ws.close();
      if (evaluation.exceptionDetails) {
        const message =
          evaluation.exceptionDetails.exception?.description ||
          evaluation.exceptionDetails.text ||
          "runtime_evaluate_exception";
        return {
          name: viewport.name,
          viewport: `${viewport.width}x${viewport.height}`,
          status: "failed",
          navigation_error: navigateResult.errorText || null,
          errors: [String(message).slice(0, 240)],
        };
      }
      const value = evaluation.result?.value || { errors: ["runtime_evaluate_no_value"] };
      if (!Array.isArray(value.errors)) {
        value.errors = [];
      }
      return {
        name: viewport.name,
        viewport: `${viewport.width}x${viewport.height}`,
        status: value.errors.length === 0 ? "ready" : "failed",
        navigation_error: navigateResult.errorText || null,
        ...value,
      };
    } finally {
      if (!chrome.killed) {
        chrome.kill("SIGTERM");
      }
      setTimeout(() => {
        if (!chrome.killed) {
          chrome.kill("SIGKILL");
        }
      }, 1000).unref();
    }
  }

  const results = [];
  for (const viewport of viewports) {
    results.push(await inspectViewport(viewport));
  }

	  const failures = results.flatMap((result) =>
	    result.status === "ready" ? [] : (result.errors || []).map((error) => `${result.name}:${error}`),
	  );

  const report = {
    gate: "control_ui_visual_density_qa",
    status: failures.length === 0 ? "ready" : "failed",
    control_ui_visual_density_qa_ready: failures.length === 0,
    viewport_count: results.length,
    narrow_composer_non_overlap_ready: results.every((result) => result.narrow_composer_non_overlap_ready === true),
    phone320_ready: results.some((result) => result.name === "phone320" && result.status === "ready"),
    default_submenus_closed_ready: results.every((result) => result.default_submenus_closed_ready === true),
    single_submenu_audit_ready: results.every((result) => result.single_submenu_audit_ready === true),
    unavailable_submenu_items_ready: results.every((result) => result.unavailable_submenu_items_ready === true),
    disabled_submenu_item_count: results.reduce((count, result) => count + (result.disabled_submenu_item_count || 0), 0),
    row_menu_distinct_positions_ready: results.every((result) => result.row_menu_distinct_positions_ready === true),
    mobile_pane_navigation_ready: results.every((result) => result.mobile_pane_navigation_ready === true),
    engineering_session_chips_suppressed_ready: results.every((result) => result.engineering_session_chips_suppressed_ready === true),
    preferred_touch_targets_ready: results.every((result) => result.preferred_touch_target_ready === true),
	    control_glass_action_ready: results.every((result) => result.control_glass_action_ready === true),
	    harsh_referee_ready: results.every((result) => result.harsh_referee_ready === true),
	    shallow_light_glass_ready: results.every((result) => result.shallow_light_glass_ready === true),
	    light_theme_semantics_ready: results.every((result) => result.light_theme_semantics_ready === true),
	    stable_content_surface_ready: results.every((result) => result.stable_content_surface_ready === true),
	    native_popover_interaction_ready: results.every((result) => result.native_popover_interaction_ready === true),
	    native_popover_compatibility_source: "native_actual_click_single_submenu_audit",
	    legacy_menu_compatibility_uses_actual_click: results.every((result) => result.legacy_menu_compatibility_uses_actual_click === true),
	    shallow_floating_surface_ready: results.every((result) => result.shallow_floating_surface_ready === true),
	    restrained_optics_ready: results.every((result) => result.restrained_optics_ready === true),
	    restrained_mobile_metadata_ready: results.every((result) => result.restrained_mobile_metadata_ready === true),
	    key_touch_controls_ready: results.every((result) => result.key_touch_controls_ready === true),
    rail_action_icon_ready: results.every((result) => result.rail_action_icon_ready === true),
    icon_button_ready: results.every((result) => result.icon_button_ready === true),
    icon_prismatic_control_light_glass_ready: results.every((result) => result.icon_prismatic_control_light_glass_ready === true),
    topbar_action_light_glass_ready: results.every((result) => result.topbar_action_light_glass_ready === true),
    chrome_bar_translucency_light_glass_ready: results.every((result) => result.chrome_bar_translucency_light_glass_ready === true),
    chrome_refractive_skin_light_glass_ready: results.every((result) => result.chrome_refractive_skin_light_glass_ready === true),
    clear_white_balance_light_glass_ready: results.every((result) => result.clear_white_balance_light_glass_ready === true),
    chamfer_cut_edge_light_glass_ready: results.every((result) => result.chamfer_cut_edge_light_glass_ready === true),
    prismatic_cut_edge_light_glass_ready: results.every((result) => result.prismatic_cut_edge_light_glass_ready === true),
    pane_prismatic_perimeter_light_glass_ready: results.every((result) => result.pane_prismatic_perimeter_light_glass_ready === true),
    composer_prismatic_control_light_glass_ready: results.every((result) => result.composer_prismatic_control_light_glass_ready === true),
    primary_shell_light_glass_ready: results.every((result) => result.primary_shell_light_glass_ready === true),
	    translucent_shell_light_glass_ready: results.every((result) => result.translucent_shell_light_glass_ready === true),
	    refractive_depth_light_glass_ready: results.every((result) => result.refractive_depth_light_glass_ready === true),
	    optical_clarity_light_glass_ready: results.every((result) => result.optical_clarity_light_glass_ready === true),
	    surface_clear_alpha_light_glass_ready: results.every((result) => result.surface_clear_alpha_light_glass_ready === true),
	    substrate_caustic_field_light_glass_ready: results.every((result) => result.substrate_caustic_field_light_glass_ready === true),
		    specular_edge_light_glass_ready: results.every((result) => result.specular_edge_light_glass_ready === true),
		    prismatic_dispersion_light_glass_ready: results.every((result) => result.prismatic_dispersion_light_glass_ready === true),
		    caustic_highlight_light_glass_ready: results.every((result) => result.caustic_highlight_light_glass_ready === true),
		    caustic_depth_shift_light_glass_ready: results.every((result) => result.caustic_depth_shift_light_glass_ready === true),
			    optical_thickness_tiers_light_glass_ready: results.every((result) => result.optical_thickness_tiers_light_glass_ready === true),
			    faceted_reflection_light_glass_ready: results.every((result) => result.faceted_reflection_light_glass_ready === true),
			    beveled_rim_light_glass_ready: results.every((result) => result.beveled_rim_light_glass_ready === true),
			    micro_refraction_light_glass_ready: results.every((result) => result.micro_refraction_light_glass_ready === true),
			    sparkle_glint_light_glass_ready: results.every((result) => result.sparkle_glint_light_glass_ready === true),
			    lens_bloom_light_glass_ready: results.every((result) => result.lens_bloom_light_glass_ready === true),
			    spectral_fusion_light_glass_ready: results.every((result) => result.spectral_fusion_light_glass_ready === true),
			    optical_magnification_light_glass_ready: results.every((result) => result.optical_magnification_light_glass_ready === true),
			    biaxial_magnification_light_glass_ready: results.every((result) => result.biaxial_magnification_light_glass_ready === true),
			    anisotropic_magnification_light_glass_ready: results.every((result) => result.anisotropic_magnification_light_glass_ready === true),
			    phase_separated_refraction_light_glass_ready: results.every((result) => result.phase_separated_refraction_light_glass_ready === true),
			    two_axis_phase_refraction_light_glass_ready: results.every((result) => result.two_axis_phase_refraction_light_glass_ready === true),
			    surface_phase_drift_light_glass_ready: results.every((result) => result.surface_phase_drift_light_glass_ready === true),
			    surface_lens_scale_drift_light_glass_ready: results.every((result) => result.surface_lens_scale_drift_light_glass_ready === true),
			    layer_scale_parallax_light_glass_ready: results.every((result) => result.layer_scale_parallax_light_glass_ready === true),
			    surface_spectral_angle_drift_light_glass_ready: results.every((result) => result.surface_spectral_angle_drift_light_glass_ready === true),
			    surface_glint_focal_drift_light_glass_ready: results.every((result) => result.surface_glint_focal_drift_light_glass_ready === true),
			    composer_glint_focal_decoupling_light_glass_ready: results.every((result) => result.composer_glint_focal_decoupling_light_glass_ready === true),
			    composer_spectral_angle_decoupling_light_glass_ready: results.every((result) => result.composer_spectral_angle_decoupling_light_glass_ready === true),
			    composer_phase_decoupling_light_glass_ready: results.every((result) => result.composer_phase_decoupling_light_glass_ready === true),
			    composer_layer_scale_decoupling_light_glass_ready: results.every((result) => result.composer_layer_scale_decoupling_light_glass_ready === true),
			    menu_trigger_ready: results.every((result) => result.menu_trigger_ready === true),
		    folder_chip_touch_ready: results.every((result) => result.folder_chip_touch_ready === true),
		    folder_chip_label_prismatic_etch_light_glass_ready: results.every((result) => result.folder_chip_label_prismatic_etch_light_glass_ready === true),
		    row_menu_touch_ready: results.every((result) => result.row_menu_touch_ready === true),
			    row_menu_all_rows_ready: results.every((result) => result.row_menu_all_rows_ready === true),
		    row_menu_light_glass_ready: results.every((result) => result.row_menu_light_glass_ready === true),
		    command_palette_ready: results.every((result) => result.command_palette_ready === true),
		    command_palette_surface_light_glass_ready: results.every((result) => result.command_palette_surface_light_glass_ready === true),
		    command_palette_surface_prismatic_perimeter_light_glass_ready: results.every((result) => result.command_palette_surface_prismatic_perimeter_light_glass_ready === true),
		    command_palette_backdrop_caustic_veil_light_glass_ready: results.every((result) => result.command_palette_backdrop_caustic_veil_light_glass_ready === true),
		    command_palette_trigger_light_glass_ready: results.every((result) => result.command_palette_trigger_light_glass_ready === true),
		    command_palette_close_light_glass_ready: results.every((result) => result.command_palette_close_light_glass_ready === true),
		    command_palette_close_prismatic_icon_light_glass_ready: results.every((result) => result.command_palette_close_prismatic_icon_light_glass_ready === true),
				    command_palette_input_light_glass_ready: results.every((result) => result.command_palette_input_light_glass_ready === true),
				    command_palette_input_text_prismatic_etch_light_glass_ready: results.every((result) => result.command_palette_input_text_prismatic_etch_light_glass_ready === true),
				    command_palette_input_placeholder_prismatic_etch_light_glass_ready: results.every((result) => result.command_palette_input_placeholder_prismatic_etch_light_glass_ready === true),
				    command_palette_input_row_prismatic_separator_light_glass_ready: results.every((result) => result.command_palette_input_row_prismatic_separator_light_glass_ready === true),
				    command_palette_results_well_light_glass_ready: results.every((result) => result.command_palette_results_well_light_glass_ready === true),
				    command_palette_results_well_prismatic_rim_light_glass_ready: results.every((result) => result.command_palette_results_well_prismatic_rim_light_glass_ready === true),
					    command_palette_input_icon_light_glass_ready: results.every((result) => result.command_palette_input_icon_light_glass_ready === true),
					    command_palette_input_icon_prismatic_light_glass_ready: results.every((result) => result.command_palette_input_icon_prismatic_light_glass_ready === true),
				    command_palette_item_light_glass_ready: results.every((result) => result.command_palette_item_light_glass_ready === true),
				    command_palette_item_prismatic_rim_light_glass_ready: results.every((result) => result.command_palette_item_prismatic_rim_light_glass_ready === true),
			    command_palette_kind_chip_light_glass_ready: results.every((result) => result.command_palette_kind_chip_light_glass_ready === true),
			    command_palette_item_hover_prismatic_light_glass_ready: results.every((result) => result.command_palette_item_hover_prismatic_light_glass_ready === true),
			    command_palette_item_label_prismatic_etch_light_glass_ready: results.every((result) => result.command_palette_item_label_prismatic_etch_light_glass_ready === true),
		    control_form_control_title_touch_ready: results.every((result) => result.control_form_control_title_touch_ready === true),
		    chat_row_option_semantic_touch_ready: results.every((result) => result.chat_row_option_semantic_touch_ready === true),
		    rail_chat_row_prismatic_slab_light_glass_ready: results.every((result) => result.rail_chat_row_prismatic_slab_light_glass_ready === true),
		    menu_item_icon_ready: results.every((result) => result.menu_item_icon_ready === true),
    icon_button_title_match_ready: results.every((result) => (result.icon_button_details || []).every((item) => item.title_matches_aria_label === true)),
    menu_trigger_title_match_ready: results.every((result) => (result.menu_trigger_details || []).every((item) => item.title_matches_aria_label === true)),
    menu_surface_ready: results.every((result) => result.menu_surface_ready === true),
    thread_tools_menu_ready: results.every((result) => result.thread_tools_menu_ready === true),
    composer_tools_menu_ready: results.every((result) => result.composer_tools_menu_ready === true),
    composer_tools_trigger_light_glass_ready: results.every((result) => result.composer_tools_trigger_light_glass_ready === true),
    composer_popover_ready: results.every((result) => result.composer_popover_ready === true),
    composer_popover_item_label_prismatic_etch_light_glass_ready: results.every((result) => result.composer_popover_item_label_prismatic_etch_light_glass_ready === true),
    composer_popover_header_prismatic_etch_light_glass_ready: results.every((result) => result.composer_popover_header_prismatic_etch_light_glass_ready === true),
    composer_popover_search_light_glass_ready: results.every((result) => result.composer_popover_search_light_glass_ready === true),
    composer_popover_search_placeholder_prismatic_etch_light_glass_ready: results.every((result) => result.composer_popover_search_placeholder_prismatic_etch_light_glass_ready === true),
    rail_search_light_glass_ready: results.every((result) => result.rail_search_light_glass_ready === true),
    rail_search_placeholder_prismatic_etch_light_glass_ready: results.every((result) => result.rail_search_placeholder_prismatic_etch_light_glass_ready === true),
    rail_prismatic_filter_light_glass_ready: results.every((result) => result.rail_prismatic_filter_light_glass_ready === true),
    micro_surface_light_glass_ready: results.every((result) => result.micro_surface_light_glass_ready === true),
    micro_prismatic_badge_light_glass_ready: results.every((result) => result.micro_prismatic_badge_light_glass_ready === true),
    micro_badge_label_prismatic_etch_light_glass_ready: results.every((result) => result.micro_badge_label_prismatic_etch_light_glass_ready === true),
    message_metadata_prismatic_light_glass_ready: results.every((result) => result.message_metadata_prismatic_light_glass_ready === true),
    thread_subtitle_prismatic_light_glass_ready: results.every((result) => result.thread_subtitle_prismatic_light_glass_ready === true),
    composer_shortcut_hint_prismatic_light_glass_ready: results.every((result) => result.composer_shortcut_hint_prismatic_light_glass_ready === true),
	    rail_metadata_chip_prismatic_light_glass_ready: results.every((result) => result.rail_metadata_chip_prismatic_light_glass_ready === true),
	    rail_status_count_prismatic_light_glass_ready: results.every((result) => result.rail_status_count_prismatic_light_glass_ready === true),
	    rail_preview_prismatic_etch_light_glass_ready: results.every((result) => result.rail_preview_prismatic_etch_light_glass_ready === true),
	    rail_chat_title_prismatic_etch_light_glass_ready: results.every((result) => result.rail_chat_title_prismatic_etch_light_glass_ready === true),
	    message_body_prismatic_etch_light_glass_ready: results.every((result) => result.message_body_prismatic_etch_light_glass_ready === true),
	    message_speaker_prismatic_chip_light_glass_ready: results.every((result) => result.message_speaker_prismatic_chip_light_glass_ready === true),
	    composer_placeholder_prismatic_etch_light_glass_ready: results.every((result) => result.composer_placeholder_prismatic_etch_light_glass_ready === true),
	    header_title_prismatic_etch_light_glass_ready: results.every((result) => result.header_title_prismatic_etch_light_glass_ready === true),
    message_routing_badge_light_glass_ready: results.every((result) => result.message_routing_badge_light_glass_ready === true),
    thread_intro_badge_light_glass_ready: results.every((result) => result.thread_intro_badge_light_glass_ready === true),
    status_trust_strip_light_glass_ready: results.every((result) => result.status_trust_strip_light_glass_ready === true),
    menu_surface_viewport_guard_ready: results.every((result) => (result.menu_surface_details || []).every((item) =>
      item.in_viewport === true
      && item.vertical_in_viewport === true
      && item.top_clipped === false
      && item.bottom_clipped === false
    )),
    nav_icon_ready: results.every((result) => result.nav_icon_ready === true),
	    scroll_edge_ready: results.every((result) => result.scroll_edge_ready === true),
	    microcopy_word_split_guard_ready: results.every((result) => result.microcopy_word_split_guard_ready === true),
	    logo_clip_guard_ready: results.every((result) => result.logo_clip_guard_ready === true),
	    avatar_prismatic_rim_light_glass_ready: results.every((result) => result.avatar_prismatic_rim_light_glass_ready === true),
	    active_chat_readability_ready: results.every((result) => result.active_chat_readability_ready === true),
	    placeholder_readability_ready: results.every((result) => result.placeholder_readability_ready === true),
	    small_control_readability_ready: results.every((result) => result.small_control_readability_ready === true)
	      && results.reduce((count, result) => count + (result.small_control_readability_details || []).length, 0) >= 8,
	    visible_text_integrity_ready: results.every((result) => result.visible_text_integrity_ready === true),
	    horizontal_overflow_free: results.every((result) => result.horizontal_overflow_free === true),
	    browser_error_page_absent: results.every((result) => !(result.errors || []).includes("browser_error_page_visible")),
    results: results.map((result) => ({
      name: result.name,
      viewport: result.viewport,
      status: result.status,
      errors: result.errors || [],
      default_submenus_closed_ready: result.default_submenus_closed_ready,
      default_submenus_closed_details: result.default_submenus_closed_details,
      single_submenu_audit_ready: result.single_submenu_audit_ready,
      unavailable_submenu_items_ready: result.unavailable_submenu_items_ready,
      disabled_submenu_item_count: result.disabled_submenu_item_count,
      row_menu_distinct_positions_ready: result.row_menu_distinct_positions_ready,
      mobile_pane_navigation_ready: result.mobile_pane_navigation_ready,
      mobile_pane_route_details: result.mobile_pane_route_details,
      mobile_pane_row_menu_ready: result.mobile_pane_row_menu_ready,
      single_submenu_audit_target_count: result.single_submenu_audit_target_count,
      single_submenu_audit_details: result.single_submenu_audit_details,
      engineering_session_chips_suppressed_ready: result.engineering_session_chips_suppressed_ready,
      engineering_session_chip_details: result.engineering_session_chip_details,
      narrow_composer_non_overlap_ready: result.narrow_composer_non_overlap_ready,
      narrow_composer_non_overlap_details: result.narrow_composer_non_overlap_details,
      preferred_touch_target_ready: result.preferred_touch_target_ready,
	      control_glass_action_ready: result.control_glass_action_ready,
	      harsh_referee_ready: result.harsh_referee_ready,
	      shallow_light_glass_ready: result.shallow_light_glass_ready,
	      light_theme_semantics_ready: result.light_theme_semantics_ready,
	      light_theme_semantics_details: result.light_theme_semantics_details,
	      stable_content_surface_ready: result.stable_content_surface_ready,
	      stable_content_surface_details: result.stable_content_surface_details,
	      native_popover_interaction_ready: result.native_popover_interaction_ready,
	      shallow_floating_surface_ready: result.shallow_floating_surface_ready,
	      floating_surface_details: result.floating_surface_details,
	      restrained_optics_ready: result.restrained_optics_ready,
	      restrained_mobile_metadata_ready: result.restrained_mobile_metadata_ready,
	      restrained_mobile_metadata_details: result.restrained_mobile_metadata_details,
	      visible_mobile_status_count: result.visible_mobile_status_count,
	      key_touch_controls_ready: result.key_touch_controls_ready,
	      key_touch_control_details: result.key_touch_control_details,
	      legacy_extreme_optics_diagnostic_ready: result.legacy_extreme_optics_diagnostic_ready,
      rail_visible: result.rail_visible,
      rail_action_icon_ready: result.rail_action_icon_ready,
      icon_button_ready: result.icon_button_ready,
      icon_button_details: result.icon_button_details,
      icon_prismatic_control_light_glass_ready: result.icon_prismatic_control_light_glass_ready,
      icon_prismatic_control_details: result.icon_prismatic_control_details,
      topbar_action_light_glass_ready: result.topbar_action_light_glass_ready,
      topbar_action_details: result.topbar_action_details,
      chrome_bar_translucency_light_glass_ready: result.chrome_bar_translucency_light_glass_ready,
      chrome_bar_translucency_details: result.chrome_bar_translucency_details,
      chrome_refractive_skin_light_glass_ready: result.chrome_refractive_skin_light_glass_ready,
      chrome_refractive_skin_details: result.chrome_refractive_skin_details,
      clear_white_balance_light_glass_ready: result.clear_white_balance_light_glass_ready,
      clear_white_balance_details: result.clear_white_balance_details,
      chamfer_cut_edge_light_glass_ready: result.chamfer_cut_edge_light_glass_ready,
      chamfer_cut_edge_details: result.chamfer_cut_edge_details,
      prismatic_cut_edge_light_glass_ready: result.prismatic_cut_edge_light_glass_ready,
      prismatic_cut_edge_details: result.prismatic_cut_edge_details,
      pane_prismatic_perimeter_light_glass_ready: result.pane_prismatic_perimeter_light_glass_ready,
      pane_prismatic_perimeter_details: result.pane_prismatic_perimeter_details,
      composer_prismatic_control_light_glass_ready: result.composer_prismatic_control_light_glass_ready,
      composer_prismatic_control_details: result.composer_prismatic_control_details,
      primary_shell_light_glass_ready: result.primary_shell_light_glass_ready,
      primary_shell_surface_details: result.primary_shell_surface_details,
      translucent_shell_light_glass_ready: result.translucent_shell_light_glass_ready,
      translucent_glass_details: result.translucent_glass_details,
	      refractive_depth_light_glass_ready: result.refractive_depth_light_glass_ready,
	      optical_clarity_light_glass_ready: result.optical_clarity_light_glass_ready,
	      surface_clear_alpha_light_glass_ready: result.surface_clear_alpha_light_glass_ready,
	      substrate_caustic_field_light_glass_ready: result.substrate_caustic_field_light_glass_ready,
		      specular_edge_light_glass_ready: result.specular_edge_light_glass_ready,
		      prismatic_dispersion_light_glass_ready: result.prismatic_dispersion_light_glass_ready,
		      caustic_highlight_light_glass_ready: result.caustic_highlight_light_glass_ready,
		      caustic_depth_shift_light_glass_ready: result.caustic_depth_shift_light_glass_ready,
		      caustic_depth_shift_key_count: result.caustic_depth_shift_key_count,
		      optical_thickness_tiers_light_glass_ready: result.optical_thickness_tiers_light_glass_ready,
		      optical_thickness_blur_tier_count: result.optical_thickness_blur_tier_count,
		      optical_thickness_alpha_tier_count: result.optical_thickness_alpha_tier_count,
		      faceted_reflection_light_glass_ready: result.faceted_reflection_light_glass_ready,
		      beveled_rim_light_glass_ready: result.beveled_rim_light_glass_ready,
		      refractive_depth_details: result.refractive_depth_details,
		      substrate_caustic_field_details: result.substrate_caustic_field_details,
		      specular_edge_details: result.specular_edge_details,
		      prismatic_dispersion_details: result.prismatic_dispersion_details,
		      caustic_highlight_details: result.caustic_highlight_details,
		      caustic_depth_shift_details: result.caustic_depth_shift_details,
			      optical_thickness_tier_details: result.optical_thickness_tier_details,
			      faceted_reflection_details: result.faceted_reflection_details,
			      beveled_rim_details: result.beveled_rim_details,
			      surface_clear_alpha_details: result.surface_clear_alpha_details,
			      micro_refraction_light_glass_ready: result.micro_refraction_light_glass_ready,
			      micro_refraction_details: result.micro_refraction_details,
			      sparkle_glint_light_glass_ready: result.sparkle_glint_light_glass_ready,
			      sparkle_glint_details: result.sparkle_glint_details,
			      lens_bloom_light_glass_ready: result.lens_bloom_light_glass_ready,
			      lens_bloom_details: result.lens_bloom_details,
			      spectral_fusion_light_glass_ready: result.spectral_fusion_light_glass_ready,
			      spectral_fusion_details: result.spectral_fusion_details,
			      optical_magnification_light_glass_ready: result.optical_magnification_light_glass_ready,
			      optical_magnification_details: result.optical_magnification_details,
			      biaxial_magnification_light_glass_ready: result.biaxial_magnification_light_glass_ready,
			      biaxial_magnification_details: result.biaxial_magnification_details,
			      anisotropic_magnification_light_glass_ready: result.anisotropic_magnification_light_glass_ready,
			      anisotropic_magnification_details: result.anisotropic_magnification_details,
			      phase_separated_refraction_light_glass_ready: result.phase_separated_refraction_light_glass_ready,
			      phase_separated_refraction_details: result.phase_separated_refraction_details,
			      two_axis_phase_refraction_light_glass_ready: result.two_axis_phase_refraction_light_glass_ready,
			      two_axis_phase_refraction_details: result.two_axis_phase_refraction_details,
			      surface_phase_drift_light_glass_ready: result.surface_phase_drift_light_glass_ready,
			      surface_phase_drift_position_count: result.surface_phase_drift_position_count,
			      surface_phase_drift_details: result.surface_phase_drift_details,
			      surface_lens_scale_drift_light_glass_ready: result.surface_lens_scale_drift_light_glass_ready,
			      surface_lens_scale_drift_size_count: result.surface_lens_scale_drift_size_count,
			      surface_lens_scale_drift_details: result.surface_lens_scale_drift_details,
			      layer_scale_parallax_light_glass_ready: result.layer_scale_parallax_light_glass_ready,
			      layer_scale_parallax_details: result.layer_scale_parallax_details,
			      surface_spectral_angle_drift_light_glass_ready: result.surface_spectral_angle_drift_light_glass_ready,
			      surface_spectral_angle_drift_signature_count: result.surface_spectral_angle_drift_signature_count,
			      surface_spectral_angle_drift_details: result.surface_spectral_angle_drift_details,
			      surface_glint_focal_drift_light_glass_ready: result.surface_glint_focal_drift_light_glass_ready,
			      surface_glint_focal_drift_signature_count: result.surface_glint_focal_drift_signature_count,
			      surface_glint_focal_drift_details: result.surface_glint_focal_drift_details,
			      composer_glint_focal_decoupling_light_glass_ready: result.composer_glint_focal_decoupling_light_glass_ready,
			      composer_glint_focal_decoupling_details: result.composer_glint_focal_decoupling_details,
			      composer_spectral_angle_decoupling_light_glass_ready: result.composer_spectral_angle_decoupling_light_glass_ready,
			      composer_spectral_angle_decoupling_details: result.composer_spectral_angle_decoupling_details,
			      composer_phase_decoupling_light_glass_ready: result.composer_phase_decoupling_light_glass_ready,
			      composer_phase_decoupling_details: result.composer_phase_decoupling_details,
			      composer_layer_scale_decoupling_light_glass_ready: result.composer_layer_scale_decoupling_light_glass_ready,
			      composer_layer_scale_decoupling_details: result.composer_layer_scale_decoupling_details,
			      menu_trigger_ready: result.menu_trigger_ready,
      menu_trigger_details: result.menu_trigger_details,
      folder_chip_touch_ready: result.folder_chip_touch_ready,
      folder_chip_label_prismatic_etch_light_glass_ready: result.folder_chip_label_prismatic_etch_light_glass_ready,
      folder_chip_details: result.folder_chip_details,
      row_menu_touch_ready: result.row_menu_touch_ready,
      row_menu_all_rows_ready: result.row_menu_all_rows_ready,
      row_menu_light_glass_ready: result.row_menu_light_glass_ready,
      row_menu_toggle_details: result.row_menu_toggle_details,
      row_menu_panel_details: result.row_menu_panel_details,
      row_menu_visible_item_count: result.row_menu_visible_item_count,
      row_menu_item_details: result.row_menu_item_details,
      menu_item_icon_ready: result.menu_item_icon_ready,
      menu_item_details: result.menu_item_details,
      menu_surface_ready: result.menu_surface_ready,
      menu_surface_details: result.menu_surface_details,
      command_palette_ready: result.command_palette_ready,
      command_palette_surface_light_glass_ready: result.command_palette_surface_light_glass_ready,
      command_palette_surface_prismatic_perimeter_light_glass_ready: result.command_palette_surface_prismatic_perimeter_light_glass_ready,
      command_palette_backdrop_caustic_veil_light_glass_ready: result.command_palette_backdrop_caustic_veil_light_glass_ready,
      command_palette_panel_details: result.command_palette_panel_details,
      command_palette_backdrop_details: result.command_palette_backdrop_details,
      command_palette_trigger_light_glass_ready: result.command_palette_trigger_light_glass_ready,
      command_palette_close_light_glass_ready: result.command_palette_close_light_glass_ready,
      command_palette_close_prismatic_icon_light_glass_ready: result.command_palette_close_prismatic_icon_light_glass_ready,
      command_palette_close_details: result.command_palette_close_details,
      command_palette_trigger_details: result.command_palette_trigger_details,
		      command_palette_input_light_glass_ready: result.command_palette_input_light_glass_ready,
		      command_palette_input_text_prismatic_etch_light_glass_ready: result.command_palette_input_text_prismatic_etch_light_glass_ready,
		      command_palette_input_placeholder_prismatic_etch_light_glass_ready: result.command_palette_input_placeholder_prismatic_etch_light_glass_ready,
		      command_palette_input_row_prismatic_separator_light_glass_ready: result.command_palette_input_row_prismatic_separator_light_glass_ready,
		      command_palette_results_well_light_glass_ready: result.command_palette_results_well_light_glass_ready,
			      command_palette_input_icon_light_glass_ready: result.command_palette_input_icon_light_glass_ready,
			      command_palette_input_icon_prismatic_light_glass_ready: result.command_palette_input_icon_prismatic_light_glass_ready,
		      command_palette_input_icon_details: result.command_palette_input_icon_details,
	      command_palette_item_light_glass_ready: result.command_palette_item_light_glass_ready,
	      command_palette_item_prismatic_rim_light_glass_ready: result.command_palette_item_prismatic_rim_light_glass_ready,
	      command_palette_item_hover_prismatic_light_glass_ready: result.command_palette_item_hover_prismatic_light_glass_ready,
	      command_palette_item_label_prismatic_etch_light_glass_ready: result.command_palette_item_label_prismatic_etch_light_glass_ready,
      command_palette_item_details: result.command_palette_item_details,
	      command_palette_input_details: result.command_palette_input_details,
	      command_palette_input_row_details: result.command_palette_input_row_details,
	      command_palette_results_well_details: result.command_palette_results_well_details,
      control_form_control_title_touch_ready: result.control_form_control_title_touch_ready,
      control_form_control_details: result.control_form_control_details,
      chat_row_option_semantic_touch_ready: result.chat_row_option_semantic_touch_ready,
      chat_row_option_details: result.chat_row_option_details,
      rail_chat_row_prismatic_slab_light_glass_ready: result.rail_chat_row_prismatic_slab_light_glass_ready,
      rail_chat_row_prismatic_slab_details: result.rail_chat_row_prismatic_slab_details,
      thread_tools_menu_ready: result.thread_tools_menu_ready,
      thread_tools_trigger_details: result.thread_tools_trigger_details,
      thread_tools_panel_details: result.thread_tools_panel_details,
      thread_tools_item_details: result.thread_tools_item_details,
      composer_tools_menu_ready: result.composer_tools_menu_ready,
      composer_tools_trigger_light_glass_ready: result.composer_tools_trigger_light_glass_ready,
      composer_tools_trigger_details: result.composer_tools_trigger_details,
      composer_tools_panel_details: result.composer_tools_panel_details,
      composer_tools_item_details: result.composer_tools_item_details,
      composer_popover_ready: result.composer_popover_ready,
      composer_popover_item_label_prismatic_etch_light_glass_ready: result.composer_popover_item_label_prismatic_etch_light_glass_ready,
      composer_popover_header_prismatic_etch_light_glass_ready: result.composer_popover_header_prismatic_etch_light_glass_ready,
      composer_popover_header_prismatic_etch_details: result.composer_popover_header_prismatic_etch_details,
      composer_popover_search_light_glass_ready: result.composer_popover_search_light_glass_ready,
      composer_popover_search_placeholder_prismatic_etch_light_glass_ready: result.composer_popover_search_placeholder_prismatic_etch_light_glass_ready,
      rail_search_light_glass_ready: result.rail_search_light_glass_ready,
      rail_search_placeholder_prismatic_etch_light_glass_ready: result.rail_search_placeholder_prismatic_etch_light_glass_ready,
      rail_search_placeholder_prismatic_etch_details: result.rail_search_placeholder_prismatic_etch_details,
      rail_prismatic_filter_light_glass_ready: result.rail_prismatic_filter_light_glass_ready,
      rail_prismatic_filter_details: result.rail_prismatic_filter_details,
      rail_search_visible_count: result.rail_search_visible_count,
      rail_search_details: result.rail_search_details,
      composer_popover_panel_details: result.composer_popover_panel_details,
      composer_popover_search_details: result.composer_popover_search_details,
      composer_popover_item_details: result.composer_popover_item_details,
      micro_surface_light_glass_ready: result.micro_surface_light_glass_ready,
      micro_prismatic_badge_light_glass_ready: result.micro_prismatic_badge_light_glass_ready,
      micro_badge_label_prismatic_etch_light_glass_ready: result.micro_badge_label_prismatic_etch_light_glass_ready,
      micro_surface_details: result.micro_surface_details,
      message_metadata_prismatic_light_glass_ready: result.message_metadata_prismatic_light_glass_ready,
      message_metadata_prismatic_details: result.message_metadata_prismatic_details,
      thread_subtitle_prismatic_light_glass_ready: result.thread_subtitle_prismatic_light_glass_ready,
      thread_subtitle_prismatic_details: result.thread_subtitle_prismatic_details,
      composer_shortcut_hint_prismatic_light_glass_ready: result.composer_shortcut_hint_prismatic_light_glass_ready,
      composer_shortcut_hint_expected_visible: result.composer_shortcut_hint_expected_visible,
      composer_shortcut_hint_prismatic_details: result.composer_shortcut_hint_prismatic_details,
      rail_metadata_chip_prismatic_light_glass_ready: result.rail_metadata_chip_prismatic_light_glass_ready,
      rail_metadata_chip_expected_visible: result.rail_metadata_chip_expected_visible,
      rail_metadata_chip_prismatic_details: result.rail_metadata_chip_prismatic_details,
	      rail_status_count_prismatic_light_glass_ready: result.rail_status_count_prismatic_light_glass_ready,
	      rail_status_count_expected_visible: result.rail_status_count_expected_visible,
	      rail_status_count_prismatic_details: result.rail_status_count_prismatic_details,
	      rail_preview_prismatic_etch_light_glass_ready: result.rail_preview_prismatic_etch_light_glass_ready,
	      rail_preview_expected_visible: result.rail_preview_expected_visible,
	      rail_preview_prismatic_etch_details: result.rail_preview_prismatic_etch_details,
	      rail_chat_title_prismatic_etch_light_glass_ready: result.rail_chat_title_prismatic_etch_light_glass_ready,
	      rail_chat_title_expected_visible: result.rail_chat_title_expected_visible,
	      rail_chat_title_prismatic_etch_details: result.rail_chat_title_prismatic_etch_details,
	      message_body_prismatic_etch_light_glass_ready: result.message_body_prismatic_etch_light_glass_ready,
	      message_body_prismatic_etch_details: result.message_body_prismatic_etch_details,
	      message_speaker_prismatic_chip_light_glass_ready: result.message_speaker_prismatic_chip_light_glass_ready,
	      message_speaker_prismatic_chip_details: result.message_speaker_prismatic_chip_details,
	      composer_placeholder_prismatic_etch_light_glass_ready: result.composer_placeholder_prismatic_etch_light_glass_ready,
	      composer_placeholder_prismatic_etch_details: result.composer_placeholder_prismatic_etch_details,
	      header_title_prismatic_etch_light_glass_ready: result.header_title_prismatic_etch_light_glass_ready,
      header_title_expected_count: result.header_title_expected_count,
      header_title_prismatic_etch_details: result.header_title_prismatic_etch_details,
      message_routing_badge_light_glass_ready: result.message_routing_badge_light_glass_ready,
      thread_intro_badge_light_glass_ready: result.thread_intro_badge_light_glass_ready,
      thread_intro_badge_visible: result.thread_intro_badge_visible,
      thread_intro_badge_details: result.thread_intro_badge_details,
      status_trust_strip_light_glass_ready: result.status_trust_strip_light_glass_ready,
      status_trust_strip_visible: result.status_trust_strip_visible,
      status_trust_badge_details: result.status_trust_badge_details,
      nav_icon_ready: result.nav_icon_ready,
      scroll_edge_ready: result.scroll_edge_ready,
      microcopy_word_split_guard_ready: result.microcopy_word_split_guard_ready,
      microcopy_wrap_details: result.microcopy_wrap_details,
      logo_clip_guard_ready: result.logo_clip_guard_ready,
      logo_clip_details: result.logo_clip_details,
      avatar_prismatic_rim_light_glass_ready: result.avatar_prismatic_rim_light_glass_ready,
      avatar_prismatic_rim_details: result.avatar_prismatic_rim_details,
      active_chat_readability_ready: result.active_chat_readability_ready,
      active_chat_readability_details: result.active_chat_readability_details,
      placeholder_readability_ready: result.placeholder_readability_ready,
      placeholder_readability_details: result.placeholder_readability_details,
      small_control_readability_ready: result.small_control_readability_ready,
      small_control_readability_details: result.small_control_readability_details,
      visible_text_integrity_ready: result.visible_text_integrity_ready,
      visible_text_integrity_probe: result.visible_text_integrity_probe,
      composer_glass_ready: result.composer_glass_ready,
      send_glass_ready: result.send_glass_ready,
      horizontal_overflow_free: result.horizontal_overflow_free,
      selectors: result.selectors,
    })),
    failures,
  };

  fs.writeSync(1, JSON.stringify(report) + "\n");
  if (failures.length > 0) {
    process.exit(1);
  }
})().catch((error) => {
  console.error(error?.stack || error);
  process.exit(1);
});
