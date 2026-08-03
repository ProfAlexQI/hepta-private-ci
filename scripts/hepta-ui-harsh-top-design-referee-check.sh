#!/usr/bin/env bash
set -euo pipefail

READINESS_DIR="${1:-${HEPTA_UI_PRODUCT_READINESS_DIR:-}}"
REQUIRE_FULL_HARD="${HEPTA_UI_HARSH_TOP_DESIGN_REQUIRE_FULL_HARD:-1}"

if [[ -z "$READINESS_DIR" ]]; then
  echo "usage: $0 <hepta-ui-product-readiness-dir>" >&2
  exit 2
fi

TOP_DESIGN_JSON="$READINESS_DIR/ui-top-design-referee-refresh-gate.json"
FIXTURE_HTML="$READINESS_DIR/native-fixture/hepta-native-fixture.html"

if [[ ! -s "$TOP_DESIGN_JSON" ]]; then
  echo "missing top-design referee report: $TOP_DESIGN_JSON" >&2
  exit 1
fi

if [[ ! -s "$FIXTURE_HTML" ]]; then
  echo "missing native fixture html: $FIXTURE_HTML" >&2
  exit 1
fi

jq -e --arg require_full_hard "$REQUIRE_FULL_HARD" '
  .status == "ready"
  and .top_design_harsh_2026_referee_ready == true
  and .control_ui_harsh_2026_ready == true
  and .native_secondary_harsh_action_matrix_ready == true
	  and .referee_matrix.control_ui.menu_surfaces_ready == true
	  and .referee_matrix.control_ui.menu_surface_viewport_guard_ready == true
	  and .referee_matrix.control_ui.icon_button_title_match_ready == true
	  and .referee_matrix.control_ui.menu_trigger_title_match_ready == true
	  and .referee_matrix.control_ui.rail_action_icon_ready == true
	  and .referee_matrix.control_ui.microcopy_word_split_guard_ready == true
	  and .referee_matrix.control_ui.logo_clip_guard_ready == true
	  and .referee_matrix.control_ui.active_chat_readability_ready == true
	  and .referee_matrix.control_ui.placeholder_readability_ready == true
		  and .referee_matrix.control_ui.small_control_readability_ready == true
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
			  and .referee_matrix.control_ui.rail_search_light_glass_ready == true
			  and .referee_matrix.control_ui.micro_surface_light_glass_ready == true
			  and .referee_matrix.control_ui.message_routing_badge_light_glass_ready == true
			  and .referee_matrix.control_ui.thread_intro_badge_light_glass_ready == true
			  and .referee_matrix.control_ui.status_trust_strip_light_glass_ready == true
			  and .referee_matrix.control_ui.visible_text_integrity_ready == true
	  and .referee_matrix.tempered_glass_2026.control_microcopy_word_split_guard_ready == true
	  and .referee_matrix.tempered_glass_2026.control_logo_clip_guard_ready == true
	  and .referee_matrix.tempered_glass_2026.control_active_chat_readability_ready == true
	  and .referee_matrix.tempered_glass_2026.control_placeholder_readability_ready == true
		  and .referee_matrix.tempered_glass_2026.control_small_control_readability_ready == true
		  and .referee_matrix.tempered_glass_2026.control_row_menu_touch_ready == true
		  and .referee_matrix.tempered_glass_2026.control_row_menu_all_rows_ready == true
			  and .referee_matrix.tempered_glass_2026.control_row_menu_light_glass_ready == true
			  and .referee_matrix.tempered_glass_2026.control_command_palette_ready == true
			  and .referee_matrix.tempered_glass_2026.control_command_palette_surface_light_glass_ready == true
		  and .referee_matrix.tempered_glass_2026.control_command_palette_trigger_light_glass_ready == true
			  and .referee_matrix.tempered_glass_2026.control_command_palette_close_light_glass_ready == true
			  and .referee_matrix.tempered_glass_2026.control_command_palette_input_light_glass_ready == true
			  and .referee_matrix.tempered_glass_2026.control_command_palette_item_light_glass_ready == true
			  and .referee_matrix.tempered_glass_2026.control_form_control_title_touch_ready == true
			  and .referee_matrix.tempered_glass_2026.control_chat_row_option_semantic_touch_ready == true
			  and .referee_matrix.tempered_glass_2026.control_thread_tools_menu_ready == true
			  and .referee_matrix.tempered_glass_2026.control_composer_tools_menu_ready == true
			  and .referee_matrix.tempered_glass_2026.control_composer_popover_ready == true
			  and .referee_matrix.tempered_glass_2026.control_composer_popover_search_light_glass_ready == true
			  and .referee_matrix.tempered_glass_2026.control_rail_search_light_glass_ready == true
			  and .referee_matrix.tempered_glass_2026.control_micro_surface_light_glass_ready == true
			  and .referee_matrix.tempered_glass_2026.control_message_routing_badge_light_glass_ready == true
			  and .referee_matrix.tempered_glass_2026.control_thread_intro_badge_light_glass_ready == true
			  and .referee_matrix.tempered_glass_2026.control_status_trust_strip_light_glass_ready == true
			  and .referee_matrix.tempered_glass_2026.control_visible_text_integrity_ready == true
  and .referee_matrix.control_level.secondary_surface_icon_svg_ready == true
  and .referee_matrix.control_level.secondary_surface_icon_text_placeholder_absent == true
  and .referee_matrix.control_level.secondary_surface_icon_text_placeholder_failure_count == 0
  and .referee_matrix.control_level.secondary_surface_title_tooltip_ready == true
  and .referee_matrix.control_level.secondary_surface_title_tooltip_failure_count == 0
  and .referee_matrix.native_fixture.secondary_surface_icon_svg_ready == true
  and .referee_matrix.native_fixture.secondary_surface_icon_text_placeholder_absent == true
  and .referee_matrix.native_fixture.secondary_surface_icon_text_placeholder_failure_count == 0
  and .referee_matrix.native_fixture.secondary_surface_title_tooltip_ready == true
  and .referee_matrix.native_fixture.secondary_surface_title_tooltip_failure_count == 0
  and .referee_matrix.tempered_glass_2026.native_secondary_title_tooltip_ready == true
  and .referee_matrix.tempered_glass_2026.native_secondary_title_tooltip_failure_count == 0
  and .refresh_version >= 46
  and .standards_version == "2026-06-24-harsh-badge-micro-surface-light-glass"
  and .aesthetic_standard == "2026_tempered_glass_liquid_glass"
  and (
    $require_full_hard != "1"
    or (
      .true_window_evidence_mode == "full_hard_true_window"
      and .hard_true_window_evidence_ready == true
    )
  )
' "$TOP_DESIGN_JSON" >/dev/null

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
CONTROL_HTML_SOURCE="$REPO_ROOT/apps/hepta-control-ui/index.html"
CONTROL_RUST_SOURCE="$REPO_ROOT/codex-rs/hepta-core/src/control_ui.rs"
CONTROL_STYLE_SOURCES=(
  "$REPO_ROOT/apps/hepta-control-ui/light-glass-tokens.generated.css"
  "$REPO_ROOT/apps/hepta-control-ui/styles.legacy.css"
  "$REPO_ROOT/apps/hepta-control-ui/styles.foundation.css"
  "$REPO_ROOT/apps/hepta-control-ui/styles.components.css"
  "$REPO_ROOT/apps/hepta-control-ui/styles.responsive.css"
  "$REPO_ROOT/apps/hepta-control-ui/styles.accessibility.css"
)
CONTROL_BROWSER_SMOKE_SOURCE="$REPO_ROOT/scripts/hepta-browser-visual-smoke.sh"
CONTROL_BROWSER_SMOKE_SOURCES=(
  "$CONTROL_BROWSER_SMOKE_SOURCE"
  "$REPO_ROOT/scripts/lib/hepta-browser-visual-smoke-v1/"*.sh
  "$REPO_ROOT/scripts/lib/hepta-browser-visual-smoke-v1/"*.cjs
  "$REPO_ROOT/scripts/lib/hepta-browser-visual-smoke-v1/density-probe/"*.cjs
)

