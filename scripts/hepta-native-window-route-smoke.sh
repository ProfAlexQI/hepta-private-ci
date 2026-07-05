#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

APP_MANIFEST="apps/hepta-native/Cargo.toml"
OUT_DIR="${HEPTA_NATIVE_WINDOW_ROUTE_SMOKE_DIR:-$(mktemp -d "${TMPDIR:-/tmp}/hepta-native-window-route-smoke.XXXXXX")}"
REPORT_PATH="${HEPTA_NATIVE_WINDOW_ROUTE_SMOKE_REPORT_PATH:-$OUT_DIR/native-window-route-smoke.json}"
STARTUP_TIMEOUT_SEC="${HEPTA_NATIVE_WINDOW_ROUTE_SMOKE_STARTUP_TIMEOUT_SEC:-240}"
ALLOW_BLOCKED="${HEPTA_NATIVE_WINDOW_ROUTE_SMOKE_ALLOW_BLOCKED:-0}"
WRAPPER_PREFLIGHT="${HEPTA_NATIVE_WINDOW_ROUTE_SMOKE_PREFLIGHT:-1}"
ASSUME_PREFLIGHT_READY="${HEPTA_NATIVE_WINDOW_ROUTE_SMOKE_ASSUME_PREFLIGHT_READY:-0}"
WRAPPER_PREBUILD="${HEPTA_NATIVE_WINDOW_ROUTE_SMOKE_PREBUILD:-1}"
DESKTOP_BOUNDS="${HEPTA_NATIVE_WINDOW_ROUTE_SMOKE_DESKTOP_BOUNDS:-40,120,1200,720}"
WINDOW_SMOKE_CARGO_TARGET_DIR="${HEPTA_NATIVE_WINDOW_SMOKE_CARGO_TARGET_DIR:-${HEPTA_NATIVE_CARGO_TARGET_DIR:-$OUT_DIR/cargo-target}}"

mkdir -p "$OUT_DIR" "$WINDOW_SMOKE_CARGO_TARGET_DIR"

if ! command -v jq >/dev/null 2>&1; then
  echo "jq is required for the Hepta Native route window smoke gate" >&2
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
  CARGO_TARGET_DIR="$WINDOW_SMOKE_CARGO_TARGET_DIR" cargo "$@"
}

run_window_smoke_preflight_test() {
  local test_name="$1"
  if ! cargo_with_window_target test --manifest-path "$APP_MANIFEST" -q "$test_name" >&2; then
    echo "Hepta Native route window smoke preflight test failed: $test_name" >&2
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
    echo "Hepta Native route window smoke prebuild failed" >&2
    exit 1
  fi
}

route_label_for_slug() {
  case "$1" in
    actions)
      printf 'Actions'
      ;;
    approvals)
      printf 'Approvals'
      ;;
    inspector)
      printf 'Inspector'
      ;;
    *)
      printf 'Home'
      ;;
  esac
}

ROUTES=(home actions approvals inspector)
REPORT_PATHS=()
CHILD_SKIP_PREFLIGHT=0
if flag_enabled "$WRAPPER_PREFLIGHT" || flag_enabled "$ASSUME_PREFLIGHT_READY"; then
  CHILD_SKIP_PREFLIGHT=1
fi

run_window_smoke_preflight_suite
prebuild_window_smoke_app

