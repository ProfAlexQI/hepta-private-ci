#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

# shellcheck source=scripts/lib/hepta-ui-rust-toolchain.sh
source "scripts/lib/hepta-ui-rust-toolchain.sh"

APP_MANIFEST="apps/hepta-native/Cargo.toml"
OUT_DIR="${HEPTA_NATIVE_WINDOW_ROUTE_MOBILE_SMOKE_DIR:-$(mktemp -d "${TMPDIR:-/tmp}/hepta-native-window-route-mobile-smoke.XXXXXX")}"
REPORT_PATH="${HEPTA_NATIVE_WINDOW_ROUTE_MOBILE_SMOKE_REPORT_PATH:-$OUT_DIR/native-window-route-mobile-smoke.json}"
STARTUP_TIMEOUT_SEC="${HEPTA_NATIVE_WINDOW_ROUTE_MOBILE_SMOKE_STARTUP_TIMEOUT_SEC:-240}"
ALLOW_BLOCKED="${HEPTA_NATIVE_WINDOW_ROUTE_MOBILE_SMOKE_ALLOW_BLOCKED:-0}"
WRAPPER_PREFLIGHT="${HEPTA_NATIVE_WINDOW_ROUTE_MOBILE_SMOKE_PREFLIGHT:-1}"
ASSUME_PREFLIGHT_READY="${HEPTA_NATIVE_WINDOW_ROUTE_MOBILE_SMOKE_ASSUME_PREFLIGHT_READY:-0}"
WRAPPER_PREBUILD="${HEPTA_NATIVE_WINDOW_ROUTE_MOBILE_SMOKE_PREBUILD:-1}"
MOBILE_BOUNDS="${HEPTA_NATIVE_WINDOW_ROUTE_MOBILE_SMOKE_MOBILE_BOUNDS:-80,40,390,844}"
WINDOW_SMOKE_CARGO_TARGET_DIR="${HEPTA_NATIVE_WINDOW_SMOKE_CARGO_TARGET_DIR:-${HEPTA_NATIVE_CARGO_TARGET_DIR:-$OUT_DIR/cargo-target}}"

mkdir -p "$OUT_DIR" "$WINDOW_SMOKE_CARGO_TARGET_DIR"

if ! command -v jq >/dev/null 2>&1; then
  echo "jq is required for the Hepta Native mobile route window smoke gate" >&2
  exit 2
fi

flag_enabled() {
  case "${1:-}" in
    1 | true | TRUE | yes | YES | on | ON) return 0 ;;
    *) return 1 ;;
  esac
}

json_bool_for_flag() {
  if flag_enabled "$1"; then printf 'true'; else printf 'false'; fi
}

cargo_with_window_target() {
  CARGO_TARGET_DIR="$WINDOW_SMOKE_CARGO_TARGET_DIR" hepta_ui_cargo "$@"
}

run_window_smoke_preflight_test() {
  local test_name="$1"
  if ! cargo_with_window_target test --manifest-path "$APP_MANIFEST" -q "$test_name" >&2; then
    echo "Hepta Native mobile route window smoke preflight test failed: $test_name" >&2
    exit 1
  fi
}

if flag_enabled "$WRAPPER_PREFLIGHT"; then
  run_window_smoke_preflight_test current_codex_fixture_smoke_is_ready_without_live_side_effects
  run_window_smoke_preflight_test hepta_fixture_cockpit_has_a_card_for_each_sample_event
  run_window_smoke_preflight_test hepta_fixture_layout_policy_collapses_mobile_to_task_first_without_live_mutation
fi

if flag_enabled "$WRAPPER_PREBUILD"; then
  cargo_with_window_target build --manifest-path "$APP_MANIFEST" -q >&2
fi

route_label_for_slug() {
  case "$1" in
    actions) printf 'Actions' ;;
    approvals) printf 'Approvals' ;;
    inspector) printf 'Inspector' ;;
    *) printf 'Home' ;;
  esac
}

ROUTES=(home actions approvals inspector)
REPORT_PATHS=()
CHILD_SKIP_PREFLIGHT=0
if flag_enabled "$WRAPPER_PREFLIGHT" || flag_enabled "$ASSUME_PREFLIGHT_READY"; then
  CHILD_SKIP_PREFLIGHT=1
fi