for source in "$CONTROL_HTML_SOURCE" "$CONTROL_RUST_SOURCE"; do
  if [[ ! -s "$source" ]]; then
    echo "missing Control UI source for harsh iconography check: $source" >&2
    exit 1
  fi
done

for control_style_source in "${CONTROL_STYLE_SOURCES[@]}"; do
  if [[ ! -s "$control_style_source" ]]; then
    echo "missing Control UI stylesheet for harsh microcopy check: $control_style_source" >&2
    exit 1
  fi
done

if [[ ! -s "$CONTROL_BROWSER_SMOKE_SOURCE" ]]; then
  echo "missing Control UI browser smoke source for visible-text integrity check: $CONTROL_BROWSER_SMOKE_SOURCE" >&2
  exit 1
fi

if grep -Eq 'overflow-wrap:[[:space:]]*anywhere|word-break:[[:space:]]*(break-word|break-all)' "${CONTROL_STYLE_SOURCES[@]}"; then
  echo "Control UI stylesheet still allows arbitrary word splitting in visible microcopy" >&2
  exit 1
fi

if ! grep -Fq '/*mslgtiblgbmslgstslgrslgbsprsp*/[data-view=chat] .telegram-chat-shell :is([data-control-ui-micro-surface],.tg-message small,.tg-thread-header p,[data-chat-shortcut-hint],.tg-chat-item__topline span,.tg-rail-status__item,.tg-bubble>span)' "${CONTROL_STYLE_SOURCES[@]}"; then
  echo "Control UI stylesheet is missing compact thread-header/composer/message-meta/chat-row readable text protection" >&2
  exit 1
fi

if ! grep -Fq 'input[data-control-ui-rail-search-input=light-glass]::placeholder{color:#456!important;opacity:1;text-shadow:var(--x)}' "${CONTROL_STYLE_SOURCES[@]}"; then
  echo "Control UI stylesheet is missing compact search/composer placeholder readability protection" >&2
  exit 1
fi

if ! grep -Fq '/*cmvctlgcplgcpsgttlg*/.tg-compose-footer .tg-thread-command-menu__panel' "${CONTROL_STYLE_SOURCES[@]}"; then
  echo "Control UI stylesheet is missing composer menu viewport guard marker" >&2
  exit 1
fi

if ! grep -Fq 'bottom:calc(100% + 8px)' "${CONTROL_STYLE_SOURCES[@]}"; then
  echo "Control UI stylesheet does not force composer command menu to open upward inside the viewport" >&2
  exit 1
fi

if ! grep -Fq '[data-control-ui-thread-tools-panel=light-glass],[data-control-ui-composer-tools-panel=light-glass]{background:#f2fbff!important;color:#123!important}' "${CONTROL_STYLE_SOURCES[@]}"; then
  echo "Control UI stylesheet is missing compact chat-row glass contrast protection" >&2
  exit 1
fi

if ! grep -Fq '.command-palette[data-control-ui-command-palette-surface="light-glass"]' "${CONTROL_STYLE_SOURCES[@]}"; then
  echo "Control UI stylesheet is missing command palette light glass surface protection" >&2
  exit 1
fi

if ! grep -Fq '/*cptrmlg*/[data-view=chat] :is(.tg-icon-action[data-control-ui-command-palette-trigger=light-glass],[data-control-ui-thread-tools-trigger=light-glass],[data-control-ui-composer-tools-trigger=light-glass],[data-control-ui-row-menu-trigger=light-glass])' "${CONTROL_STYLE_SOURCES[@]}"; then
  echo "Control UI stylesheet is missing command palette trigger light glass protection" >&2
  exit 1
fi

if ! grep -Fq 'document.querySelectorAll(".tg-chat-item :is(.tg-chat-item__topline strong,.tg-chat-item__topline span,.tg-chat-item__body p)")' "${CONTROL_BROWSER_SMOKE_SOURCES[@]}"; then
  echo "Control UI browser smoke still checks thread-header/chat-row readability too narrowly" >&2
  exit 1
fi

if ! grep -Fq 'document.querySelectorAll(".tg-thread-header__main p")' "${CONTROL_BROWSER_SMOKE_SOURCES[@]}"; then
  echo "Control UI browser smoke is missing thread-header status readability sampling" >&2
  exit 1
fi

if ! grep -Fq 'document.querySelectorAll(".tg-compose-footer [data-chat-shortcut-hint]")' "${CONTROL_BROWSER_SMOKE_SOURCES[@]}"; then
  echo "Control UI browser smoke is missing composer footer status readability sampling" >&2
  exit 1
fi

if ! grep -Fq 'document.querySelectorAll(".tg-message small")' "${CONTROL_BROWSER_SMOKE_SOURCES[@]}"; then
  echo "Control UI browser smoke is missing message metadata readability sampling" >&2
  exit 1
fi

if ! grep -Fq 'document.querySelectorAll(".tg-search-shell input[placeholder],.tg-compose-bar textarea[placeholder]")' "${CONTROL_BROWSER_SMOKE_SOURCES[@]}"; then
  echo "Control UI browser smoke is missing placeholder readability sampling" >&2
  exit 1
fi

if ! grep -Fq 'document.querySelectorAll(".tg-folder-chip,.tg-folder-chip small,.tg-thread-hepta-controls span,.tg-thread-hepta-controls select,.tg-autoscroll-select,.tg-autoscroll-select select,.tg-menu-item__label")' "${CONTROL_BROWSER_SMOKE_SOURCES[@]}"; then
  echo "Control UI browser smoke is missing small control readability sampling" >&2
  exit 1
fi

if ! grep -Fq 'vertical_in_viewport: verticalInViewport' "${CONTROL_BROWSER_SMOKE_SOURCES[@]}"; then
  echo "Control UI browser smoke is missing vertical menu viewport sampling" >&2
  exit 1
fi

if ! grep -Fq 'bottom_clipped: rect.bottom > window.innerHeight + 1' "${CONTROL_BROWSER_SMOKE_SOURCES[@]}"; then
  echo "Control UI browser smoke is missing bottom-clipped submenu detection" >&2
  exit 1
fi

if ! grep -Fq 'title_matches_aria_label: title === ariaLabel' "${CONTROL_BROWSER_SMOKE_SOURCES[@]}"; then
  echo "Control UI browser smoke is missing icon/menu title parity sampling" >&2
  exit 1
fi

if ! grep -Fq 'control_ui_icon_button_title_match_ready:$density_qa.icon_button_title_match_ready' "${CONTROL_BROWSER_SMOKE_SOURCES[@]}"; then
  echo "Control UI browser smoke is missing icon button title parity aggregate" >&2
  exit 1
fi

if ! grep -Fq 'control_ui_rail_action_icon_ready:$density_qa.rail_action_icon_ready' "${CONTROL_BROWSER_SMOKE_SOURCES[@]}"; then
  echo "Control UI browser smoke is missing rail action icon aggregate" >&2
  exit 1
fi

