#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

READINESS_DIR="${HEPTA_UI_PRODUCT_READINESS_DIR:-${1:-}}"
REPORT_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V5_REPORT_PATH:-}"
V4_REPORT_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V4_REPORT_PATH:-}"
NATIVE_REPORT_PATH="${HEPTA_NATIVE_FIXTURE_VISUAL_SMOKE_REPORT_PATH:-}"
NATIVE_DETAIL_REPORT_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V5_NATIVE_DETAIL_REPORT_PATH:-}"
V4_LOG="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V5_V4_LOG:-}"

if [[ -z "$READINESS_DIR" ]]; then
  echo "usage: $0 <hepta-ui-product-readiness-dir>" >&2
  exit 2
fi
if [[ -z "$REPORT_PATH" ]]; then
  REPORT_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v5-native-detail-census-gate.json"
fi
if [[ -z "$V4_REPORT_PATH" ]]; then
  V4_REPORT_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v4-button-census-gate.json"
fi
if [[ -z "$NATIVE_REPORT_PATH" ]]; then
  NATIVE_REPORT_PATH="$READINESS_DIR/native-fixture/native-fixture-visual-smoke.json"
fi
if [[ -z "$NATIVE_DETAIL_REPORT_PATH" ]]; then
  NATIVE_DETAIL_REPORT_PATH="$READINESS_DIR/native-v5-secondary-action-detail-census.json"
fi
if [[ -z "$V4_LOG" ]]; then
  V4_LOG="$READINESS_DIR/v4-button-census.log"
fi

if [[ ! -s "$NATIVE_REPORT_PATH" ]]; then
  echo "missing native fixture visual smoke report: $NATIVE_REPORT_PATH" >&2
  exit 1
fi
jq empty "$NATIVE_REPORT_PATH" >/dev/null

mkdir -p "$READINESS_DIR" "$(dirname "$REPORT_PATH")" "$(dirname "$NATIVE_DETAIL_REPORT_PATH")"

HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V4_REPORT_PATH="$V4_REPORT_PATH" \
HEPTA_NATIVE_FIXTURE_VISUAL_SMOKE_REPORT_PATH="$NATIVE_REPORT_PATH" \
  bash scripts/hepta-ui-harsh-top-design-referee-v4-button-census-gate.sh "$READINESS_DIR" >"$V4_LOG" 2>&1 || {
    echo "v4 button census prerequisite failed" >&2
    tail -n 120 "$V4_LOG" >&2 || true
    exit 1
  }

if [[ "$(jq -r '.status' "$V4_REPORT_PATH")" != "ready" ]]; then
  echo "v4 button census prerequisite was not ready: $V4_REPORT_PATH" >&2
  exit 1
fi