for route in "${ROUTES[@]}"; do
  route_dir="$OUT_DIR/route-$route"
  route_report="$route_dir/native-window-smoke.json"
  mkdir -p "$route_dir"

  if ! env \
    HEPTA_NATIVE_FIXTURE_LAYOUT=mobile-task-first \
    HEPTA_NATIVE_FIXTURE_ROUTE="$route" \
    HEPTA_NATIVE_FIXTURE_ROW=1 \
    HEPTA_NATIVE_WINDOW_SMOKE_CAPTURE_PROFILE=mobile-route \
    HEPTA_NATIVE_WINDOW_SMOKE_MOBILE_BOUNDS="$MOBILE_BOUNDS" \
    HEPTA_NATIVE_WINDOW_SMOKE_DIR="$route_dir" \
    HEPTA_NATIVE_WINDOW_SMOKE_REPORT_PATH="$route_report" \
    HEPTA_NATIVE_WINDOW_SMOKE_CARGO_TARGET_DIR="$WINDOW_SMOKE_CARGO_TARGET_DIR" \
    HEPTA_NATIVE_WINDOW_SMOKE_STARTUP_TIMEOUT_SEC="$STARTUP_TIMEOUT_SEC" \
    HEPTA_NATIVE_WINDOW_SMOKE_ALLOW_BLOCKED="$ALLOW_BLOCKED" \
    HEPTA_NATIVE_WINDOW_SMOKE_SKIP_PREFLIGHT="$CHILD_SKIP_PREFLIGHT" \
    ./scripts/hepta-native-window-smoke.sh >"$route_dir/stdout.json" 2>"$route_dir/stderr.log"; then
    if [[ -s "$route_report" ]] && flag_enabled "$ALLOW_BLOCKED"; then
      status="$(jq -r '.status // ""' "$route_report")"
      case "$status" in
        blocked_by_locked_screen | blocked_by_local_macos_permissions)
          jq -n \
            --arg product "Hepta Native" \
            --arg runtime "hepta" \
            --arg output_dir "$OUT_DIR" \
            --arg report_path "$REPORT_PATH" \
            --arg blocked_route "$route" \
            --arg status "$status" \
            --argjson blocked_allowed "$(json_bool_for_flag "$ALLOW_BLOCKED")" \
            '{
              product:$product,
              runtime:$runtime,
              status:$status,
              capture_profile:"mobile-route-variants",
              output_dir:$output_dir,
              report_path:$report_path,
              blocked_route:$blocked_route,
              blocked_allowed:$blocked_allowed,
              true_window_capture_performed:false,
              native_makepad_mobile_route_variants_ready:false,
              route_count:0,
              screenshot_count:0,
              routes:[],
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
    echo "Hepta Native mobile route true-window smoke failed for route=$route" >&2
    tail -n 160 "$route_dir/stderr.log" >&2 || true
    [[ ! -s "$route_report" ]] || cat "$route_report" >&2
    exit 1
  fi

  route_label="$(route_label_for_slug "$route")"
  jq -e \
    --arg route "$route" \
    --arg route_label "$route_label" \
    '
      .status == "ready"
      and .capture_profile == "mobile-route"
      and .fixture_route_slug == $route
      and .fixture_route == $route_label
      and .true_window_capture_performed == true
      and .fixture_product_shell_selected_ready == true
      and .fixture_matrix_composer_hidden_ready == true
      and .fixture_mobile_task_first_layout_ready == true
      and .fixture_route_selected_ready == true
      and .fixture_route_top_design_referee_ready == true
      and .fixture_mobile_route_content_visible_ready == true
      and .native_makepad_mobile_route_variant_ready == true
      and .native_makepad_mobile_host_window_ready == true
      and .host_window_contract_ready == true
      and .native_app_log_error_free == true
      and (.screenshots | length) == 1
      and (.screenshots | all(.viewport_contract.expected_width == 390))
      and (.screenshots | all(.viewport_contract.expected_height == 844))
      and (.screenshots | all(.viewport_contract.host_window_usable_ready == true))
      and (.screenshots | all(.visual_probe.ready == true))
      and (.screenshots | all(.visual_probe.mobile_route_content_ready == true))
      and .side_effects.external_mutation == false
    ' "$route_report" >/dev/null

  REPORT_PATHS+=("$route_report")
done