if ! grep -Fq 'rail_action_icon_ready: results.every((result) => result.rail_action_icon_ready === true)' "${CONTROL_BROWSER_SMOKE_SOURCES[@]}"; then
  echo "Control UI browser smoke is missing rail action icon density rollup" >&2
  exit 1
fi

if ! grep -Fq 'iconButtonDetails.length >= (railVisible ? 5 : 4)' "${CONTROL_BROWSER_SMOKE_SOURCES[@]}"; then
  echo "Control UI browser smoke still allows the rail action icon to be omitted from icon-button coverage" >&2
  exit 1
fi

if ! grep -Fq 'control_ui_menu_trigger_title_match_ready:$density_qa.menu_trigger_title_match_ready' "${CONTROL_BROWSER_SMOKE_SOURCES[@]}"; then
  echo "Control UI browser smoke is missing menu trigger title parity aggregate" >&2
  exit 1
fi

if ! grep -Fq 'control_ui_folder_chip_touch_ready:$density_qa.folder_chip_touch_ready' "${CONTROL_BROWSER_SMOKE_SOURCES[@]}"; then
  echo "Control UI browser smoke is missing folder chip touch aggregate" >&2
  exit 1
fi

if ! grep -Fq 'folder_chip_touch_ready: results.every((result) => result.folder_chip_touch_ready === true)' "${CONTROL_BROWSER_SMOKE_SOURCES[@]}"; then
  echo "Control UI browser smoke is missing folder chip touch density rollup" >&2
  exit 1
fi

if ! grep -Fq 'control_ui_row_menu_touch_ready:$density_qa.row_menu_touch_ready' "${CONTROL_BROWSER_SMOKE_SOURCES[@]}"; then
  echo "Control UI browser smoke is missing row menu touch aggregate" >&2
  exit 1
fi

if ! grep -Fq 'control_ui_row_menu_light_glass_ready:$density_qa.row_menu_light_glass_ready' "${CONTROL_BROWSER_SMOKE_SOURCES[@]}"; then
  echo "Control UI browser smoke is missing row menu light glass aggregate" >&2
  exit 1
fi

if ! grep -Fq 'row_menu_touch_ready: results.every((result) => result.row_menu_touch_ready === true)' "${CONTROL_BROWSER_SMOKE_SOURCES[@]}"; then
  echo "Control UI browser smoke is missing row menu touch density rollup" >&2
  exit 1
fi

if ! grep -Fq 'row_menu_light_glass_ready: results.every((result) => result.row_menu_light_glass_ready === true)' "${CONTROL_BROWSER_SMOKE_SOURCES[@]}"; then
  echo "Control UI browser smoke is missing row menu light glass density rollup" >&2
  exit 1
fi

if ! grep -Fq 'control_ui_command_palette_ready:$density_qa.command_palette_ready' "${CONTROL_BROWSER_SMOKE_SOURCES[@]}"; then
  echo "Control UI browser smoke is missing command palette aggregate" >&2
  exit 1
fi

if ! grep -Fq 'control_ui_command_palette_surface_light_glass_ready:$density_qa.command_palette_surface_light_glass_ready' "${CONTROL_BROWSER_SMOKE_SOURCES[@]}"; then
  echo "Control UI browser smoke is missing command palette surface light glass aggregate" >&2
  exit 1
fi

if ! grep -Fq 'control_ui_command_palette_trigger_light_glass_ready:$density_qa.command_palette_trigger_light_glass_ready' "${CONTROL_BROWSER_SMOKE_SOURCES[@]}"; then
  echo "Control UI browser smoke is missing command palette trigger light glass aggregate" >&2
  exit 1
fi

if ! grep -Fq 'control_ui_command_palette_input_light_glass_ready:$density_qa.command_palette_input_light_glass_ready' "${CONTROL_BROWSER_SMOKE_SOURCES[@]}"; then
  echo "Control UI browser smoke is missing command palette input light glass aggregate" >&2
  exit 1
fi

if ! grep -Fq 'control_ui_command_palette_close_light_glass_ready:$density_qa.command_palette_close_light_glass_ready' "${CONTROL_BROWSER_SMOKE_SOURCES[@]}"; then
  echo "Control UI browser smoke is missing command palette trigger light glass aggregate" >&2
  exit 1
fi

if ! grep -Fq 'control_ui_command_palette_item_light_glass_ready:$density_qa.command_palette_item_light_glass_ready' "${CONTROL_BROWSER_SMOKE_SOURCES[@]}"; then
  echo "Control UI browser smoke is missing command palette result light glass aggregate" >&2
  exit 1
fi

if ! grep -Fq 'command_palette_ready: results.every((result) => result.command_palette_ready === true)' "${CONTROL_BROWSER_SMOKE_SOURCES[@]}"; then
  echo "Control UI browser smoke is missing command palette density rollup" >&2
  exit 1
fi

if ! grep -Fq 'command_palette_surface_light_glass_ready: results.every((result) => result.command_palette_surface_light_glass_ready === true)' "${CONTROL_BROWSER_SMOKE_SOURCES[@]}"; then
  echo "Control UI browser smoke is missing command palette surface light glass density rollup" >&2
  exit 1
fi

if ! grep -Fq 'command_palette_trigger_light_glass_ready: results.every((result) => result.command_palette_trigger_light_glass_ready === true)' "${CONTROL_BROWSER_SMOKE_SOURCES[@]}"; then
  echo "Control UI browser smoke is missing command palette trigger light glass density rollup" >&2
  exit 1
fi

if ! grep -Fq 'command_palette_input_light_glass_ready: results.every((result) => result.command_palette_input_light_glass_ready === true)' "${CONTROL_BROWSER_SMOKE_SOURCES[@]}"; then
  echo "Control UI browser smoke is missing command palette input light glass density rollup" >&2
  exit 1
fi

if ! grep -Fq 'command_palette_close_light_glass_ready: results.every((result) => result.command_palette_close_light_glass_ready === true)' "${CONTROL_BROWSER_SMOKE_SOURCES[@]}"; then
  echo "Control UI browser smoke is missing command palette trigger light glass density rollup" >&2
  exit 1
fi

if ! grep -Fq 'command_palette_item_light_glass_ready: results.every((result) => result.command_palette_item_light_glass_ready === true)' "${CONTROL_BROWSER_SMOKE_SOURCES[@]}"; then
  echo "Control UI browser smoke is missing command palette result light glass density rollup" >&2
  exit 1
fi

if ! grep -Fq 'control_ui_form_control_title_touch_ready:$density_qa.control_form_control_title_touch_ready' "${CONTROL_BROWSER_SMOKE_SOURCES[@]}"; then
  echo "Control UI browser smoke is missing form control title/touch aggregate" >&2
  exit 1
fi

if ! grep -Fq 'control_form_control_title_touch_ready: results.every((result) => result.control_form_control_title_touch_ready === true)' "${CONTROL_BROWSER_SMOKE_SOURCES[@]}"; then
  echo "Control UI browser smoke is missing form control title/touch density rollup" >&2
  exit 1
fi

if ! grep -Fq 'document.querySelectorAll("[data-chat-search],[data-chat-composer-input],[data-chat-routing-mode],[data-chat-autoscroll-mode]")' "${CONTROL_BROWSER_SMOKE_SOURCES[@]}"; then
  echo "Control UI browser smoke is missing visible form control sampling" >&2
  exit 1
fi

if ! grep -Fq 'expectedVisibleFormControlCount = railVisible ? 4 : 1' "${CONTROL_BROWSER_SMOKE_SOURCES[@]}"; then
  echo "Control UI browser smoke does not require the desktop and mobile form control coverage counts" >&2
  exit 1
