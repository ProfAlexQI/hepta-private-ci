#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

READINESS_DIR="${HEPTA_UI_PRODUCT_READINESS_DIR:-/Users/qianqi/.openclaw/tmp/hepta-ui-product-readiness.mention-taxonomy-20260615}"
REPORT_PATH="${HEPTA_UI_TOP_DESIGN_REFEREE_REFRESH_REPORT_PATH:-$READINESS_DIR/ui-top-design-referee-refresh-gate.json}"
REFRESH_DIR="${HEPTA_UI_TOP_DESIGN_REFEREE_REFRESH_DIR:-$READINESS_DIR/top-design-referee-refresh}"
REFRESH_MARKDOWN_PATH="$REFRESH_DIR/top-design-referee-refresh.md"

CONTROL_BROWSER_REPORT_PATH="$READINESS_DIR/control-ui-browser-smoke.json"
NATIVE_FIXTURE_REPORT_PATH="$READINESS_DIR/native-fixture/native-fixture-visual-smoke.json"
SELECTED_ROW_MANIFEST_PATH="$READINESS_DIR/native-fixture/selected-row-variant-screenshots.json"
NATIVE_WINDOW_ROUTE_REPORT_PATH="$READINESS_DIR/native-window-routes-smoke.json"
NATIVE_WINDOW_SECONDARY_REPORT_PATH="$READINESS_DIR/native-window-secondary-smoke.json"
NATIVE_WINDOW_SECONDARY_MOBILE_REPORT_PATH="$READINESS_DIR/native-window-secondary-mobile-smoke.json"
SCREENSHOT_MANIFEST_PATH="$READINESS_DIR/screenshot-manifest.json"
FUTURE_PLAN_REFRESH_REPORT_PATH="$READINESS_DIR/ui-future-plan-refresh-gate.json"
OPERATOR_BRIEFING_REFRESH_REPORT_PATH="$READINESS_DIR/ui-operator-briefing-refresh-gate.json"
EVIDENCE_ARCHIVE_REPORT_PATH="$READINESS_DIR/ui-evidence-archive-gate.json"

require_command() {
  if ! command -v "$1" >/dev/null 2>&1; then
    printf '%s is required for the Hepta UI top-design referee refresh gate\n' "$1" >&2
    exit 2
  fi
}

require_report() {
  local path="$1"
  if [[ ! -s "$path" ]]; then
    printf 'Missing required top-design referee refresh input: %s\n' "$path" >&2
    exit 1
  fi
  jq empty "$path" >/dev/null
}

file_sha256() {
  shasum -a 256 "$1" | awk '{print $1}'
}

file_bytes() {
  wc -c <"$1" | tr -d ' '
}

require_command jq
require_command shasum

require_report "$CONTROL_BROWSER_REPORT_PATH"
require_report "$NATIVE_FIXTURE_REPORT_PATH"
require_report "$SELECTED_ROW_MANIFEST_PATH"
require_report "$NATIVE_WINDOW_ROUTE_REPORT_PATH"
require_report "$NATIVE_WINDOW_SECONDARY_REPORT_PATH"
require_report "$NATIVE_WINDOW_SECONDARY_MOBILE_REPORT_PATH"
require_report "$SCREENSHOT_MANIFEST_PATH"
require_report "$FUTURE_PLAN_REFRESH_REPORT_PATH"
require_report "$OPERATOR_BRIEFING_REFRESH_REPORT_PATH"
require_report "$EVIDENCE_ARCHIVE_REPORT_PATH"

rm -rf "$REFRESH_DIR"
mkdir -p "$REFRESH_DIR"

TMP_DIR="$(mktemp -d "${TMPDIR:-/tmp}/hepta-ui-top-design-referee-refresh.XXXXXX")"
REPORT_DRAFT="$TMP_DIR/top-design-referee-refresh-draft.json"
REPORT_TMP="$TMP_DIR/top-design-referee-refresh-report.json"
MARKDOWN_TMP="$TMP_DIR/top-design-referee-refresh.md"
trap 'rm -rf "$TMP_DIR"' EXIT

control_sha="$(file_sha256 "$CONTROL_BROWSER_REPORT_PATH")"
native_sha="$(file_sha256 "$NATIVE_FIXTURE_REPORT_PATH")"
selected_row_manifest_sha="$(file_sha256 "$SELECTED_ROW_MANIFEST_PATH")"
route_sha="$(file_sha256 "$NATIVE_WINDOW_ROUTE_REPORT_PATH")"
secondary_sha="$(file_sha256 "$NATIVE_WINDOW_SECONDARY_REPORT_PATH")"
secondary_mobile_sha="$(file_sha256 "$NATIVE_WINDOW_SECONDARY_MOBILE_REPORT_PATH")"
screenshot_manifest_sha="$(file_sha256 "$SCREENSHOT_MANIFEST_PATH")"
future_plan_sha="$(file_sha256 "$FUTURE_PLAN_REFRESH_REPORT_PATH")"
operator_refresh_sha="$(file_sha256 "$OPERATOR_BRIEFING_REFRESH_REPORT_PATH")"
evidence_archive_sha="$(file_sha256 "$EVIDENCE_ARCHIVE_REPORT_PATH")"

