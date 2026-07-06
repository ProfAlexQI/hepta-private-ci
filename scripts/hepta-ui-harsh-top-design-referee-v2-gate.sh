#!/usr/bin/env bash
set -euo pipefail

READINESS_DIR="${HEPTA_UI_PRODUCT_READINESS_DIR:-${1:-}}"
CONTROL_REPORT_PATH="${HEPTA_CONTROL_UI_BROWSER_SMOKE_REPORT_PATH:-}"
NATIVE_REPORT_PATH="${HEPTA_NATIVE_FIXTURE_VISUAL_SMOKE_REPORT_PATH:-}"
REPORT_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V2_REPORT_PATH:-}"

if [[ -z "$READINESS_DIR" ]]; then
  echo "usage: $0 <hepta-ui-product-readiness-dir>" >&2
  exit 2
fi

if [[ -z "$CONTROL_REPORT_PATH" ]]; then
  CONTROL_REPORT_PATH="$READINESS_DIR/control-ui-browser-smoke.json"
fi
if [[ -z "$NATIVE_REPORT_PATH" ]]; then
  NATIVE_REPORT_PATH="$READINESS_DIR/native-fixture/native-fixture-visual-smoke.json"
fi
if [[ -z "$REPORT_PATH" ]]; then
  REPORT_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v2-gate.json"
fi

for path in "$CONTROL_REPORT_PATH" "$NATIVE_REPORT_PATH"; do
  if [[ ! -s "$path" ]]; then
    echo "missing harsh top-design v2 input: $path" >&2
    exit 1
  fi
  jq empty "$path" >/dev/null
done

mkdir -p "$(dirname "$REPORT_PATH")"
tmp_report="$(mktemp "${TMPDIR:-/tmp}/hepta-ui-harsh-top-design-referee-v2.XXXXXX")"
trap 'rm -f "$tmp_report"' EXIT

control_sha="$(shasum -a 256 "$CONTROL_REPORT_PATH" | awk '{print $1}')"
native_sha="$(shasum -a 256 "$NATIVE_REPORT_PATH" | awk '{print $1}')"