fi

if ! grep -Fq 'control_ui_chat_row_option_semantic_touch_ready:$density_qa.chat_row_option_semantic_touch_ready' "${CONTROL_BROWSER_SMOKE_SOURCES[@]}"; then
  echo "Control UI browser smoke is missing chat row option semantic/touch aggregate" >&2
  exit 1
fi

if ! grep -Fq 'control_ui_thread_tools_menu_ready:$density_qa.thread_tools_menu_ready' "${CONTROL_BROWSER_SMOKE_SOURCES[@]}"; then
  echo "Control UI browser smoke is missing thread tools menu aggregate" >&2
  exit 1
fi

if ! grep -Fq 'control_ui_composer_tools_menu_ready:$density_qa.composer_tools_menu_ready' "${CONTROL_BROWSER_SMOKE_SOURCES[@]}"; then
  echo "Control UI browser smoke is missing composer tools menu aggregate" >&2
  exit 1
fi

if ! grep -Fq 'control_ui_composer_popover_ready:$density_qa.composer_popover_ready' "${CONTROL_BROWSER_SMOKE_SOURCES[@]}"; then
  echo "Control UI browser smoke is missing composer popover aggregate" >&2
  exit 1
fi

if ! grep -Fq 'control_ui_composer_popover_search_light_glass_ready:$density_qa.composer_popover_search_light_glass_ready' "${CONTROL_BROWSER_SMOKE_SOURCES[@]}"; then
  echo "Control UI browser smoke is missing composer popover search light glass aggregate" >&2
  exit 1
fi

if ! grep -Fq 'control_ui_rail_search_light_glass_ready:$density_qa.rail_search_light_glass_ready' "${CONTROL_BROWSER_SMOKE_SOURCES[@]}"; then
  echo "Control UI browser smoke is missing rail search light glass aggregate" >&2
  exit 1
fi

if ! grep -Fq 'control_ui_micro_surface_light_glass_ready:$density_qa.micro_surface_light_glass_ready' "${CONTROL_BROWSER_SMOKE_SOURCES[@]}"; then
  echo "Control UI browser smoke is missing micro-surface light glass aggregate" >&2
  exit 1
fi

if ! grep -Fq 'control_ui_message_routing_badge_light_glass_ready:$density_qa.message_routing_badge_light_glass_ready' "${CONTROL_BROWSER_SMOKE_SOURCES[@]}"; then
  echo "Control UI browser smoke is missing message routing badge light glass aggregate" >&2
  exit 1
fi

if ! grep -Fq 'control_ui_thread_intro_badge_light_glass_ready:$density_qa.thread_intro_badge_light_glass_ready' "${CONTROL_BROWSER_SMOKE_SOURCES[@]}"; then
  echo "Control UI browser smoke is missing thread intro badge light glass aggregate" >&2
  exit 1
fi

if ! grep -Fq 'control_ui_status_trust_strip_light_glass_ready:$density_qa.status_trust_strip_light_glass_ready' "${CONTROL_BROWSER_SMOKE_SOURCES[@]}"; then
  echo "Control UI browser smoke is missing status trust strip light glass aggregate" >&2
  exit 1
fi

if ! grep -Fq 'chat_row_option_semantic_touch_ready: results.every((result) => result.chat_row_option_semantic_touch_ready === true)' "${CONTROL_BROWSER_SMOKE_SOURCES[@]}"; then
  echo "Control UI browser smoke is missing chat row option semantic/touch density rollup" >&2
  exit 1
fi

if ! grep -Fq 'thread_tools_menu_ready: results.every((result) => result.thread_tools_menu_ready === true)' "${CONTROL_BROWSER_SMOKE_SOURCES[@]}"; then
  echo "Control UI browser smoke is missing thread tools menu density rollup" >&2
  exit 1
fi

if ! grep -Fq 'composer_tools_menu_ready: results.every((result) => result.composer_tools_menu_ready === true)' "${CONTROL_BROWSER_SMOKE_SOURCES[@]}"; then
  echo "Control UI browser smoke is missing composer tools menu density rollup" >&2
  exit 1
fi

if ! grep -Fq 'composer_popover_ready: results.every((result) => result.composer_popover_ready === true)' "${CONTROL_BROWSER_SMOKE_SOURCES[@]}"; then
  echo "Control UI browser smoke is missing composer popover density rollup" >&2
  exit 1
fi

if ! grep -Fq 'composer_popover_search_light_glass_ready: results.every((result) => result.composer_popover_search_light_glass_ready === true)' "${CONTROL_BROWSER_SMOKE_SOURCES[@]}"; then
  echo "Control UI browser smoke is missing composer popover search light glass density rollup" >&2
  exit 1
fi

if ! grep -Fq 'rail_search_light_glass_ready: results.every((result) => result.rail_search_light_glass_ready === true)' "${CONTROL_BROWSER_SMOKE_SOURCES[@]}"; then
  echo "Control UI browser smoke is missing rail search light glass density rollup" >&2
  exit 1
fi

if ! grep -Fq 'micro_surface_light_glass_ready: results.every((result) => result.micro_surface_light_glass_ready === true)' "${CONTROL_BROWSER_SMOKE_SOURCES[@]}"; then
  echo "Control UI browser smoke is missing micro-surface light glass density rollup" >&2
  exit 1
fi

if ! grep -Fq 'message_routing_badge_light_glass_ready: results.every((result) => result.message_routing_badge_light_glass_ready === true)' "${CONTROL_BROWSER_SMOKE_SOURCES[@]}"; then
  echo "Control UI browser smoke is missing message routing badge light glass density rollup" >&2
  exit 1
fi

if ! grep -Fq 'thread_intro_badge_light_glass_ready: results.every((result) => result.thread_intro_badge_light_glass_ready === true)' "${CONTROL_BROWSER_SMOKE_SOURCES[@]}"; then
  echo "Control UI browser smoke is missing thread intro badge light glass density rollup" >&2
  exit 1
fi

if ! grep -Fq 'status_trust_strip_light_glass_ready: results.every((result) => result.status_trust_strip_light_glass_ready === true)' "${CONTROL_BROWSER_SMOKE_SOURCES[@]}"; then
  echo "Control UI browser smoke is missing status trust strip light glass density rollup" >&2
  exit 1
fi

if ! grep -Fq 'control_ui_row_menu_all_rows_ready:$density_qa.row_menu_all_rows_ready' "${CONTROL_BROWSER_SMOKE_SOURCES[@]}"; then
  echo "Control UI browser smoke is missing all-row row menu aggregate" >&2
  exit 1
fi

if ! grep -Fq 'row_menu_all_rows_ready: results.every((result) => result.row_menu_all_rows_ready === true)' "${CONTROL_BROWSER_SMOKE_SOURCES[@]}"; then
  echo "Control UI browser smoke is missing all-row row menu density rollup" >&2
  exit 1
fi

if ! grep -Fq 'expectedVisibleRowMenuPanelCount = railVisible ? 3 : 0' "${CONTROL_BROWSER_SMOKE_SOURCES[@]}"; then
  echo "Control UI browser smoke does not require every visible conversation row to own a menu panel" >&2
  exit 1
fi

if ! grep -Fq 'data-control-ui-thread-tools-trigger="light-glass"' "${CONTROL_BROWSER_SMOKE_SOURCES[@]}"; then
  echo "Control UI browser smoke is missing thread tools light-glass trigger source marker sampling" >&2
  exit 1
