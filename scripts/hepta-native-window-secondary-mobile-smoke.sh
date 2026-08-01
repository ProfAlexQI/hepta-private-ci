#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

# shellcheck source=scripts/lib/hepta-ui-rust-toolchain.sh
source "scripts/lib/hepta-ui-rust-toolchain.sh"

APP_MANIFEST="apps/hepta-native/Cargo.toml"
OUT_DIR="${HEPTA_NATIVE_WINDOW_SECONDARY_MOBILE_SMOKE_DIR:-$(mktemp -d "${TMPDIR:-/tmp}/hepta-native-window-secondary-mobile-smoke.XXXXXX")}"
REPORT_PATH="${HEPTA_NATIVE_WINDOW_SECONDARY_MOBILE_SMOKE_REPORT_PATH:-$OUT_DIR/native-window-secondary-mobile-smoke.json}"
STARTUP_TIMEOUT_SEC="${HEPTA_NATIVE_WINDOW_SECONDARY_MOBILE_SMOKE_STARTUP_TIMEOUT_SEC:-240}"
ALLOW_BLOCKED="${HEPTA_NATIVE_WINDOW_SECONDARY_MOBILE_SMOKE_ALLOW_BLOCKED:-0}"
WRAPPER_PREFLIGHT="${HEPTA_NATIVE_WINDOW_SECONDARY_MOBILE_SMOKE_PREFLIGHT:-1}"
ASSUME_PREFLIGHT_READY="${HEPTA_NATIVE_WINDOW_SECONDARY_MOBILE_SMOKE_ASSUME_PREFLIGHT_READY:-0}"
WRAPPER_PREBUILD="${HEPTA_NATIVE_WINDOW_SECONDARY_MOBILE_SMOKE_PREBUILD:-1}"
MOBILE_BOUNDS="${HEPTA_NATIVE_WINDOW_SECONDARY_MOBILE_SMOKE_MOBILE_BOUNDS:-80,40,390,844}"
WINDOW_SMOKE_CARGO_TARGET_DIR="${HEPTA_NATIVE_WINDOW_SMOKE_CARGO_TARGET_DIR:-${HEPTA_NATIVE_CARGO_TARGET_DIR:-$OUT_DIR/cargo-target}}"

mkdir -p "$OUT_DIR" "$WINDOW_SMOKE_CARGO_TARGET_DIR"

if ! command -v jq >/dev/null 2>&1; then
  echo "jq is required for the Hepta Native secondary mobile window smoke gate" >&2
  exit 2
fi

flag_enabled() {
  case "${1:-}" in
    1 | true | TRUE | yes | YES | on | ON)
      return 0
      ;;
    *)
      return 1
      ;;
  esac
}

json_bool_for_flag() {
  if flag_enabled "$1"; then
    printf 'true'
  else
    printf 'false'
  fi
}

cargo_with_window_target() {
  CARGO_TARGET_DIR="$WINDOW_SMOKE_CARGO_TARGET_DIR" hepta_ui_cargo "$@"
}

run_window_smoke_preflight_test() {
  local test_name="$1"
  if ! cargo_with_window_target test --manifest-path "$APP_MANIFEST" -q "$test_name" >&2; then
    echo "Hepta Native secondary mobile window smoke preflight test failed: $test_name" >&2
    exit 1
  fi
}

run_window_smoke_preflight_suite() {
  if ! flag_enabled "$WRAPPER_PREFLIGHT"; then
    return
  fi
  run_window_smoke_preflight_test current_codex_fixture_smoke_is_ready_without_live_side_effects
  run_window_smoke_preflight_test hepta_fixture_cockpit_has_a_card_for_each_sample_event
  run_window_smoke_preflight_test hepta_fixture_layout_policy_collapses_mobile_to_task_first_without_live_mutation
}