jq -n \
  --slurpfile native_file "$NATIVE_REPORT_PATH" '
  ($native_file[0]) as $native
  | ($native.secondary_product_surfaces.results // []) as $results
  | def sum_values: if length == 0 then 0 else add end;
    def style_ready($style):
      (($style.border_radius // 0) >= 8)
      and (($style.background_image // "") != "none")
      and (($style.backdrop_filter // "") | contains("blur("))
      and (($style.box_shadow // "") != "none");
    def action_ready:
      (.harsh_action_ready == true)
      and (.role_ready == true)
      and ((.role // "") == (.expected_role // ""))
      and (.icon_present == true)
      and (.icon_svg_ready == true)
      and (.icon_text_placeholder_absent == true)
      and ((.icon_text // "") == "")
      and ((.icon_href // "") | startswith("#icon-"))
      and ((.label // "") | length > 0)
      and (.tag == "button")
      and (.type == "button")
      and ((.aria_label // "") | length > 0)
      and ((.title // "") | length > 0)
      and (.aria_label == .title)
      and (.title_tooltip_ready == true)
      and (.disabled == false)
      and (.semantic_ready == true)
      and (.button_style_ready == true)
      and (.label_nowrap_ready == true)
      and (.label_word_break_ready == true)
      and ((.width // 0) >= 44)
      and ((.height // 0) >= 44)
      and ((.scrollWidth // 0) <= ((.width // 0) + 2))
      and ((.scrollHeight // 0) <= ((.height // 0) + 2))
      and ((.label_style.white_space // "") == "nowrap")
      and ((.label_style.overflow_wrap // "") == "normal")
      and ((.label_style.word_break // "") == "keep-all");
    def action_failures:
      [
        (if .harsh_action_ready != true then "harsh_action_ready" else empty end),
        (if .role_ready != true or ((.role // "") != (.expected_role // "")) then "role" else empty end),
        (if .icon_present != true or .icon_svg_ready != true or .icon_text_placeholder_absent != true or ((.icon_text // "") != "") or (((.icon_href // "") | startswith("#icon-")) | not) then "icon" else empty end),
        (if ((.label // "") | length == 0) then "label" else empty end),
        (if .tag != "button" or .type != "button" or .disabled != false or .semantic_ready != true then "semantic_button" else empty end),
        (if ((.aria_label // "") | length == 0) or ((.title // "") | length == 0) or (.aria_label != .title) or .title_tooltip_ready != true then "aria_title_tooltip" else empty end),
        (if .button_style_ready != true or ((.width // 0) < 44) or ((.height // 0) < 44) or ((.scrollWidth // 0) > ((.width // 0) + 2)) or ((.scrollHeight // 0) > ((.height // 0) + 2)) then "size_or_clip" else empty end),
        (if .label_nowrap_ready != true or .label_word_break_ready != true or ((.label_style.white_space // "") != "nowrap") or ((.label_style.overflow_wrap // "") != "normal") or ((.label_style.word_break // "") != "keep-all") then "label_layout" else empty end)
      ];
    def case_ready:
      . as $case
      |
      (.ready == true)
      and (.visible_surface_count == 1)
      and (.active_matches == true)
      and (.surface_in_viewport == true)
      and (.expected_actions_present == true)
      and (.semantic_button_ready == true)
      and (.action_matrix_ready == true)
      and (.harsh_action_matrix_ready == true)
      and (.surface_glass_ready == true)
      and (.action_group_glass_ready == true)
      and (.actions_usable == true)
      and (.preferred_touch_target_ready == true)
      and (.actions_in_surface == true)
      and (.horizontal_overflow_free == true)
      and ((.text_clipping_failure_count // 0) == 0)
      and ((.content_edge_failure_count // 0) == 0)
      and ((.visible_audit_failure_count // 0) == 0)
      and ((.active_rect.left // -1) >= 0)
      and ((.active_rect.top // -1) >= 0)
      and ((.active_rect.right // 99999) <= (.viewport.width + 1))
      and ((.active_rect.bottom // 99999) <= (.viewport.height + 1))
      and ((.active_rect.scrollWidth // 0) <= ((.active_rect.width // 0) + 2))
      and ((.active_rect.scrollHeight // 0) <= ((.active_rect.height // 0) + 2))
      and style_ready(.surface_style)
      and (($case.action_details // []) | length == (($case.expected_action_ids // []) | length))
      and (($case.action_details // []) | all(action_ready));
    def case_failures:
      . as $case
      |
      [
        (if .ready != true then "case_ready" else empty end),
        (if .visible_surface_count != 1 or .active_matches != true or .surface_in_viewport != true then "surface_visibility" else empty end),
        (if .expected_actions_present != true or .semantic_button_ready != true or .action_matrix_ready != true or .harsh_action_matrix_ready != true then "action_matrix" else empty end),
        (if .surface_glass_ready != true or style_ready(.surface_style) != true then "surface_glass" else empty end),
        (if .action_group_glass_ready != true then "action_group_glass" else empty end),
        (if .actions_usable != true or .preferred_touch_target_ready != true or .actions_in_surface != true then "action_ergonomics" else empty end),
        (if .horizontal_overflow_free != true or ((.text_clipping_failure_count // 0) != 0) or ((.content_edge_failure_count // 0) != 0) or ((.visible_audit_failure_count // 0) != 0) then "overflow_or_visible_audit" else empty end),
        (if ((.active_rect.left // -1) < 0) or ((.active_rect.top // -1) < 0) or ((.active_rect.right // 99999) > (.viewport.width + 1)) or ((.active_rect.bottom // 99999) > (.viewport.height + 1)) or ((.active_rect.scrollWidth // 0) > ((.active_rect.width // 0) + 2)) or ((.active_rect.scrollHeight // 0) > ((.active_rect.height // 0) + 2)) then "surface_bounds" else empty end),
        (if (($case.action_details // []) | length) != (($case.expected_action_ids // []) | length) or ((($case.action_details // []) | all(action_ready)) | not) then "action_details" else empty end)
      ];
    ($results | map(. as $case | {
      viewport:$case.viewport,
      viewport_size:("\($case.viewport.width)x\($case.viewport.height)"),
      surface:$case.surface,
      expected_action_ids:$case.expected_action_ids,
      action_count:$case.action_count,
      active_rect:$case.active_rect,
      case_ready:($case | case_ready),
      case_failures:($case | case_failures),
      action_results:(($case.action_details // []) | map(. + {
        action_ready:(. | action_ready),
        action_failures:(. | action_failures)
      })),
      action_failure_count:(($case.action_details // []) | map(select((. | action_ready) != true)) | length)
    })) as $case_results
  | ($case_results | map(.action_count) | sum_values) as $action_instances
  | ($case_results | map(.action_failure_count) | sum_values) as $action_failures
  | ($case_results | map(select(.case_ready != true)) | length) as $case_failures
  | ($results | map(.surface) | unique) as $surfaces
  | ($results | map("\(.viewport.width)x\(.viewport.height)") | unique) as $viewport_sizes
  | {
      schema_version:"hepta-ui-native-v5-secondary-action-detail-census/v0",
      standards_version:"2026-06-27-native-secondary-surface-action-detail-light-tempered-glass",
      status:(if (
        $native.status == "ready"
        and $native.native_top_design_referee_ready == true
        and $native.native_tempered_glass_visual_contract_ready == true
        and $native.native_secondary_harsh_action_matrix_ready == true
        and $native.native_visible_audit_failure_count == 0
        and $native.screenshot_count >= 41
        and ($native.secondary_product_surfaces.surface_count == 5)
        and ($native.secondary_product_surfaces.viewport_count == 3)
        and ($results | length) == 15
        and ($surfaces == ["attachment","modal","search","settings","voice"])
        and ($viewport_sizes == ["1280x800","320x844","390x844"])
        and $action_instances == 57
        and $case_failures == 0
        and $action_failures == 0
      ) then "ready" else "failed" end),
      output_dir:$native.output_dir,
      screenshot_count:$native.screenshot_count,
      surface_count:($surfaces | length),
      viewport_count:($viewport_sizes | length),
      case_count:($results | length),
      action_instance_count:$action_instances,
      case_failure_count:$case_failures,
      action_failure_count:$action_failures,
      surfaces:$surfaces,
      viewport_sizes:$viewport_sizes,
      case_results:$case_results
    }
  ' >"$NATIVE_DETAIL_REPORT_PATH"

v4_sha="$(shasum -a 256 "$V4_REPORT_PATH" | awk '{print $1}')"
native_sha="$(shasum -a 256 "$NATIVE_REPORT_PATH" | awk '{print $1}')"
native_detail_sha="$(shasum -a 256 "$NATIVE_DETAIL_REPORT_PATH" | awk '{print $1}')"
tmp_report="$(mktemp "${TMPDIR:-/tmp}/hepta-ui-v5-final.XXXXXX")"

jq -n \
  --arg v4_path "$V4_REPORT_PATH" \
  --arg native_path "$NATIVE_REPORT_PATH" \
  --arg native_detail_path "$NATIVE_DETAIL_REPORT_PATH" \
  --arg v4_sha "$v4_sha" \
  --arg native_sha "$native_sha" \
  --arg native_detail_sha "$native_detail_sha" \
  --slurpfile v4_file "$V4_REPORT_PATH" \
  --slurpfile native_file "$NATIVE_REPORT_PATH" \
  --slurpfile native_detail_file "$NATIVE_DETAIL_REPORT_PATH" '
  ($v4_file[0]) as $v4
  | ($native_file[0]) as $native
  | ($native_detail_file[0]) as $native_detail
  | def v4_ready:
      $v4.status == "ready"
      and $v4.v3_ready == true
      and $v4.button_census_ready == true
      and $v4.native_ready == true
      and $v4.summary.control_visual_matrix.viewport_count == 4
      and $v4.summary.control_button_census.failure_count == 0
      and $v4.summary.control_button_census.interactive_instance_count >= 480
      and $v4.summary.control_button_census.button_like_instance_count >= 330
      and $v4.summary.control_button_census.module_instance_count >= 280;
    def native_detail_ready:
      $native_detail.status == "ready"
      and $native_detail.screenshot_count >= 41
      and $native_detail.surface_count == 5
      and $native_detail.viewport_count == 3
      and $native_detail.case_count == 15
      and $native_detail.action_instance_count == 57
      and $native_detail.case_failure_count == 0
      and $native_detail.action_failure_count == 0
      and ($native_detail.case_results | all(.case_ready == true and .action_failure_count == 0 and (.action_results | all(.action_ready == true and (.action_failures | length) == 0))));
    {
      schema_version:"hepta-ui-harsh-top-design-referee-v5-gate/v0",
      standards_version:"2026-06-27-harsh-control-v4-plus-native-detail-census-light-tempered-glass",
      status:(if (v4_ready and native_detail_ready) then "ready" else "failed" end),
      inputs:{
        v4_control_button_census:{path:$v4_path, sha256:$v4_sha},
        native_fixture:{path:$native_path, sha256:$native_sha},
        native_detail_census:{path:$native_detail_path, sha256:$native_detail_sha}
      },
      summary:{
        control_visual_matrix:$v4.summary.control_visual_matrix,
        control_button_census:$v4.summary.control_button_census,
        native_fixture:{
          screenshot_count:$native.screenshot_count,
          min_contrast:$native.tempered_glass_visual_contract.min_contrast_ratio,
          visible_audit_failure_count:$native.native_visible_audit_failure_count
        },
        native_detail_census:{
          surface_count:$native_detail.surface_count,
          viewport_count:$native_detail.viewport_count,
          case_count:$native_detail.case_count,
          action_instance_count:$native_detail.action_instance_count,
          case_failure_count:$native_detail.case_failure_count,
          action_failure_count:$native_detail.action_failure_count,
          surfaces:$native_detail.surfaces,
          viewport_sizes:$native_detail.viewport_sizes
        }
      },
      v4_ready:v4_ready,
      native_detail_ready:native_detail_ready,
      native_detail_census:$native_detail
    }
  ' >"$tmp_report"

cp "$tmp_report" "$REPORT_PATH"
rm -f "$tmp_report"
cat "$REPORT_PATH"

if [[ "$(jq -r '.status' "$REPORT_PATH")" != "ready" ]]; then
  echo "Hepta UI harsh top-design referee v5 native detail census failed" >&2
  exit 1
fi

echo "$REPORT_PATH"