fi

if ! grep -Fq 'data-control-ui-thread-tools-panel="light-glass"' "${CONTROL_BROWSER_SMOKE_SOURCES[@]}"; then
  echo "Control UI browser smoke is missing thread tools light-glass panel source marker sampling" >&2
  exit 1
fi

if ! grep -Fq 'data-control-ui-composer-tools-trigger="light-glass"' "${CONTROL_BROWSER_SMOKE_SOURCES[@]}"; then
  echo "Control UI browser smoke is missing composer tools light-glass trigger source marker sampling" >&2
  exit 1
fi

if ! grep -Fq 'data-control-ui-composer-tools-panel="light-glass"' "${CONTROL_BROWSER_SMOKE_SOURCES[@]}"; then
  echo "Control UI browser smoke is missing composer tools light-glass panel source marker sampling" >&2
  exit 1
fi

if ! grep -Fq 'data-control-ui-composer-popover-panel="light-glass"' "${CONTROL_BROWSER_SMOKE_SOURCES[@]}"; then
  echo "Control UI browser smoke is missing composer popover light-glass source marker sampling" >&2
  exit 1
fi

if ! grep -Fq 'data-control-ui-composer-popover-search' "${CONTROL_BROWSER_SMOKE_SOURCES[@]}"; then
  echo "Control UI browser smoke is missing composer popover search light-glass source marker sampling" >&2
  exit 1
fi

if ! grep -Fq 'data-control-ui-rail-search-input' "${CONTROL_BROWSER_SMOKE_SOURCES[@]}"; then
  echo "Control UI browser smoke is missing badge micro-surface light-glass source marker sampling" >&2
  exit 1
fi

if ! grep -Fq 'data-control-ui-micro-surface' "${CONTROL_BROWSER_SMOKE_SOURCES[@]}"; then
  echo "Control UI browser smoke is missing micro-surface source marker sampling" >&2
  exit 1
fi

if ! grep -Fq 'routingBadgeDetails = microSurfaceDetails.filter((item) => item.key === "routing-safe-preview" || item.key === "routing-local-only")' "${CONTROL_BROWSER_SMOKE_SOURCES[@]}"; then
  echo "Control UI browser smoke is missing message routing badge marker sampling" >&2
  exit 1
fi

if ! grep -Fq 'threadIntroBadgeDetails = microSurfaceDetails.filter((item) => item.key.startsWith("thread-intro-"))' "${CONTROL_BROWSER_SMOKE_SOURCES[@]}"; then
  echo "Control UI browser smoke is missing thread intro badge marker sampling" >&2
  exit 1
fi

if ! grep -Fq 'statusTrustBadgeNodes = Array.from(document.querySelectorAll("[data-control-ui-status-trust-badge]"))' "${CONTROL_BROWSER_SMOKE_SOURCES[@]}"; then
  echo "Control UI browser smoke is missing status trust badge marker sampling" >&2
  exit 1
fi

if ! grep -Fq '/*mslgtiblgbmslgstslgrslgbsprsp*/[data-view=chat] .telegram-chat-shell :is([data-control-ui-micro-surface],.tg-message small,.tg-thread-header p,[data-chat-shortcut-hint],.tg-chat-item__topline span,.tg-rail-status__item,.tg-bubble>span)' "${CONTROL_STYLE_SOURCES[@]}"; then
  echo "Control UI stylesheet is missing micro-surface light-glass marker" >&2
  exit 1
fi

if ! grep -Fq '/*mslgtiblgbmslgstslgrslgbsprsp*/[data-view=chat] .telegram-chat-shell :is([data-control-ui-micro-surface],.tg-message small,.tg-thread-header p,[data-chat-shortcut-hint],.tg-chat-item__topline span,.tg-rail-status__item,.tg-bubble>span)' "${CONTROL_STYLE_SOURCES[@]}"; then
  echo "Control UI stylesheet is missing thread intro badge light-glass marker" >&2
  exit 1
fi

if ! grep -Fq '/*mslgtiblgbmslgstslgrslgbsprsp*/[data-view=chat] .telegram-chat-shell :is([data-control-ui-micro-surface],.tg-message small,.tg-thread-header p,[data-chat-shortcut-hint],.tg-chat-item__topline span,.tg-rail-status__item,.tg-bubble>span)' "${CONTROL_STYLE_SOURCES[@]}"; then
  echo "Control UI stylesheet is missing badge micro-surface light-glass marker" >&2
  exit 1
fi

if ! grep -Fq '/*mslgtiblgbmslgstslgrslgbsprsp*/[data-view=chat] .telegram-chat-shell :is([data-control-ui-micro-surface],.tg-message small,.tg-thread-header p,[data-chat-shortcut-hint],.tg-chat-item__topline span,.tg-rail-status__item,.tg-bubble>span)' "${CONTROL_STYLE_SOURCES[@]}"; then
  echo "Control UI stylesheet is missing status trust strip light-glass marker" >&2
  exit 1
fi

for source in "$CONTROL_HTML_SOURCE" "$CONTROL_RUST_SOURCE"; do
  if ! grep -Fq 'data-control-ui-rail-search-input="light-glass"' "$source"; then
    echo "Control UI source is missing badge micro-surface light-glass marker: $source" >&2
    exit 1
  fi
  if ! grep -Fq 'data-control-ui-status-trust-strip="local-safe-review" role="group" aria-label="Thread status trust"' "$source"; then
    echo "Control UI source is missing status trust strip accessibility marker: $source" >&2
    exit 1
  fi
  if ! grep -Fq 'data-control-ui-status-trust-badge="local" data-control-ui-micro-surface="thread-status-local" aria-label="Local trust status" title="Local trust status"' "$source"; then
    echo "Control UI source is missing local status trust badge marker: $source" >&2
    exit 1
  fi
  if ! grep -Fq 'data-control-ui-status-trust-badge="safe-review" data-control-ui-micro-surface="thread-status-safe-review" aria-label="Safe review status" title="Safe review status"' "$source"; then
    echo "Control UI source is missing safe-review status trust badge marker: $source" >&2
    exit 1
  fi
  if ! grep -Fq 'data-control-ui-micro-surface="date-divider"' "$source"; then
    echo "Control UI source is missing date-divider micro-surface marker: $source" >&2
    exit 1
  fi
  if ! grep -Fq 'data-control-ui-micro-surface="composer-status-ready"' "$source"; then
    echo "Control UI source is missing composer status micro-surface marker: $source" >&2
    exit 1
  fi
  if ! grep -Fq 'data-control-ui-micro-surface="routing-safe-preview"' "$source"; then
    echo "Control UI source is missing safe-preview message routing badge marker: $source" >&2
    exit 1
  fi
  if ! grep -Fq 'data-control-ui-micro-surface="routing-local-only"' "$source"; then
    echo "Control UI source is missing local-only message routing badge marker: $source" >&2
    exit 1
  fi
done

for key in telegram-shell message-workflow evidence-inline approval-chat; do
  if ! grep -Fq "data-control-ui-thread-intro-badge=\"$key\"" "$CONTROL_HTML_SOURCE"; then
    echo "Control UI static HTML is missing thread intro badge marker $key" >&2
    exit 1
  fi
  if ! grep -Fq "data-control-ui-thread-intro-badge=\\\"$key\\\"" "$CONTROL_RUST_SOURCE"; then
    echo "Control UI Rust product path is missing thread intro badge marker $key" >&2
    exit 1
  fi
done