prebuild_window_smoke_app() {
  if ! flag_enabled "$WRAPPER_PREBUILD"; then
    return
  fi
  if ! cargo_with_window_target build --manifest-path "$APP_MANIFEST" -q >&2; then
    echo "Hepta Native secondary mobile window smoke prebuild failed" >&2
    exit 1
  fi
}

surface_label_for_slug() {
  case "$1" in
    search)
      printf 'Search'
      ;;
    settings)
      printf 'Settings'
      ;;
    attachment)
      printf 'Attachment'
      ;;
    voice)
      printf 'Voice'
      ;;
    modal)
      printf 'Modal'
      ;;
    *)
      printf 'None'
      ;;
  esac
}

SURFACES=(search settings attachment voice modal)
REPORT_PATHS=()
CHILD_SKIP_PREFLIGHT=0
if flag_enabled "$WRAPPER_PREFLIGHT" || flag_enabled "$ASSUME_PREFLIGHT_READY"; then
  CHILD_SKIP_PREFLIGHT=1
fi

run_window_smoke_preflight_suite
prebuild_window_smoke_app

for surface in "${SURFACES[@]}"; do
  surface_dir="$OUT_DIR/surface-$surface"
  surface_report="$surface_dir/native-window-smoke.json"
  rm -rf "$surface_dir"
  mkdir -p "$surface_dir"

  if ! env \
    HEPTA_NATIVE_FIXTURE_LAYOUT=mobile-task-first \
    HEPTA_NATIVE_FIXTURE_ROUTE=home \
    HEPTA_NATIVE_FIXTURE_ROW=0 \
    HEPTA_NATIVE_FIXTURE_SURFACE="$surface" \
    HEPTA_NATIVE_WINDOW_SMOKE_CAPTURE_PROFILE=mobile-secondary \
    HEPTA_NATIVE_WINDOW_SMOKE_MOBILE_BOUNDS="$MOBILE_BOUNDS" \
    HEPTA_NATIVE_WINDOW_SMOKE_DIR="$surface_dir" \
    HEPTA_NATIVE_WINDOW_SMOKE_REPORT_PATH="$surface_report" \
    HEPTA_NATIVE_WINDOW_SMOKE_CARGO_TARGET_DIR="$WINDOW_SMOKE_CARGO_TARGET_DIR" \
    HEPTA_NATIVE_WINDOW_SMOKE_STARTUP_TIMEOUT_SEC="$STARTUP_TIMEOUT_SEC" \
    HEPTA_NATIVE_WINDOW_SMOKE_ALLOW_BLOCKED="$ALLOW_BLOCKED" \
    HEPTA_NATIVE_WINDOW_SMOKE_SKIP_PREFLIGHT="$CHILD_SKIP_PREFLIGHT" \
    ./scripts/hepta-native-window-smoke.sh >"$surface_dir/stdout.json" 2>"$surface_dir/stderr.log"; then
    if [[ -s "$surface_report" ]] && flag_enabled "$ALLOW_BLOCKED"; then
      status="$(jq -r '.status // ""' "$surface_report")"
      case "$status" in
        blocked_by_locked_screen | blocked_by_local_macos_permissions)
          jq -n \
            --arg product "Hepta Native" \
            --arg runtime "hepta" \
            --arg output_dir "$OUT_DIR" \
            --arg report_path "$REPORT_PATH" \
            --arg blocked_surface "$surface" \
            --arg status "$status" \
            --argjson blocked_allowed "$(json_bool_for_flag "$ALLOW_BLOCKED")" \
            '{
              product:$product,
              runtime:$runtime,
              status:$status,
              capture_profile:"mobile-secondary-surfaces",
              output_dir:$output_dir,
              report_path:$report_path,
              blocked_surface:$blocked_surface,
              blocked_allowed:$blocked_allowed,
              true_window_capture_performed:false,
              native_makepad_secondary_mobile_surfaces_ready:false,
              mobile_secondary_content_probe_ready:false,
              mobile_secondary_content_visible_count:0,
              surface_count:0,
              surface_screenshot_unique_count:0,
              surface_screenshot_unique_ready:false,
              screenshot_count:0,
              surfaces:[],
              screenshots:[],
              side_effects:{
                matrix_login:false,
                gateway_call:false,
                provider_invoked:false,
                channel_delivery:false,
                external_mutation:false
              }
            }' | tee "$REPORT_PATH"
          exit 0
          ;;
      esac
    fi
    echo "Hepta Native secondary mobile true-window smoke failed for surface=$surface" >&2
    tail -n 120 "$surface_dir/stderr.log" >&2 || true
    if [[ -s "$surface_report" ]]; then
      cat "$surface_report" >&2
    fi
    exit 1
  fi

  surface_label="$(surface_label_for_slug "$surface")"
  jq -e \
    --arg surface "$surface" \
    --arg surface_label "$surface_label" \
    '
      .status == "ready"
      and .capture_profile == "mobile-secondary"
      and .fixture_secondary_surface_slug == $surface
      and .fixture_secondary_surface == $surface_label
      and .true_window_capture_performed == true
      and .fixture_product_shell_selected_ready == true
      and .fixture_matrix_composer_hidden_ready == true
      and .fixture_mobile_task_first_layout_ready == true
      and .fixture_secondary_surface_selected_ready == true
      and .fixture_mobile_secondary_content_visible_ready == true
      and .native_makepad_secondary_surface_ready == true
      and .native_makepad_secondary_mobile_surface_ready == true
      and .native_makepad_mobile_host_window_ready == true
      and .host_window_contract_ready == true
      and .native_app_log_error_free == true
      and (.screenshots | length) == 1
      and (.screenshots | all(.visual_probe.ready == true))
      and (.screenshots | all(.visual_probe.mobile_secondary_content_ready == true))
      and .side_effects.external_mutation == false
    ' "$surface_report" >/dev/null

  REPORT_PATHS+=("$surface_report")
