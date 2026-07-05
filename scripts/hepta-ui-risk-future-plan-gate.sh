#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

READINESS_DIR="${HEPTA_UI_PRODUCT_READINESS_DIR:-/Users/qianqi/.openclaw/tmp/hepta-ui-product-readiness.mention-taxonomy-20260615}"
REPORT_PATH="${HEPTA_UI_RISK_FUTURE_PLAN_REPORT_PATH:-$READINESS_DIR/ui-risk-future-plan-gate.json}"
RISK_PLAN_DIR="${HEPTA_UI_RISK_FUTURE_PLAN_DIR:-$READINESS_DIR/risk-future-plan}"
RISK_PLAN_MARKDOWN_PATH="$RISK_PLAN_DIR/risk-future-plan.md"

TOP_DESIGN_REFEREE_REFRESH_REPORT_PATH="$READINESS_DIR/ui-top-design-referee-refresh-gate.json"
CURRENT_PLAN_REFRESH_REPORT_PATH="$READINESS_DIR/ui-current-plan-refresh-gate.json"
BLOCKER_CLOSURE_REPORT_PATH="$READINESS_DIR/ui-blocker-closure-gate.json"
BACKEND_DELIVERY_AUDIT_REPORT_PATH="$READINESS_DIR/ui-backend-delivery-audit-gate.json"
BACKEND_DELIVERY_RECEIPT_ROUNDTRIP_REPORT_PATH="$READINESS_DIR/ui-backend-delivery-receipt-roundtrip-gate.json"
EVIDENCE_ARCHIVE_REPORT_PATH="$READINESS_DIR/ui-evidence-archive-gate.json"
SCREENSHOT_MANIFEST_PATH="$READINESS_DIR/screenshot-manifest.json"

require_command() {
  if ! command -v "$1" >/dev/null 2>&1; then
    printf '%s is required for the Hepta UI risk/future-plan gate\n' "$1" >&2
    exit 2
  fi
}