if ! grep -Fq "document.querySelectorAll('[data-thread-command-menu=\"true\"] [data-control-ui-menu-item]')" "${CONTROL_BROWSER_SMOKE_SOURCES[@]}"; then
  echo "Control UI browser smoke is missing thread tools menu item sampling" >&2
  exit 1
fi

if ! grep -Fq "document.querySelectorAll('[data-control-ui-composer-more] [data-control-ui-composer-tool-item]')" "${CONTROL_BROWSER_SMOKE_SOURCES[@]}"; then
  echo "Control UI browser smoke is missing composer tools menu item sampling" >&2
  exit 1
fi

if ! grep -Fq 'document.querySelectorAll("[data-chat-composer-picker-item]")' "${CONTROL_BROWSER_SMOKE_SOURCES[@]}"; then
  echo "Control UI browser smoke is missing composer popover menu item sampling" >&2
  exit 1
fi

for source in "$CONTROL_HTML_SOURCE" "$CONTROL_RUST_SOURCE"; do
  if ! grep -Fq 'id="composer-popover-artifact" data-chat-composer-popover="artifact" data-control-ui-composer-popover-panel="light-glass"' "$source"; then
    echo "Control UI source is missing the artifact composer popover light-glass panel: $source" >&2
    exit 1
  fi
  if ! grep -Fq 'id="composer-popover-command" data-chat-composer-popover="command" data-control-ui-composer-popover-panel="light-glass"' "$source"; then
    echo "Control UI source is missing the command composer popover light-glass panel: $source" >&2
    exit 1
  fi
  if ! grep -Fq 'data-control-ui-composer-popover-search="light-glass"' "$source"; then
    echo "Control UI source is missing composer popover search light-glass fields: $source" >&2
    exit 1
  fi
done

if ! grep -Fq 'data-chat-row-menu-panel="task-queue"' "$CONTROL_HTML_SOURCE"; then
  echo "Static Control UI is missing the Actions row action menu panel" >&2
  exit 1
fi

if ! grep -Fq 'data-chat-row-menu-panel="operator-plane"' "$CONTROL_HTML_SOURCE"; then
  echo "Static Control UI is missing the Evidence row action menu panel" >&2
  exit 1
fi

if ! grep -Fq 'data-chat-row-menu-panel="task-queue"' "$CONTROL_RUST_SOURCE"; then
  echo "Rust Control UI render is missing the Actions row action menu panel" >&2
  exit 1
fi

if ! grep -Fq 'data-chat-row-menu-panel="operator-plane"' "$CONTROL_RUST_SOURCE"; then
  echo "Rust Control UI render is missing the Evidence row action menu panel" >&2
  exit 1
fi

if ! grep -Fq 'document.querySelectorAll("[data-chat-conversation]")' "${CONTROL_BROWSER_SMOKE_SOURCES[@]}"; then
  echo "Control UI browser smoke is missing chat row option sampling" >&2
  exit 1
fi

if ! grep -Fq 'expectedVisibleChatRowOptionCount = railVisible ? 3 : 0' "${CONTROL_BROWSER_SMOKE_SOURCES[@]}"; then
  echo "Control UI browser smoke does not require desktop row options and hidden mobile row options" >&2
  exit 1
fi

if ! grep -Fq 'data-control-ui-command-palette-surface=\"light-glass\"' "${CONTROL_BROWSER_SMOKE_SOURCES[@]}"; then
  echo "Control UI browser smoke is missing command palette light-glass source marker sampling" >&2
  exit 1
fi

if ! grep -Fq 'data-control-ui-command-palette-trigger="light-glass"' "${CONTROL_BROWSER_SMOKE_SOURCES[@]}"; then
  echo "Control UI browser smoke is missing command palette trigger light-glass source marker sampling" >&2
  exit 1
fi

if ! grep -Fq 'data-control-ui-command-palette-input=\"light-glass\"' "${CONTROL_BROWSER_SMOKE_SOURCES[@]}"; then
  echo "Control UI browser smoke is missing command palette input light-glass source marker sampling" >&2
  exit 1
fi

if ! grep -Fq 'data-control-ui-command-palette-result=\"light-glass\"' "${CONTROL_BROWSER_SMOKE_SOURCES[@]}"; then
  echo "Control UI browser smoke is missing command palette result light-glass source marker sampling" >&2
  exit 1
fi

if ! grep -Fq 'data-control-ui-command-palette-input=light-glass' "${CONTROL_STYLE_SOURCES[@]}"; then
  echo "Control UI stylesheet is missing command palette input light-glass marker" >&2
  exit 1
fi

if ! grep -Fq 'data-control-ui-command-palette-result=light-glass' "${CONTROL_STYLE_SOURCES[@]}"; then
  echo "Control UI stylesheet is missing command palette result light-glass marker" >&2
  exit 1
fi

for source in "$CONTROL_HTML_SOURCE" "$CONTROL_RUST_SOURCE"; do
  if ! grep -Fq 'data-control-ui-command-palette-input="light-glass" type="search" placeholder="Search"' "$source"; then
    echo "Control UI source is missing command palette input light-glass marker: $source" >&2
    exit 1
  fi
  if ! grep -Fq 'data-control-ui-command-palette-result="light-glass"' "$source"; then
    echo "Control UI source is missing command palette result light-glass marker: $source" >&2
    exit 1
  fi
done

if ! grep -Fq 'document.querySelectorAll("[data-chat-row-menu-toggle]")' "${CONTROL_BROWSER_SMOKE_SOURCES[@]}"; then
  echo "Control UI browser smoke is missing row menu toggle sampling" >&2
  exit 1
fi

if ! grep -Fq 'document.querySelectorAll("[data-chat-row-menu-panel]")' "${CONTROL_BROWSER_SMOKE_SOURCES[@]}"; then
  echo "Control UI browser smoke is missing row menu panel sampling" >&2
  exit 1
fi

if ! grep -Fq 'document.querySelectorAll("[data-chat-row-menu-item]")' "${CONTROL_BROWSER_SMOKE_SOURCES[@]}"; then
  echo "Control UI browser smoke is missing row menu item sampling" >&2
  exit 1
fi

if ! grep -Fq 'active_state_matches_aria_pressed: active ? ariaPressed === "true" : ariaPressed === "false"' "${CONTROL_BROWSER_SMOKE_SOURCES[@]}"; then
  echo "Control UI browser smoke is missing folder chip aria-pressed state parity sampling" >&2
  exit 1
fi

if ! grep -Fq 'min-height:44px' "${CONTROL_STYLE_SOURCES[@]}"; then
  echo "Control UI stylesheet is missing 44px folder chip touch target protection" >&2
  exit 1
fi

if grep -Eq '\.tg-folder-chip[^}]*min-height:[[:space:]]*30px' "${CONTROL_STYLE_SOURCES[@]}"; then
  echo "Control UI folder chips still use compact 30px touch targets" >&2
  exit 1
fi

if ! grep -Fq '.tg-pin-toggle,.tg-row-menu-toggle{width:44px;height:44px' "${CONTROL_STYLE_SOURCES[@]}"; then
  echo "Control UI row menu toggle is missing 44px touch target protection" >&2
  exit 1
fi

if ! grep -Fq '.tg-row-action{display:inline-flex;align-items:center;min-height:44px' "${CONTROL_STYLE_SOURCES[@]}"; then
  echo "Control UI row action items are missing 44px submenu item protection" >&2
  exit 1
fi

if ! grep -Fq '.tg-row-action-popover{position:absolute;top:54px;right:10px' "${CONTROL_STYLE_SOURCES[@]}"; then
  echo "Control UI row action popover is missing light glass stylesheet marker" >&2
  exit 1
