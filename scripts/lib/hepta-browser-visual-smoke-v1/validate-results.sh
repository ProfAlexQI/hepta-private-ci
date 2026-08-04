# Checks the three machine-readable browser probes without altering their
# historical fail-closed jq contract.
hepta_browser_validate_results() {
if [[ "$progressive_qa_status" != "0" ]] || ! jq -e '
  .status == "ready"
  and (.command_catalog | length) == 51
  and ([.command_catalog[].id] | unique | length) == 51
  and ([.command_catalog[] | select(.palette == true)] | length) == 18
  and ([.command_catalog[] | select(.route != null)] | length) == 21
  and ([.command_catalog[] | select(
    (.id | type) != "string"
    or (.label | type) != "string"
    or (.command | type) != "string"
    or (.id | length) == 0
    or (.label | length) == 0
    or (.command | length) == 0
    or (.route != null and (.route | startswith("/api/") | not))
  )] | length) == 0
  and .registry_route_count == 21
  and .successful_route_count == 21
  and .snapshot_request_count == 1
  and .copy_interaction_ready == true
  and .chat_search_ready == true
  and .command_palette_search_ready == true
  and .command_palette_navigation_ready == true
  and .route_link_navigation_ready == true
  and .current_route_entries_ready == true
  and .route_directory_ready == true
  and .current_route_entry_audit.directory_entry_count == 26
  and .top_nav_navigation_ready == true
  and .route_history_ready == true
  and .route_view_screenshots_ready == true
  and (.route_view_screenshots | length) == 2
  and ([.route_view_screenshots[] | select(.width == 1365 and .height == 900)] | length) == 1
  and ([.route_view_screenshots[] | select(.width == 320 and .height == 844)] | length) == 1
  and ([.route_view_screenshots[] | select(
    .actual_visible_route_card_count != 1
    or .actual_visible_route_card_ids[0] != .target_id
    or .target_visibility.visible != true
  )] | length) == 0
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
  and .no_script_product_truth.route_ready == true
  and .no_script_product_truth.current_route_entries_ready == true
  and .no_script_product_truth.current_route_hashes_ready == true
  and .no_script_product_truth.current_route_anchor_count == 55
  and .no_script_product_truth.current_row_route_action_count == 3
  and .no_script_product_truth.route_directory_entry_count == 26
  and .no_script_product_truth.route_directory_ready == true
  and .no_script_product_truth.route_directory_summary_visible == true
  and .no_script_product_truth.unavailable_control_count == 14
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
  .status == "ready"
  and .schema_version == 2
  and .control_ui_visual_density_qa_ready == true
  and .viewport_count == 4
  and .phone320_ready == true
  and .expected_visibility_ready == true
  and .browser_error_page_absent == true
  and .horizontal_overflow_free == true
  and .stable_content_surface_ready == true
  and .tempered_surface_budget_ready == true
  and .visible_text_floor_ready == true
  and .key_touch_controls_ready == true
  and .mobile_single_topbar_ready == true
  and .mobile_single_bottom_action_layer_ready == true
  and .mobile_primary_actions_ready == true
  and .maximum_visible_glass_surface_count <= 4
  and .maximum_shadow_layer_count <= 1
  and .maximum_gradient_layer_count <= 1
  and .maximum_border_layer_count <= 1
  and .visible_under_12px_count == 0
  and (.results | length) == 4
  and (.results | all(.status == "ready"))
  and (.failures | length) == 0
' <<<"$density_qa_json" >/dev/null; then
  echo "Control UI tempered-glass density QA failed" >&2
  jq '{
    status,
    failures,
    viewport_count,
    phone320_ready,
    expected_visibility_ready,
    browser_error_page_absent,
    horizontal_overflow_free,
    stable_content_surface_ready,
    tempered_surface_budget_ready,
    visible_text_floor_ready,
    key_touch_controls_ready,
    mobile_single_topbar_ready,
    mobile_single_bottom_action_layer_ready,
    mobile_primary_actions_ready,
    maximum_visible_glass_surface_count,
    maximum_shadow_layer_count,
    maximum_gradient_layer_count,
    maximum_border_layer_count,
    visible_under_12px_count,
    results
  }' <<<"$density_qa_json" >&2 || true
  exit 1
fi
}