for route in "${ROUTES[@]}"; do
  route_dir="$OUT_DIR/route-$route"
  route_report="$route_dir/native-window-smoke.json"
  mkdir -p "$route_dir"

  if ! env \
    HEPTA_NATIVE_FIXTURE_LAYOUT=desktop-full \
    HEPTA_NATIVE_FIXTURE_ROUTE="$route" \
    HEPTA_NATIVE_FIXTURE_ROW=1 \
    HEPTA_NATIVE_WINDOW_SMOKE_CAPTURE_PROFILE=desktop-full-route \
    HEPTA_NATIVE_WINDOW_SMOKE_DESKTOP_BOUNDS="$DESKTOP_BOUNDS" \
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
              capture_profile:"desktop-full-route-variants",
              output_dir:$output_dir,
              report_path:$report_path,
              blocked_route:$blocked_route,
              blocked_allowed:$blocked_allowed,
              true_window_capture_performed:false,
              native_makepad_route_variants_ready:false,
              route_count:0,
              route_content_probe_ready:false,
              route_screenshot_unique_count:0,
              route_screenshot_unique_ready:false,
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
    echo "Hepta Native route true-window smoke failed for route=$route" >&2
    tail -n 120 "$route_dir/stderr.log" >&2 || true
    if [[ -s "$route_report" ]]; then
      cat "$route_report" >&2
    fi
    exit 1
  fi

  route_label="$(route_label_for_slug "$route")"
  jq -e \
    --arg route "$route" \
    --arg route_label "$route_label" \
    '
      .status == "ready"
      and .capture_profile == "desktop-full-route"
      and .fixture_route_slug == $route
      and .fixture_route == $route_label
      and .true_window_capture_performed == true
      and .fixture_product_shell_selected_ready == true
      and .fixture_matrix_composer_hidden_ready == true
      and .fixture_desktop_full_layout_ready == true
      and .fixture_route_selected_ready == true
      and .fixture_route_top_design_referee_ready == true
      and .native_makepad_route_variant_ready == true
      and .native_app_log_error_free == true
      and (.screenshots | length) == 1
      and (.screenshots | all(.visual_probe.ready == true))
      and (.screenshots | all(.visual_probe.route_content_ready == true))
      and .side_effects.external_mutation == false
    ' "$route_report" >/dev/null

  REPORT_PATHS+=("$route_report")
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
    capture_profile:"desktop-full-route-variants",
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
    native_makepad_route_variants_ready:(
      ($reports | length) == 4
      and ($reports | all(.status == "ready"))
      and ($reports | all(.capture_profile == "desktop-full-route"))
      and ($reports | all(.fixture_desktop_full_layout_ready == true))
      and ($reports | all(.fixture_route_selected_ready == true))
      and ($reports | all(.fixture_route_top_design_referee_ready == true))
      and ($reports | all(.native_makepad_route_variant_ready == true))
      and ($reports | all(.native_app_log_error_free == true))
      and ($reports | all((.screenshots | length) == 1))
      and ($reports | all(.screenshots | all(.visual_probe.ready == true)))
      and ($reports | all(.screenshots | all(.visual_probe.route_content_ready == true)))
      and ([$reports[].screenshots[0].sha256] | unique | length) == 4
    ),
    route_count:($reports | length),
    route_content_probe_ready:($reports | all(.screenshots | all(.visual_probe.route_content_ready == true))),
    route_screenshot_unique_count:([$reports[].screenshots[0].sha256] | unique | length),
    route_screenshot_unique_ready:(([$reports[].screenshots[0].sha256] | unique | length) == 4),
    route_top_design_referee_ready:($reports | all(.fixture_route_top_design_referee_ready == true)),
    routes:($reports | map({
      route:.fixture_route_slug,
      label:.fixture_route,
      row:.fixture_row,
      report:.output_dir + "/native-window-smoke.json",
      app_log:.app_log,
      runner:.runner,
      screenshot:.screenshots[0],
      visual_probe:.screenshots[0].visual_probe,
      route_content_ready:(.screenshots[0].visual_probe.route_content_ready // false),
      route_content_probe:(.screenshots[0].visual_probe.route_content_probe // null),
      desktop_full_layout_ready:.fixture_desktop_full_layout_ready,
      route_selected_ready:.fixture_route_selected_ready,
      route_top_design_referee_ready:.fixture_route_top_design_referee_ready,
      route_top_design_referee_count:.fixture_route_top_design_referee_count
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

echo "Hepta Native route true-window smoke passed" >&2
