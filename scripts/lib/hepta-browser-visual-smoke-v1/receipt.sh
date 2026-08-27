# Emits the canonical v1 receipt. Keep producer and field names stable for
# current-readiness and downstream evidence consumers.
hepta_browser_emit_receipt() {
report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg output_dir "$OUT_DIR" \
  --arg candidate_commit "$(git rev-parse HEAD 2>/dev/null || printf unknown)" \
  --arg candidate_tree "$(git rev-parse 'HEAD^{tree}' 2>/dev/null || printf unknown)" \
  --arg browser_executable "$CHROME_BIN" \
  --arg browser_version "$CHROME_VERSION" \
  --arg browser_executable_sha256 "$CHROME_SHA256" \
  --argjson telegram_live_send_enabled "$telegram_live_send_enabled" \
  --argjson native_post_real_activation_enabled "$native_post_real_activation_enabled" \
  --argjson control_ui_v4_runtime_bound "$control_ui_v4_runtime_bound" \
  --arg logo_dimensions "$logo_dimensions" \
  --arg logo_sha "$(shasum -a 256 "$logo_png" | awk '{print $1}')" \
  --arg control_ui_base_js_sha "$source_js_sha" \
  --arg control_ui_runtime_js_sha "$runtime_js_sha" \
  --arg control_ui_expected_bundle_sha "$expected_bundle_js_sha" \
  --arg control_ui_js_sha "$served_js_sha" \
  --arg control_ui_js_etag "$served_js_etag" \
  --arg desktop_sha "$(shasum -a 256 "$OUT_DIR/desktop.png" | awk '{print $1}')" \
  --arg narrow_sha "$(shasum -a 256 "$OUT_DIR/narrow.png" | awk '{print $1}')" \
  --arg mobile_sha "$(shasum -a 256 "$OUT_DIR/mobile.png" | awk '{print $1}')" \
  --arg phone320_sha "$(shasum -a 256 "$OUT_DIR/phone320.png" | awk '{print $1}')" \
  --argjson phone320_bytes "$(wc -c <"$OUT_DIR/phone320.png" | tr -d ' ')" \
  --argjson density_qa "$density_qa_json" \
  --argjson progressive_enhancement_qa "$progressive_qa_json" \
  --argjson progressive_enhancement_adversarial_qa "$progressive_adversarial_qa_json" \
  '{
    schema_version:1,
    kind:"hepta-browser-visual-smoke",
    producer:"scripts/hepta-browser-visual-smoke.sh",
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    output_dir:$output_dir,
    candidate:{commit:$candidate_commit,tree:$candidate_tree},
    browser:"playwright-explicit-chromium-executable",
    browser_executable:$browser_executable,
    browser_version:$browser_version,
    browser_executable_sha256:$browser_executable_sha256,
    rust_served_runtime_asset_bound:$control_ui_v4_runtime_bound,
    rust_runtime_validation:true,
    browser_validation:true,
    device_validation:false,
    production_authority:false,
    effect_authority:false,
    operator_acceptance:false,
    promotion:false,
    control_ui_js_bundle:{
      base_sha256:$control_ui_base_js_sha,
      runtime_sha256:$control_ui_runtime_js_sha,
      expected_bundle_sha256:$control_ui_expected_bundle_sha,
      served_bundle_sha256:$control_ui_js_sha,
      etag:$control_ui_js_etag,
      source_order:["apps/hepta-control-ui/control-ui.js","hepta-ui-v4-runtime-bundle-boundary","apps/hepta-control-ui/control-ui-v4-runtime.js"],
      exact_bytes_bound:($control_ui_expected_bundle_sha == $control_ui_js_sha),
      etag_bound:($control_ui_js_etag == ("\"sha256-" + $control_ui_js_sha + "\"")),
      runtime_bound:$control_ui_v4_runtime_bound,
      single_served_path:"/control-ui.js"
    },
    checked_text:[
      "data-rust-frontend-renderer=\"hepta-core::control_ui\"",
      "data-no-js-fallback=\"navigation\"",
      "data-progressive-enhancement=\"same-origin-read-only\"",
      "data-control-ui-capability-mode=\"local-read-only\"",
      "data-control-ui-live-adapter-bound=\"false\"",
      "defer src=\"./control-ui.js\"",
      "data-telegram-multi-agent-chat=\"true\"",
      "data-control-ui-product-first=\"true\"",
      "data-control-ui-primary-path=\"telegram-chat-shell\"",
      "data-control-ui-telegram-shell=\"true\"",
      "data-control-ui-top-design-referee=\"liquid-glass-2026-wcag22-320-reflow\"",
      "data-control-ui-harsh-referee=\"2026-06-08-liquid-glass-menus-sidebars-scroll-search\"",
      "data-control-ui-secondary-map=\"collapsed\"",
      "data-control-ui-runtime-rail=\"local-review-safety-evidence\"",
      "data-control-ui-secondary-nav=\"collapsed\"",
      "data-control-ui-composer-product-first=\"true\"",
      "data-mobile-compact-composer=\"true\"",
      "data-control-ui-composer-more=\"collapsed\"",
      "data-control-ui-composer-tools-trigger=\"light-glass\"",
      "data-control-ui-composer-tools-panel=\"light-glass\"",
      "data-control-ui-composer-popover-panel=\"light-glass\"",
      "data-control-ui-topbar-action=\"light-glass\"",
      "data-control-ui-micro-surface",
      "data-control-ui-thread-intro-badge",
      "data-control-ui-status-trust-badge",
      "data-control-ui-work-rail=\"product-first\"",
      "data-control-ui-compact-product-path=\"narrow-mobile\"",
      "data-control-ui-thread-tools-trigger=\"light-glass\"",
      "data-control-ui-thread-tools-panel=\"light-glass\"",
      "data-open-command-palette",
      "id=\"command-palette\"",
      "data-control-ui-command-palette-surface=\"light-glass\"",
      "data-control-ui-command-palette-input=\"light-glass\"",
      "data-control-ui-command-palette-close=\"light-glass\"",
      "data-control-ui-catalog-mount=\"palette\"",
      "hepta-ui-v4-runtime-bundle-boundary",
      "HeptaUiV4ReadState",
      "controlUiV4Runtime=ready",
      "controlUiV4RuntimeAuthority=local-ui-only"
    ],
    control_ui_product_first_ready:true,
    control_ui_primary_path:"telegram-chat-shell",
    control_ui_telegram_shell_ready:true,
    control_ui_dashboard_cards_hidden:true,
    control_ui_secondary_map_collapsed:true,
    control_ui_runtime_rail_product_first_ready:true,
    control_ui_secondary_nav_collapsed:true,
    control_ui_composer_product_first_ready:true,
    control_ui_work_rail_product_first_ready:true,
    control_ui_compact_product_path_ready:true,
    control_ui_engineering_copy_hidden:true,
    control_ui_top_design_referee_ready:true,
    control_ui_320_reflow_ready:$density_qa.phone320_ready,
    control_ui_expected_visibility_ready:$density_qa.expected_visibility_ready,
    control_ui_stable_content_surface_ready:$density_qa.stable_content_surface_ready,
    control_ui_tempered_surface_budget_ready:$density_qa.tempered_surface_budget_ready,
    control_ui_visible_text_floor_ready:$density_qa.visible_text_floor_ready,
    control_ui_key_touch_controls_ready:$density_qa.key_touch_controls_ready,
    control_ui_mobile_single_topbar_ready:$density_qa.mobile_single_topbar_ready,
    control_ui_mobile_topbar_semantics_ready:$density_qa.mobile_topbar_semantics_ready,
    control_ui_mobile_single_bottom_action_layer_ready:$density_qa.mobile_single_bottom_action_layer_ready,
    control_ui_mobile_primary_actions_ready:$density_qa.mobile_primary_actions_ready,
    control_ui_narrow_shell_density_ready:$density_qa.narrow_shell_density_ready,
    control_ui_narrow_single_action_row_ready:$density_qa.narrow_single_action_row_ready,
    control_ui_maximum_visible_glass_surface_count:$density_qa.maximum_visible_glass_surface_count,
    control_ui_maximum_shadow_layer_count:$density_qa.maximum_shadow_layer_count,
    control_ui_maximum_gradient_layer_count:$density_qa.maximum_gradient_layer_count,
    control_ui_maximum_border_layer_count:$density_qa.maximum_border_layer_count,
    control_ui_visible_under_12px_count:$density_qa.visible_under_12px_count,
    control_ui_visual_density_qa_ready:$density_qa.control_ui_visual_density_qa_ready,
    control_ui_browser_error_page_absent:$density_qa.browser_error_page_absent,
    control_ui_horizontal_overflow_free:$density_qa.horizontal_overflow_free,
    control_ui_progressive_enhancement_ready:true,
    control_ui_unavailable_controls_ready:$progressive_enhancement_qa.unavailable_controls_ready,
    control_ui_unavailable_click_noop_ready:$progressive_enhancement_qa.unavailable_click_noop_ready,
    control_ui_seeded_conversations_ready:$progressive_enhancement_qa.seeded_conversations_ready,
    control_ui_local_json_inspector_ready:$progressive_enhancement_qa.local_json_inspector_ready,
    control_ui_composer_picker_search_ready:$progressive_enhancement_qa.composer_picker_search_ready,
    control_ui_local_draft_insertion_ready:$progressive_enhancement_qa.local_draft_insertion_ready,
    control_ui_local_route_navigation_ready:$progressive_enhancement_qa.local_route_navigation_ready,
    control_ui_mobile_pane_transition_ready:$progressive_enhancement_qa.mobile_pane_transition_ready,
    control_ui_current_route_entries_ready:$progressive_enhancement_qa.current_route_entries_ready,
    control_ui_route_view_screenshots_ready:$progressive_enhancement_qa.route_view_screenshots_ready,
    control_ui_route_page_context_complete_ready:$progressive_enhancement_qa.route_page_context_complete_ready,
    control_ui_route_view_screenshots:$progressive_enhancement_qa.route_view_screenshots,
    control_ui_readonly_registry_route_count:$progressive_enhancement_qa.registry_route_count,
    control_ui_readonly_registry_successful_route_count:$progressive_enhancement_qa.successful_route_count,
    control_ui_cross_origin_request_count:$progressive_enhancement_qa.cross_origin_request_count,
    control_ui_mutation_endpoint_called:$progressive_enhancement_qa.mutation_endpoint_called,
    control_ui_live_adapter_bound:$progressive_enhancement_qa.live_adapter_bound,
    control_ui_progressive_adversarial_ready:($progressive_enhancement_adversarial_qa.status == "ready"),
    density_qa:$density_qa,
    progressive_enhancement_qa:$progressive_enhancement_qa,
    progressive_enhancement_adversarial_qa:$progressive_enhancement_adversarial_qa,
    checked_assets:[
      {path:"/styles.css", markers:[".tg-conversation-rail",".tg-thread-panel",".command-palette","safe-area-inset-bottom","mrog","data-control-ui-compact-product-path","data-control-ui-primary-shell-light-glass","crs","cwb","cce","pce","ppe","cpe","mpb","ipc","avr","rpf","rcs","mmp","tsp","csh","rms","hte","rsc","rpe","mbp","bsp","rsp","fcp","strong){filter","--x:0 1px #fff6","text-shadow:var(--x)","rdlg","oclg","data-control-ui-tspcfrg","dsc","mecs","cmv","ctlg","cplg","cpsg","rmlg","ttlg","bmslg","mslg","tiblg","stslg","talg","body[data-view=chat] .hepta-secondary-map{display:none}","gar26","cps","cpis","cpt","cpc","cpir","cph","cprw","cprr","cpkc","cpilg","data-control-ui-command-palette-input=light-glass","data-control-ui-command-palette-result=light-glass"]},
      {path:"/assets/hepta-agent-logo.png", dimensions:$logo_dimensions, sha256:$logo_sha},
      {
        path:"/control-ui.js",
        sha256:$control_ui_js_sha,
        etag:$control_ui_js_etag,
        source_bound:true,
        etag_bound:true,
        inline_script:false,
        runtime_bound:$control_ui_v4_runtime_bound,
        base_sha256:$control_ui_base_js_sha,
        runtime_sha256:$control_ui_runtime_js_sha
      }
    ],
    telegram_live_send_enabled:$telegram_live_send_enabled,
    native_post_real_activation_enabled:$native_post_real_activation_enabled,
    screenshots:[
      {name:"desktop", viewport:"1365x900", sha256:$desktop_sha},
      {name:"narrow", viewport:"768x900", sha256:$narrow_sha},
      {name:"mobile", viewport:"500x844", sha256:$mobile_sha},
      {name:"phone320", viewport:"320x844", sha256:$phone320_sha, bytes:$phone320_bytes, path:($output_dir + "/phone320.png")}
    ],
    side_effects:{
      telegram_read:false,
      telegram_send:false,
      native_post_real_mutation:false,
      provider_invoked:false
    }
  }')"

printf '%s\n' "$report"

if [[ -n "$REPORT_PATH" ]]; then
  hepta_safe_output_atomic_write_text "$REPORT_PATH" "$report" || {
    echo "could not atomically write browser smoke receipt" >&2
    return 1
  }
fi
}