jq -s \
  --arg product "Hepta Native" \
  --arg runtime "hepta" \
  --arg output_dir "$OUT_DIR" \
  --arg report_path "$REPORT_PATH" \
  --arg mobile_bounds "$MOBILE_BOUNDS" \
  --argjson blocked_allowed "$(json_bool_for_flag "$ALLOW_BLOCKED")" \
  --argjson wrapper_preflight_tests_run "$(json_bool_for_flag "$WRAPPER_PREFLIGHT")" \
  --argjson wrapper_preflight_assumed_ready "$(json_bool_for_flag "$ASSUME_PREFLIGHT_READY")" \
  --argjson wrapper_prebuild_performed "$(json_bool_for_flag "$WRAPPER_PREBUILD")" \
  '. as $reports | {
    product:$product,
    runtime:$runtime,
    status:"ready",
    capture_backend:"peekaboo",
    capture_profile:"mobile-route-variants",
    output_dir:$output_dir,
    report_path:$report_path,
    requested_mobile_bounds:$mobile_bounds,
    runner:{
      cargo_target_dir:(.[0].runner.cargo_target_dir // null),
      rust_toolchain:(.[0].runner.rust_toolchain // null),
      wrapper_preflight_tests_run:$wrapper_preflight_tests_run,
      wrapper_preflight_assumed_ready:$wrapper_preflight_assumed_ready,
      wrapper_prebuild_performed:$wrapper_prebuild_performed,
      child_preflight_skipped:($reports | all(.runner.preflight_skipped == true))
    },
    automation_host:(.[0].automation_host // null),
    platform_assurance:(.[0].platform_assurance // null),
    blocked_allowed:$blocked_allowed,
    true_window_capture_performed:true,
    native_makepad_highlight_area_ready:($reports | all(.native_makepad_highlight_area_ready == true)),
    native_makepad_highlight_pixel_luma_threshold:245,
    native_makepad_highlight_pixel_fraction_threshold:0.75,
    native_makepad_highlight_pixel_fraction_max:([$reports[].screenshots[].visual_probe.highlight_pixel_fraction] | max),
    native_makepad_mobile_route_variants_ready:(
      ($reports | length) == 4
      and ($reports | all(.status == "ready"))
      and ($reports | all(.capture_profile == "mobile-route"))
      and ($reports | all(.fixture_mobile_task_first_layout_ready == true))
      and ($reports | all(.fixture_route_selected_ready == true))
      and ($reports | all(.fixture_route_top_design_referee_ready == true))
      and ($reports | all(.fixture_mobile_route_content_visible_ready == true))
      and ($reports | all(.native_makepad_mobile_route_variant_ready == true))
      and ($reports | all(.native_makepad_mobile_host_window_ready == true))
      and ($reports | all(.host_window_contract_ready == true))
      and ($reports | all(.native_app_log_error_free == true))
      and ($reports | all((.screenshots | length) == 1))
      and ($reports | all(.screenshots | all(.viewport_contract.expected_width == 390)))
      and ($reports | all(.screenshots | all(.viewport_contract.expected_height == 844)))
      and ($reports | all(.screenshots | all(.viewport_contract.host_window_usable_ready == true)))
      and ($reports | all(.screenshots | all(.visual_probe.ready == true)))
      and ([$reports[].screenshots[0].sha256] | unique | length) == 4
    ),
    route_count:($reports | length),
    non_home_content_log_signature_count:([$reports[].fixture_mobile_route_content_visible_count] | add),
    route_screenshot_unique_count:([$reports[].screenshots[0].sha256] | unique | length),
    route_screenshot_unique_ready:(([$reports[].screenshots[0].sha256] | unique | length) == 4),
    exact_390x844_ready:($reports | all(.screenshots | all(.dimensions == "390x844" and .viewport_contract.exact_size_ready == true))),
    mobile_host_window_ready:($reports | all(.native_makepad_mobile_host_window_ready == true and .host_window_contract_ready == true)),
    host_constrained_count:([$reports[].screenshots[] | select(.viewport_contract.host_constrained == true)] | length),
    routes:($reports | map({
      route:.fixture_route_slug,
      label:.fixture_route,
      row:.fixture_row,
      report:.output_dir + "/native-window-smoke.json",
      app_log:.app_log,
      screenshot:.screenshots[0],
      route_selected_ready:.fixture_route_selected_ready,
      route_content_log_signature_count:.fixture_mobile_route_content_visible_count,
      mobile_route_content_ready:.native_makepad_mobile_route_variant_ready
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
  and .native_makepad_mobile_route_variants_ready == true
  and .route_count == 4
  and .route_screenshot_unique_count == 4
  and .non_home_content_log_signature_count >= 3
  and .mobile_host_window_ready == true
  and .screenshot_count == 4
  and .side_effects.external_mutation == false
' "$REPORT_PATH" >/dev/null

echo "Hepta Native mobile route macOS window smoke passed" >&2