done

jq -s \
  --arg product "Hepta Native" \
  --arg runtime "hepta" \
  --arg output_dir "$OUT_DIR" \
  --arg report_path "$REPORT_PATH" \
  --argjson blocked_allowed "$(json_bool_for_flag "$ALLOW_BLOCKED")" \
  --argjson wrapper_preflight_tests_run "$(json_bool_for_flag "$WRAPPER_PREFLIGHT")" \
  --argjson wrapper_preflight_assumed_ready "$(json_bool_for_flag "$ASSUME_PREFLIGHT_READY")" \
  --argjson wrapper_prebuild_performed "$(json_bool_for_flag "$WRAPPER_PREBUILD")" \
  '. as $reports | {
    product:$product,
    runtime:$runtime,
    status:"ready",
    capture_backend:"peekaboo",
    capture_profile:"mobile-secondary-surfaces",
    output_dir:$output_dir,
    report_path:$report_path,
    runner:{
      cargo_target_dir:(.[0].runner.cargo_target_dir // null),
      wrapper_preflight_tests_run:$wrapper_preflight_tests_run,
      wrapper_preflight_assumed_ready:$wrapper_preflight_assumed_ready,
      wrapper_prebuild_performed:$wrapper_prebuild_performed,
      child_preflight_skipped:($reports | all(.runner.preflight_skipped == true))
    },
    blocked_allowed:$blocked_allowed,
    true_window_capture_performed:true,
    native_makepad_highlight_area_ready:($reports | all(.native_makepad_highlight_area_ready == true)),
    native_makepad_highlight_pixel_luma_threshold:245,
    native_makepad_highlight_pixel_fraction_threshold:0.75,
    native_makepad_highlight_pixel_fraction_max:([$reports[].screenshots[].visual_probe.highlight_pixel_fraction] | max),
    native_makepad_secondary_mobile_surfaces_ready:(
      ($reports | length) == 5
      and ($reports | all(.status == "ready"))
      and ($reports | all(.capture_profile == "mobile-secondary"))
      and ($reports | all(.fixture_mobile_task_first_layout_ready == true))
      and ($reports | all(.fixture_secondary_surface_selected_ready == true))
      and ($reports | all(.fixture_mobile_secondary_content_visible_ready == true))
      and ($reports | all(.native_makepad_secondary_surface_ready == true))
      and ($reports | all(.native_makepad_secondary_mobile_surface_ready == true))
      and ($reports | all(.native_makepad_mobile_host_window_ready == true))
      and ($reports | all(.host_window_contract_ready == true))
      and ($reports | all(.native_app_log_error_free == true))
      and ($reports | all((.screenshots | length) == 1))
      and ($reports | all(.screenshots | all(.visual_probe.ready == true)))
      and ($reports | all(.screenshots | all(.visual_probe.mobile_secondary_content_ready == true)))
      and ([$reports[].screenshots[0].sha256] | unique | length) == 5
    ),
    surface_count:($reports | length),
    surface_screenshot_unique_count:([$reports[].screenshots[0].sha256] | unique | length),
    surface_screenshot_unique_ready:(([$reports[].screenshots[0].sha256] | unique | length) == 5),
    mobile_secondary_content_probe_ready:(
      ($reports | length) == 5
      and ($reports | all(.fixture_mobile_secondary_content_visible_ready == true))
      and ($reports | all(.screenshots | all(.visual_probe.mobile_secondary_content_ready == true)))
    ),
    mobile_secondary_content_visible_count:(
      [$reports[].fixture_mobile_secondary_content_visible_count] | add
    ),
    mobile_host_window_ready:($reports | all(.native_makepad_mobile_host_window_ready == true and .host_window_contract_ready == true)),
    exact_390x844_ready:($reports | all(.native_makepad_mobile_390x844_ready == true and .viewport_contract_ready == true)),
    host_constrained_count:([$reports[].screenshots[] | select(.viewport_contract.host_constrained == true)] | length),
    surfaces:($reports | map({
      surface:.fixture_secondary_surface_slug,
      label:.fixture_secondary_surface,
      report:.output_dir + "/native-window-smoke.json",
      app_log:.app_log,
      runner:.runner,
      screenshot:.screenshots[0],
      visual_probe:.screenshots[0].visual_probe,
      mobile_task_first_layout_ready:.fixture_mobile_task_first_layout_ready,
      secondary_surface_selected_ready:.fixture_secondary_surface_selected_ready,
      mobile_secondary_content_visible_ready:.fixture_mobile_secondary_content_visible_ready,
      mobile_secondary_content_visible_count:.fixture_mobile_secondary_content_visible_count,
      mobile_secondary_content_probe:.screenshots[0].visual_probe.mobile_secondary_content_probe
    })),
    screenshots:[$reports[].screenshots[]],
    screenshot_count:([$reports[].screenshots[]] | length),
    native_app_log_error_free:($reports | all(.native_app_log_error_free == true)),
    side_effects:{
      matrix_login:false,
      gateway_call:false,
      provider_invoked:false,
      channel_delivery:false,
      external_mutation:false
    }
  }' "${REPORT_PATHS[@]}" | tee "$REPORT_PATH"

jq -e '
  .status == "ready"
  and .native_makepad_highlight_area_ready == true
  and .native_makepad_highlight_pixel_fraction_max <= .native_makepad_highlight_pixel_fraction_threshold
  and .native_makepad_secondary_mobile_surfaces_ready == true
  and .mobile_secondary_content_probe_ready == true
  and .mobile_secondary_content_visible_count >= 5
  and .mobile_host_window_ready == true
  and .surface_count == 5
  and .surface_screenshot_unique_count == 5
  and .surface_screenshot_unique_ready == true
  and .screenshot_count == 5
  and .native_app_log_error_free == true
  and .side_effects.external_mutation == false
' "$REPORT_PATH" >/dev/null

echo "Hepta Native secondary mobile true-window smoke passed" >&2