fi

if ! grep -Fq 'background:#f2fbff' "${CONTROL_STYLE_SOURCES[@]}"; then
  echo "Control UI row action popover is missing light glass background protection" >&2
  exit 1
fi

if ! grep -Fq 'data-control-ui-thread-tools-panel=light-glass' "${CONTROL_STYLE_SOURCES[@]}"; then
  echo "Control UI stylesheet is missing thread tools light glass marker" >&2
  exit 1
fi

if ! grep -Fq 'data-control-ui-composer-tools-panel=light-glass' "${CONTROL_STYLE_SOURCES[@]}"; then
  echo "Control UI stylesheet is missing composer tools light glass marker" >&2
  exit 1
fi

if ! grep -Fq '.tg-composer-popover{' "${CONTROL_STYLE_SOURCES[@]}"; then
  echo "Control UI stylesheet is missing composer popover light glass marker" >&2
  exit 1
fi

if ! grep -Fq '.tg-composer-popover__search{' "${CONTROL_STYLE_SOURCES[@]}"; then
  echo "Control UI stylesheet is missing composer popover search light glass marker" >&2
  exit 1
fi

if ! grep -Fq 'data-control-ui-icon-button="new-conversation" data-chat-add aria-label="New conversation" title="New conversation"' "$CONTROL_HTML_SOURCE"; then
  echo "Control UI static HTML is missing harsh coverage for the rail New conversation icon button" >&2
  exit 1
fi

if ! grep -Fq 'data-control-ui-icon-button="new-conversation" data-chat-add aria-label="New conversation" title="New conversation"' "$CONTROL_RUST_SOURCE"; then
  echo "Control UI Rust renderer is missing harsh coverage for the rail New conversation icon button" >&2
  exit 1
fi

if ! grep -Fq 'data-chat-conversation="ui-chat-agent" role="listitem" aria-current="true" tabindex="0" aria-label="Hepta conversation, local review ready" title="Hepta conversation, local review ready"' "$CONTROL_HTML_SOURCE"; then
  echo "Control UI static HTML is missing semantic title/aria coverage for the Hepta chat row option" >&2
  exit 1
fi

if ! grep -Fq 'data-chat-conversation="task-queue" role="listitem" tabindex="0" aria-label="Actions conversation, local approval queue" title="Actions conversation, local approval queue"' "$CONTROL_RUST_SOURCE"; then
  echo "Control UI Rust renderer is missing semantic title/aria coverage for the Actions chat row option" >&2
  exit 1
fi

if grep -Eq '\.tg-icon-action[^}]*width:[[:space:]]*3[0-9]px' "${CONTROL_STYLE_SOURCES[@]}"; then
  echo "Control UI rail icon action still uses sub-44px sizing" >&2
  exit 1
fi

if grep -Eq '<button type="button" data-secondary-action="[^"]+" data-secondary-action-role="[^"]+" aria-label="[^"]+"><span class="surface-action-icon"' "$FIXTURE_HTML"; then
  echo "native secondary action buttons lack title tooltips" >&2
  exit 1
fi

if grep -Eq '<button class="icon-btn"[^>]*aria-label="[^"]+"><svg' "$FIXTURE_HTML"; then
  echo "native composer icon buttons lack title tooltips" >&2
  exit 1
fi

if ! grep -Fq 'title="Jump to matching message"' "$FIXTURE_HTML"; then
  echo "native secondary search action tooltip is missing" >&2
  exit 1
fi

if ! grep -Fq 'title="Add attachment"' "$FIXTURE_HTML"; then
  echo "native composer attachment tooltip is missing" >&2
  exit 1
fi

if ! grep -Fq 'visibleTextIntegrityExpected = "safe status source is"' "${CONTROL_BROWSER_SMOKE_SOURCES[@]}"; then
  echo "Control UI browser smoke is missing visible-text integrity self-test" >&2
  exit 1
fi

if ! grep -Fq 'replace(/\\s+/g, " ").trim()' "${CONTROL_BROWSER_SMOKE_SOURCES[@]}"; then
  echo "Control UI browser smoke visibleText normalization is not safely escaped" >&2
  exit 1
fi

if grep -Eq 'overflow-wrap:[[:space:]]*anywhere|word-break:[[:space:]]*(break-word|break-all)' "$FIXTURE_HTML"; then
  echo "native fixture HTML still allows arbitrary word splitting in visible microcopy" >&2
  exit 1
fi