jq -n \
  --arg control_path "$CONTROL_REPORT_PATH" \
  --arg native_path "$NATIVE_REPORT_PATH" \
  --arg control_sha "$control_sha" \
  --arg native_sha "$native_sha" \
  --slurpfile control_file "$CONTROL_REPORT_PATH" \
  --slurpfile native_file "$NATIVE_REPORT_PATH" '
  ($control_file[0]) as $control
  | ($native_file[0]) as $native
  | def viewport_id: "\(.viewport.width)x\(.viewport.height)";
    def expected_control_viewports: ["1365x900","320x844","500x844","768x900"];
    def expected_rail_row_keys: ["operator-plane","task-queue","ui-chat-agent"];
    def expected_secondary_surfaces: ["attachment","modal","search","settings","voice"];
    def expected_secondary_viewports: ["1280x800","320x844","390x844"];
    def expected_secondary_action_matrix: {
      attachment:["gallery","camera","files","share"],
      modal:["cancel","keep-reviewing","approve"],
      search:["jump","copy","source","filter"],
      settings:["rename","members","mute","apply-after-review"],
      voice:["record","play","drop","send"]
    };
    def light_glass_panel_ready($panel; $min_h):
      $panel.exists == true
      and $panel.visible == true
      and $panel.role == "menu"
      and (($panel.aria_label // "") | length) > 0
      and ($panel.item_count // 0) >= 1
      and ($panel.width // 0) >= 180
      and ($panel.height // 0) >= $min_h
      and ($panel.border_radius // 0) >= 16
      and $panel.light_glass_ready == true
      and ($panel.effective_luminance // 0) >= 0.72
      and ($panel.effective_luminance // 1) <= 0.98
      and (($panel.backdrop_filter // "") | contains("blur("))
      and ($panel.box_shadow // "none") != "none"
      and $panel.in_viewport == true
      and $panel.top_clipped == false
      and $panel.bottom_clipped == false;
    def menu_item_ready:
      .visible == true
      and .role == "menuitem"
      and (.height // 0) >= 44
      and ((.aria_label // "") | length) > 0
      and ((.title // "") | length) > 0
      and .title_matches_aria_label == true
      and .icon_svg_present == true
      and .label_nowrap_ready == true
      and .readable == true
      and (.contrast_ratio // 0) >= 4.8;
    def control_viewport_ready:
      .status == "ready"
      and (.errors | length) == 0
      and .harsh_referee_ready == true
      and .primary_shell_light_glass_ready == true
      and .optical_clarity_light_glass_ready == true
      and .surface_clear_alpha_light_glass_ready == true
      and .substrate_caustic_field_light_glass_ready == true
      and .horizontal_overflow_free == true
      and .visible_text_integrity_ready == true
      and .placeholder_readability_ready == true
      and .small_control_readability_ready == true
      and .active_chat_readability_ready == true
      and .single_submenu_audit_ready == true
      and (.single_submenu_audit_target_count == (if .rail_visible == true then 8 else 5 end))
      and ((.single_submenu_audit_details // []) | length) == (if .rail_visible == true then 8 else 5 end)
      and ((.single_submenu_audit_details // []) | all(
        . as $audit
        | $audit.ready == true
        and $audit.unexpected_visible_count == 0
        and $audit.horizontal_overflow_free == true
        and $audit.visible_target_count == $audit.expected_visible_count
        and $audit.visible_item_count == $audit.expected_item_count
        and ($audit.surface_details | all(.in_viewport == true and .light_glass_ready == true and .effective_luminance >= 0.72 and .effective_luminance <= 0.98 and ((.backdrop_filter // "") | contains("blur(")) and .box_shadow != "none"))
        and ($audit.item_details | all(
          .height >= 44
          and (if $audit.requires_item_svg == true then .svg_icon_present == true else true end)
          and .readable == true
          and .contrast_ratio >= 4.5
          and .title_matches_aria_label == true
          and (if $audit.requires_item_nowrap == true then .label_nowrap_ready == true else true end)
        ))
      ))
      and .micro_surface_light_glass_ready == true
      and .status_trust_strip_light_glass_ready == true
      and .message_routing_badge_light_glass_ready == true
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
      and .command_palette_item_light_glass_ready == true
      and .command_palette_item_prismatic_rim_light_glass_ready == true
      and .command_palette_item_label_prismatic_etch_light_glass_ready == true
      and ((.command_palette_item_details // []) | length) >= 2
      and ((.command_palette_item_details // []) | all(.visible == true and .marker == "light-glass" and .light_glass_ready == true and .readable == true and (.contrast_ratio // 0) >= 4.8 and .title_matches_aria_label == true))
      and ((.icon_button_details // []) | length) >= (if .rail_visible == true then 5 else 4 end)
      and ((.icon_button_details // []) | all(.visible == true and .svg_icon_present == true and .visible_icon_text_absent == true and .title_matches_aria_label == true and .width >= 44 and .height >= 44))
      and ((.menu_trigger_details // []) | length) >= 2
      and ((.menu_trigger_details // []) | all(.visible == true and .svg_icon_present == true and .visible_icon_text_absent == true and .title_matches_aria_label == true and .width >= 44 and .height >= 44))
      and ((.menu_surface_details // []) | length) >= 2
      and ((.menu_surface_details // []) | all(.visible == true and .item_count >= 1 and .border_radius >= 16 and ((.backdrop_filter // "") | contains("blur(")) and .box_shadow != "none" and .in_viewport == true and .top_clipped == false and .bottom_clipped == false))
      and .thread_tools_menu_ready == true
      and light_glass_panel_ready(.thread_tools_panel_details; 44)
      and (.thread_tools_panel_details.item_count == 3)
      and ((.thread_tools_item_details // []) | length) == 3
      and ((.thread_tools_item_details // []) | all(menu_item_ready))
      and .composer_tools_menu_ready == true
      and .composer_tools_trigger_light_glass_ready == true
      and light_glass_panel_ready(.composer_tools_panel_details; 44)
      and (.composer_tools_panel_details.item_count == 2)
      and ((.composer_tools_item_details // []) | length) == 2
      and ((.composer_tools_item_details // []) | all(menu_item_ready and .select_present == true and .select_visible == true and .select_title_matches_aria_label == true and .select_readable == true and (.select_contrast_ratio // 0) >= 4.8 and (.select_height // 0) >= 44))
      and .composer_popover_ready == true
      and ((.composer_popover_panel_details // []) | length) == 2
      and ((.composer_popover_panel_details // []) | all(.visible == true and .role == "menu" and ((.aria_label // "") | length) > 0 and .search_count == 1 and .item_count == 2 and .border_radius >= 16 and .light_glass_ready == true and (.effective_luminance // 0) >= 0.72 and (.effective_luminance // 1) <= 0.98 and ((.backdrop_filter // "") | contains("blur(")) and .box_shadow != "none" and .in_viewport == true and .top_clipped == false and .bottom_clipped == false))
      and ((.composer_popover_search_details // []) | length) == 2
      and ((.composer_popover_search_details // []) | all(.visible == true and .marker == "light-glass" and .height >= 44 and .light_glass_ready == true and .title_matches_aria_label == true and .readable == true and (.contrast_ratio // 0) >= 4.8 and .composer_popover_search_placeholder_prismatic_etch_ready == true))
      and ((.composer_popover_item_details // []) | length) == 4
      and ((.composer_popover_item_details // []) | all(menu_item_ready and .detail_nowrap_ready == true and .detail_readable == true and (.detail_contrast_ratio // 0) >= 4.8 and .composer_popover_item_label_prismatic_etch_ready == true))
      and (if .rail_visible == true then
        .row_menu_touch_ready == true
        and .row_menu_all_rows_ready == true
        and .row_menu_light_glass_ready == true
        and ((.row_menu_toggle_details // []) | length) == 3
        and ((.row_menu_panel_details // []) | length) == 3
        and ((.row_menu_item_details // []) | length) == 9
        and ([(.row_menu_panel_details // [])[].owner_key] | sort) == expected_rail_row_keys
        and ((.row_menu_toggle_details // []) | all(.visible == true and .marker == "light-glass" and .width >= 44 and .height >= 44 and .svg_icon_present == true and .visible_icon_text_absent == true and .title_matches_aria_label == true and .box_shadow != "none"))
        and ((.row_menu_panel_details // []) | all(.visible == true and .marker == "light-glass" and .item_count == 3 and .width >= 180 and .height >= 132 and .border_radius >= 16 and .light_glass_ready == true and (.effective_luminance // 0) >= 0.72 and (.effective_luminance // 1) <= 0.98 and ((.backdrop_filter // "") | contains("blur(")) and .box_shadow != "none" and .in_viewport == true and .top_clipped == false and .bottom_clipped == false))
        and ((.row_menu_item_details // []) | all(.visible == true and .height >= 44 and .icon_svg_present == true and .title_matches_aria_label == true and .label_nowrap_ready == true and .readable == true and (.contrast_ratio // 0) >= 4.8))
      else
        ((.row_menu_toggle_details // []) | length) == 0
        and ((.row_menu_panel_details // []) | length) == 0
        and ((.row_menu_item_details // []) | length) == 0
      end);
    def control_ready:
      $control.status == "ready"
      and $control.control_ui_harsh_2026_ready == true
      and $control.control_ui_320_reflow_ready == true
      and $control.control_ui_single_submenu_audit_ready == true
      and $control.control_ui_browser_error_page_absent == true
      and $control.control_ui_horizontal_overflow_free == true
      and $control.density_qa.status == "ready"
      and $control.density_qa.viewport_count == 4
      and ([$control.density_qa.results[] | viewport_id] | sort) == expected_control_viewports
      and ($control.density_qa.results | all(control_viewport_ready));
    def native_action_ready:
      .tag == "button"
      and .type == "button"
      and .disabled == false
      and .semantic_ready == true
      and .button_style_ready == true
      and .harsh_action_ready == true
      and .icon_present == true
      and .icon_svg_ready == true
      and .icon_text_placeholder_absent == true
      and .title_tooltip_ready == true
      and .title == .aria_label
      and .role_ready == true
      and ((.role // "") | length) > 0
      and ((.aria_label // "") | length) > 0
      and .label_nowrap_ready == true
      and .label_word_break_ready == true
      and (.width // 0) >= 44
      and (.height // 0) >= 44;
    def native_secondary_case_ready:
      .ready == true
      and .expected_actions_present == true
      and .semantic_button_ready == true
      and .action_matrix_ready == true
      and .harsh_action_matrix_ready == true
      and .surface_glass_ready == true
      and .action_group_glass_ready == true
      and .actions_usable == true
      and .preferred_touch_target_ready == true
      and .horizontal_overflow_free == true
      and .text_clipping_failure_count == 0
      and .content_edge_failure_count == 0
      and .visible_audit_failure_count == 0
      and .expected_action_ids == expected_secondary_action_matrix[.surface]
      and .action_ids == .expected_action_ids
      and (.action_count == (.expected_action_ids | length))
      and (.action_details | all(native_action_ready));
    def native_ready:
      $native.status == "ready"
      and ($native.screenshot_count // 0) >= 41
      and $native.native_top_design_referee_ready == true
      and $native.native_tempered_glass_visual_contract_ready == true
      and $native.native_secondary_harsh_action_matrix_ready == true
      and $native.native_320_reflow_ready == true
      and $native.native_mobile_touch_target_preferred_ready == true
      and $native.native_telegram_mobile_safe_area_keyboard_ready == true
      and $native.native_readability_contrast_clip_ready == true
      and $native.native_visible_audit_failure_count == 0
      and $native.tempered_glass_visual_contract.status == "ready"
      and $native.tempered_glass_visual_contract.viewport_count == 4
      and $native.tempered_glass_visual_contract.desktop_ready == true
      and $native.tempered_glass_visual_contract.mobile_ready == true
      and $native.tempered_glass_visual_contract.phone_ready == true
      and $native.tempered_glass_visual_contract.phone320_ready == true
      and $native.tempered_glass_visual_contract.preferred_touch_target_ready == true
      and $native.tempered_glass_visual_contract.light_surface_failure_count == 0
      and $native.tempered_glass_visual_contract.readability_failure_count == 0
      and $native.tempered_glass_visual_contract.min_contrast_ratio >= 4.8
      and $native.mobile_safe_area_keyboard.status == "ready"
      and $native.mobile_safe_area_keyboard.content_bounds_ready == true
      and $native.mobile_safe_area_keyboard.content_clipping_failure_count == 0
      and $native.mobile_safe_area_keyboard.horizontal_overflow_free == true
      and $native.secondary_product_surfaces.status == "ready"
      and $native.secondary_product_surfaces.surface_count == 5
      and $native.secondary_product_surfaces.viewport_count == 3
      and $native.secondary_product_surfaces.case_count == 15
      and $native.secondary_product_surfaces.action_matrix_case_count == 15
      and $native.secondary_product_surfaces.harsh_action_matrix_case_count == 15
      and $native.secondary_product_surfaces.total_action_instance_count == 57
      and $native.secondary_product_surfaces.harsh_action_failure_count == 0
      and $native.secondary_product_surfaces.icon_text_placeholder_failure_count == 0
      and $native.secondary_product_surfaces.title_tooltip_failure_count == 0
      and $native.secondary_product_surfaces.label_layout_failure_count == 0
      and $native.secondary_product_surfaces.glass_action_failure_count == 0
      and $native.secondary_product_surfaces.expected_action_matrix == expected_secondary_action_matrix
      and ([$native.secondary_product_surfaces.results[].surface] | unique | sort) == expected_secondary_surfaces
      and ([$native.secondary_product_surfaces.results[] | viewport_id] | unique | sort) == expected_secondary_viewports
      and ($native.secondary_product_surfaces.results | all(native_secondary_case_ready));
    def control_summary:
      {
        viewport_count:$control.density_qa.viewport_count,
        viewports:[$control.density_qa.results[] | {
          name,
          viewport:viewport_id,
          rail_visible,
          icon_buttons:(.icon_button_details | length),
          menu_triggers:(.menu_trigger_details | length),
          menu_surfaces:(.menu_surface_details | length),
          row_menu_panels:(.row_menu_panel_details | length),
          row_menu_items:(.row_menu_item_details | length),
          thread_tools_items:(.thread_tools_item_details | length),
          composer_tools_items:(.composer_tools_item_details | length),
          composer_popover_panels:(.composer_popover_panel_details | length),
          composer_popover_items:(.composer_popover_item_details | length),
          command_palette_items:(.command_palette_item_details | length),
          single_submenu_targets:(.single_submenu_audit_target_count // 0),
          single_submenu_items_total:((.single_submenu_audit_details // []) | map(.visible_item_count // 0) | add // 0),
          single_submenu_command_palette_items:((.single_submenu_audit_details // []) | map(select(.key == "command-palette") | .visible_item_count // 0) | first // 0),
          ready:control_viewport_ready
        }]
      };
    def native_summary:
      {
        screenshot_count:$native.screenshot_count,
        tempered_glass_min_contrast:$native.tempered_glass_visual_contract.min_contrast_ratio,
        secondary_case_count:$native.secondary_product_surfaces.case_count,
        secondary_action_instance_count:$native.secondary_product_surfaces.total_action_instance_count,
        secondary_cases:[$native.secondary_product_surfaces.results[] | {
          surface,
          viewport:viewport_id,
          action_count,
          ready:native_secondary_case_ready
        }]
      };
    {
      schema_version:"hepta-ui-harsh-top-design-referee-v2/v0",
      standards_version:"2026-06-27-harsh-module-button-submenu-light-tempered-glass",
      aesthetic_standard:"2026_light_bright_tempered_glass",
      status:(if (control_ready and native_ready) then "ready" else "failed" end),
      harsh_top_design_referee_v2_ready:(control_ready and native_ready),
      control_ui_full_module_button_submenu_referee_ready:control_ready,
      native_full_module_button_submenu_referee_ready:native_ready,
      requirements:{
        control:"4 viewports; every visible icon button/menu trigger/menu surface/thread tools/composer tools/composer popover/command palette/row menu item must be light-glass, readable, SVG-backed, title/aria matched, and in viewport",
        native:"4 tempered-glass visual contract viewports plus 5 secondary surfaces across 3 viewports; all 57 action instances must be semantic, SVG-backed, titled, readable, touch-safe, and glass styled"
      },
      inputs:{
        control_ui_browser_smoke:{path:$control_path, sha256:$control_sha},
        native_fixture_visual_smoke:{path:$native_path, sha256:$native_sha}
      },
      summary:{
        control:control_summary,
        native:native_summary
      }
    }
  ' >"$tmp_report"

mv "$tmp_report" "$REPORT_PATH"

jq -e '
  .status == "ready"
  and .harsh_top_design_referee_v2_ready == true
  and .control_ui_full_module_button_submenu_referee_ready == true
  and .native_full_module_button_submenu_referee_ready == true
' "$REPORT_PATH" >/dev/null

echo "$REPORT_PATH"