jq -n \
  --arg product "Hepta UI" \
  --arg runtime "hepta" \
  --arg gate "hepta_ui_top_design_referee_refresh_gate" \
  --arg readiness_dir "$READINESS_DIR" \
  --arg report_path "$REPORT_PATH" \
  --arg refresh_dir "$REFRESH_DIR" \
  --arg refresh_markdown_path "$REFRESH_MARKDOWN_PATH" \
  --arg control_report_path "$CONTROL_BROWSER_REPORT_PATH" \
  --arg native_report_path "$NATIVE_FIXTURE_REPORT_PATH" \
  --arg selected_row_manifest_path "$SELECTED_ROW_MANIFEST_PATH" \
  --arg route_report_path "$NATIVE_WINDOW_ROUTE_REPORT_PATH" \
  --arg secondary_report_path "$NATIVE_WINDOW_SECONDARY_REPORT_PATH" \
  --arg secondary_mobile_report_path "$NATIVE_WINDOW_SECONDARY_MOBILE_REPORT_PATH" \
  --arg screenshot_manifest_path "$SCREENSHOT_MANIFEST_PATH" \
  --arg future_plan_path "$FUTURE_PLAN_REFRESH_REPORT_PATH" \
  --arg operator_refresh_path "$OPERATOR_BRIEFING_REFRESH_REPORT_PATH" \
  --arg evidence_archive_path "$EVIDENCE_ARCHIVE_REPORT_PATH" \
  --arg control_sha "$control_sha" \
  --arg native_sha "$native_sha" \
  --arg selected_row_manifest_sha "$selected_row_manifest_sha" \
  --arg route_sha "$route_sha" \
  --arg secondary_sha "$secondary_sha" \
  --arg secondary_mobile_sha "$secondary_mobile_sha" \
  --arg screenshot_manifest_sha "$screenshot_manifest_sha" \
  --arg future_plan_sha "$future_plan_sha" \
  --arg operator_refresh_sha "$operator_refresh_sha" \
  --arg evidence_archive_sha "$evidence_archive_sha" \
  --slurpfile control_file "$CONTROL_BROWSER_REPORT_PATH" \
  --slurpfile native_file "$NATIVE_FIXTURE_REPORT_PATH" \
  --slurpfile selected_row_manifest_file "$SELECTED_ROW_MANIFEST_PATH" \
  --slurpfile route_file "$NATIVE_WINDOW_ROUTE_REPORT_PATH" \
  --slurpfile secondary_file "$NATIVE_WINDOW_SECONDARY_REPORT_PATH" \
  --slurpfile secondary_mobile_file "$NATIVE_WINDOW_SECONDARY_MOBILE_REPORT_PATH" \
  --slurpfile screenshot_manifest_file "$SCREENSHOT_MANIFEST_PATH" \
  --slurpfile future_plan_file "$FUTURE_PLAN_REFRESH_REPORT_PATH" \
  --slurpfile operator_refresh_file "$OPERATOR_BRIEFING_REFRESH_REPORT_PATH" \
  --slurpfile evidence_archive_file "$EVIDENCE_ARCHIVE_REPORT_PATH" \
  '
  ($control_file[0]) as $control
  | ($native_file[0]) as $native
  | ($selected_row_manifest_file[0]) as $selected_rows
  | ($route_file[0]) as $route
  | ($secondary_file[0]) as $secondary
  | ($secondary_mobile_file[0]) as $secondary_mobile
  | ($screenshot_manifest_file[0]) as $manifest
  | ($future_plan_file[0]) as $future
  | ($operator_refresh_file[0]) as $operator_refresh
  | ($evidence_archive_file[0]) as $archive
  | def sha_ready($sha): ($sha | test("^[0-9a-f]{64}$"));
    def controlled_backend_agent_probe_present:
      ((env.HEPTA_UI_BACKEND_AGENT_PROBE_PATH // "") | length) > 0;
    def expected_dispatch_blocker_count:
      if controlled_backend_agent_probe_present then 2 else 3 end;
    def dim_ready:
      (.screenshot.dimensions // "" | capture("^(?<w>[0-9]+)x(?<h>[0-9]+)$")) as $dim
      | ($dim.w | tonumber) <= 430 and ($dim.h | tonumber) >= 800;
    def control_ready:
      $control.status == "ready"
      and $control.control_ui_product_first_ready == true
      and $control.control_ui_top_design_referee_ready == true
      and $control.control_ui_320_reflow_ready == true
      and $control.control_ui_preferred_touch_targets_ready == true
      and $control.control_ui_glass_action_contract_ready == true
      and $control.control_ui_harsh_2026_ready == true
      and $control.control_ui_rail_action_icon_ready == true
      and $control.control_ui_icon_buttons_ready == true
	      and $control.control_ui_menu_triggers_ready == true
	      and $control.control_ui_folder_chip_touch_ready == true
	      and $control.control_ui_row_menu_touch_ready == true
	      and $control.control_ui_row_menu_all_rows_ready == true
	      and $control.control_ui_row_menu_light_glass_ready == true
	      and $control.control_ui_command_palette_ready == true
	      and $control.control_ui_command_palette_surface_light_glass_ready == true
      and $control.control_ui_command_palette_trigger_light_glass_ready == true
      and $control.control_ui_command_palette_close_light_glass_ready == true
	      and $control.control_ui_command_palette_input_light_glass_ready == true
          and $control.control_ui_command_palette_item_light_glass_ready == true
	      and $control.control_ui_form_control_title_touch_ready == true
	      and $control.control_ui_chat_row_option_semantic_touch_ready == true
	      and $control.control_ui_thread_tools_menu_ready == true
	      and $control.control_ui_composer_tools_menu_ready == true
	      and $control.control_ui_composer_popover_ready == true
	      and $control.control_ui_composer_popover_search_light_glass_ready == true
	      and $control.control_ui_rail_search_light_glass_ready == true
	      and $control.control_ui_micro_surface_light_glass_ready == true
	      and $control.control_ui_message_routing_badge_light_glass_ready == true
	      and $control.control_ui_thread_intro_badge_light_glass_ready == true
	      and $control.control_ui_status_trust_strip_light_glass_ready == true
	      and $control.control_ui_icon_button_title_match_ready == true
      and $control.control_ui_menu_trigger_title_match_ready == true
      and $control.control_ui_menu_item_icons_ready == true
      and $control.control_ui_menu_surfaces_ready == true
      and $control.control_ui_menu_surface_viewport_guard_ready == true
	      and $control.control_ui_navigation_icons_ready == true
	      and $control.control_ui_scroll_edge_ready == true
	      and $control.control_ui_microcopy_word_split_guard_ready == true
	      and $control.control_ui_logo_clip_guard_ready == true
	      and $control.control_ui_active_chat_readability_ready == true
	      and $control.control_ui_placeholder_readability_ready == true
	      and $control.control_ui_small_control_readability_ready == true
	      and $control.control_ui_visible_text_integrity_ready == true
	      and $control.control_ui_visual_density_qa_ready == true
      and $control.control_ui_horizontal_overflow_free == true
      and $control.control_ui_browser_error_page_absent == true
      and $control.density_qa.status == "ready"
      and $control.density_qa.viewport_count == 4
      and $control.density_qa.phone320_ready == true
      and $control.density_qa.preferred_touch_targets_ready == true
      and $control.density_qa.control_glass_action_ready == true
      and $control.density_qa.harsh_referee_ready == true
      and $control.density_qa.rail_action_icon_ready == true
      and $control.density_qa.icon_button_ready == true
	      and $control.density_qa.menu_trigger_ready == true
	      and $control.density_qa.folder_chip_touch_ready == true
	      and $control.density_qa.row_menu_touch_ready == true
	      and $control.density_qa.row_menu_all_rows_ready == true
	      and $control.density_qa.row_menu_light_glass_ready == true
	      and $control.density_qa.command_palette_ready == true
	      and $control.density_qa.command_palette_close_light_glass_ready == true
	      and $control.density_qa.command_palette_input_light_glass_ready == true
	      and $control.density_qa.command_palette_item_light_glass_ready == true
	      and $control.density_qa.control_form_control_title_touch_ready == true
	      and $control.density_qa.chat_row_option_semantic_touch_ready == true
	      and $control.density_qa.thread_tools_menu_ready == true
	      and $control.density_qa.composer_tools_menu_ready == true
	      and $control.density_qa.composer_popover_ready == true
	      and $control.density_qa.composer_popover_search_light_glass_ready == true
	      and $control.density_qa.rail_search_light_glass_ready == true
	      and $control.density_qa.micro_surface_light_glass_ready == true
	      and $control.density_qa.message_routing_badge_light_glass_ready == true
	      and $control.density_qa.thread_intro_badge_light_glass_ready == true
	      and $control.density_qa.status_trust_strip_light_glass_ready == true
	      and $control.density_qa.icon_button_title_match_ready == true
      and $control.density_qa.menu_trigger_title_match_ready == true
      and $control.density_qa.menu_item_icon_ready == true
      and $control.density_qa.menu_surface_ready == true
      and $control.density_qa.menu_surface_viewport_guard_ready == true
      and $control.density_qa.nav_icon_ready == true
	      and $control.density_qa.scroll_edge_ready == true
	      and $control.density_qa.active_chat_readability_ready == true
	      and $control.density_qa.placeholder_readability_ready == true
	      and $control.density_qa.small_control_readability_ready == true
	      and $control.density_qa.visible_text_integrity_ready == true
	      and $control.density_qa.horizontal_overflow_free == true
      and ($control.density_qa.results | all(.status == "ready"))
      and ($control.density_qa.failures | length) == 0;
    def control_phone320_screenshot_ready:
      ($control.screenshots // []
        | map(select(
          .name == "phone320"
          and .viewport == "320x844"
          and ((.path // "") | endswith("/phone320.png"))
          and ((.bytes // 0) >= 50000)
          and ((.sha256 // "") | test("^[0-9a-f]{64}$"))
        ))
        | length) == 1;
    def native_fixture_ready:
      $native.status == "ready"
      and $native.native_top_design_referee_ready == true
      and $native.native_320_reflow_ready == true
      and $native.native_mobile_touch_target_preferred_ready == true
      and $native.native_tempered_glass_visual_contract_ready == true
      and $native.native_readability_contrast_clip_ready == true
      and $native.native_telegram_header_icon_affordance_ready == true
      and $native.native_secondary_product_surfaces_ready == true
      and $native.native_secondary_harsh_action_matrix_ready == true
      and $native.native_product_first_visible_copy_ready == true
      and $native.native_visible_audit_chrome_absent == true
      and $native.native_visible_audit_failure_count == 0
      and $native.tempered_glass_visual_contract.status == "ready"
      and $native.tempered_glass_visual_contract.viewport_count == 4
      and $native.tempered_glass_visual_contract.phone320_ready == true
      and $native.tempered_glass_visual_contract.preferred_touch_target_ready == true
      and $native.tempered_glass_visual_contract.light_surface_failure_count == 0
      and $native.tempered_glass_visual_contract.readability_failure_count == 0
      and $native.tempered_glass_visual_contract.min_contrast_ratio >= 4.5
      and $native.secondary_product_surfaces.status == "ready"
      and $native.secondary_product_surfaces.case_count == 15
      and $native.secondary_product_surfaces.action_matrix_ready == true
      and $native.secondary_product_surfaces.action_matrix_case_count == 15
      and $native.secondary_product_surfaces.harsh_action_matrix_ready == true
      and $native.secondary_product_surfaces.harsh_action_matrix_case_count == 15
      and $native.secondary_product_surfaces.harsh_action_failure_count == 0
      and $native.secondary_product_surfaces.icon_svg_ready == true
      and $native.secondary_product_surfaces.icon_text_placeholder_absent == true
      and $native.secondary_product_surfaces.icon_text_placeholder_failure_count == 0
      and $native.secondary_product_surfaces.title_tooltip_ready == true
      and $native.secondary_product_surfaces.title_tooltip_failure_count == 0
      and $native.secondary_product_surfaces.glass_surface_ready == true
      and $native.secondary_product_surfaces.glass_action_group_ready == true
      and $native.secondary_product_surfaces.glass_action_failure_count == 0
      and $native.secondary_product_surfaces.total_action_instance_count == 57
      and $native.secondary_product_surfaces.preferred_touch_target_ready == true
      and $native.secondary_product_surfaces.text_clipping_failure_count == 0
      and $native.secondary_product_surfaces.content_edge_failure_count == 0
      and $native.secondary_product_surfaces.visible_audit_failure_count == 0
      and $native.mobile_safe_area_keyboard.status == "ready"
      and $native.mobile_safe_area_keyboard.content_bounds_ready == true
      and $native.mobile_safe_area_keyboard.content_clipping_failure_count == 0
      and $native.mobile_safe_area_keyboard.horizontal_overflow_free == true
      and $native.selected_row_unique_count == $native.selected_row_variant_count
      and $native.route_variant_unique_count == 4
      and $native.mobile_route_variant_unique_count == 4;
    def true_window_route_hard_ready:
      $route.status == "ready"
      and $route.enabled == true
      and $route.true_window_capture_performed == true
      and $route.route_top_design_referee_ready == true
      and $route.route_content_probe_ready == true
      and $route.route_count == 4
      and $route.route_screenshot_unique_count == 4
      and $route.route_screenshot_unique_ready == true
      and $route.screenshot_count == 4
      and ($route.screenshots | all(.visual_probe.ready == true and .visual_probe.route_content_ready == true))
      and ($route.routes | all(.route_top_design_referee_ready == true and .route_content_ready == true))
      and $route.native_app_log_error_free == true
      and $route.blocked_allowed == false;
    def true_window_route_no_window_accepted:
      $route.status == "not_run"
      and $route.enabled == false
      and (($route.screenshot_count // 0) == 0);
    def true_window_route_ready:
      true_window_route_hard_ready or true_window_route_no_window_accepted;
    def true_window_secondary_hard_ready:
      $secondary.status == "ready"
      and $secondary.enabled == true
      and $secondary.true_window_capture_performed == true
      and $secondary.surface_count == 5
      and $secondary.surface_screenshot_unique_count == 5
      and $secondary.surface_screenshot_unique_ready == true
      and $secondary.screenshot_count == 5
      and ($secondary.screenshots | all(.visual_probe.ready == true))
      and ($secondary.surfaces | all(.visual_probe.ready == true))
      and $secondary.native_app_log_error_free == true
      and $secondary.blocked_allowed == false;
    def true_window_secondary_no_window_accepted:
      $secondary.status == "not_run"
      and $secondary.enabled == false
      and (($secondary.screenshot_count // 0) == 0);
    def true_window_secondary_ready:
      true_window_secondary_hard_ready or true_window_secondary_no_window_accepted;
    def true_window_secondary_mobile_hard_ready:
      $secondary_mobile.status == "ready"
      and $secondary_mobile.enabled == true
      and $secondary_mobile.true_window_capture_performed == true
      and $secondary_mobile.mobile_secondary_content_probe_ready == true
      and $secondary_mobile.mobile_secondary_content_visible_count >= 10
      and $secondary_mobile.surface_count == 5
      and $secondary_mobile.surface_screenshot_unique_count == 5
      and $secondary_mobile.surface_screenshot_unique_ready == true
      and $secondary_mobile.screenshot_count == 5
      and ($secondary_mobile.screenshots | all(.visual_probe.ready == true and .visual_probe.mobile_secondary_content_ready == true))
      and ($secondary_mobile.surfaces | all(.mobile_secondary_content_visible_ready == true and .mobile_secondary_content_visible_count >= 2 and dim_ready))
      and $secondary_mobile.native_app_log_error_free == true
      and $secondary_mobile.blocked_allowed == false;
    def true_window_secondary_mobile_no_window_accepted:
      $secondary_mobile.status == "not_run"
      and $secondary_mobile.enabled == false
      and (($secondary_mobile.screenshot_count // 0) == 0);
    def true_window_secondary_mobile_ready:
      true_window_secondary_mobile_hard_ready or true_window_secondary_mobile_no_window_accepted;
    def hard_true_window_evidence_ready:
      true_window_route_hard_ready
      and true_window_secondary_hard_ready
      and true_window_secondary_mobile_hard_ready;
    def no_window_evidence_accepted:
      true_window_route_no_window_accepted
      and true_window_secondary_no_window_accepted
      and true_window_secondary_mobile_no_window_accepted;
    def expected_secondary_surfaces:
      ["attachment","modal","search","settings","voice"];
    def expected_secondary_action_matrix:
      {
        attachment:["gallery","camera","files","share"],
        modal:["cancel","keep-reviewing","approve"],
        search:["jump","copy","source","filter"],
        settings:["rename","members","mute","apply-after-review"],
        voice:["record","play","drop","send"]
      };
    def expected_control_viewports:
      ["1365x900","320x844","500x844","768x900"];
    def expected_row_routes:
      ["Actions","Approvals","Inspector"];
    def expected_row_indexes:
      [0,1,2];
    def expected_row_viewports:
      ["1280x800","500x844"];
    def control_primary_button_coverage_ready:
      ($control.density_qa.results | length) == 4
      and ([$control.density_qa.results[]
        | "\(.viewport.width)x\(.viewport.height)"
      ] | sort) == expected_control_viewports
      and ($control.density_qa.results | all(. as $result |
        .preferred_touch_target_ready == true
        and .control_glass_action_ready == true
        and .harsh_referee_ready == true
        and .icon_button_ready == true
        and .menu_trigger_ready == true
        and .rail_action_icon_ready == true
        and .menu_item_icon_ready == true
        and .menu_surface_ready == true
        and .thread_tools_menu_ready == true
        and .composer_tools_menu_ready == true
        and .composer_popover_ready == true
        and .micro_surface_light_glass_ready == true
        and .message_routing_badge_light_glass_ready == true
        and .nav_icon_ready == true
        and .scroll_edge_ready == true
        and .visible_text_integrity_ready == true
        and .composer_glass_ready == true
        and .send_glass_ready == true
        and .horizontal_overflow_free == true
        and ((.icon_button_details // []) | length) >= (if .rail_visible == true then 5 else 4 end)
        and (.rail_visible != true or ((.icon_button_details // []) | any(.role == "new-conversation" and .aria_label == "New conversation" and .title_matches_aria_label == true and .width >= 44 and .height >= 44)))
        and ((.icon_button_details // []) | all(.svg_icon_present == true and .visible_icon_text_absent == true and .width >= 44 and .height >= 44))
        and ((.menu_trigger_details // []) | length) >= 2
        and ((.menu_trigger_details // []) | all(.svg_icon_present == true and .visible_icon_text_absent == true and .width >= 44 and .height >= 44))
        and ((.menu_item_details // []) | length) >= 5
        and ((.menu_item_details // []) | all(.icon_svg_present == true and .label_nowrap_ready == true and .height >= 36))
        and ((.menu_surface_details // []) | length) >= 2
        and ((.menu_surface_details // []) | all(.visible == true and .border_radius >= 16 and .item_count >= 1 and .in_viewport == true and .vertical_in_viewport == true and .bottom_clipped == false))
        and ((.thread_tools_panel_details // {}) | .exists == true and .visible == true and .role == "menu" and .aria_label == "Thread tools" and .item_count == 3 and .marker == "light-glass" and .light_glass_ready == true and .effective_luminance >= 0.72 and .effective_luminance <= 0.98 and .in_viewport == true and .top_clipped == false and .bottom_clipped == false)
        and ((.thread_tools_item_details // []) | length) == 3
        and ((.thread_tools_item_details // []) | all(.visible == true and .role == "menuitem" and .height >= 44 and .title_matches_aria_label == true and .icon_svg_present == true and .label_nowrap_ready == true and .readable == true and .contrast_ratio >= 4.5))
        and .composer_tools_trigger_light_glass_ready == true
        and ((.composer_tools_trigger_details // {}) | .exists == true and .marker == "light-glass" and .visible == true and .width >= 44 and .height >= 44 and .light_glass_ready == true and .effective_luminance >= 0.72 and .effective_luminance <= 0.98 and .title_matches_aria_label == true and .svg_icon_present == true and .visible_icon_text_absent == true and .readable == true and .contrast_ratio >= 4.5)
        and ((.composer_tools_panel_details // {}) | .exists == true and .visible == true and .role == "menu" and .aria_label == "Composer tools" and .item_count == 2 and .marker == "light-glass" and .light_glass_ready == true and .effective_luminance >= 0.72 and .effective_luminance <= 0.98 and .in_viewport == true and .top_clipped == false and .bottom_clipped == false)
        and ((.composer_tools_item_details // []) | length) == 2
        and ((.composer_tools_item_details // []) | all(.visible == true and .role == "menuitem" and .height >= 44 and .title_matches_aria_label == true and .icon_svg_present == true and .label_nowrap_ready == true and .select_present == true and .select_visible == true and .select_height >= 44 and .select_title_matches_aria_label == true and .select_readable == true and .select_contrast_ratio >= 4.5 and .readable == true and .contrast_ratio >= 4.5))
        and ((.selectors // [])
          | any(.selector == "[data-agent-chat-send]" and .visible == true and .rect.width >= 44 and .rect.height >= 44))
        and ((.selectors // [])
          | any(.selector == "[data-chat-composer-input]" and .visible == true and .rect.width >= (if $result.viewport.width <= 360 then 120 else 160 end) and .rect.height >= 32))
        and ((.selectors // [])
          | any(.selector == ".tg-compose-bar" and .visible == true and .rect.height >= 44))
      ));
    def secondary_action_matrix_ready:
      $native.secondary_product_surfaces.action_matrix_ready == true
      and $native.secondary_product_surfaces.action_matrix_case_count == 15
      and $native.secondary_product_surfaces.harsh_action_matrix_ready == true
      and $native.secondary_product_surfaces.harsh_action_matrix_case_count == 15
      and $native.secondary_product_surfaces.harsh_action_failure_count == 0
      and $native.secondary_product_surfaces.total_action_instance_count == 57
      and $native.secondary_product_surfaces.expected_action_matrix == expected_secondary_action_matrix
      and ($native.secondary_product_surfaces.results | all(
        .expected_actions_present == true
        and .semantic_button_ready == true
        and .action_matrix_ready == true
        and .harsh_action_matrix_ready == true
        and .surface_glass_ready == true
        and .action_group_glass_ready == true
        and .expected_action_ids == expected_secondary_action_matrix[.surface]
        and .action_ids == .expected_action_ids
        and .action_count == (.expected_action_ids | length)
        and (.action_details | all(
          .tag == "button"
          and .type == "button"
          and .semantic_ready == true
          and .button_style_ready == true
          and .harsh_action_ready == true
          and .icon_present == true
          and .icon_svg_ready == true
          and .icon_text_placeholder_absent == true
          and .title_tooltip_ready == true
          and ((.title // "") | type == "string")
          and ((.title // "") | length > 0)
          and .title == .aria_label
          and .role_ready == true
          and ((.role // "") | type == "string")
          and ((.role // "") | length > 0)
          and ((.aria_label // "") | type == "string")
          and ((.aria_label // "") | length > 0)
          and .disabled == false
        ))
      ));
    def secondary_surface_coverage_ready:
      $native.secondary_product_surfaces.status == "ready"
      and $native.secondary_product_surfaces.surface_count == 5
      and $native.secondary_product_surfaces.viewport_count == 3
      and $native.secondary_product_surfaces.case_count == 15
      and secondary_action_matrix_ready
      and ([$native.secondary_product_surfaces.results[].surface] | unique | sort) == expected_secondary_surfaces
      and ([$native.secondary_product_surfaces.results[] | "\(.viewport.width)x\(.viewport.height)"] | unique | sort) == ["1280x800","320x844","390x844"]
      and ([$native.secondary_product_surfaces.results[].action_count] | add) == 57
      and ($native.secondary_product_surfaces.results | all(
        .ready == true
        and .action_matrix_ready == true
        and .harsh_action_matrix_ready == true
        and .surface_glass_ready == true
        and .action_group_glass_ready == true
        and .expected_actions_present == true
        and .semantic_button_ready == true
        and .actions_usable == true
        and .preferred_touch_target_ready == true
        and .actions_in_surface == true
        and .text_clipping_failure_count == 0
        and .content_edge_failure_count == 0
        and .visible_audit_failure_count == 0
        and .horizontal_overflow_free == true
      ))
      and ([$native.secondary_product_surfaces.results[]
        | select(.surface == "modal" and .action_count >= 3)
      ] | length) == 3;
    def selected_row_coverage_ready:
      ($selected_rows | length) == 18
      and ($native.selected_row_variant_count == 18)
      and ($native.selected_row_unique_count == 18)
      and ([$selected_rows[].name] | unique | length) == 18
      and ([$selected_rows[].sha256] | unique | length) == 18
      and ($selected_rows | all((.sha256 // "") | test("^[0-9a-f]{64}$")))
      and ([$selected_rows[].selected_route] | unique | sort) == expected_row_routes
      and ([$selected_rows[].selected_row] | unique | sort) == expected_row_indexes
      and ([$selected_rows[].viewport] | unique | sort) == expected_row_viewports
      and (expected_row_routes | all(. as $route | ([$selected_rows[] | select(.selected_route == $route)] | length) == 6))
      and (expected_row_routes | all(. as $route | expected_row_indexes | all(. as $row | ([$selected_rows[] | select(.selected_route == $route and .selected_row == $row)] | length) == 2)));
    def true_window_submenu_coverage_ready:
      (
        hard_true_window_evidence_ready
        and ([$route.routes[].route] | unique | sort) == ["actions","approvals","home","inspector"]
        and ([$route.routes[] | select(.route_content_ready == true and .route_top_design_referee_ready == true)] | length) == 4
        and ([$secondary.surfaces[].surface] | unique | sort) == expected_secondary_surfaces
        and ([$secondary.surfaces[] | select(.secondary_surface_selected_ready == true and .visual_probe.ready == true)] | length) == 5
        and ([$secondary_mobile.surfaces[].surface] | unique | sort) == expected_secondary_surfaces
        and ([$secondary_mobile.surfaces[] | select(.mobile_secondary_content_visible_ready == true and .mobile_secondary_content_visible_count >= 2 and .visual_probe.ready == true)] | length) == 5
      )
      or no_window_evidence_accepted;
    def control_level_referee_ready:
      control_primary_button_coverage_ready
      and secondary_surface_coverage_ready
      and selected_row_coverage_ready
      and true_window_submenu_coverage_ready;
    def tempered_glass_2026_ready:
      $native.tempered_glass_visual_contract.status == "ready"
      and $native.tempered_glass_visual_contract.viewport_count == 4
      and $native.tempered_glass_visual_contract.desktop_ready == true
      and $native.tempered_glass_visual_contract.mobile_ready == true
      and $native.tempered_glass_visual_contract.phone_ready == true
      and $native.tempered_glass_visual_contract.phone320_ready == true
      and $native.tempered_glass_visual_contract.preferred_touch_target_ready == true
      and $native.tempered_glass_visual_contract.body_light_ready == true
      and $native.tempered_glass_visual_contract.translucent_panels_ready == true
      and $native.tempered_glass_visual_contract.glass_hairlines_ready == true
      and $native.tempered_glass_visual_contract.backdrop_blur_ready == true
      and $native.tempered_glass_visual_contract.light_accent_ready == true
      and $native.tempered_glass_visual_contract.horizontal_overflow_free == true
      and $native.tempered_glass_visual_contract.light_surface_failure_count == 0
      and $native.tempered_glass_visual_contract.readability_contrast_clip_ready == true
      and $native.tempered_glass_visual_contract.readability_failure_count == 0
      and $native.tempered_glass_visual_contract.min_contrast_ratio >= 4.5
      and $native.tempered_glass_visual_contract.header_icon_affordance_ready == true
      and ($native.tempered_glass_visual_contract.header_text_action_failures | length) == 0
      and $native.tempered_glass_visual_contract.product_first_visible_copy_ready == true
      and $native.tempered_glass_visual_contract.visible_audit_chrome_absent == true
      and $native.tempered_glass_visual_contract.visible_audit_failure_count == 0
      and control_primary_button_coverage_ready
      and secondary_surface_coverage_ready
      and $control.control_ui_320_reflow_ready == true
      and $control.control_ui_preferred_touch_targets_ready == true
      and $control.control_ui_harsh_2026_ready == true
      and $control.control_ui_rail_action_icon_ready == true
      and $control.control_ui_icon_buttons_ready == true
      and $control.control_ui_menu_triggers_ready == true
	      and $control.control_ui_folder_chip_touch_ready == true
		      and $control.control_ui_row_menu_touch_ready == true
		      and $control.control_ui_row_menu_all_rows_ready == true
		      and $control.control_ui_row_menu_light_glass_ready == true
	      and $control.control_ui_command_palette_ready == true
	      and $control.control_ui_command_palette_surface_light_glass_ready == true
      and $control.control_ui_command_palette_trigger_light_glass_ready == true
      and $control.control_ui_command_palette_close_light_glass_ready == true
	      and $control.control_ui_form_control_title_touch_ready == true
	      and $control.control_ui_chat_row_option_semantic_touch_ready == true
	      and $control.control_ui_thread_tools_menu_ready == true
	      and $control.control_ui_composer_tools_menu_ready == true
	      and $control.control_ui_composer_popover_ready == true
	      and $control.control_ui_composer_popover_search_light_glass_ready == true
	      and $control.control_ui_rail_search_light_glass_ready == true
	      and $control.control_ui_micro_surface_light_glass_ready == true
	      and $control.control_ui_message_routing_badge_light_glass_ready == true
	      and $control.control_ui_thread_intro_badge_light_glass_ready == true
	      and $control.control_ui_status_trust_strip_light_glass_ready == true
	      and $control.control_ui_menu_item_icons_ready == true
      and $control.control_ui_menu_surfaces_ready == true
	      and $control.control_ui_navigation_icons_ready == true
	      and $control.control_ui_scroll_edge_ready == true
	      and $control.control_ui_microcopy_word_split_guard_ready == true
	      and $control.control_ui_logo_clip_guard_ready == true
	      and $control.control_ui_active_chat_readability_ready == true
	      and $control.control_ui_placeholder_readability_ready == true
	      and $control.control_ui_small_control_readability_ready == true
	      and $control.control_ui_visible_text_integrity_ready == true
	      and $control.control_ui_horizontal_overflow_free == true;
	    def plan_guardrail_ready:
	      $future.future_plan_refresh_gate_ready == true
	      and ($future.future_plan | map(.id)) == ["r52_minimum_ui_demo_gate","backend_real_receipt_return","ui_refresh_after_real_receipt"]
	      and (
	        $future.r52_minimum_gate.current_full_hard_evidence_ready == true
	        or no_window_evidence_accepted
	      )
      and ($future.backend_receipt_refresh_contract.real_backend_receipt_present | type) == "boolean"
      and $operator_refresh.operator_briefing_refresh_gate_ready == true
      and ($operator_refresh.updated_critical_risk_count >= 1 and $operator_refresh.updated_critical_risk_count <= 4)
      and $operator_refresh.current_next_plan_ids == ["r52_minimum_ui_demo_gate","backend_real_receipt_return","ui_refresh_after_real_receipt"]
      and $operator_refresh.claim_boundary.live_product_claim_ready == false
      and $operator_refresh.claim_boundary.public_distribution_claim_ready == false
      and $operator_refresh.claim_boundary.release_claim_ready == false
      and $archive.evidence_archive_gate_ready == true
      and $archive.all_extracted_items_sha256_match == true;
    def screenshot_manifest_base_ready:
      $manifest.screenshot_manifest_ready == true
      and $manifest.screenshot_count.control_ui == 4
      and $manifest.screenshot_count.native >= 40
      and $manifest.screenshot_count.total >= 44
      and ($manifest.key_screenshots // []
        | map(select(
          .surface == "control-ui"
          and .name == "phone320"
          and .viewport == "320x844"
          and ((.path // "") | endswith("/phone320.png"))
          and ((.bytes // 0) >= 50000)
          and ((.sha256 // "") | test("^[0-9a-f]{64}$"))
        ))
        | length) == 1;
    def screenshot_manifest_hard_ready:
      screenshot_manifest_base_ready
      and $manifest.screenshot_count.native_true_window == 2
      and $manifest.screenshot_count.native_true_window_route == 4
      and $manifest.screenshot_count.native_true_window_secondary == 5
      and $manifest.screenshot_count.native_true_window_secondary_mobile == 5
      and $manifest.screenshot_count.total >= 60;
    def screenshot_manifest_no_window_ready:
      screenshot_manifest_base_ready
      and (($manifest.screenshot_count.native_true_window // 0) == 0)
      and (($manifest.screenshot_count.native_true_window_route // 0) == 0)
      and (($manifest.screenshot_count.native_true_window_secondary // 0) == 0)
      and (($manifest.screenshot_count.native_true_window_secondary_mobile // 0) == 0);
    def screenshot_manifest_ready:
      (hard_true_window_evidence_ready and screenshot_manifest_hard_ready)
      or (no_window_evidence_accepted and screenshot_manifest_no_window_ready);
    (
      control_ready
      and control_phone320_screenshot_ready
      and native_fixture_ready
      and control_level_referee_ready
      and tempered_glass_2026_ready
      and true_window_route_ready
      and true_window_secondary_ready
      and true_window_secondary_mobile_ready
      and screenshot_manifest_ready
      and plan_guardrail_ready
      and sha_ready($control_sha)
      and sha_ready($native_sha)
      and sha_ready($selected_row_manifest_sha)
      and sha_ready($route_sha)
      and sha_ready($secondary_sha)
      and sha_ready($secondary_mobile_sha)
      and sha_ready($screenshot_manifest_sha)
      and sha_ready($future_plan_sha)
      and sha_ready($operator_refresh_sha)
      and sha_ready($evidence_archive_sha)
    ) as $ready
  | {
      product:$product,
      runtime:$runtime,
      gate:$gate,
      status:(if $ready then "ready" else "failed" end),
      top_design_referee_refresh_gate_ready:$ready,
      top_design_current_standards_referee_ready:$ready,
      top_design_harsh_2026_referee_ready:$ready,
      control_ui_harsh_2026_ready:$control.control_ui_harsh_2026_ready,
      native_secondary_harsh_action_matrix_ready:$native.native_secondary_harsh_action_matrix_ready,
      refresh_kind:"local_ui_top_design_referee_2026_refresh",
      refresh_version:46,
      standards_version:"2026-06-24-harsh-badge-micro-surface-light-glass",
      aesthetic_standard:"2026_tempered_glass_liquid_glass",
      readiness_dir:$readiness_dir,
      report_path:$report_path,
      refresh_dir:$refresh_dir,
      refresh_markdown_path:$refresh_markdown_path,
      referee_basis:[
        {
          id:"apple_26_content_first_liquid_glass",
          source:"Apple iOS 26 / macOS Tahoe 26 Liquid Glass design direction",
          url:"https://www.apple.com/newsroom/2025/06/apple-introduces-a-delightful-and-elegant-new-software-design/",
          applied_as:["content_first_hierarchy","material_chrome_as_functional_layer","clear_navigation_control_hierarchy"]
        },
        {
          id:"apple_2026_cross_device_design_system",
          source:"Apple WWDC25 new design system",
          url:"https://developer.apple.com/videos/play/wwdc2025/356/",
          applied_as:["consistent_across_devices","consistent_across_screen_sizes","consistent_across_input_modes"]
        },
        {
          id:"apple_hig_liquid_glass_platform_guidance",
          source:"Apple Human Interface Guidelines Liquid Glass platform guidance",
          url:"https://developer.apple.com/design/human-interface-guidelines",
          applied_as:["platform_native_materials","clear_toolbar_sidebar_layers","system_consistent_controls"]
        },
        {
          id:"apple_hig_2026_liquid_glass_control_refresh",
          source:"Apple Human Interface Guidelines June 2026 Liquid Glass control refresh",
          url:"https://developer.apple.com/design/whats-new/",
          applied_as:["liquid_glass_updated_components","menus_sidebars_scroll_search_tab_views","functional_glass_controls"]
        },
        {
          id:"apple_liquid_glass_adoption_overview",
          source:"Apple Liquid Glass adoption overview",
          url:"https://developer.apple.com/documentation/TechnologyOverviews/adopting-liquid-glass",
          applied_as:["system_material_semantics","controls_over_content","avoid_decorative_glass_noise"]
        },
        {
          id:"apple_hig_materials_liquid_glass_functional_layer",
          source:"Apple Human Interface Guidelines Materials guidance",
          url:"https://developer.apple.com/design/human-interface-guidelines/materials",
          applied_as:["liquid_glass_as_functional_control_layer","floating_controls_over_content","visible_content_separation"]
        },
        {
          id:"material_3_expressive_research",
          source:"Google Material 3 Expressive research",
          url:"https://design.google/library/expressive-material-design-google-research",
          applied_as:["expressive_but_scannable_hierarchy","key_actions_stand_out","grouped_repeated_use_surfaces"]
        },
        {
          id:"material_io26_expressive_adaptive_layout",
          source:"Google I/O 2026 Material expressive adaptive layout guidance",
          url:"https://m3.material.io/blog/whats-new-at-io26",
          applied_as:["mobile_desktop_spatial_adaptive_layouts","expressive_layout_scaffold","cross_device_navigation_patterns"]
        },
        {
          id:"material_3_adaptive_large_screens",
          source:"Android Material 3 adaptive layout guidance",
          url:"https://developer.android.com/develop/ui/compose/designsystems/material3",
          applied_as:["desktop_tablet_phone_adaptive_layouts","large_screen_navigation_ergonomics"]
        },
        {
          id:"wcag_2_2_reflow_target_size",
          source:"W3C WCAG 2.2 reflow and pointer target guidance",
          url:"https://www.w3.org/TR/WCAG22/",
          applied_as:["320_css_px_reflow","minimum_target_size_or_spaced_exception","no_horizontal_overflow"]
        },
        {
          id:"wcag_2_2_mobile_app_guidance",
          source:"W3C guidance on applying WCAG 2.2 to mobile applications",
          url:"https://www.w3.org/TR/wcag2mobile-22/",
          applied_as:["native_mobile_accessibility_baseline","mobile_touch_target_review","mobile_reflow_and_orientation_review"]
        }
      ],
      source_reports:{
        control_ui_browser_smoke:$control_report_path,
        native_fixture_visual_smoke:$native_report_path,
        selected_row_manifest:$selected_row_manifest_path,
        native_window_route_smoke:$route_report_path,
        native_window_secondary_smoke:$secondary_report_path,
        native_window_secondary_mobile_smoke:$secondary_mobile_report_path,
        screenshot_manifest:$screenshot_manifest_path,
        future_plan_refresh:$future_plan_path,
        operator_briefing_refresh:$operator_refresh_path,
        evidence_archive:$evidence_archive_path
      },
	      source_report_sha256:{
        control_ui_browser_smoke:$control_sha,
        native_fixture_visual_smoke:$native_sha,
        selected_row_manifest:$selected_row_manifest_sha,
        native_window_route_smoke:$route_sha,
        native_window_secondary_smoke:$secondary_sha,
        native_window_secondary_mobile_smoke:$secondary_mobile_sha,
        screenshot_manifest:$screenshot_manifest_sha,
        future_plan_refresh:$future_plan_sha,
        operator_briefing_refresh:$operator_refresh_sha,
        evidence_archive:$evidence_archive_sha
	      },
	      true_window_evidence_mode:(
	        if hard_true_window_evidence_ready then "full_hard_true_window"
	        elif no_window_evidence_accepted then "no_window_fixture"
	        else "incomplete_true_window"
	        end
	      ),
	      hard_true_window_evidence_ready:hard_true_window_evidence_ready,
	      no_window_evidence_accepted:no_window_evidence_accepted,
	      referee_matrix:{
        control_ui:{
          ready:control_ready,
          top_design_referee_ready:$control.control_ui_top_design_referee_ready,
          reflow_320_ready:$control.control_ui_320_reflow_ready,
          preferred_touch_targets_ready:$control.control_ui_preferred_touch_targets_ready,
          glass_action_contract_ready:$control.control_ui_glass_action_contract_ready,
          harsh_2026_ready:$control.control_ui_harsh_2026_ready,
          rail_action_icon_ready:$control.control_ui_rail_action_icon_ready,
          icon_buttons_ready:$control.control_ui_icon_buttons_ready,
          menu_triggers_ready:$control.control_ui_menu_triggers_ready,
	          folder_chip_touch_ready:$control.control_ui_folder_chip_touch_ready,
	          row_menu_touch_ready:$control.control_ui_row_menu_touch_ready,
	          row_menu_all_rows_ready:$control.control_ui_row_menu_all_rows_ready,
	          row_menu_light_glass_ready:$control.control_ui_row_menu_light_glass_ready,
	          command_palette_ready:$control.control_ui_command_palette_ready,
	          command_palette_surface_light_glass_ready:$control.control_ui_command_palette_surface_light_glass_ready,
	          command_palette_trigger_light_glass_ready:$control.control_ui_command_palette_trigger_light_glass_ready,
	          command_palette_close_light_glass_ready:$control.control_ui_command_palette_close_light_glass_ready,
	          command_palette_input_light_glass_ready:$control.control_ui_command_palette_input_light_glass_ready,
	          command_palette_item_light_glass_ready:$control.control_ui_command_palette_item_light_glass_ready,
	          form_control_title_touch_ready:$control.control_ui_form_control_title_touch_ready,
	          chat_row_option_semantic_touch_ready:$control.control_ui_chat_row_option_semantic_touch_ready,
	          thread_tools_menu_ready:$control.control_ui_thread_tools_menu_ready,
	          composer_tools_menu_ready:$control.control_ui_composer_tools_menu_ready,
	          composer_popover_ready:$control.control_ui_composer_popover_ready,
	          composer_popover_search_light_glass_ready:$control.control_ui_composer_popover_search_light_glass_ready,
	          rail_search_light_glass_ready:$control.control_ui_rail_search_light_glass_ready,
	          micro_surface_light_glass_ready:$control.control_ui_micro_surface_light_glass_ready,
	          message_routing_badge_light_glass_ready:$control.control_ui_message_routing_badge_light_glass_ready,
	          thread_intro_badge_light_glass_ready:$control.control_ui_thread_intro_badge_light_glass_ready,
	          status_trust_strip_light_glass_ready:$control.control_ui_status_trust_strip_light_glass_ready,
	          icon_button_title_match_ready:$control.control_ui_icon_button_title_match_ready,
          menu_trigger_title_match_ready:$control.control_ui_menu_trigger_title_match_ready,
          menu_item_icons_ready:$control.control_ui_menu_item_icons_ready,
          menu_surfaces_ready:$control.control_ui_menu_surfaces_ready,
          menu_surface_viewport_guard_ready:$control.control_ui_menu_surface_viewport_guard_ready,
	          navigation_icons_ready:$control.control_ui_navigation_icons_ready,
	          scroll_edge_ready:$control.control_ui_scroll_edge_ready,
	          microcopy_word_split_guard_ready:$control.control_ui_microcopy_word_split_guard_ready,
	          logo_clip_guard_ready:$control.control_ui_logo_clip_guard_ready,
	          active_chat_readability_ready:$control.control_ui_active_chat_readability_ready,
	          placeholder_readability_ready:$control.control_ui_placeholder_readability_ready,
	          small_control_readability_ready:$control.control_ui_small_control_readability_ready,
	          visible_text_integrity_ready:$control.control_ui_visible_text_integrity_ready,
	          visual_density_qa_ready:$control.control_ui_visual_density_qa_ready,
          horizontal_overflow_free:$control.control_ui_horizontal_overflow_free,
          browser_error_page_absent:$control.control_ui_browser_error_page_absent,
          viewport_count:$control.density_qa.viewport_count,
          phone320_ready:$control.density_qa.phone320_ready,
          persisted_phone320_screenshot_ready:control_phone320_screenshot_ready,
          persisted_phone320_screenshot:($control.screenshots // [] | map(select(.name == "phone320")) | first)
        },
        control_level:{
          ready:control_level_referee_ready,
          requested_scope:"desktop_mobile_all_modules_buttons_submenus",
          control_ui_viewport_count:$control.density_qa.viewport_count,
          control_ui_primary_button_coverage_ready:control_primary_button_coverage_ready,
          control_ui_glass_action_contract_ready:$control.control_ui_glass_action_contract_ready,
          native_route_variant_count:$native.route_variant_unique_count,
          native_mobile_route_variant_count:$native.mobile_route_variant_unique_count,
          selected_row_variant_count:($selected_rows | length),
          selected_row_unique_count:$native.selected_row_unique_count,
          selected_row_coverage_ready:selected_row_coverage_ready,
          selected_row_routes:([$selected_rows[].selected_route] | unique | sort),
          selected_row_indexes:([$selected_rows[].selected_row] | unique | sort),
          selected_row_viewports:([$selected_rows[].viewport] | unique | sort),
          secondary_surface_case_count:$native.secondary_product_surfaces.case_count,
          secondary_surface_total_action_count:$native.secondary_product_surfaces.total_action_instance_count,
          secondary_surface_action_matrix_ready:$native.secondary_product_surfaces.action_matrix_ready,
          secondary_surface_action_matrix_case_count:$native.secondary_product_surfaces.action_matrix_case_count,
          secondary_surface_harsh_action_matrix_ready:$native.secondary_product_surfaces.harsh_action_matrix_ready,
          secondary_surface_harsh_action_matrix_case_count:$native.secondary_product_surfaces.harsh_action_matrix_case_count,
          secondary_surface_harsh_action_failure_count:$native.secondary_product_surfaces.harsh_action_failure_count,
          secondary_surface_icon_svg_ready:$native.secondary_product_surfaces.icon_svg_ready,
          secondary_surface_icon_text_placeholder_absent:$native.secondary_product_surfaces.icon_text_placeholder_absent,
          secondary_surface_icon_text_placeholder_failure_count:$native.secondary_product_surfaces.icon_text_placeholder_failure_count,
          secondary_surface_title_tooltip_ready:$native.secondary_product_surfaces.title_tooltip_ready,
          secondary_surface_title_tooltip_failure_count:$native.secondary_product_surfaces.title_tooltip_failure_count,
          secondary_surface_glass_surface_ready:$native.secondary_product_surfaces.glass_surface_ready,
          secondary_surface_glass_action_group_ready:$native.secondary_product_surfaces.glass_action_group_ready,
          secondary_surface_glass_action_failure_count:$native.secondary_product_surfaces.glass_action_failure_count,
          secondary_surface_expected_action_matrix:$native.secondary_product_surfaces.expected_action_matrix,
          secondary_surface_action_matrix:($native.secondary_product_surfaces.results | map({
            surface,
            viewport,
            expected_action_ids,
            action_ids,
            action_labels,
            action_count,
            expected_actions_present,
            semantic_button_ready,
            action_matrix_ready,
            harsh_action_matrix_ready,
            surface_glass_ready,
            action_group_glass_ready,
            action_details
          })),
          secondary_surface_names:([$native.secondary_product_surfaces.results[].surface] | unique | sort),
          secondary_surface_viewports:([$native.secondary_product_surfaces.results[] | "\(.viewport.width)x\(.viewport.height)"] | unique | sort),
          secondary_surface_coverage_ready:secondary_surface_coverage_ready,
          true_window_submenu_coverage_ready:true_window_submenu_coverage_ready,
          desktop_secondary_surface_count:($secondary.surface_count // 0),
          mobile_secondary_surface_count:($secondary_mobile.surface_count // 0),
          mobile_secondary_content_visible_count:($secondary_mobile.mobile_secondary_content_visible_count // 0),
          clipping_failure_count:(
            $native.secondary_product_surfaces.text_clipping_failure_count
            + $native.secondary_product_surfaces.content_edge_failure_count
            + $native.mobile_safe_area_keyboard.content_clipping_failure_count
          )
        },
        native_fixture:{
          ready:native_fixture_ready,
          top_design_referee_ready:$native.native_top_design_referee_ready,
          reflow_320_ready:$native.native_320_reflow_ready,
          mobile_touch_target_preferred_ready:$native.native_mobile_touch_target_preferred_ready,
          tempered_glass_visual_contract_ready:$native.native_tempered_glass_visual_contract_ready,
          min_contrast_ratio:$native.tempered_glass_visual_contract.min_contrast_ratio,
          secondary_surface_case_count:$native.secondary_product_surfaces.case_count,
          secondary_surface_harsh_action_matrix_ready:$native.secondary_product_surfaces.harsh_action_matrix_ready,
          secondary_surface_harsh_action_failure_count:$native.secondary_product_surfaces.harsh_action_failure_count,
          secondary_surface_icon_svg_ready:$native.secondary_product_surfaces.icon_svg_ready,
          secondary_surface_icon_text_placeholder_absent:$native.secondary_product_surfaces.icon_text_placeholder_absent,
          secondary_surface_icon_text_placeholder_failure_count:$native.secondary_product_surfaces.icon_text_placeholder_failure_count,
          secondary_surface_title_tooltip_ready:$native.secondary_product_surfaces.title_tooltip_ready,
          secondary_surface_title_tooltip_failure_count:$native.secondary_product_surfaces.title_tooltip_failure_count,
          secondary_surface_glass_action_group_ready:$native.secondary_product_surfaces.glass_action_group_ready,
          secondary_surface_glass_action_failure_count:$native.secondary_product_surfaces.glass_action_failure_count,
          secondary_text_clipping_failure_count:$native.secondary_product_surfaces.text_clipping_failure_count,
          secondary_content_edge_failure_count:$native.secondary_product_surfaces.content_edge_failure_count,
          mobile_content_clipping_failure_count:$native.mobile_safe_area_keyboard.content_clipping_failure_count
        },
        tempered_glass_2026:{
          ready:tempered_glass_2026_ready,
          aesthetic_standard:"2026_tempered_glass_liquid_glass",
          viewport_count:$native.tempered_glass_visual_contract.viewport_count,
          desktop_ready:$native.tempered_glass_visual_contract.desktop_ready,
          mobile_ready:$native.tempered_glass_visual_contract.mobile_ready,
          phone_ready:$native.tempered_glass_visual_contract.phone_ready,
          phone320_ready:$native.tempered_glass_visual_contract.phone320_ready,
          preferred_touch_target_ready:$native.tempered_glass_visual_contract.preferred_touch_target_ready,
          body_light_ready:$native.tempered_glass_visual_contract.body_light_ready,
          translucent_panels_ready:$native.tempered_glass_visual_contract.translucent_panels_ready,
          glass_hairlines_ready:$native.tempered_glass_visual_contract.glass_hairlines_ready,
          backdrop_blur_ready:$native.tempered_glass_visual_contract.backdrop_blur_ready,
          light_accent_ready:$native.tempered_glass_visual_contract.light_accent_ready,
          horizontal_overflow_free:$native.tempered_glass_visual_contract.horizontal_overflow_free,
          light_surface_failure_count:$native.tempered_glass_visual_contract.light_surface_failure_count,
          readability_contrast_clip_ready:$native.tempered_glass_visual_contract.readability_contrast_clip_ready,
          readability_failure_count:$native.tempered_glass_visual_contract.readability_failure_count,
          min_contrast_ratio:$native.tempered_glass_visual_contract.min_contrast_ratio,
          header_icon_affordance_ready:$native.tempered_glass_visual_contract.header_icon_affordance_ready,
          header_text_action_failure_count:($native.tempered_glass_visual_contract.header_text_action_failures | length),
          product_first_visible_copy_ready:$native.tempered_glass_visual_contract.product_first_visible_copy_ready,
          visible_audit_chrome_absent:$native.tempered_glass_visual_contract.visible_audit_chrome_absent,
          visible_audit_failure_count:$native.tempered_glass_visual_contract.visible_audit_failure_count,
          control_primary_button_coverage_ready:control_primary_button_coverage_ready,
          control_glass_action_contract_ready:$control.control_ui_glass_action_contract_ready,
          control_harsh_2026_ready:$control.control_ui_harsh_2026_ready,
          control_rail_action_icon_ready:$control.control_ui_rail_action_icon_ready,
	          control_folder_chip_touch_ready:$control.control_ui_folder_chip_touch_ready,
	          control_row_menu_touch_ready:$control.control_ui_row_menu_touch_ready,
	          control_row_menu_all_rows_ready:$control.control_ui_row_menu_all_rows_ready,
	          row_menu_touch_ready:$control.control_ui_row_menu_touch_ready,
	          row_menu_all_rows_ready:$control.control_ui_row_menu_all_rows_ready,
	          control_row_menu_light_glass_ready:$control.control_ui_row_menu_light_glass_ready,
	          command_palette_ready:$control.control_ui_command_palette_ready,
	          command_palette_surface_light_glass_ready:$control.control_ui_command_palette_surface_light_glass_ready,
	          command_palette_trigger_light_glass_ready:$control.control_ui_command_palette_trigger_light_glass_ready,
	          command_palette_close_light_glass_ready:$control.control_ui_command_palette_close_light_glass_ready,
	          command_palette_input_light_glass_ready:$control.control_ui_command_palette_input_light_glass_ready,
	          command_palette_item_light_glass_ready:$control.control_ui_command_palette_item_light_glass_ready,
	          row_menu_light_glass_ready:$control.control_ui_row_menu_light_glass_ready,
	          command_palette_ready:$control.control_ui_command_palette_ready,
	          command_palette_surface_light_glass_ready:$control.control_ui_command_palette_surface_light_glass_ready,
	          command_palette_trigger_light_glass_ready:$control.control_ui_command_palette_trigger_light_glass_ready,
	          command_palette_close_light_glass_ready:$control.control_ui_command_palette_close_light_glass_ready,
	          control_command_palette_ready:$control.control_ui_command_palette_ready,
	          control_command_palette_surface_light_glass_ready:$control.control_ui_command_palette_surface_light_glass_ready,
	          control_command_palette_trigger_light_glass_ready:$control.control_ui_command_palette_trigger_light_glass_ready,
	          command_palette_trigger_light_glass_ready:$control.control_ui_command_palette_trigger_light_glass_ready,
	          command_palette_surface_light_glass_ready:$control.control_ui_command_palette_surface_light_glass_ready,
	          command_palette_trigger_light_glass_ready:$control.control_ui_command_palette_trigger_light_glass_ready,
	          control_command_palette_close_light_glass_ready:$control.control_ui_command_palette_close_light_glass_ready,
	          command_palette_close_light_glass_ready:$control.control_ui_command_palette_close_light_glass_ready,
	          control_command_palette_input_light_glass_ready:$control.control_ui_command_palette_input_light_glass_ready,
	          control_command_palette_item_light_glass_ready:$control.control_ui_command_palette_item_light_glass_ready,
	          command_palette_item_light_glass_ready:$control.control_ui_command_palette_item_light_glass_ready,
	          command_palette_ready:$control.control_ui_command_palette_ready,
	          command_palette_surface_light_glass_ready:$control.control_ui_command_palette_surface_light_glass_ready,
	          command_palette_trigger_light_glass_ready:$control.control_ui_command_palette_trigger_light_glass_ready,
	          command_palette_close_light_glass_ready:$control.control_ui_command_palette_close_light_glass_ready,
	          command_palette_input_light_glass_ready:$control.control_ui_command_palette_input_light_glass_ready,
	          command_palette_item_light_glass_ready:$control.control_ui_command_palette_item_light_glass_ready,
	          control_form_control_title_touch_ready:$control.control_ui_form_control_title_touch_ready,
	          control_chat_row_option_semantic_touch_ready:$control.control_ui_chat_row_option_semantic_touch_ready,
	          control_thread_tools_menu_ready:$control.control_ui_thread_tools_menu_ready,
	          control_composer_tools_menu_ready:$control.control_ui_composer_tools_menu_ready,
	          control_composer_popover_ready:$control.control_ui_composer_popover_ready,
	          control_composer_popover_search_light_glass_ready:$control.control_ui_composer_popover_search_light_glass_ready,
	          control_rail_search_light_glass_ready:$control.control_ui_rail_search_light_glass_ready,
	          rail_search_light_glass_ready:$control.control_ui_rail_search_light_glass_ready,
	          composer_popover_search_light_glass_ready:$control.control_ui_composer_popover_search_light_glass_ready,
	          rail_search_light_glass_ready:$control.control_ui_rail_search_light_glass_ready,
	          composer_popover_ready:$control.control_ui_composer_popover_ready,
	          composer_popover_search_light_glass_ready:$control.control_ui_composer_popover_search_light_glass_ready,
	          rail_search_light_glass_ready:$control.control_ui_rail_search_light_glass_ready,
	          control_micro_surface_light_glass_ready:$control.control_ui_micro_surface_light_glass_ready,
	          micro_surface_light_glass_ready:$control.control_ui_micro_surface_light_glass_ready,
	          control_message_routing_badge_light_glass_ready:$control.control_ui_message_routing_badge_light_glass_ready,
	          control_thread_intro_badge_light_glass_ready:$control.control_ui_thread_intro_badge_light_glass_ready,
	          control_status_trust_strip_light_glass_ready:$control.control_ui_status_trust_strip_light_glass_ready,
	          status_trust_strip_light_glass_ready:$control.control_ui_status_trust_strip_light_glass_ready,
	          thread_intro_badge_light_glass_ready:$control.control_ui_thread_intro_badge_light_glass_ready,
	          status_trust_strip_light_glass_ready:$control.control_ui_status_trust_strip_light_glass_ready,
	          message_routing_badge_light_glass_ready:$control.control_ui_message_routing_badge_light_glass_ready,
	          thread_intro_badge_light_glass_ready:$control.control_ui_thread_intro_badge_light_glass_ready,
	          status_trust_strip_light_glass_ready:$control.control_ui_status_trust_strip_light_glass_ready,
	          control_icon_button_title_match_ready:$control.control_ui_icon_button_title_match_ready,
          control_menu_trigger_title_match_ready:$control.control_ui_menu_trigger_title_match_ready,
          control_menu_item_icons_ready:$control.control_ui_menu_item_icons_ready,
          control_menu_surfaces_ready:$control.control_ui_menu_surfaces_ready,
	          control_scroll_edge_ready:$control.control_ui_scroll_edge_ready,
	          control_microcopy_word_split_guard_ready:$control.control_ui_microcopy_word_split_guard_ready,
	          control_logo_clip_guard_ready:$control.control_ui_logo_clip_guard_ready,
	          control_active_chat_readability_ready:$control.control_ui_active_chat_readability_ready,
	          control_placeholder_readability_ready:$control.control_ui_placeholder_readability_ready,
	          control_small_control_readability_ready:$control.control_ui_small_control_readability_ready,
	          control_visible_text_integrity_ready:$control.control_ui_visible_text_integrity_ready,
	          secondary_surface_coverage_ready:secondary_surface_coverage_ready,
          secondary_surface_action_matrix_ready:secondary_action_matrix_ready,
          secondary_surface_harsh_action_matrix_ready:$native.secondary_product_surfaces.harsh_action_matrix_ready,
          secondary_surface_harsh_action_failure_count:$native.secondary_product_surfaces.harsh_action_failure_count,
          native_secondary_title_tooltip_ready:$native.secondary_product_surfaces.title_tooltip_ready,
          native_secondary_title_tooltip_failure_count:$native.secondary_product_surfaces.title_tooltip_failure_count,
          secondary_surface_glass_action_group_ready:$native.secondary_product_surfaces.glass_action_group_ready,
          clipping_failure_count:(
            $native.tempered_glass_visual_contract.light_surface_failure_count
            + $native.tempered_glass_visual_contract.readability_failure_count
            + $native.tempered_glass_visual_contract.visible_audit_failure_count
            + $native.secondary_product_surfaces.text_clipping_failure_count
            + $native.secondary_product_surfaces.content_edge_failure_count
          )
        },
	        true_window_routes:{
	          ready:true_window_route_ready,
	          hard_ready:true_window_route_hard_ready,
	          no_window_accepted:true_window_route_no_window_accepted,
	          enabled:($route.enabled // false),
	          status:($route.status // "unknown"),
	          route_count:($route.route_count // 0),
	          screenshot_count:($route.screenshot_count // 0),
	          unique_count:($route.route_screenshot_unique_count // 0),
	          top_design_referee_ready:($route.route_top_design_referee_ready // false),
	          content_probe_ready:($route.route_content_probe_ready // false),
	          blocked_allowed:($route.blocked_allowed // false)
	        },
	        true_window_secondary_desktop:{
	          ready:true_window_secondary_ready,
	          hard_ready:true_window_secondary_hard_ready,
	          no_window_accepted:true_window_secondary_no_window_accepted,
	          enabled:($secondary.enabled // false),
	          status:($secondary.status // "unknown"),
	          surface_count:($secondary.surface_count // 0),
	          screenshot_count:($secondary.screenshot_count // 0),
	          unique_count:($secondary.surface_screenshot_unique_count // 0),
	          blocked_allowed:($secondary.blocked_allowed // false)
	        },
	        true_window_secondary_mobile:{
	          ready:true_window_secondary_mobile_ready,
	          hard_ready:true_window_secondary_mobile_hard_ready,
	          no_window_accepted:true_window_secondary_mobile_no_window_accepted,
	          enabled:($secondary_mobile.enabled // false),
	          status:($secondary_mobile.status // "unknown"),
	          surface_count:($secondary_mobile.surface_count // 0),
	          screenshot_count:($secondary_mobile.screenshot_count // 0),
	          unique_count:($secondary_mobile.surface_screenshot_unique_count // 0),
	          content_probe_ready:($secondary_mobile.mobile_secondary_content_probe_ready // false),
	          content_visible_count:($secondary_mobile.mobile_secondary_content_visible_count // 0),
	          blocked_allowed:($secondary_mobile.blocked_allowed // false)
	        }
	      },
      current_standards_referee:{
        standards_version:"2026-06",
        ready:$ready,
        apple_26_content_first_material_layer_ready:(
          control_ready
          and native_fixture_ready
          and $control.control_ui_engineering_copy_hidden == true
          and $native.native_visible_audit_chrome_absent == true
          and (
            true_window_route_hard_ready
            or true_window_route_no_window_accepted
          )
        ),
        apple_26_cross_device_consistency_ready:(
          screenshot_manifest_ready
          and $manifest.screenshot_count.control_ui == 4
          and $manifest.screenshot_count.native >= 40
          and (
            hard_true_window_evidence_ready
            or no_window_evidence_accepted
          )
        ),
        material_3_expressive_scannable_hierarchy_ready:(
          $control.control_ui_visual_density_qa_ready == true
          and $control.control_ui_glass_action_contract_ready == true
          and $native.secondary_product_surfaces.case_count == 15
          and $native.secondary_product_surfaces.glass_action_group_ready == true
          and $native.secondary_product_surfaces.visible_audit_failure_count == 0
          and control_level_referee_ready
          and (
            true_window_secondary_hard_ready
            or true_window_secondary_no_window_accepted
          )
        ),
        apple_2026_menu_iconography_ready:(
          $control.control_ui_menu_triggers_ready == true
          and $control.control_ui_menu_trigger_title_match_ready == true
          and $control.control_ui_menu_item_icons_ready == true
	          and $control.control_ui_menu_surfaces_ready == true
	          and $control.control_ui_menu_surface_viewport_guard_ready == true
	          and $control.control_ui_row_menu_touch_ready == true
		          and $control.control_ui_row_menu_all_rows_ready == true
		          and $control.control_ui_row_menu_light_glass_ready == true
	      and $control.control_ui_command_palette_ready == true
	      and $control.control_ui_command_palette_surface_light_glass_ready == true
      and $control.control_ui_command_palette_trigger_light_glass_ready == true
      and $control.control_ui_command_palette_close_light_glass_ready == true
		          and $control.control_ui_chat_row_option_semantic_touch_ready == true
		          and $control.control_ui_thread_tools_menu_ready == true
		          and $control.control_ui_composer_tools_menu_ready == true
	      and $control.control_ui_composer_popover_ready == true
	      and $control.control_ui_composer_popover_search_light_glass_ready == true
	      and $control.control_ui_rail_search_light_glass_ready == true
		          and $native.secondary_product_surfaces.harsh_action_matrix_ready == true
          and $native.secondary_product_surfaces.harsh_action_failure_count == 0
        ),
        apple_2026_sidebar_scroll_search_ready:(
          $control.control_ui_navigation_icons_ready == true
          and $control.control_ui_scroll_edge_ready == true
          and $control.control_ui_320_reflow_ready == true
          and $native.native_telegram_rail_tabs_density_ready == true
          and $native.native_telegram_header_icon_affordance_ready == true
        ),
        harsh_control_microinteractions_ready:(
          $control.control_ui_harsh_2026_ready == true
          and $control.control_ui_rail_action_icon_ready == true
          and $control.control_ui_icon_buttons_ready == true
          and $control.control_ui_menu_triggers_ready == true
		          and $control.control_ui_icon_button_title_match_ready == true
		          and $control.control_ui_menu_trigger_title_match_ready == true
		          and $control.control_ui_row_menu_touch_ready == true
			          and $control.control_ui_row_menu_all_rows_ready == true
			          and $control.control_ui_row_menu_light_glass_ready == true
		      and $control.control_ui_command_palette_ready == true
	      and $control.control_ui_command_palette_surface_light_glass_ready == true
      and $control.control_ui_command_palette_trigger_light_glass_ready == true
      and $control.control_ui_command_palette_close_light_glass_ready == true
			          and $control.control_ui_chat_row_option_semantic_touch_ready == true
			          and $control.control_ui_thread_tools_menu_ready == true
			          and $control.control_ui_composer_tools_menu_ready == true
	      and $control.control_ui_composer_popover_ready == true
	      and $control.control_ui_composer_popover_search_light_glass_ready == true
	      and $control.control_ui_rail_search_light_glass_ready == true
			          and $control.control_ui_menu_item_icons_ready == true
	          and $control.control_ui_menu_surfaces_ready == true
	          and $control.control_ui_menu_surface_viewport_guard_ready == true
	          and $control.control_ui_microcopy_word_split_guard_ready == true
	          and $control.control_ui_active_chat_readability_ready == true
	          and $control.control_ui_placeholder_readability_ready == true
	          and $control.control_ui_small_control_readability_ready == true
	          and $control.control_ui_visible_text_integrity_ready == true
	          and $native.native_secondary_harsh_action_matrix_ready == true
        ),
        tempered_glass_2026_ready:tempered_glass_2026_ready,
        wcag22_reflow_touch_contrast_ready:(
          $control.control_ui_320_reflow_ready == true
          and $control.control_ui_preferred_touch_targets_ready == true
          and $control.control_ui_horizontal_overflow_free == true
          and $native.native_320_reflow_ready == true
          and $native.native_mobile_touch_target_preferred_ready == true
          and $native.tempered_glass_visual_contract.min_contrast_ratio >= 4.5
          and $native.mobile_safe_area_keyboard.horizontal_overflow_free == true
        ),
        repeated_use_product_density_ready:(
          $control.density_qa.viewport_count == 4
          and $control.density_qa.failures == []
          and $control.density_qa.control_glass_action_ready == true
          and $native.secondary_product_surfaces.glass_action_failure_count == 0
          and $native.secondary_product_surfaces.text_clipping_failure_count == 0
          and $native.secondary_product_surfaces.content_edge_failure_count == 0
        ),
        no_marketing_or_demo_chrome_in_primary_flow_ready:(
          $control.control_ui_product_first_ready == true
          and $control.control_ui_dashboard_cards_hidden == true
          and $native.native_product_first_visible_copy_ready == true
          and $native.native_visible_audit_failure_count == 0
        )
      },
	      screenshot_manifest:{
	        ready:screenshot_manifest_ready,
	        base_ready:screenshot_manifest_base_ready,
	        hard_ready:screenshot_manifest_hard_ready,
	        no_window_ready:screenshot_manifest_no_window_ready,
	        counts:$manifest.screenshot_count,
	        manifest_sha256:$screenshot_manifest_sha
	      },
      current_referee_alignment:{
        current_minimum_gate_id:"r62_minimum_ui_demo_gate",
        current_plan_ids:[
          "r62_minimum_ui_demo_gate",
          "backend_real_receipt_return",
          "ui_refresh_after_real_receipt",
          "release_artifact_roundtrip_and_signed_artifact_gate"
        ],
        release_artifact_roundtrip_required:true,
        release_artifact_roundtrip_present_branch_required:true,
        signed_notarized_stapled_artifact_required_for_release:true,
        root_report_replay_required_count_after_roundtrip:41,
        blocker_closure_critical_blocker_count_expected:expected_dispatch_blocker_count,
        backend_delivery_audit_critical_blocker_count_expected:expected_dispatch_blocker_count,
        backend_dispatch_delivery_receipt_required:true,
        real_backend_receipt_required:true
      },
      machine_requirements:[
        "control_ui_top_design_referee_ready",
        "control_ui_320_reflow_ready",
        "control_ui_preferred_touch_targets_ready",
        "control_ui_persisted_phone320_screenshot_ready",
        "control_level_referee_ready",
        "control_ui_primary_button_coverage_ready",
        "control_ui_glass_action_contract_ready",
        "control_ui_harsh_2026_ready",
        "control_ui_rail_action_icon_ready",
        "control_ui_icon_buttons_ready",
        "control_ui_menu_triggers_ready",
        "control_ui_folder_chip_touch_ready",
        "control_ui_row_menu_touch_ready",
        "control_ui_row_menu_all_rows_ready",
        "control_ui_form_control_title_touch_ready",
	        "control_ui_chat_row_option_semantic_touch_ready",
	        "control_ui_thread_tools_menu_ready",
	        "control_ui_composer_tools_menu_ready",
	        "control_ui_composer_popover_ready",
	        "control_ui_composer_popover_search_light_glass_ready",
	        "control_ui_rail_search_light_glass_ready",
	        "control_ui_command_palette_input_light_glass_ready",
	        "control_ui_command_palette_trigger_light_glass_ready",
        "control_ui_command_palette_item_light_glass_ready",
	        "control_ui_micro_surface_light_glass_ready",
	        "control_ui_message_routing_badge_light_glass_ready",
	        "control_ui_thread_intro_badge_light_glass_ready",
	        "control_ui_status_trust_strip_light_glass_ready",
	        "control_ui_menu_item_icons_ready",
        "control_ui_menu_surfaces_ready",
        "control_ui_navigation_icons_ready",
	        "control_ui_scroll_edge_ready",
	        "control_ui_microcopy_word_split_guard_ready",
	        "control_ui_logo_clip_guard_ready",
	        "control_ui_active_chat_readability_ready",
	        "control_ui_placeholder_readability_ready",
	        "control_ui_small_control_readability_ready",
	        "control_ui_visible_text_integrity_ready",
	        "native_selected_row_variant_coverage_ready",
        "native_secondary_surface_action_coverage_ready",
        "native_secondary_surface_exact_action_matrix_ready",
        "native_secondary_surface_harsh_action_matrix_ready",
        "native_secondary_surface_action_icons_and_roles_ready",
        "native_secondary_surface_semantic_button_ready",
        "native_secondary_surface_glass_action_group_ready",
        "native_true_window_submenu_coverage_ready_when_enabled",
        "native_top_design_referee_ready",
        "tempered_glass_2026_ready",
        "tempered_glass_translucent_panels_ready",
        "tempered_glass_backdrop_blur_ready",
        "tempered_glass_hairlines_ready",
        "tempered_glass_min_contrast_ratio_gte_4_5",
        "native_320_reflow_ready",
        "native_mobile_touch_target_preferred_ready",
        "apple_26_content_first_material_layer_ready",
        "apple_26_cross_device_consistency_ready",
        "material_3_expressive_scannable_hierarchy_ready",
        "wcag22_reflow_touch_contrast_ready",
        "repeated_use_product_density_ready",
        "no_marketing_or_demo_chrome_in_primary_flow_ready",
        "native_secondary_surface_clipping_zero",
	        "true_window_route_top_design_referee_ready_when_enabled",
	        "true_window_route_content_probe_ready_when_enabled",
	        "true_window_secondary_mobile_content_probe_ready_when_enabled",
	        "true_window_secondary_mobile_width_lte_430_height_gte_800_when_enabled",
	        "no_window_fixture_mode_accepted_when_true_window_reports_are_not_requested",
	        "blocked_allowed_false_for_true_window_design_claims"
	      ],
      human_referee_verdict:{
        status:(if $ready then "pass_with_non_design_blockers" else "failed" end),
        design_standard_status:(if $ready then "passes_harsh_current_2026_desktop_mobile_design_referee" else "failed" end),
        pass_risks:[
          "desktop_and_mobile_design_evidence_matches_current_gate_contract",
          "all_control_level_routes_rows_buttons_and_submenus_have_machine_coverage",
          "small_controls_have_icon_accessibility_touch_and_glass_microinteraction_coverage",
          "control_submenu_panels_are_opened_measured_and_glass_ready_at_320px",
          "submenu_actions_have_icon_and_role_matrix_coverage",
          "sidebar_menu_search_and_scroll_edge_semantics_are_hard_asserted",
          "control_composer_and_send_actions_match_glass_action_contract",
          "secondary_submenu_actions_match_exact_semantic_button_matrix",
          "secondary_submenu_buttons_match_glass_action_group_contract",
          "320_reflow_touch_target_and_persisted_control_phone320_screenshot_ready",
	          (if hard_true_window_evidence_ready then
	            "route_and_secondary_true_window_content_probes_ready"
	          else
	            "true_window_content_probes_not_requested_for_no_window_refresh"
	          end)
	        ],
        remaining_non_design_blockers:$operator_refresh.refreshed_operator_briefing.updated_critical_risks,
        screenshot_specific_concerns:[]
      },
      source_alignment:{
        control_ready:control_ready,
        control_phone320_screenshot_ready:control_phone320_screenshot_ready,
        control_level_referee_ready:control_level_referee_ready,
        control_harsh_2026_ready:$control.control_ui_harsh_2026_ready,
        control_rail_action_icon_ready:$control.control_ui_rail_action_icon_ready,
        control_icon_buttons_ready:$control.control_ui_icon_buttons_ready,
        control_menu_triggers_ready:$control.control_ui_menu_triggers_ready,
        control_folder_chip_touch_ready:$control.control_ui_folder_chip_touch_ready,
	          control_row_menu_touch_ready:$control.control_ui_row_menu_touch_ready,
	          control_row_menu_all_rows_ready:$control.control_ui_row_menu_all_rows_ready,
	          row_menu_touch_ready:$control.control_ui_row_menu_touch_ready,
	          row_menu_all_rows_ready:$control.control_ui_row_menu_all_rows_ready,
	          control_row_menu_light_glass_ready:$control.control_ui_row_menu_light_glass_ready,
	          command_palette_ready:$control.control_ui_command_palette_ready,
	          command_palette_surface_light_glass_ready:$control.control_ui_command_palette_surface_light_glass_ready,
	          command_palette_trigger_light_glass_ready:$control.control_ui_command_palette_trigger_light_glass_ready,
	          command_palette_close_light_glass_ready:$control.control_ui_command_palette_close_light_glass_ready,
	          command_palette_input_light_glass_ready:$control.control_ui_command_palette_input_light_glass_ready,
	          command_palette_item_light_glass_ready:$control.control_ui_command_palette_item_light_glass_ready,
	          row_menu_light_glass_ready:$control.control_ui_row_menu_light_glass_ready,
		          command_palette_ready:$control.control_ui_command_palette_ready,
	          command_palette_surface_light_glass_ready:$control.control_ui_command_palette_surface_light_glass_ready,
	          command_palette_trigger_light_glass_ready:$control.control_ui_command_palette_trigger_light_glass_ready,
	          command_palette_close_light_glass_ready:$control.control_ui_command_palette_close_light_glass_ready,
			          control_command_palette_ready:$control.control_ui_command_palette_ready,
	          control_command_palette_surface_light_glass_ready:$control.control_ui_command_palette_surface_light_glass_ready,
	          control_command_palette_trigger_light_glass_ready:$control.control_ui_command_palette_trigger_light_glass_ready,
	          command_palette_trigger_light_glass_ready:$control.control_ui_command_palette_trigger_light_glass_ready,
	          command_palette_surface_light_glass_ready:$control.control_ui_command_palette_surface_light_glass_ready,
	          command_palette_trigger_light_glass_ready:$control.control_ui_command_palette_trigger_light_glass_ready,
	          control_command_palette_close_light_glass_ready:$control.control_ui_command_palette_close_light_glass_ready,
	          command_palette_close_light_glass_ready:$control.control_ui_command_palette_close_light_glass_ready,
			          control_command_palette_input_light_glass_ready:$control.control_ui_command_palette_input_light_glass_ready,
	          control_command_palette_item_light_glass_ready:$control.control_ui_command_palette_item_light_glass_ready,
	          command_palette_item_light_glass_ready:$control.control_ui_command_palette_item_light_glass_ready,
			          command_palette_ready:$control.control_ui_command_palette_ready,
	          command_palette_surface_light_glass_ready:$control.control_ui_command_palette_surface_light_glass_ready,
	          command_palette_trigger_light_glass_ready:$control.control_ui_command_palette_trigger_light_glass_ready,
	          command_palette_close_light_glass_ready:$control.control_ui_command_palette_close_light_glass_ready,
			          command_palette_input_light_glass_ready:$control.control_ui_command_palette_input_light_glass_ready,
	          command_palette_item_light_glass_ready:$control.control_ui_command_palette_item_light_glass_ready,
			        control_form_control_title_touch_ready:$control.control_ui_form_control_title_touch_ready,
			        control_chat_row_option_semantic_touch_ready:$control.control_ui_chat_row_option_semantic_touch_ready,
			        control_thread_tools_menu_ready:$control.control_ui_thread_tools_menu_ready,
			        control_composer_tools_menu_ready:$control.control_ui_composer_tools_menu_ready,
	          control_composer_popover_ready:$control.control_ui_composer_popover_ready,
	          control_composer_popover_search_light_glass_ready:$control.control_ui_composer_popover_search_light_glass_ready,
	          control_rail_search_light_glass_ready:$control.control_ui_rail_search_light_glass_ready,
	          rail_search_light_glass_ready:$control.control_ui_rail_search_light_glass_ready,
	          composer_popover_search_light_glass_ready:$control.control_ui_composer_popover_search_light_glass_ready,
	          rail_search_light_glass_ready:$control.control_ui_rail_search_light_glass_ready,
	          composer_popover_ready:$control.control_ui_composer_popover_ready,
	          composer_popover_search_light_glass_ready:$control.control_ui_composer_popover_search_light_glass_ready,
	          rail_search_light_glass_ready:$control.control_ui_rail_search_light_glass_ready,
	          control_micro_surface_light_glass_ready:$control.control_ui_micro_surface_light_glass_ready,
	          micro_surface_light_glass_ready:$control.control_ui_micro_surface_light_glass_ready,
	          control_message_routing_badge_light_glass_ready:$control.control_ui_message_routing_badge_light_glass_ready,
	          control_thread_intro_badge_light_glass_ready:$control.control_ui_thread_intro_badge_light_glass_ready,
	          control_status_trust_strip_light_glass_ready:$control.control_ui_status_trust_strip_light_glass_ready,
	          status_trust_strip_light_glass_ready:$control.control_ui_status_trust_strip_light_glass_ready,
	          thread_intro_badge_light_glass_ready:$control.control_ui_thread_intro_badge_light_glass_ready,
	          status_trust_strip_light_glass_ready:$control.control_ui_status_trust_strip_light_glass_ready,
	          message_routing_badge_light_glass_ready:$control.control_ui_message_routing_badge_light_glass_ready,
	          thread_intro_badge_light_glass_ready:$control.control_ui_thread_intro_badge_light_glass_ready,
	          status_trust_strip_light_glass_ready:$control.control_ui_status_trust_strip_light_glass_ready,
		        control_icon_button_title_match_ready:$control.control_ui_icon_button_title_match_ready,
        control_menu_trigger_title_match_ready:$control.control_ui_menu_trigger_title_match_ready,
          control_menu_item_icons_ready:$control.control_ui_menu_item_icons_ready,
          control_menu_surfaces_ready:$control.control_ui_menu_surfaces_ready,
          control_menu_surface_viewport_guard_ready:$control.control_ui_menu_surface_viewport_guard_ready,
	        control_navigation_icons_ready:$control.control_ui_navigation_icons_ready,
	        control_scroll_edge_ready:$control.control_ui_scroll_edge_ready,
	        control_microcopy_word_split_guard_ready:$control.control_ui_microcopy_word_split_guard_ready,
	        control_logo_clip_guard_ready:$control.control_ui_logo_clip_guard_ready,
	        control_active_chat_readability_ready:$control.control_ui_active_chat_readability_ready,
	        control_placeholder_readability_ready:$control.control_ui_placeholder_readability_ready,
	        control_small_control_readability_ready:$control.control_ui_small_control_readability_ready,
	        control_visible_text_integrity_ready:$control.control_ui_visible_text_integrity_ready,
	        tempered_glass_2026_ready:tempered_glass_2026_ready,
        selected_row_manifest_sha256:$selected_row_manifest_sha,
        native_fixture_ready:native_fixture_ready,
        native_secondary_harsh_action_matrix_ready:$native.native_secondary_harsh_action_matrix_ready,
        native_secondary_title_tooltip_ready:$native.secondary_product_surfaces.title_tooltip_ready,
        native_secondary_title_tooltip_failure_count:$native.secondary_product_surfaces.title_tooltip_failure_count,
	        true_window_route_ready:true_window_route_ready,
	        hard_true_window_evidence_ready:hard_true_window_evidence_ready,
	        no_window_evidence_accepted:no_window_evidence_accepted,
	        true_window_secondary_ready:true_window_secondary_ready,
	        true_window_secondary_mobile_ready:true_window_secondary_mobile_ready,
        screenshot_manifest_ready:screenshot_manifest_ready,
        future_plan_refresh_ready:$future.future_plan_refresh_gate_ready,
        operator_briefing_refresh_ready:$operator_refresh.operator_briefing_refresh_gate_ready,
        operator_briefing_refresh_critical_risk_count:$operator_refresh.updated_critical_risk_count,
        current_plan_ids:$operator_refresh.current_next_plan_ids,
        legacy_operator_plan_ids:$operator_refresh.current_next_plan_ids,
        current_roundtrip_plan_ids:[
          "r62_minimum_ui_demo_gate",
          "backend_real_receipt_return",
          "ui_refresh_after_real_receipt",
          "release_artifact_roundtrip_and_signed_artifact_gate"
        ],
        real_backend_receipt_present:$operator_refresh.current_state.real_backend_receipt_present,
        evidence_archive_ready:$archive.evidence_archive_gate_ready
      },
      current_state:{
        root_report_replay_required_count_after_top_design_refresh:35,
        downstream_minimum_gate_expected:"r62_minimum_ui_demo_gate",
        downstream_root_report_replay_required_count_after_release_artifact_roundtrip:41,
        downstream_current_plan_ids:[
          "r62_minimum_ui_demo_gate",
          "backend_real_receipt_return",
          "ui_refresh_after_real_receipt",
          "release_artifact_roundtrip_and_signed_artifact_gate"
        ],
        downstream_blocker_closure_critical_blocker_count_expected:expected_dispatch_blocker_count,
        downstream_backend_delivery_audit_critical_blocker_count_expected:expected_dispatch_blocker_count,
        control_ui_persisted_phone320_screenshot_ready:control_phone320_screenshot_ready,
        screenshot_manifest_total_after_phone320:$manifest.screenshot_count.total
      },
      answer_guardrail:{
        forbidden_claims:(
          ["live_product_ready","public_distribution_ready","release_ready"]
          + (if $operator_refresh.claim_boundary.real_backend_receipt_claim_ready then [] else ["real_backend_receipt_ready"] end)
        ),
	        allowed_claims:[
	          "local_design_referee_refresh_ready",
	          "desktop_mobile_ui_design_evidence_ready",
	          (if hard_true_window_evidence_ready then
	            "local_fixture_and_true_window_visual_evidence_ready"
	          else
	            "local_fixture_visual_evidence_ready"
	          end)
	        ]
      },
      claim_boundary:{
        local_top_design_referee_refresh_ready:$ready,
        desktop_mobile_design_claim_ready:$ready,
        real_backend_receipt_claim_ready:$operator_refresh.claim_boundary.real_backend_receipt_claim_ready,
        backend_receipt_claim_ready:$operator_refresh.claim_boundary.backend_receipt_claim_ready,
        backend_adapter_promoted:false,
        readback_evidence_recorded:false,
        live_runtime_mutation:false,
        live_product_claim_ready:false,
        public_distribution_claim_ready:false,
        release_claim_ready:false,
        external_actions_allowed:false
      },
      side_effects:{
        local_markdown_written:true,
        external_mutation:false,
        matrix_login:false,
        gateway_call:false,
        provider_invoked:false,
        channel_delivery:false,
        signing_notarization:false,
        public_upload:false
      }
    }
  ' >"$REPORT_DRAFT"

jq -r '
	  "# Hepta UI Top-Design Referee Refresh\n\n"
	  + "- Status: `" + .status + "`\n"
	  + "- Refresh kind: `" + .refresh_kind + "`\n"
	  + "- True-window evidence mode: `" + .true_window_evidence_mode + "`\n"
	  + "- Control UI ready: `" + (.referee_matrix.control_ui.ready | tostring) + "`\n"
	  + "- Control UI harsh 2026 ready: `" + (.referee_matrix.control_ui.harsh_2026_ready | tostring) + "`\n"
	  + "- Control-level referee ready: `" + (.referee_matrix.control_level.ready | tostring) + "`\n"
	  + "- Tempered-glass 2026 ready: `" + (.referee_matrix.tempered_glass_2026.ready | tostring) + "`\n"
	  + "- Tempered-glass min contrast ratio: `" + (.referee_matrix.tempered_glass_2026.min_contrast_ratio | tostring) + "`\n"
	  + "- Selected row variants: `" + (.referee_matrix.control_level.selected_row_variant_count | tostring) + "`\n"
	  + "- Secondary surface cases/actions: `" + (.referee_matrix.control_level.secondary_surface_case_count | tostring) + "` / `" + (.referee_matrix.control_level.secondary_surface_total_action_count | tostring) + "`\n"
	  + "- Secondary surface exact action matrix ready: `" + (.referee_matrix.control_level.secondary_surface_action_matrix_ready | tostring) + "`\n"
	  + "- Secondary surface harsh action matrix ready: `" + (.referee_matrix.control_level.secondary_surface_harsh_action_matrix_ready | tostring) + "`\n"
	  + "- Native fixture ready: `" + (.referee_matrix.native_fixture.ready | tostring) + "`\n"
	  + "- True-window routes ready: `" + (.referee_matrix.true_window_routes.ready | tostring) + "`\n"
  + "- True-window desktop secondary ready: `" + (.referee_matrix.true_window_secondary_desktop.ready | tostring) + "`\n"
  + "- True-window mobile secondary ready: `" + (.referee_matrix.true_window_secondary_mobile.ready | tostring) + "`\n"
  + "- Screenshot total: `" + (.screenshot_manifest.counts.total | tostring) + "`\n"
  + "- Legacy operator plan ids: `" + (.source_alignment.legacy_operator_plan_ids | join(",")) + "`\n"
  + "- Current roundtrip plan ids: `" + (.current_referee_alignment.current_plan_ids | join(",")) + "`\n"
  + "- Current roundtrip root replay: `" + (.current_referee_alignment.root_report_replay_required_count_after_roundtrip | tostring) + "`\n"
  + "- Updated critical risk count: `" + (.source_alignment.operator_briefing_refresh_critical_risk_count | tostring) + "`\n\n"
  + "## Referee Basis\n\n"
  + (.referee_basis | map("- `" + .id + "`: " + .url) | join("\n"))
  + "\n\n## Machine Requirements\n\n"
  + (.machine_requirements | map("- `" + . + "`") | join("\n"))
  + "\n\n## Claim Boundary\n\n"
  + "- Live product claim ready: `" + (.claim_boundary.live_product_claim_ready | tostring) + "`\n"
  + "- Public distribution claim ready: `" + (.claim_boundary.public_distribution_claim_ready | tostring) + "`\n"
  + "- Release claim ready: `" + (.claim_boundary.release_claim_ready | tostring) + "`\n"
  + "- External actions allowed: `" + (.claim_boundary.external_actions_allowed | tostring) + "`\n"
' "$REPORT_DRAFT" >"$MARKDOWN_TMP"

markdown_sha="$(file_sha256 "$MARKDOWN_TMP")"
markdown_bytes="$(file_bytes "$MARKDOWN_TMP")"

jq \
  --arg refresh_markdown_sha256 "$markdown_sha" \
  --argjson refresh_markdown_bytes "$markdown_bytes" \
  '. + {
    refresh_markdown_sha256:$refresh_markdown_sha256,
    refresh_markdown_bytes:$refresh_markdown_bytes
  }' "$REPORT_DRAFT" >"$REPORT_TMP"

if [[ "${HEPTA_UI_TOP_DESIGN_REFEREE_REFRESH_DEBUG_COPY:-0}" == "1" ]]; then
  cp "$REPORT_TMP" "$REPORT_PATH.debug"
fi

jq -e '
	  .status == "ready"
	  and .top_design_referee_refresh_gate_ready == true
	  and .top_design_harsh_2026_referee_ready == true
	  and .control_ui_harsh_2026_ready == true
	  and .native_secondary_harsh_action_matrix_ready == true
	  and .refresh_kind == "local_ui_top_design_referee_2026_refresh"
	  and .refresh_version == 46
	  and .standards_version == "2026-06-24-harsh-badge-micro-surface-light-glass"
	  and .aesthetic_standard == "2026_tempered_glass_liquid_glass"
	  and .referee_matrix.control_ui.ready == true
	  and .referee_matrix.control_ui.harsh_2026_ready == true
	  and .referee_matrix.control_ui.rail_action_icon_ready == true
	  and .referee_matrix.control_ui.icon_buttons_ready == true
	  and .referee_matrix.control_ui.menu_triggers_ready == true
		  and .referee_matrix.control_ui.folder_chip_touch_ready == true
		  and .referee_matrix.control_ui.row_menu_touch_ready == true
		  and .referee_matrix.control_ui.row_menu_all_rows_ready == true
		  and .referee_matrix.control_ui.row_menu_light_glass_ready == true
		  and .referee_matrix.control_ui.command_palette_ready == true
		  and .referee_matrix.control_ui.command_palette_surface_light_glass_ready == true
		  and .referee_matrix.control_ui.command_palette_trigger_light_glass_ready == true
		  and .referee_matrix.control_ui.command_palette_close_light_glass_ready == true
		  and .referee_matrix.control_ui.command_palette_input_light_glass_ready == true
		  and .referee_matrix.control_ui.command_palette_item_light_glass_ready == true
		  and .referee_matrix.control_ui.form_control_title_touch_ready == true
		  and .referee_matrix.control_ui.chat_row_option_semantic_touch_ready == true
		  and .referee_matrix.control_ui.thread_tools_menu_ready == true
		  and .referee_matrix.control_ui.composer_tools_menu_ready == true
		  and .referee_matrix.control_ui.composer_popover_ready == true
		  and .referee_matrix.control_ui.composer_popover_search_light_glass_ready == true
		  and .referee_matrix.control_ui.micro_surface_light_glass_ready == true
		  and .referee_matrix.control_ui.message_routing_badge_light_glass_ready == true
		  and .referee_matrix.control_ui.icon_button_title_match_ready == true
	  and .referee_matrix.control_ui.menu_trigger_title_match_ready == true
	  and .referee_matrix.control_ui.menu_item_icons_ready == true
	  and .referee_matrix.control_ui.menu_surfaces_ready == true
	  and .referee_matrix.control_ui.menu_surface_viewport_guard_ready == true
		  and .referee_matrix.control_ui.navigation_icons_ready == true
		  and .referee_matrix.control_ui.scroll_edge_ready == true
	  and .referee_matrix.control_ui.microcopy_word_split_guard_ready == true
	  and .referee_matrix.control_ui.logo_clip_guard_ready == true
	  and .referee_matrix.control_ui.active_chat_readability_ready == true
	  and .referee_matrix.control_ui.placeholder_readability_ready == true
	  and .referee_matrix.control_ui.small_control_readability_ready == true
	  and .referee_matrix.control_ui.visible_text_integrity_ready == true
	  and .referee_matrix.control_level.ready == true
	  and .referee_matrix.tempered_glass_2026.ready == true
	  and .referee_matrix.tempered_glass_2026.aesthetic_standard == "2026_tempered_glass_liquid_glass"
	  and .referee_matrix.tempered_glass_2026.viewport_count == 4
	  and .referee_matrix.tempered_glass_2026.desktop_ready == true
	  and .referee_matrix.tempered_glass_2026.mobile_ready == true
	  and .referee_matrix.tempered_glass_2026.phone_ready == true
	  and .referee_matrix.tempered_glass_2026.phone320_ready == true
	  and .referee_matrix.tempered_glass_2026.translucent_panels_ready == true
	  and .referee_matrix.tempered_glass_2026.glass_hairlines_ready == true
	  and .referee_matrix.tempered_glass_2026.backdrop_blur_ready == true
	  and .referee_matrix.tempered_glass_2026.light_accent_ready == true
	  and .referee_matrix.tempered_glass_2026.horizontal_overflow_free == true
	  and .referee_matrix.tempered_glass_2026.light_surface_failure_count == 0
	  and .referee_matrix.tempered_glass_2026.readability_contrast_clip_ready == true
	  and .referee_matrix.tempered_glass_2026.readability_failure_count == 0
	  and .referee_matrix.tempered_glass_2026.min_contrast_ratio >= 4.5
	  and .referee_matrix.tempered_glass_2026.header_icon_affordance_ready == true
	  and .referee_matrix.tempered_glass_2026.header_text_action_failure_count == 0
	  and .referee_matrix.tempered_glass_2026.product_first_visible_copy_ready == true
	  and .referee_matrix.tempered_glass_2026.visible_audit_chrome_absent == true
	  and .referee_matrix.tempered_glass_2026.visible_audit_failure_count == 0
			  and .referee_matrix.tempered_glass_2026.control_primary_button_coverage_ready == true
		  and .referee_matrix.tempered_glass_2026.control_microcopy_word_split_guard_ready == true
		  and .referee_matrix.tempered_glass_2026.control_logo_clip_guard_ready == true
			  and .referee_matrix.tempered_glass_2026.control_active_chat_readability_ready == true
			  and .referee_matrix.tempered_glass_2026.control_row_menu_touch_ready == true
			  and .referee_matrix.tempered_glass_2026.control_row_menu_all_rows_ready == true
			  and .referee_matrix.tempered_glass_2026.control_row_menu_light_glass_ready == true
			  and .referee_matrix.tempered_glass_2026.control_form_control_title_touch_ready == true
			  and .referee_matrix.tempered_glass_2026.control_chat_row_option_semantic_touch_ready == true
			  and .referee_matrix.tempered_glass_2026.control_thread_tools_menu_ready == true
			  and .referee_matrix.tempered_glass_2026.control_composer_tools_menu_ready == true
			  and .referee_matrix.tempered_glass_2026.control_composer_popover_ready == true
			  and .referee_matrix.tempered_glass_2026.control_composer_popover_search_light_glass_ready == true
			  and .referee_matrix.tempered_glass_2026.control_micro_surface_light_glass_ready == true
			  and .referee_matrix.tempered_glass_2026.control_message_routing_badge_light_glass_ready == true
			  and .referee_matrix.tempered_glass_2026.control_placeholder_readability_ready == true
		  and .referee_matrix.tempered_glass_2026.control_small_control_readability_ready == true
		  and .referee_matrix.tempered_glass_2026.control_visible_text_integrity_ready == true
		  and .referee_matrix.tempered_glass_2026.secondary_surface_coverage_ready == true
	  and .referee_matrix.tempered_glass_2026.secondary_surface_action_matrix_ready == true
	  and .referee_matrix.tempered_glass_2026.secondary_surface_harsh_action_matrix_ready == true
	  and .referee_matrix.tempered_glass_2026.secondary_surface_harsh_action_failure_count == 0
	  and .referee_matrix.tempered_glass_2026.native_secondary_title_tooltip_ready == true
	  and .referee_matrix.tempered_glass_2026.native_secondary_title_tooltip_failure_count == 0
	  and .referee_matrix.tempered_glass_2026.clipping_failure_count == 0
	  and .referee_matrix.control_level.requested_scope == "desktop_mobile_all_modules_buttons_submenus"
	  and .referee_matrix.control_level.control_ui_viewport_count == 4
	  and .referee_matrix.control_level.control_ui_primary_button_coverage_ready == true
	  and .referee_matrix.control_level.control_ui_glass_action_contract_ready == true
	  and .referee_matrix.control_level.native_route_variant_count == 4
	  and .referee_matrix.control_level.native_mobile_route_variant_count == 4
	  and .referee_matrix.control_level.selected_row_variant_count == 18
	  and .referee_matrix.control_level.selected_row_unique_count == 18
	  and .referee_matrix.control_level.selected_row_coverage_ready == true
	  and .referee_matrix.control_level.selected_row_routes == ["Actions","Approvals","Inspector"]
	  and .referee_matrix.control_level.selected_row_indexes == [0,1,2]
	  and .referee_matrix.control_level.selected_row_viewports == ["1280x800","500x844"]
	  and .referee_matrix.control_level.secondary_surface_case_count == 15
	  and .referee_matrix.control_level.secondary_surface_total_action_count == 57
	  and .referee_matrix.control_level.secondary_surface_action_matrix_ready == true
	  and .referee_matrix.control_level.secondary_surface_action_matrix_case_count == 15
	  and .referee_matrix.control_level.secondary_surface_harsh_action_matrix_ready == true
	  and .referee_matrix.control_level.secondary_surface_harsh_action_matrix_case_count == 15
	  and .referee_matrix.control_level.secondary_surface_harsh_action_failure_count == 0
	  and .referee_matrix.control_level.secondary_surface_icon_svg_ready == true
	  and .referee_matrix.control_level.secondary_surface_icon_text_placeholder_absent == true
	  and .referee_matrix.control_level.secondary_surface_icon_text_placeholder_failure_count == 0
	  and .referee_matrix.control_level.secondary_surface_title_tooltip_ready == true
	  and .referee_matrix.control_level.secondary_surface_title_tooltip_failure_count == 0
	  and .referee_matrix.control_level.secondary_surface_glass_surface_ready == true
	  and .referee_matrix.control_level.secondary_surface_glass_action_group_ready == true
	  and .referee_matrix.control_level.secondary_surface_glass_action_failure_count == 0
	  and .referee_matrix.control_level.secondary_surface_expected_action_matrix == {
	    attachment:["gallery","camera","files","share"],
	    modal:["cancel","keep-reviewing","approve"],
	    search:["jump","copy","source","filter"],
	    settings:["rename","members","mute","apply-after-review"],
	    voice:["record","play","drop","send"]
	  }
	  and (.referee_matrix.control_level.secondary_surface_action_matrix | length) == 15
	  and (.referee_matrix.control_level.secondary_surface_action_matrix | all(.expected_actions_present == true and .semantic_button_ready == true and .action_matrix_ready == true and .harsh_action_matrix_ready == true and .surface_glass_ready == true and .action_group_glass_ready == true))
	  and (.referee_matrix.control_level.secondary_surface_action_matrix | all(.action_details | all(.harsh_action_ready == true and .icon_present == true and .icon_svg_ready == true and .icon_text_placeholder_absent == true and .title_tooltip_ready == true and .role_ready == true)))
	  and .referee_matrix.control_level.secondary_surface_names == ["attachment","modal","search","settings","voice"]
	  and .referee_matrix.control_level.secondary_surface_viewports == ["1280x800","320x844","390x844"]
	  and .referee_matrix.control_level.secondary_surface_coverage_ready == true
	  and .referee_matrix.control_level.true_window_submenu_coverage_ready == true
	  and .referee_matrix.control_level.clipping_failure_count == 0
  and .referee_matrix.control_ui.reflow_320_ready == true
  and .referee_matrix.control_ui.glass_action_contract_ready == true
  and .referee_matrix.control_ui.persisted_phone320_screenshot_ready == true
  and .referee_matrix.control_ui.persisted_phone320_screenshot.viewport == "320x844"
  and (.referee_matrix.control_ui.persisted_phone320_screenshot.sha256 | test("^[0-9a-f]{64}$"))
  and .referee_matrix.control_ui.persisted_phone320_screenshot.bytes >= 50000
  and .referee_matrix.control_ui.preferred_touch_targets_ready == true
  and .referee_matrix.native_fixture.ready == true
  and .referee_matrix.native_fixture.reflow_320_ready == true
	  and .referee_matrix.native_fixture.mobile_touch_target_preferred_ready == true
		  and .referee_matrix.native_fixture.secondary_surface_harsh_action_matrix_ready == true
		  and .referee_matrix.native_fixture.secondary_surface_harsh_action_failure_count == 0
		  and .referee_matrix.native_fixture.secondary_surface_icon_svg_ready == true
		  and .referee_matrix.native_fixture.secondary_surface_icon_text_placeholder_absent == true
		  and .referee_matrix.native_fixture.secondary_surface_icon_text_placeholder_failure_count == 0
		  and .referee_matrix.native_fixture.secondary_surface_title_tooltip_ready == true
		  and .referee_matrix.native_fixture.secondary_surface_title_tooltip_failure_count == 0
		  and .referee_matrix.native_fixture.secondary_surface_glass_action_group_ready == true
	  and .referee_matrix.native_fixture.secondary_surface_glass_action_failure_count == 0
	  and .referee_matrix.native_fixture.secondary_text_clipping_failure_count == 0
	  and .referee_matrix.native_fixture.secondary_content_edge_failure_count == 0
	  and (
	    (
	      .true_window_evidence_mode == "full_hard_true_window"
	      and .hard_true_window_evidence_ready == true
	      and .no_window_evidence_accepted == false
	      and .referee_matrix.true_window_routes.ready == true
	      and .referee_matrix.true_window_routes.hard_ready == true
	      and .referee_matrix.true_window_routes.no_window_accepted == false
	      and .referee_matrix.true_window_routes.top_design_referee_ready == true
	      and .referee_matrix.true_window_routes.content_probe_ready == true
	      and .referee_matrix.true_window_secondary_desktop.ready == true
	      and .referee_matrix.true_window_secondary_desktop.hard_ready == true
	      and .referee_matrix.true_window_secondary_desktop.no_window_accepted == false
	      and .referee_matrix.true_window_secondary_mobile.ready == true
	      and .referee_matrix.true_window_secondary_mobile.hard_ready == true
	      and .referee_matrix.true_window_secondary_mobile.no_window_accepted == false
	      and .referee_matrix.true_window_secondary_mobile.content_probe_ready == true
	      and .referee_matrix.true_window_secondary_mobile.content_visible_count >= 10
	      and .screenshot_manifest.hard_ready == true
	      and .screenshot_manifest.counts.native_true_window == 2
	      and .screenshot_manifest.counts.native_true_window_route == 4
	      and .screenshot_manifest.counts.native_true_window_secondary == 5
	      and .screenshot_manifest.counts.native_true_window_secondary_mobile == 5
	      and .screenshot_manifest.counts.total >= 60
	    )
	    or (
	      .true_window_evidence_mode == "no_window_fixture"
	      and .hard_true_window_evidence_ready == false
	      and .no_window_evidence_accepted == true
	      and .referee_matrix.true_window_routes.ready == true
	      and .referee_matrix.true_window_routes.hard_ready == false
	      and .referee_matrix.true_window_routes.no_window_accepted == true
	      and .referee_matrix.true_window_routes.enabled == false
	      and .referee_matrix.true_window_routes.status == "not_run"
	      and .referee_matrix.true_window_secondary_desktop.ready == true
	      and .referee_matrix.true_window_secondary_desktop.hard_ready == false
	      and .referee_matrix.true_window_secondary_desktop.no_window_accepted == true
	      and .referee_matrix.true_window_secondary_desktop.enabled == false
	      and .referee_matrix.true_window_secondary_desktop.status == "not_run"
	      and .referee_matrix.true_window_secondary_mobile.ready == true
	      and .referee_matrix.true_window_secondary_mobile.hard_ready == false
	      and .referee_matrix.true_window_secondary_mobile.no_window_accepted == true
	      and .referee_matrix.true_window_secondary_mobile.enabled == false
	      and .referee_matrix.true_window_secondary_mobile.status == "not_run"
	      and .screenshot_manifest.no_window_ready == true
	      and .screenshot_manifest.counts.native_true_window == 0
	      and .screenshot_manifest.counts.native_true_window_route == 0
	      and .screenshot_manifest.counts.native_true_window_secondary == 0
	      and .screenshot_manifest.counts.native_true_window_secondary_mobile == 0
	      and .screenshot_manifest.counts.total >= 44
	    )
	  )
	  and .screenshot_manifest.ready == true
	  and .screenshot_manifest.base_ready == true
	  and .screenshot_manifest.counts.control_ui == 4
	  and .current_standards_referee.apple_2026_menu_iconography_ready == true
	  and .current_standards_referee.apple_2026_sidebar_scroll_search_ready == true
	  and .current_standards_referee.harsh_control_microinteractions_ready == true
	  and .source_alignment.operator_briefing_refresh_ready == true
  and .source_alignment.control_harsh_2026_ready == true
  and .source_alignment.control_rail_action_icon_ready == true
  and .source_alignment.control_icon_buttons_ready == true
  and .source_alignment.control_menu_triggers_ready == true
  and .source_alignment.control_folder_chip_touch_ready == true
  and .source_alignment.control_row_menu_touch_ready == true
  and .source_alignment.control_row_menu_all_rows_ready == true
	  and .source_alignment.control_row_menu_light_glass_ready == true
	  and .source_alignment.control_command_palette_ready == true
	  and .source_alignment.control_command_palette_surface_light_glass_ready == true
	  and .source_alignment.control_command_palette_trigger_light_glass_ready == true
	  and .source_alignment.control_command_palette_close_light_glass_ready == true
	  and .source_alignment.control_command_palette_input_light_glass_ready == true
	  and .source_alignment.control_command_palette_item_light_glass_ready == true
	  and .source_alignment.control_form_control_title_touch_ready == true
	  and .source_alignment.control_chat_row_option_semantic_touch_ready == true
	  and .source_alignment.control_thread_tools_menu_ready == true
	  and .source_alignment.control_composer_tools_menu_ready == true
	  and .source_alignment.control_composer_popover_ready == true
	  and .source_alignment.control_composer_popover_search_light_glass_ready == true
	  and .source_alignment.control_micro_surface_light_glass_ready == true
	  and .source_alignment.control_message_routing_badge_light_glass_ready == true
	  and .source_alignment.control_icon_button_title_match_ready == true
  and .source_alignment.control_menu_trigger_title_match_ready == true
	  and .source_alignment.control_menu_item_icons_ready == true
	  and .source_alignment.control_menu_surfaces_ready == true
	  and .source_alignment.control_menu_surface_viewport_guard_ready == true
	  and .source_alignment.control_navigation_icons_ready == true
		  and .source_alignment.control_scroll_edge_ready == true
	  and .source_alignment.control_microcopy_word_split_guard_ready == true
	  and .source_alignment.control_logo_clip_guard_ready == true
	  and .source_alignment.control_active_chat_readability_ready == true
	  and .source_alignment.control_placeholder_readability_ready == true
	  and .source_alignment.control_small_control_readability_ready == true
	  and .source_alignment.control_visible_text_integrity_ready == true
	  and .source_alignment.native_secondary_harsh_action_matrix_ready == true
	  and .source_alignment.native_secondary_title_tooltip_ready == true
	  and .source_alignment.native_secondary_title_tooltip_failure_count == 0
  and (.source_alignment.operator_briefing_refresh_critical_risk_count >= 1 and .source_alignment.operator_briefing_refresh_critical_risk_count <= 4)
  and .source_alignment.current_plan_ids == ["r52_minimum_ui_demo_gate","backend_real_receipt_return","ui_refresh_after_real_receipt"]
  and .source_alignment.current_roundtrip_plan_ids == ["r62_minimum_ui_demo_gate","backend_real_receipt_return","ui_refresh_after_real_receipt","release_artifact_roundtrip_and_signed_artifact_gate"]
  and (.source_alignment.real_backend_receipt_present | type) == "boolean"
  and .source_alignment.control_phone320_screenshot_ready == true
  and .current_referee_alignment.current_minimum_gate_id == "r62_minimum_ui_demo_gate"
  and .current_referee_alignment.current_plan_ids == ["r62_minimum_ui_demo_gate","backend_real_receipt_return","ui_refresh_after_real_receipt","release_artifact_roundtrip_and_signed_artifact_gate"]
  and .current_referee_alignment.release_artifact_roundtrip_required == true
  and .current_referee_alignment.release_artifact_roundtrip_present_branch_required == true
  and .current_referee_alignment.signed_notarized_stapled_artifact_required_for_release == true
  and .current_referee_alignment.root_report_replay_required_count_after_roundtrip == 41
  and (.current_referee_alignment.blocker_closure_critical_blocker_count_expected == 2 or .current_referee_alignment.blocker_closure_critical_blocker_count_expected == 3)
  and (.current_referee_alignment.backend_delivery_audit_critical_blocker_count_expected == 2 or .current_referee_alignment.backend_delivery_audit_critical_blocker_count_expected == 3)
  and .current_state.root_report_replay_required_count_after_top_design_refresh == 35
  and .current_state.downstream_minimum_gate_expected == "r62_minimum_ui_demo_gate"
  and .current_state.downstream_root_report_replay_required_count_after_release_artifact_roundtrip == 41
  and .current_state.downstream_current_plan_ids == ["r62_minimum_ui_demo_gate","backend_real_receipt_return","ui_refresh_after_real_receipt","release_artifact_roundtrip_and_signed_artifact_gate"]
  and (.current_state.downstream_blocker_closure_critical_blocker_count_expected == 2 or .current_state.downstream_blocker_closure_critical_blocker_count_expected == 3)
  and (.current_state.downstream_backend_delivery_audit_critical_blocker_count_expected == 2 or .current_state.downstream_backend_delivery_audit_critical_blocker_count_expected == 3)
  and .current_state.control_ui_persisted_phone320_screenshot_ready == true
  and (.answer_guardrail.forbidden_claims | index("live_product_ready") != null)
  and (.answer_guardrail.forbidden_claims | index("public_distribution_ready") != null)
  and (.refresh_markdown_sha256 | test("^[0-9a-f]{64}$"))
  and .refresh_markdown_bytes > 0
  and .claim_boundary.local_top_design_referee_refresh_ready == true
  and .claim_boundary.desktop_mobile_design_claim_ready == true
  and (.claim_boundary.real_backend_receipt_claim_ready | type) == "boolean"
  and (.claim_boundary.backend_receipt_claim_ready | type) == "boolean"
  and .claim_boundary.backend_adapter_promoted == false
  and .claim_boundary.live_runtime_mutation == false
  and .claim_boundary.live_product_claim_ready == false
  and .claim_boundary.public_distribution_claim_ready == false
  and .claim_boundary.release_claim_ready == false
  and .claim_boundary.external_actions_allowed == false
  and .side_effects.local_markdown_written == true
  and .side_effects.external_mutation == false
' "$REPORT_TMP" >/dev/null

cp "$MARKDOWN_TMP" "$REFRESH_MARKDOWN_PATH"
cp "$REPORT_TMP" "$REPORT_PATH"

printf 'Hepta UI top-design referee refresh gate wrote %s\n' "$REPORT_PATH" >&2