for source in "$CONTROL_HTML_SOURCE" "$CONTROL_RUST_SOURCE"; do
  if grep -Eq 'data-control-ui-icon-button[^>]*>[^<]*(＋|⌘|➤|⌁|⋯|⇅|↪)' "$source"; then
    echo "Control UI icon buttons still use visible text glyph placeholders: $source" >&2
    exit 1
  fi
  if grep -Eq 'class="tg-menu-(icon|item__icon)"[^>]*>[^<]*(＋|⌘|➤|⌁|⋯|⇅|↪|↺|☷|▣)' "$source"; then
    echo "Control UI menu icons still use visible text glyph placeholders: $source" >&2
    exit 1
  fi
  if ! grep -Eq '<svg class="hepta-svg-icon"[^>]*>' "$source"; then
    echo "Control UI source is missing inline SVG icon surfaces: $source" >&2
    exit 1
  fi
  if grep -Eq 'data-control-ui-icon-button="attach"[^>]*aria-label="Attach local context"[^>]*title="Attach"' "$source"; then
    echo "Control UI attach icon title does not match aria-label: $source" >&2
    exit 1
  fi
  if grep -Eq 'data-control-ui-icon-button="command"[^>]*aria-label="Insert command"[^>]*title="Command"' "$source"; then
    echo "Control UI command icon title does not match aria-label: $source" >&2
    exit 1
  fi
  if grep -Eq 'data-control-ui-icon-button="send"[^>]*aria-label="Send message"[^>]*title="Send"' "$source"; then
    echo "Control UI send icon title does not match aria-label: $source" >&2
    exit 1
  fi
  if grep -Eq 'data-control-ui-icon-button="plan"[^>]*aria-label="Plan next step"[^>]*title="Plan"' "$source"; then
    echo "Control UI plan icon title does not match aria-label: $source" >&2
    exit 1
  fi
  if grep -Eq 'data-control-ui-menu-trigger="icon"[^>]*aria-label="Open (thread|composer) tools"[^>]*title="(Thread|Composer) tools"' "$source"; then
    echo "Control UI menu trigger title does not match aria-label: $source" >&2
    exit 1
  fi
  if ! grep -Fq 'data-chat-folder="all" aria-pressed="true" aria-label="All chats" title="All chats"' "$source"; then
    echo "Control UI All folder chip is missing active state and full tooltip parity: $source" >&2
    exit 1
  fi
  if ! grep -Fq 'data-chat-folder="pinned" aria-pressed="false" aria-label="Pinned chats" title="Pinned chats"' "$source"; then
    echo "Control UI Pinned folder chip is missing inactive state and full tooltip parity: $source" >&2
    exit 1
  fi
  if ! grep -Fq 'data-chat-folder="archived" aria-pressed="false" aria-label="Archived chats" title="Archived chats"' "$source"; then
    echo "Control UI Archived folder chip is missing inactive state and full tooltip parity: $source" >&2
    exit 1
  fi
  if ! grep -Fq 'data-chat-row-menu-toggle="ui-chat-agent" aria-label="Open Hepta conversation actions" title="Open Hepta conversation actions"' "$source"; then
    echo "Control UI row menu toggle is missing full tooltip parity: $source" >&2
    exit 1
  fi
  if ! grep -Fq 'data-control-ui-row-menu-trigger="light-glass" data-chat-row-menu-toggle="ui-chat-agent"' "$source"; then
    echo "Control UI row menu toggle is missing light glass marker: $source" >&2
    exit 1
  fi
  if ! grep -Fq 'data-chat-row-menu-panel="ui-chat-agent" role="group" aria-label="Hepta conversation actions"' "$source"; then
    echo "Control UI row menu panel is missing submenu semantics: $source" >&2
    exit 1
  fi
  if ! grep -Fq 'data-control-ui-row-menu-panel="light-glass" data-chat-row-menu-panel="ui-chat-agent"' "$source"; then
    echo "Control UI row menu panel is missing light glass marker: $source" >&2
    exit 1
  fi
  if ! grep -Fq 'data-chat-row-menu-item="archive" aria-label="Archive Hepta conversation" title="Archive Hepta conversation"' "$source"; then
    echo "Control UI row action is missing native-control semantics and tooltip parity: $source" >&2
    exit 1
  fi
  if ! grep -Fq 'data-thread-command-menu="true"' "$source"; then
    echo "Control UI thread header is missing thread tools menu trigger: $source" >&2
    exit 1
  fi
  if ! grep -Fq 'data-control-ui-thread-tools-trigger="light-glass" aria-label="Open thread tools" title="Open thread tools"' "$source"; then
    echo "Control UI thread tools trigger is missing light-glass marker and tooltip parity: $source" >&2
    exit 1
  fi
  if ! grep -Fq 'data-control-ui-thread-tools-panel="light-glass" role="group" aria-label="Thread tools"' "$source"; then
    echo "Control UI thread tools panel is missing light-glass group semantics: $source" >&2
    exit 1
  fi
  if ! grep -Eq 'data-control-ui-menu-item="history"([^>]*autofocus)? href="#screen-card-transcript" aria-label="Open thread history" title="Open thread history"' "$source"; then
    echo "Control UI thread tools History item is missing native-link semantics and tooltip parity: $source" >&2
    exit 1
  fi
  if ! grep -Fq 'data-control-ui-menu-item="tasks" href="#screen-card-tasks" aria-label="Open thread tasks" title="Open thread tasks"' "$source"; then
    echo "Control UI thread tools Tasks item is missing native-link semantics and tooltip parity: $source" >&2
    exit 1
  fi
  if ! grep -Fq 'data-control-ui-menu-item="sessions" href="#screen-card-sessions" aria-label="Open thread sessions" title="Open thread sessions"' "$source"; then
    echo "Control UI thread tools Sessions item is missing native-link semantics and tooltip parity: $source" >&2
    exit 1
  fi
  if ! grep -Fq 'data-control-ui-composer-tools-trigger="light-glass" aria-label="Open composer tools" title="Open composer tools"' "$source"; then
    echo "Control UI composer tools trigger is missing light-glass tooltip parity: $source" >&2
    exit 1
  fi
  if ! grep -Fq 'data-control-ui-composer-tools-panel="light-glass" role="group" aria-label="Composer tools"' "$source"; then
    echo "Control UI composer tools panel is missing light-glass group semantics: $source" >&2
    exit 1
  fi
  if ! grep -Fq 'data-control-ui-composer-tool-item="reply-mode" data-control-ui-menu-item="reply-mode" aria-label="Set reply mode" title="Set reply mode"' "$source"; then
    echo "Control UI composer tools Reply item is missing native-label semantics and tooltip parity: $source" >&2
    exit 1
  fi
  if ! grep -Fq 'data-control-ui-composer-tool-item="scroll-mode" data-control-ui-menu-item="scroll-mode" data-chat-autoscroll-persisted="local-storage-contract" aria-label="Set auto-scroll mode" title="Set auto-scroll mode"' "$source"; then
    echo "Control UI composer tools Scroll item is missing native-label semantics and tooltip parity: $source" >&2
    exit 1
  fi
  if ! grep -Fq 'data-control-ui-command-palette-surface="light-glass"' "$source"; then
    echo "Control UI command palette is missing light-glass surface marker: $source" >&2
    exit 1
  fi
  if ! grep -Fq 'data-control-ui-command-palette-trigger="light-glass" popovertarget="command-palette" aria-haspopup="dialog" aria-controls="command-palette" aria-label="Open command palette" title="Open command palette"' "$source"; then
    echo "Control UI command palette trigger is missing light-glass marker and tooltip parity: $source" >&2
    exit 1
  fi
  if ! grep -Fq 'data-control-ui-command-palette-close="light-glass" popovertarget="command-palette" popovertargetaction="hide" aria-label="Close command palette" title="Close command palette"' "$source"; then
    echo "Control UI command palette close control is missing 2026 tooltip parity: $source" >&2
    exit 1
  fi
  if ! grep -Fq 'data-control-ui-command-palette-item' "$source"; then
    echo "Control UI command palette result is missing harsh item coverage: $source" >&2
    exit 1
  fi
  if ! grep -Fq 'aria-label="Open command result:' "$source" && ! grep -Fq 'aria-label=\"Open command result:' "$source"; then
    echo "Control UI command palette result is missing aria-label coverage: $source" >&2
    exit 1
  fi
  if ! grep -Fq 'title="Open command result:' "$source" && ! grep -Fq 'title=\"Open command result:' "$source"; then
    echo "Control UI command palette result is missing title coverage: $source" >&2
    exit 1
  fi
done

action_button_css="$(
  awk '
    /\.surface-actions[[:space:]]+:is\(button, span\)[[:space:]]*\{/ { capture=1 }
    capture { print }
    capture && /\}/ { exit }
  ' "$FIXTURE_HTML"
)"

if [[ -z "$action_button_css" ]]; then
  echo "native secondary action button CSS block was not found" >&2
  exit 1
fi

if grep -Eq 'overflow-wrap:[[:space:]]*anywhere' <<<"$action_button_css"; then
  echo "native secondary action labels may split inside words: .surface-actions uses overflow-wrap:anywhere" >&2
  exit 1
fi

label_nowrap_css="$(
  awk '
    /\.surface-actions[[:space:]]+button[[:space:]]+>[[:space:]]+span:not\(\.surface-action-icon\)[[:space:]]*\{/ { capture=1 }
    capture { print }
    capture && /\}/ { exit }
  ' "$FIXTURE_HTML"
)"

if ! grep -Eq 'white-space:[[:space:]]*nowrap' <<<"$label_nowrap_css"; then
  echo "native secondary action text labels lack nowrap protection" >&2
  exit 1
fi

if grep -Eq '<span class="surface-action-icon"[^>]*>[[:space:]]*([^<[:space:]]|OK|mic|-&gt;|--|\\[\\])' "$FIXTURE_HTML"; then
  echo "native secondary action icons still use visible text placeholders instead of SVG iconography" >&2
  exit 1
fi

if grep -Eq '<button class="icon-btn"[^>]*>[[:space:]]*([^<[:space:]]|mic|@|\\+)' "$FIXTURE_HTML"; then
  echo "native composer icon buttons still use visible text placeholders instead of SVG iconography" >&2
  exit 1
fi

if ! grep -Eq '<span class="surface-action-icon"[^>]*>[[:space:]]*<svg><use href="#icon-' "$FIXTURE_HTML"; then
  echo "native secondary action icons are missing SVG use references" >&2
  exit 1
fi

echo "Hepta UI harsh top-design referee check passed"