require_report() {
  local path="$1"
  if [[ ! -s "$path" ]]; then
    printf 'Missing required risk/future-plan input: %s\n' "$path" >&2
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

require_report "$TOP_DESIGN_REFEREE_REFRESH_REPORT_PATH"
require_report "$CURRENT_PLAN_REFRESH_REPORT_PATH"
require_report "$BLOCKER_CLOSURE_REPORT_PATH"
require_report "$BACKEND_DELIVERY_AUDIT_REPORT_PATH"
require_report "$BACKEND_DELIVERY_RECEIPT_ROUNDTRIP_REPORT_PATH"
require_report "$EVIDENCE_ARCHIVE_REPORT_PATH"
require_report "$SCREENSHOT_MANIFEST_PATH"

rm -rf "$RISK_PLAN_DIR"
mkdir -p "$RISK_PLAN_DIR"

TMP_DIR="$(mktemp -d "${TMPDIR:-/tmp}/hepta-ui-risk-future-plan.XXXXXX")"
REPORT_DRAFT="$TMP_DIR/risk-future-plan-draft.json"
REPORT_TMP="$TMP_DIR/risk-future-plan-report.json"
MARKDOWN_TMP="$TMP_DIR/risk-future-plan.md"
trap 'rm -rf "$TMP_DIR"' EXIT

top_design_sha="$(file_sha256 "$TOP_DESIGN_REFEREE_REFRESH_REPORT_PATH")"
current_plan_sha="$(file_sha256 "$CURRENT_PLAN_REFRESH_REPORT_PATH")"
blocker_closure_sha="$(file_sha256 "$BLOCKER_CLOSURE_REPORT_PATH")"
backend_delivery_sha="$(file_sha256 "$BACKEND_DELIVERY_AUDIT_REPORT_PATH")"
backend_delivery_roundtrip_sha="$(file_sha256 "$BACKEND_DELIVERY_RECEIPT_ROUNDTRIP_REPORT_PATH")"
evidence_archive_sha="$(file_sha256 "$EVIDENCE_ARCHIVE_REPORT_PATH")"
screenshot_manifest_sha="$(file_sha256 "$SCREENSHOT_MANIFEST_PATH")"

jq -n \
  --arg product "Hepta UI" \
  --arg runtime "hepta" \
  --arg gate "hepta_ui_risk_future_plan_gate" \
  --arg readiness_dir "$READINESS_DIR" \
  --arg report_path "$REPORT_PATH" \
  --arg risk_plan_dir "$RISK_PLAN_DIR" \
  --arg risk_plan_markdown_path "$RISK_PLAN_MARKDOWN_PATH" \
  --arg top_design_path "$TOP_DESIGN_REFEREE_REFRESH_REPORT_PATH" \
  --arg current_plan_path "$CURRENT_PLAN_REFRESH_REPORT_PATH" \
  --arg blocker_closure_path "$BLOCKER_CLOSURE_REPORT_PATH" \
  --arg backend_delivery_path "$BACKEND_DELIVERY_AUDIT_REPORT_PATH" \
  --arg backend_delivery_roundtrip_path "$BACKEND_DELIVERY_RECEIPT_ROUNDTRIP_REPORT_PATH" \
  --arg evidence_archive_path "$EVIDENCE_ARCHIVE_REPORT_PATH" \
  --arg screenshot_manifest_path "$SCREENSHOT_MANIFEST_PATH" \
  --arg top_design_sha "$top_design_sha" \
  --arg current_plan_sha "$current_plan_sha" \
  --arg blocker_closure_sha "$blocker_closure_sha" \
  --arg backend_delivery_sha "$backend_delivery_sha" \
  --arg backend_delivery_roundtrip_sha "$backend_delivery_roundtrip_sha" \
  --arg evidence_archive_sha "$evidence_archive_sha" \
  --arg screenshot_manifest_sha "$screenshot_manifest_sha" \
  --slurpfile top_design_file "$TOP_DESIGN_REFEREE_REFRESH_REPORT_PATH" \
  --slurpfile current_plan_file "$CURRENT_PLAN_REFRESH_REPORT_PATH" \
  --slurpfile blocker_closure_file "$BLOCKER_CLOSURE_REPORT_PATH" \
  --slurpfile backend_delivery_file "$BACKEND_DELIVERY_AUDIT_REPORT_PATH" \
  --slurpfile backend_delivery_roundtrip_file "$BACKEND_DELIVERY_RECEIPT_ROUNDTRIP_REPORT_PATH" \
  --slurpfile evidence_archive_file "$EVIDENCE_ARCHIVE_REPORT_PATH" \
  --slurpfile screenshot_manifest_file "$SCREENSHOT_MANIFEST_PATH" \
  '
  ($top_design_file[0]) as $top
  | ($current_plan_file[0]) as $current
  | ($blocker_closure_file[0]) as $blocker
  | ($backend_delivery_file[0]) as $delivery
  | ($backend_delivery_roundtrip_file[0]) as $delivery_roundtrip
  | ($evidence_archive_file[0]) as $archive
  | ($screenshot_manifest_file[0]) as $manifest
  | def selected_ids: ["message_search","file_upload_send","media_download_playback","notifications","room_settings"];
    def current_plan_ids: ["r62_minimum_ui_demo_gate","backend_real_receipt_return","ui_refresh_after_real_receipt","release_artifact_roundtrip_and_signed_artifact_gate"];
    def latest_plan_ids: ["r151_harsh_top_design_v46_badge_micro_surface_light_glass_minimum_ui_demo_gate","backend_delivery_receipt_return","backend_real_receipt_return","ui_refresh_after_real_receipt","release_artifact_roundtrip_and_signed_artifact_gate"];
    def sha_ready($sha): ($sha | test("^[0-9a-f]{64}$"));
    def top_design_surface_ready:
      $top.top_design_harsh_2026_referee_ready == true
      and $top.control_ui_harsh_2026_ready == true
      and $top.native_secondary_harsh_action_matrix_ready == true
      and $top.current_standards_referee.apple_2026_menu_iconography_ready == true
      and $top.current_standards_referee.apple_2026_sidebar_scroll_search_ready == true
      and $top.current_standards_referee.harsh_control_microinteractions_ready == true
      and $top.referee_matrix.control_ui.harsh_2026_ready == true
      and $top.referee_matrix.control_ui.rail_action_icon_ready == true
      and $top.referee_matrix.control_ui.icon_buttons_ready == true
      and $top.referee_matrix.control_ui.icon_button_title_match_ready == true
      and $top.referee_matrix.control_ui.menu_triggers_ready == true
      and $top.referee_matrix.control_ui.menu_trigger_title_match_ready == true
      and $top.referee_matrix.control_ui.menu_item_icons_ready == true
	      and $top.referee_matrix.control_ui.navigation_icons_ready == true
	      and $top.referee_matrix.control_ui.scroll_edge_ready == true
	      and $top.referee_matrix.control_ui.microcopy_word_split_guard_ready == true
	      and $top.referee_matrix.control_ui.logo_clip_guard_ready == true
	      and $top.referee_matrix.control_ui.active_chat_readability_ready == true
	      and $top.referee_matrix.control_ui.folder_chip_touch_ready == true
		      and $top.referee_matrix.control_ui.row_menu_touch_ready == true
		      and $top.referee_matrix.control_ui.row_menu_all_rows_ready == true
		      and $top.referee_matrix.control_ui.row_menu_light_glass_ready == true
		      and $top.referee_matrix.control_ui.command_palette_ready == true
		      and $top.referee_matrix.control_ui.command_palette_surface_light_glass_ready == true
      and $top.referee_matrix.control_ui.command_palette_trigger_light_glass_ready == true
		      and $top.referee_matrix.control_ui.command_palette_input_light_glass_ready == true
	      and $top.referee_matrix.control_ui.command_palette_close_light_glass_ready == true
	      and $top.referee_matrix.control_ui.command_palette_item_light_glass_ready == true
		      and $top.referee_matrix.control_ui.chat_row_option_semantic_touch_ready == true
		      and $top.referee_matrix.control_ui.thread_tools_menu_ready == true
		      and $top.referee_matrix.control_ui.composer_tools_menu_ready == true
		      and $top.referee_matrix.control_ui.composer_popover_ready == true
		      and $top.referee_matrix.control_ui.composer_popover_search_light_glass_ready == true
		      and $top.referee_matrix.control_ui.rail_search_light_glass_ready == true
		      and $top.referee_matrix.control_ui.micro_surface_light_glass_ready == true
		      and $top.referee_matrix.control_ui.message_routing_badge_light_glass_ready == true
		      and $top.referee_matrix.control_ui.thread_intro_badge_light_glass_ready == true
		      and $top.referee_matrix.control_ui.status_trust_strip_light_glass_ready == true
		      and $top.referee_matrix.control_ui.visible_text_integrity_ready == true
	      and $top.referee_matrix.control_level.ready == true
      and $top.referee_matrix.control_level.requested_scope == "desktop_mobile_all_modules_buttons_submenus"
      and $top.referee_matrix.control_level.selected_row_variant_count == 18
      and $top.referee_matrix.control_level.secondary_surface_case_count == 15
      and $top.referee_matrix.control_level.secondary_surface_total_action_count == 57
      and $top.referee_matrix.control_level.secondary_surface_action_matrix_ready == true
      and $top.referee_matrix.control_level.secondary_surface_action_matrix_case_count == 15
      and $top.referee_matrix.control_level.secondary_surface_harsh_action_matrix_ready == true
      and $top.referee_matrix.control_level.secondary_surface_harsh_action_failure_count == 0
      and $top.referee_matrix.control_level.secondary_surface_title_tooltip_ready == true
      and $top.referee_matrix.control_level.secondary_surface_title_tooltip_failure_count == 0
      and $top.referee_matrix.control_level.true_window_submenu_coverage_ready == true
      and $top.referee_matrix.tempered_glass_2026.ready == true
      and $top.referee_matrix.tempered_glass_2026.aesthetic_standard == "2026_tempered_glass_liquid_glass"
      and $top.referee_matrix.tempered_glass_2026.secondary_surface_action_matrix_ready == true
	      and $top.referee_matrix.tempered_glass_2026.secondary_surface_harsh_action_matrix_ready == true
	      and $top.referee_matrix.tempered_glass_2026.secondary_surface_harsh_action_failure_count == 0
	      and $top.referee_matrix.tempered_glass_2026.native_secondary_title_tooltip_ready == true
	      and $top.referee_matrix.tempered_glass_2026.native_secondary_title_tooltip_failure_count == 0
	      and $top.referee_matrix.tempered_glass_2026.control_microcopy_word_split_guard_ready == true
	      and $top.referee_matrix.tempered_glass_2026.control_logo_clip_guard_ready == true
	      and $top.referee_matrix.tempered_glass_2026.control_active_chat_readability_ready == true
	      and $top.referee_matrix.tempered_glass_2026.control_placeholder_readability_ready == true
	      and $top.referee_matrix.tempered_glass_2026.control_small_control_readability_ready == true
	      and $top.referee_matrix.tempered_glass_2026.control_composer_tools_menu_ready == true
	      and $top.referee_matrix.tempered_glass_2026.control_command_palette_ready == true
	      and $top.referee_matrix.tempered_glass_2026.control_command_palette_surface_light_glass_ready == true
      and $top.referee_matrix.tempered_glass_2026.control_command_palette_trigger_light_glass_ready == true
	      and $top.referee_matrix.tempered_glass_2026.control_command_palette_input_light_glass_ready == true
	      and $top.referee_matrix.tempered_glass_2026.control_command_palette_close_light_glass_ready == true
	      and $top.referee_matrix.tempered_glass_2026.control_command_palette_item_light_glass_ready == true
	      and $top.referee_matrix.tempered_glass_2026.control_composer_popover_ready == true
	      and $top.referee_matrix.tempered_glass_2026.control_composer_popover_search_light_glass_ready == true
	      and $top.referee_matrix.tempered_glass_2026.control_rail_search_light_glass_ready == true
	      and $top.referee_matrix.tempered_glass_2026.control_micro_surface_light_glass_ready == true
	      and $top.referee_matrix.tempered_glass_2026.control_message_routing_badge_light_glass_ready == true
	      and $top.referee_matrix.tempered_glass_2026.control_thread_intro_badge_light_glass_ready == true
	      and $top.referee_matrix.tempered_glass_2026.control_status_trust_strip_light_glass_ready == true
	      and $top.referee_matrix.tempered_glass_2026.control_visible_text_integrity_ready == true
	      and $top.referee_matrix.tempered_glass_2026.clipping_failure_count == 0
      and $top.referee_matrix.tempered_glass_2026.min_contrast_ratio >= 4.5;
    def top_design_hard_ready:
      top_design_surface_ready
      and $top.true_window_evidence_mode == "full_hard_true_window"
      and $top.hard_true_window_evidence_ready == true
      and $top.referee_matrix.true_window_routes.hard_ready == true
      and $top.referee_matrix.true_window_routes.content_probe_ready == true
      and $top.referee_matrix.true_window_secondary_desktop.hard_ready == true
      and $top.referee_matrix.true_window_secondary_mobile.hard_ready == true
      and $top.referee_matrix.true_window_secondary_mobile.content_probe_ready == true
      and $top.referee_matrix.true_window_secondary_mobile.content_visible_count >= 10
      and $top.screenshot_manifest.hard_ready == true
      and $top.screenshot_manifest.counts.total >= 60
      and $manifest.screenshot_count.total >= 60;
    def top_design_no_window_companion_ready:
      top_design_surface_ready
      and $top.true_window_evidence_mode == "no_window_fixture"
      and $top.hard_true_window_evidence_ready == false
      and $top.screenshot_manifest.hard_ready == false
      and $top.screenshot_manifest.counts.total >= 44
      and $manifest.screenshot_count.total >= 44;
    def top_design_current_artifact_ready:
      top_design_hard_ready or top_design_no_window_companion_ready;
    def source_chain_ready:
      $top.top_design_referee_refresh_gate_ready == true
      and $top.status == "ready"
      and $top.refresh_version == 46
      and top_design_current_artifact_ready
      and $top.claim_boundary.desktop_mobile_design_claim_ready == true
      and $top.claim_boundary.live_product_claim_ready == false
      and $current.current_plan_refresh_gate_ready == true
      and $current.status == "ready"
      and $current.current_plan_ids == current_plan_ids
      and $current.current_minimum_gate.root_report_replay_required_count_after_current_plan_refresh == 41
      and ($current.source_alignment.real_backend_receipt_present | type) == "boolean"
      and $blocker.blocker_closure_gate_ready == true
      and $blocker.status == "ready"
      and ($blocker.critical_blocker_count >= 0 and $blocker.critical_blocker_count <= 9)
      and $blocker.closure_state.root_report_replay_required_count_after_blocker_closure == 41
      and $blocker.claim_boundary.live_product_claim_ready == false
      and $delivery.backend_delivery_audit_gate_ready == true
      and $delivery.status == "ready"
      and ($delivery.critical_blocker_count >= 0 and $delivery.critical_blocker_count <= 10)
      and $delivery.delivery_state.root_report_replay_required_count_after_delivery_audit == 41
      and (
        (
          $delivery.delivery_state.delivery_receipt_present == false
          and $delivery.delivery_state.delivery_receipt_valid == false
          and $delivery.delivery_state.waiting_for_delivery_receipt == true
          and $delivery.delivery_state.backend_delivery_claim_ready == false
        )
        or
        (
          $delivery.delivery_state.delivery_receipt_present == true
          and $delivery.delivery_state.delivery_receipt_valid == true
          and $delivery.delivery_state.waiting_for_delivery_receipt == false
          and $delivery.delivery_state.backend_delivery_claim_ready == true
        )
      )
      and ($delivery.delivery_state.real_backend_receipt_present | type) == "boolean"
      and ($delivery.delivery_state.backend_receipt_valid | type) == "boolean"
      and $delivery.delivery_state.selected_ids == selected_ids
      and ($delivery.source_alignment.blocker_closure_critical_blocker_count >= 0 and $delivery.source_alignment.blocker_closure_critical_blocker_count <= 9)
      and $delivery_roundtrip.backend_delivery_receipt_roundtrip_gate_ready == true
      and $delivery_roundtrip.status == "ready"
      and $delivery_roundtrip.roundtrip_kind == "local_backend_delivery_receipt_valid_branch_replay"
      and $delivery_roundtrip.selected_ids == selected_ids
      and $delivery_roundtrip.roundtrip_ready_count == 3
      and $delivery_roundtrip.source_alignment.waiting_branch_ready == true
      and $delivery_roundtrip.source_alignment.simulated_receipt_ready == true
      and $delivery_roundtrip.source_alignment.present_branch_ready == true
      and $delivery_roundtrip.source_alignment.present_branch_delivery_receipt_valid == true
      and $delivery_roundtrip.source_alignment.present_branch_backend_delivery_claim_ready == true
      and ($delivery_roundtrip.source_alignment.present_branch_real_backend_receipt_present | type) == "boolean"
      and ($delivery_roundtrip.source_alignment.present_branch_backend_receipt_valid | type) == "boolean"
      and $delivery_roundtrip.source_alignment.root_report_replay_required_count_after_roundtrip == 43
      and $delivery_roundtrip.claim_boundary.local_backend_delivery_receipt_roundtrip_ready == true
      and $delivery_roundtrip.claim_boundary.backend_delivery_claim_ready == false
      and $delivery_roundtrip.claim_boundary.real_backend_receipt_claim_ready == false
      and $delivery_roundtrip.claim_boundary.backend_receipt_claim_ready == false
      and $archive.evidence_archive_gate_ready == true
      and $archive.all_extracted_items_sha256_match == true
      and $manifest.screenshot_manifest_ready == true
      and $manifest.screenshot_count.control_ui == 4
      and $manifest.screenshot_count.native >= 40
      and sha_ready($top_design_sha)
      and sha_ready($current_plan_sha)
      and sha_ready($blocker_closure_sha)
      and sha_ready($backend_delivery_sha)
      and sha_ready($backend_delivery_roundtrip_sha)
      and sha_ready($evidence_archive_sha)
      and sha_ready($screenshot_manifest_sha);
    source_chain_ready as $ready
  | {
      product:$product,
      runtime:$runtime,
      gate:$gate,
      status:(if $ready then "ready" else "failed" end),
      risk_future_plan_gate_ready:$ready,
      plan_kind:"local_ui_post_r151_harsh_top_design_v46_badge_micro_surface_light_glass_risk_future_plan_refresh",
      plan_version:1,
      readiness_dir:$readiness_dir,
      report_path:$report_path,
      risk_plan_dir:$risk_plan_dir,
      risk_plan_markdown_path:$risk_plan_markdown_path,
      source_reports:{
        top_design_referee_refresh:$top_design_path,
        current_plan_refresh:$current_plan_path,
        blocker_closure:$blocker_closure_path,
        backend_delivery_audit:$backend_delivery_path,
        backend_delivery_receipt_roundtrip:$backend_delivery_roundtrip_path,
        evidence_archive:$evidence_archive_path,
        screenshot_manifest:$screenshot_manifest_path
      },
      source_report_sha256:{
        top_design_referee_refresh:$top_design_sha,
        current_plan_refresh:$current_plan_sha,
        blocker_closure:$blocker_closure_sha,
        backend_delivery_audit:$backend_delivery_sha,
        backend_delivery_receipt_roundtrip:$backend_delivery_roundtrip_sha,
        evidence_archive:$evidence_archive_sha,
        screenshot_manifest:$screenshot_manifest_sha
      },
      latest_minimum_gate:{
        gate_id:"r151_harsh_top_design_v46_badge_micro_surface_light_glass_minimum_ui_demo_gate",
        current_artifact_evidence_ready:top_design_current_artifact_ready,
        current_evidence_mode:$top.true_window_evidence_mode,
        current_full_hard_evidence_ready:top_design_hard_ready,
        no_window_companion_ready:top_design_no_window_companion_ready,
        top_design_refresh_version:$top.refresh_version,
	        top_design_harsh_2026_referee_ready:$top.top_design_harsh_2026_referee_ready,
	        control_ui_harsh_2026_ready:$top.control_ui_harsh_2026_ready,
	        control_ui_rail_action_icon_ready:$top.referee_matrix.control_ui.rail_action_icon_ready,
	        control_ui_microcopy_word_split_guard_ready:$top.referee_matrix.control_ui.microcopy_word_split_guard_ready,
	        control_ui_logo_clip_guard_ready:$top.referee_matrix.control_ui.logo_clip_guard_ready,
	        control_ui_active_chat_readability_ready:$top.referee_matrix.control_ui.active_chat_readability_ready,
	        control_ui_placeholder_readability_ready:$top.referee_matrix.control_ui.placeholder_readability_ready,
	        control_ui_small_control_readability_ready:$top.referee_matrix.control_ui.small_control_readability_ready,
	        control_ui_folder_chip_touch_ready:$top.referee_matrix.control_ui.folder_chip_touch_ready,
	        control_ui_row_menu_touch_ready:$top.referee_matrix.control_ui.row_menu_touch_ready,
	        control_ui_row_menu_all_rows_ready:$top.referee_matrix.control_ui.row_menu_all_rows_ready,
	        control_ui_row_menu_light_glass_ready:$top.referee_matrix.control_ui.row_menu_light_glass_ready,
		        control_ui_command_palette_ready:$top.referee_matrix.control_ui.command_palette_ready,
		        control_ui_command_palette_surface_light_glass_ready:$top.referee_matrix.control_ui.command_palette_surface_light_glass_ready,
		        control_ui_command_palette_trigger_light_glass_ready:$top.referee_matrix.control_ui.command_palette_trigger_light_glass_ready,
		        control_ui_command_palette_close_light_glass_ready:$top.referee_matrix.control_ui.command_palette_close_light_glass_ready,
		        control_ui_command_palette_input_light_glass_ready:$top.referee_matrix.control_ui.command_palette_input_light_glass_ready,
		        control_ui_command_palette_item_light_glass_ready:$top.referee_matrix.control_ui.command_palette_item_light_glass_ready,
		        control_ui_form_control_title_touch_ready:$top.referee_matrix.control_ui.form_control_title_touch_ready,
		        control_ui_chat_row_option_semantic_touch_ready:$top.referee_matrix.control_ui.chat_row_option_semantic_touch_ready,
		        control_ui_thread_tools_menu_ready:$top.referee_matrix.control_ui.thread_tools_menu_ready,
		        control_ui_composer_tools_menu_ready:$top.referee_matrix.control_ui.composer_tools_menu_ready,
		        control_ui_composer_popover_ready:$top.referee_matrix.control_ui.composer_popover_ready,
		        control_ui_composer_popover_search_light_glass_ready:$top.referee_matrix.control_ui.composer_popover_search_light_glass_ready,
		        control_ui_rail_search_light_glass_ready:$top.referee_matrix.control_ui.rail_search_light_glass_ready,
		        control_ui_micro_surface_light_glass_ready:$top.referee_matrix.control_ui.micro_surface_light_glass_ready,
		        control_ui_message_routing_badge_light_glass_ready:$top.referee_matrix.control_ui.message_routing_badge_light_glass_ready,
		        control_ui_thread_intro_badge_light_glass_ready:$top.referee_matrix.control_ui.thread_intro_badge_light_glass_ready,
		        control_ui_status_trust_strip_light_glass_ready:$top.referee_matrix.control_ui.status_trust_strip_light_glass_ready,
		        control_ui_visible_text_integrity_ready:$top.referee_matrix.control_ui.visible_text_integrity_ready,
	        control_ui_icon_button_title_match_ready:$top.referee_matrix.control_ui.icon_button_title_match_ready,
	        control_ui_menu_trigger_title_match_ready:$top.referee_matrix.control_ui.menu_trigger_title_match_ready,
	        native_secondary_harsh_action_matrix_ready:$top.native_secondary_harsh_action_matrix_ready,
	        native_secondary_title_tooltip_ready:$top.referee_matrix.control_level.secondary_surface_title_tooltip_ready,
	        native_secondary_title_tooltip_failure_count:$top.referee_matrix.control_level.secondary_surface_title_tooltip_failure_count,
        requested_scope:$top.referee_matrix.control_level.requested_scope,
        root_report_replay_required_count_after_risk_future_plan:43,
        current_plan_root_report_required_count:$current.current_minimum_gate.root_report_replay_required_count_after_current_plan_refresh,
        top_design_control_phone320_ready:$top.referee_matrix.control_ui.persisted_phone320_screenshot_ready,
        selected_row_variant_count:$top.referee_matrix.control_level.selected_row_variant_count,
        secondary_surface_case_count:$top.referee_matrix.control_level.secondary_surface_case_count,
        secondary_surface_total_action_count:$top.referee_matrix.control_level.secondary_surface_total_action_count,
        secondary_surface_action_matrix_ready:$top.referee_matrix.control_level.secondary_surface_action_matrix_ready,
        secondary_surface_action_matrix_case_count:$top.referee_matrix.control_level.secondary_surface_action_matrix_case_count,
        secondary_surface_harsh_action_matrix_ready:$top.referee_matrix.control_level.secondary_surface_harsh_action_matrix_ready,
        secondary_surface_harsh_action_failure_count:$top.referee_matrix.control_level.secondary_surface_harsh_action_failure_count,
        secondary_surface_title_tooltip_ready:$top.referee_matrix.control_level.secondary_surface_title_tooltip_ready,
        secondary_surface_title_tooltip_failure_count:$top.referee_matrix.control_level.secondary_surface_title_tooltip_failure_count,
        true_window_submenu_coverage_ready:$top.referee_matrix.control_level.true_window_submenu_coverage_ready,
        tempered_glass_2026_ready:$top.referee_matrix.tempered_glass_2026.ready,
        tempered_glass_min_contrast_ratio:$top.referee_matrix.tempered_glass_2026.min_contrast_ratio,
        tempered_glass_clipping_failure_count:$top.referee_matrix.tempered_glass_2026.clipping_failure_count,
        screenshot_counts:$manifest.screenshot_count
      },
      latest_plan_ids:latest_plan_ids,
      latest_plan:[
        {
	          priority:1,
	          id:"r151_harsh_top_design_v46_badge_micro_surface_light_glass_minimum_ui_demo_gate",
	          owner_lane:"hepta-ui",
	          action:"keep the r150 harsh top-design v45 badge micro-surface light-glass desktop/mobile baseline as the minimum demo gate",
	          required_evidence:["top_design_refresh_version_46","2026_tempered_glass_liquid_glass","desktop_mobile_all_modules_buttons_submenus","control_ui_harsh_icon_menu_scroll_logo_microcopy_thread_header_chat_row_placeholder_control_folder_chip_all_row_menus_light_glass_composer_popover_light_glass_composer_tools_composer_popover_search_thread_intro_badge_rail_search_command_palette_surface_light_glass_command_palette_trigger_command_palette_close_micro_surface_message_routing_badge_readability_menu_viewport_native_tooltip_text_integrity_matrix","secondary_harsh_action_matrix_15_cases_57_icon_role_tooltip_buttons","control_ui_4_viewports","native_fixture_40","true_window_16","backend_delivery_receipt_valid_branch_roundtrip","root_report_replay_46_after_risk_future_plan"],
          current_artifact_evidence_ready:top_design_current_artifact_ready,
          current_evidence_mode:$top.true_window_evidence_mode,
          current_full_hard_evidence_ready:top_design_hard_ready
        },
        {
          priority:2,
          id:"backend_delivery_receipt_return",
          owner_lane:"backend_contract",
          action:(if $delivery.delivery_state.waiting_for_delivery_receipt then "deliver the backend dispatch packet to the backend lane and capture a delivery receipt before claiming backend handoff delivery" else "keep the accepted backend delivery receipt bound to the dispatch archive" end),
          selected_ids:selected_ids,
          dispatch_archive_sha256:$delivery.delivery_state.dispatch_archive_sha256,
          waiting_for_delivery_receipt:$delivery.delivery_state.waiting_for_delivery_receipt
        },
        {
          priority:3,
          id:"backend_real_receipt_return",
          owner_lane:"backend_contract",
          action:(if $delivery.claim_boundary.backend_receipt_claim_ready then "keep the accepted backend receipt bound to the dispatch archive and completed full-hard UI refresh" else "execute the first five backend dispatch items and return a real backend receipt bound to the dispatch archive" end),
          selected_ids:selected_ids,
          target_repo:$delivery.delivery_state.target_backend_repo
        },
        {
          priority:4,
          id:"ui_refresh_after_real_receipt",
          owner_lane:"hepta-ui",
          action:"rerun no-window and full-hard readiness with the real backend receipt before claiming backend receipt acceptance",
          required_commands:$current.current_plan[2].required_commands
        },
        {
          priority:5,
          id:"release_artifact_roundtrip_and_signed_artifact_gate",
          owner_lane:"release_operator",
          action:"record release approval and a real signed/notarized/stapled artifact, then refresh UI readiness before any public distribution claim",
          blockers:[
            (if $blocker.closure_state.release_approval_valid then empty else "operator_release_approval_required" end),
            (if $blocker.closure_state.release_artifact_valid then empty else "signed_notarized_stapled_artifact_missing" end),
            (if $blocker.closure_state.public_distribution_artifact_written then empty else "public_distribution_artifact_not_written" end),
            (if $delivery.delivery_state.real_backend_receipt_present then empty else "real_backend_receipt_missing" end)
          ]
        }
      ],
      latest_plan_count:5,
      critical_blockers:$delivery.critical_blockers,
      critical_blocker_count:$delivery.critical_blocker_count,
      next_unblock_sequence:$delivery.next_unblock_sequence,
      source_alignment:{
        top_design_referee_refresh_ready:$top.top_design_referee_refresh_gate_ready,
        top_design_refresh_version:$top.refresh_version,
        top_design_requested_scope:$top.referee_matrix.control_level.requested_scope,
        tempered_glass_2026_ready:$top.referee_matrix.tempered_glass_2026.ready,
	        tempered_glass_min_contrast_ratio:$top.referee_matrix.tempered_glass_2026.min_contrast_ratio,
	        tempered_glass_clipping_failure_count:$top.referee_matrix.tempered_glass_2026.clipping_failure_count,
	        control_ui_rail_action_icon_ready:$top.referee_matrix.control_ui.rail_action_icon_ready,
	        control_ui_active_chat_readability_ready:$top.referee_matrix.control_ui.active_chat_readability_ready,
	        control_ui_placeholder_readability_ready:$top.referee_matrix.control_ui.placeholder_readability_ready,
	        control_ui_small_control_readability_ready:$top.referee_matrix.control_ui.small_control_readability_ready,
	        control_ui_folder_chip_touch_ready:$top.referee_matrix.control_ui.folder_chip_touch_ready,
	        control_ui_row_menu_touch_ready:$top.referee_matrix.control_ui.row_menu_touch_ready,
	        control_ui_row_menu_all_rows_ready:$top.referee_matrix.control_ui.row_menu_all_rows_ready,
		        control_ui_row_menu_light_glass_ready:$top.referee_matrix.control_ui.row_menu_light_glass_ready,
		        control_ui_command_palette_ready:$top.referee_matrix.control_ui.command_palette_ready,
		        control_ui_command_palette_surface_light_glass_ready:$top.referee_matrix.control_ui.command_palette_surface_light_glass_ready,
		        control_ui_command_palette_trigger_light_glass_ready:$top.referee_matrix.control_ui.command_palette_trigger_light_glass_ready,
		        control_ui_command_palette_close_light_glass_ready:$top.referee_matrix.control_ui.command_palette_close_light_glass_ready,
		        control_ui_command_palette_input_light_glass_ready:$top.referee_matrix.control_ui.command_palette_input_light_glass_ready,
		        control_ui_command_palette_item_light_glass_ready:$top.referee_matrix.control_ui.command_palette_item_light_glass_ready,
		        control_ui_form_control_title_touch_ready:$top.referee_matrix.control_ui.form_control_title_touch_ready,
		        control_ui_chat_row_option_semantic_touch_ready:$top.referee_matrix.control_ui.chat_row_option_semantic_touch_ready,
		        control_ui_thread_tools_menu_ready:$top.referee_matrix.control_ui.thread_tools_menu_ready,
		        control_ui_composer_tools_menu_ready:$top.referee_matrix.control_ui.composer_tools_menu_ready,
		        control_ui_composer_popover_ready:$top.referee_matrix.control_ui.composer_popover_ready,
		        control_ui_composer_popover_search_light_glass_ready:$top.referee_matrix.control_ui.composer_popover_search_light_glass_ready,
		        control_ui_rail_search_light_glass_ready:$top.referee_matrix.control_ui.rail_search_light_glass_ready,
		        control_ui_micro_surface_light_glass_ready:$top.referee_matrix.control_ui.micro_surface_light_glass_ready,
		        control_ui_message_routing_badge_light_glass_ready:$top.referee_matrix.control_ui.message_routing_badge_light_glass_ready,
		        control_ui_thread_intro_badge_light_glass_ready:$top.referee_matrix.control_ui.thread_intro_badge_light_glass_ready,
		        control_ui_status_trust_strip_light_glass_ready:$top.referee_matrix.control_ui.status_trust_strip_light_glass_ready,
		        control_ui_visible_text_integrity_ready:$top.referee_matrix.control_ui.visible_text_integrity_ready,
	        control_ui_icon_button_title_match_ready:$top.referee_matrix.control_ui.icon_button_title_match_ready,
	        control_ui_menu_trigger_title_match_ready:$top.referee_matrix.control_ui.menu_trigger_title_match_ready,
	        native_secondary_title_tooltip_ready:$top.referee_matrix.control_level.secondary_surface_title_tooltip_ready,
	        native_secondary_title_tooltip_failure_count:$top.referee_matrix.control_level.secondary_surface_title_tooltip_failure_count,
	        secondary_surface_action_matrix_ready:$top.referee_matrix.control_level.secondary_surface_action_matrix_ready,
        secondary_surface_action_matrix_case_count:$top.referee_matrix.control_level.secondary_surface_action_matrix_case_count,
        top_design_current_artifact_evidence_ready:top_design_current_artifact_ready,
        top_design_current_evidence_mode:$top.true_window_evidence_mode,
        top_design_full_hard_evidence_ready:top_design_hard_ready,
        top_design_no_window_companion_ready:top_design_no_window_companion_ready,
        current_plan_refresh_ready:$current.current_plan_refresh_gate_ready,
        current_plan_ids:$current.current_plan_ids,
        blocker_closure_ready:$blocker.blocker_closure_gate_ready,
        blocker_closure_critical_blocker_count:$blocker.critical_blocker_count,
        backend_delivery_audit_ready:$delivery.backend_delivery_audit_gate_ready,
        backend_delivery_audit_critical_blocker_count:$delivery.critical_blocker_count,
        backend_delivery_receipt_present:$delivery.delivery_state.delivery_receipt_present,
        backend_delivery_receipt_valid:$delivery.delivery_state.delivery_receipt_valid,
        backend_delivery_receipt_roundtrip_ready:$delivery_roundtrip.backend_delivery_receipt_roundtrip_gate_ready,
        backend_delivery_receipt_roundtrip_present_branch_ready:$delivery_roundtrip.source_alignment.present_branch_ready,
        backend_delivery_receipt_roundtrip_present_branch_valid:$delivery_roundtrip.source_alignment.present_branch_delivery_receipt_valid,
        backend_delivery_receipt_roundtrip_root_report_required_count:$delivery_roundtrip.source_alignment.root_report_replay_required_count_after_roundtrip,
        real_backend_receipt_present:$delivery.delivery_state.real_backend_receipt_present,
        backend_receipt_valid:$delivery.delivery_state.backend_receipt_valid,
        evidence_archive_ready:$archive.evidence_archive_gate_ready,
        screenshot_manifest_ready:$manifest.screenshot_manifest_ready,
        selected_ids_match:($delivery.delivery_state.selected_ids == selected_ids),
        root_report_replay_required_count_after_risk_future_plan:43
      },
      claim_boundary:{
        local_risk_future_plan_ready:$ready,
        local_top_design_referee_refresh_ready:$top.claim_boundary.local_top_design_referee_refresh_ready,
        local_current_plan_refresh_ready:$current.claim_boundary.local_current_plan_refresh_ready,
        local_blocker_closure_ready:$blocker.claim_boundary.local_blocker_closure_ready,
        local_backend_delivery_audit_ready:$delivery.claim_boundary.local_backend_delivery_audit_ready,
        local_backend_delivery_receipt_roundtrip_ready:$delivery_roundtrip.claim_boundary.local_backend_delivery_receipt_roundtrip_ready,
        desktop_mobile_design_claim_ready:$top.claim_boundary.desktop_mobile_design_claim_ready,
        backend_delivery_claim_ready:$delivery.claim_boundary.backend_delivery_claim_ready,
        real_backend_receipt_claim_ready:$delivery.claim_boundary.real_backend_receipt_claim_ready,
        backend_receipt_claim_ready:$delivery.claim_boundary.backend_receipt_claim_ready,
        backend_adapter_promoted:false,
        live_runtime_mutation:false,
        live_product_claim_ready:false,
        public_distribution_claim_ready:false,
        release_claim_ready:false,
        external_actions_allowed:false,
        public_upload_performed:false,
        signing_notarization_performed:false
      },
      side_effects:{
        filesystem_read:true,
        local_markdown_written:true,
        local_report_written:true,
        backend_agent_spawned:false,
        backend_repo_write:false,
        matrix_login:false,
        gateway_call:false,
        provider_invoked:false,
        channel_delivery:false,
        credential_value_read:false,
        notary_submission_performed:false,
        app_signed:false,
        app_notarized:false,
        app_stapled:false,
        public_distribution_artifact_written:false,
        external_mutation:false
      }
    }' >"$REPORT_DRAFT"

jq -r '
  def plan_lines:
    if (.latest_plan | length) == 0 then
      "- None\n"
    else
      (.latest_plan | map("- P\(.priority) `\(.id)` (\(.owner_lane)): \(.action)\n") | join(""))
    end;
  def blocker_lines:
    if (.critical_blockers | length) == 0 then
      "- None\n"
    else
      (.critical_blockers | map("- `\(.id)` (\(.owner_lane)): \(.state)\n") | join(""))
    end;
  def next_lines:
    if (.next_unblock_sequence | length) == 0 then
      "- None\n"
    else
      (.next_unblock_sequence | map("- \(.)\n") | join(""))
    end;
  "# Hepta UI Risk / Future Plan\n\n"
  + "- Status: \(.status)\n"
  + "- Latest minimum gate: \(.latest_minimum_gate.gate_id)\n"
  + "- Top-design refresh version: \(.latest_minimum_gate.top_design_refresh_version)\n"
  + "- Current evidence mode: \(.latest_minimum_gate.current_evidence_mode)\n"
  + "- Current artifact evidence ready: \(.latest_minimum_gate.current_artifact_evidence_ready)\n"
  + "- Current full-hard evidence ready: \(.latest_minimum_gate.current_full_hard_evidence_ready)\n"
  + "- Requested scope: \(.latest_minimum_gate.requested_scope)\n"
  + "- Critical blockers: \(.critical_blocker_count)\n"
  + "- Root replay required after this gate: \(.latest_minimum_gate.root_report_replay_required_count_after_risk_future_plan)\n\n"
  + "## Latest Plan\n\n"
  + plan_lines
  + "\n## Critical Blockers\n\n"
  + blocker_lines
  + "\n## Next Unblock Sequence\n\n"
  + next_lines
' "$REPORT_DRAFT" >"$MARKDOWN_TMP"

markdown_sha="$(file_sha256 "$MARKDOWN_TMP")"
markdown_bytes="$(file_bytes "$MARKDOWN_TMP")"

jq \
  --arg markdown_sha "$markdown_sha" \
  --argjson markdown_bytes "$markdown_bytes" \
  '. + {risk_plan_markdown_sha256:$markdown_sha, risk_plan_markdown_bytes:$markdown_bytes}' \
  "$REPORT_DRAFT" >"$REPORT_TMP"

if [[ "${HEPTA_UI_RISK_FUTURE_PLAN_DEBUG_COPY:-0}" == "1" ]]; then
  cp "$REPORT_TMP" "$REPORT_PATH.debug"
fi

jq -e '
  .status == "ready"
  and .risk_future_plan_gate_ready == true
  and .plan_kind == "local_ui_post_r151_harsh_top_design_v46_badge_micro_surface_light_glass_risk_future_plan_refresh"
  and .plan_version == 1
  and .latest_minimum_gate.gate_id == "r151_harsh_top_design_v46_badge_micro_surface_light_glass_minimum_ui_demo_gate"
  and .latest_minimum_gate.current_artifact_evidence_ready == true
  and (.latest_minimum_gate.current_evidence_mode == "full_hard_true_window" or .latest_minimum_gate.current_evidence_mode == "no_window_fixture")
  and .latest_minimum_gate.top_design_refresh_version == 46
	  and .latest_minimum_gate.top_design_harsh_2026_referee_ready == true
	  and .latest_minimum_gate.control_ui_harsh_2026_ready == true
	  and .latest_minimum_gate.control_ui_rail_action_icon_ready == true
	  and .latest_minimum_gate.control_ui_microcopy_word_split_guard_ready == true
	  and .latest_minimum_gate.control_ui_logo_clip_guard_ready == true
	  and .latest_minimum_gate.control_ui_active_chat_readability_ready == true
	  and .latest_minimum_gate.control_ui_placeholder_readability_ready == true
	  and .latest_minimum_gate.control_ui_small_control_readability_ready == true
	  and .latest_minimum_gate.control_ui_folder_chip_touch_ready == true
	  and .latest_minimum_gate.control_ui_row_menu_touch_ready == true
	  and .latest_minimum_gate.control_ui_row_menu_all_rows_ready == true
	  and .latest_minimum_gate.control_ui_row_menu_light_glass_ready == true
	  and .latest_minimum_gate.control_ui_command_palette_ready == true
		  and .latest_minimum_gate.control_ui_command_palette_surface_light_glass_ready == true
		  and .latest_minimum_gate.control_ui_command_palette_trigger_light_glass_ready == true
		  and .latest_minimum_gate.control_ui_command_palette_close_light_glass_ready == true
	  and .latest_minimum_gate.control_ui_form_control_title_touch_ready == true
	  and .latest_minimum_gate.control_ui_chat_row_option_semantic_touch_ready == true
	  and .latest_minimum_gate.control_ui_thread_tools_menu_ready == true
	  and .latest_minimum_gate.control_ui_composer_tools_menu_ready == true
	  and .latest_minimum_gate.control_ui_composer_popover_ready == true
	  and .latest_minimum_gate.control_ui_composer_popover_search_light_glass_ready == true
	  and .latest_minimum_gate.control_ui_rail_search_light_glass_ready == true
	  and .latest_minimum_gate.control_ui_micro_surface_light_glass_ready == true
	  and .latest_minimum_gate.control_ui_message_routing_badge_light_glass_ready == true
	  and .latest_minimum_gate.control_ui_status_trust_strip_light_glass_ready == true
	  and .latest_minimum_gate.control_ui_visible_text_integrity_ready == true
	  and .latest_minimum_gate.control_ui_icon_button_title_match_ready == true
	  and .latest_minimum_gate.control_ui_menu_trigger_title_match_ready == true
	  and .latest_minimum_gate.native_secondary_harsh_action_matrix_ready == true
	  and .latest_minimum_gate.native_secondary_title_tooltip_ready == true
	  and .latest_minimum_gate.native_secondary_title_tooltip_failure_count == 0
  and .latest_minimum_gate.requested_scope == "desktop_mobile_all_modules_buttons_submenus"
  and .latest_minimum_gate.tempered_glass_2026_ready == true
  and .latest_minimum_gate.tempered_glass_min_contrast_ratio >= 4.5
  and .latest_minimum_gate.tempered_glass_clipping_failure_count == 0
  and .latest_minimum_gate.root_report_replay_required_count_after_risk_future_plan == 43
  and .latest_minimum_gate.current_plan_root_report_required_count == 41
  and .latest_minimum_gate.selected_row_variant_count == 18
  and .latest_minimum_gate.secondary_surface_case_count == 15
  and .latest_minimum_gate.secondary_surface_total_action_count == 57
  and .latest_minimum_gate.secondary_surface_action_matrix_ready == true
  and .latest_minimum_gate.secondary_surface_action_matrix_case_count == 15
  and .latest_minimum_gate.secondary_surface_harsh_action_matrix_ready == true
  and .latest_minimum_gate.secondary_surface_harsh_action_failure_count == 0
  and .latest_minimum_gate.secondary_surface_title_tooltip_ready == true
  and .latest_minimum_gate.secondary_surface_title_tooltip_failure_count == 0
  and .latest_minimum_gate.true_window_submenu_coverage_ready == true
  and .latest_plan_count == 5
  and .latest_plan_ids == ["r151_harsh_top_design_v46_badge_micro_surface_light_glass_minimum_ui_demo_gate","backend_delivery_receipt_return","backend_real_receipt_return","ui_refresh_after_real_receipt","release_artifact_roundtrip_and_signed_artifact_gate"]
  and (.critical_blockers | length) >= .critical_blocker_count
  and (.critical_blockers | length) <= (.critical_blocker_count + 1)
  and (.critical_blocker_count >= 0 and .critical_blocker_count <= 10)
  and (
    (
      .source_alignment.backend_delivery_receipt_present == false
      and .source_alignment.backend_delivery_receipt_valid == false
      and (.critical_blockers | map(.id) | index("backend_dispatch_delivery_receipt_missing")) != null
      and .claim_boundary.backend_delivery_claim_ready == false
    )
    or
    (
      .source_alignment.backend_delivery_receipt_present == true
      and .source_alignment.backend_delivery_receipt_valid == true
      and .claim_boundary.backend_delivery_claim_ready == true
    )
  )
  and (.next_unblock_sequence | length) >= 1
  and .source_alignment.top_design_referee_refresh_ready == true
  and .source_alignment.tempered_glass_2026_ready == true
  and .source_alignment.tempered_glass_min_contrast_ratio >= 4.5
  and .source_alignment.tempered_glass_clipping_failure_count == 0
  and .source_alignment.control_ui_rail_action_icon_ready == true
  and .source_alignment.control_ui_folder_chip_touch_ready == true
  and .source_alignment.control_ui_row_menu_touch_ready == true
  and .source_alignment.control_ui_row_menu_all_rows_ready == true
  and .source_alignment.control_ui_row_menu_light_glass_ready == true
	  and .source_alignment.control_ui_command_palette_ready == true
	  and .source_alignment.control_ui_command_palette_surface_light_glass_ready == true
	  and .source_alignment.control_ui_command_palette_trigger_light_glass_ready == true
	  and .source_alignment.control_ui_command_palette_close_light_glass_ready == true
	  and .source_alignment.control_ui_form_control_title_touch_ready == true
	  and .source_alignment.control_ui_chat_row_option_semantic_touch_ready == true
	  and .source_alignment.control_ui_thread_tools_menu_ready == true
	  and .source_alignment.control_ui_composer_tools_menu_ready == true
	  and .source_alignment.control_ui_composer_popover_ready == true
	  and .source_alignment.control_ui_composer_popover_search_light_glass_ready == true
	  and .source_alignment.control_ui_command_palette_input_light_glass_ready == true
	  and .source_alignment.control_ui_command_palette_item_light_glass_ready == true
	  and .source_alignment.control_ui_micro_surface_light_glass_ready == true
	  and .source_alignment.control_ui_message_routing_badge_light_glass_ready == true
	  and .source_alignment.control_ui_status_trust_strip_light_glass_ready == true
		  and .latest_minimum_gate.control_ui_command_palette_ready == true
		  and .latest_minimum_gate.control_ui_command_palette_surface_light_glass_ready == true
		  and .latest_minimum_gate.control_ui_command_palette_trigger_light_glass_ready == true
		  and .latest_minimum_gate.control_ui_command_palette_close_light_glass_ready == true
		  and .latest_minimum_gate.control_ui_command_palette_input_light_glass_ready == true
		  and .latest_minimum_gate.control_ui_command_palette_item_light_glass_ready == true
		  and .latest_minimum_gate.control_ui_form_control_title_touch_ready == true
		  and .latest_minimum_gate.control_ui_chat_row_option_semantic_touch_ready == true
		  and .latest_minimum_gate.control_ui_thread_tools_menu_ready == true
		  and .latest_minimum_gate.control_ui_composer_tools_menu_ready == true
	  and .latest_minimum_gate.control_ui_composer_popover_ready == true
	  and .latest_minimum_gate.control_ui_composer_popover_search_light_glass_ready == true
	  and .latest_minimum_gate.control_ui_rail_search_light_glass_ready == true
	  and .latest_minimum_gate.control_ui_micro_surface_light_glass_ready == true
	  and .latest_minimum_gate.control_ui_message_routing_badge_light_glass_ready == true
	  and .latest_minimum_gate.control_ui_status_trust_strip_light_glass_ready == true
	  and .source_alignment.control_ui_visible_text_integrity_ready == true
  and .source_alignment.control_ui_icon_button_title_match_ready == true
  and .source_alignment.control_ui_menu_trigger_title_match_ready == true
  and .source_alignment.native_secondary_title_tooltip_ready == true
  and .source_alignment.native_secondary_title_tooltip_failure_count == 0
  and .source_alignment.secondary_surface_action_matrix_ready == true
  and .source_alignment.secondary_surface_action_matrix_case_count == 15
  and .source_alignment.current_plan_refresh_ready == true
  and .source_alignment.blocker_closure_ready == true
  and .source_alignment.backend_delivery_audit_ready == true
  and .source_alignment.backend_delivery_audit_critical_blocker_count == .critical_blocker_count
  and .source_alignment.backend_delivery_receipt_roundtrip_ready == true
  and .source_alignment.backend_delivery_receipt_roundtrip_present_branch_ready == true
  and .source_alignment.backend_delivery_receipt_roundtrip_present_branch_valid == true
  and .source_alignment.backend_delivery_receipt_roundtrip_root_report_required_count == 43
  and (.source_alignment.real_backend_receipt_present | type) == "boolean"
  and (.source_alignment.backend_receipt_valid | type) == "boolean"
  and .source_alignment.root_report_replay_required_count_after_risk_future_plan == 43
  and .claim_boundary.local_risk_future_plan_ready == true
  and .claim_boundary.local_backend_delivery_receipt_roundtrip_ready == true
  and (.claim_boundary.real_backend_receipt_claim_ready | type) == "boolean"
  and (.claim_boundary.backend_receipt_claim_ready | type) == "boolean"
  and .claim_boundary.live_product_claim_ready == false
  and .claim_boundary.public_distribution_claim_ready == false
  and .claim_boundary.release_claim_ready == false
  and .claim_boundary.external_actions_allowed == false
  and .side_effects.backend_agent_spawned == false
  and .side_effects.backend_repo_write == false
  and .side_effects.external_mutation == false
  and (.risk_plan_markdown_sha256 | test("^[0-9a-f]{64}$"))
  and .risk_plan_markdown_bytes > 0
' "$REPORT_TMP" >/dev/null

cp "$MARKDOWN_TMP" "$RISK_PLAN_MARKDOWN_PATH"
cp "$REPORT_TMP" "$REPORT_PATH"
cat "$REPORT_TMP"
