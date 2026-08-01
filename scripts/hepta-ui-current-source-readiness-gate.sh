#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
cd "$ROOT_DIR"

READINESS_DIR="${HEPTA_UI_PRODUCT_READINESS_DIR:-}"
if [[ -z "$READINESS_DIR" ]]; then
  echo "HEPTA_UI_PRODUCT_READINESS_DIR is required" >&2
  exit 2
fi
if [[ ! -d "$READINESS_DIR" ]]; then
  echo "missing readiness evidence directory: $READINESS_DIR" >&2
  exit 1
fi
READINESS_DIR="$(cd "$READINESS_DIR" && pwd -P)"
REPORT_PATH="${HEPTA_UI_CURRENT_SOURCE_READINESS_REPORT_PATH:-$READINESS_DIR/ui-current-source-readiness.json}"

require_command() {
  if ! command -v "$1" >/dev/null 2>&1; then
    echo "$1 is required for the Hepta UI current-source readiness gate" >&2
    exit 2
  fi
}

require_report() {
  local path="$1"
  if [[ ! -s "$path" ]]; then
    echo "missing required current-source evidence report: $path" >&2
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

assert_evidence_path() {
  local path="$1"
  local parent
  local canonical
  if [[ ! -f "$path" || -L "$path" ]]; then
    echo "missing or symlinked evidence artifact: $path" >&2
    exit 1
  fi
  parent="$(cd "$(dirname "$path")" && pwd -P)"
  canonical="$parent/$(basename "$path")"
  case "$canonical" in
    "$READINESS_DIR"/*) ;;
    *)
      echo "evidence artifact escapes readiness directory: $path" >&2
      exit 1
      ;;
  esac
}

verify_artifact() {
  local path="$1"
  local expected_bytes="$2"
  local expected_sha="$3"
  local actual_bytes
  local actual_sha
  assert_evidence_path "$path"
  actual_bytes="$(file_bytes "$path")"
  actual_sha="$(file_sha256 "$path")"
  if [[ -n "$expected_bytes" && "$actual_bytes" != "$expected_bytes" ]]; then
    echo "evidence byte mismatch for $path: expected $expected_bytes, found $actual_bytes" >&2
    exit 1
  fi
  if [[ ! "$expected_sha" =~ ^[0-9a-f]{64}$ || "$actual_sha" != "$expected_sha" ]]; then
    echo "evidence SHA-256 mismatch for $path" >&2
    exit 1
  fi
}

append_file_record() {
  local json="$1"
  local name="$2"
  local path="$3"
  local bytes="$4"
  local sha="$5"
  jq -nc \
    --argjson current "$json" \
    --arg name "$name" \
    --arg path "$path" \
    --argjson bytes "$bytes" \
    --arg sha256 "$sha" \
    '$current + [{name:$name,path:$path,bytes:$bytes,sha256:$sha256}]'
}

require_command git
require_command jq
require_command ruby
require_command shasum

STATIC_REPORT="$READINESS_DIR/static-contract.json"
DESIGN_REPORT="$READINESS_DIR/ui-design-system-gate.json"
CONTROL_REPORT="$READINESS_DIR/control-ui-browser-smoke.json"
CONTROL_V7_REPORT="$READINESS_DIR/ui-harsh-top-design-referee-v7-real-click-gate.json"
NATIVE_FIXTURE_REPORT="$READINESS_DIR/native-fixture/native-fixture-visual-smoke.json"
PACKAGING_REPORT="$READINESS_DIR/native-packaging-gate.json"
DISTRIBUTION_REPORT="$READINESS_DIR/native-distribution-preflight-gate.json"
SIGNING_CAPABILITY_REPORT="$READINESS_DIR/ui-release-signing-capability-gate.json"
SCREENSHOT_MANIFEST="$READINESS_DIR/screenshot-manifest.json"
WINDOW_BASE_REPORT="$READINESS_DIR/native-window-smoke.json"
WINDOW_ROUTE_REPORT="$READINESS_DIR/native-window-routes-smoke.json"
WINDOW_ROUTE_MOBILE_REPORT="$READINESS_DIR/native-window-routes-mobile-smoke.json"
WINDOW_SECONDARY_REPORT="$READINESS_DIR/native-window-secondary-smoke.json"
WINDOW_SECONDARY_MOBILE_REPORT="$READINESS_DIR/native-window-secondary-mobile-smoke.json"

REPORT_SPECS=(
  "static-contract|$STATIC_REPORT"
  "design-system|$DESIGN_REPORT"
  "control-browser|$CONTROL_REPORT"
  "control-v7-real-click|$CONTROL_V7_REPORT"
  "native-fixture|$NATIVE_FIXTURE_REPORT"
  "native-packaging|$PACKAGING_REPORT"
  "native-distribution-preflight|$DISTRIBUTION_REPORT"
  "release-signing-capability|$SIGNING_CAPABILITY_REPORT"
  "screenshot-manifest|$SCREENSHOT_MANIFEST"
  "native-window-base|$WINDOW_BASE_REPORT"
  "native-window-routes|$WINDOW_ROUTE_REPORT"
  "native-window-routes-mobile|$WINDOW_ROUTE_MOBILE_REPORT"
  "native-window-secondary|$WINDOW_SECONDARY_REPORT"
  "native-window-secondary-mobile|$WINDOW_SECONDARY_MOBILE_REPORT"
)

report_inputs_json='[]'
for spec in "${REPORT_SPECS[@]}"; do
  name="${spec%%|*}"
  path="${spec#*|}"
  require_report "$path"
  assert_evidence_path "$path"
  bytes="$(file_bytes "$path")"
  sha="$(file_sha256 "$path")"
  report_inputs_json="$(append_file_record "$report_inputs_json" "$name" "$path" "$bytes" "$sha")"
done

./scripts/hepta-ui-light-glass-token-sync.rb --check >/dev/null

jq -e '
  .static_contract_ready == true
  and .marker_count >= 3642
  and .declared_marker_count == 4021
  and .deferred_backend_marker_count == 357
  and .deferred_backend_marker_budget == 357
  and (.deferred_backend_marker_files | sort) == [
    "apps/hepta-native/src/home/hepta_telegram_base_contract.rs",
    "apps/hepta-native/src/home/room_screen.rs"
  ]
  and .backend_live_adapter_source_ready == false
  and .backend_live_adapter_source_required_for_ui_gate == false
' "$STATIC_REPORT" >/dev/null

jq -e '
  .status == "ready"
  and .generated_token_sync_ready == true
  and .documentation_token_sync_ready == true
  and .dim_text_contrast.minimum >= 4.8
  and (.rust_toolchain | startswith("rustc 1.95.0"))
  and .control.css_layer_count == 6
  and .control.runtime_css_bytes <= .control.runtime_css_budget_bytes
  and .control.important_count <= .control.important_budget
  and .control.important_count < .control.important_audit_baseline
  and .control.retired_texture_asset_free == true
  and .control.accessibility_media_queries_ready == true
  and .control.static_light_theme_ready == true
  and .control.renderer_light_theme_ready == true
  and .control.document_direction_source_ready == true
  and .native.generated_tokens_registered == true
  and .native.html_fixture_token_sync_ready == true
  and .robrix.selective_module_count == 6
  and .robrix.upstream_commit == "a5a664da569c577ab1a3e5a33f45dcc9364954a0"
  and .robrix.license == "MIT"
  and .robrix.license_notice_current == true
' "$DESIGN_REPORT" >/dev/null

jq -e '
  def shallow_contract:
    .shallow_light_glass_ready == true
    and .light_theme_semantics_ready == true
    and .stable_content_surface_ready == true
    and .native_popover_interaction_ready == true
    and .shallow_floating_surface_ready == true
    and .restrained_optics_ready == true
    and .restrained_mobile_metadata_ready == true
    and .key_touch_controls_ready == true;
  .status == "ready"
  and (.screenshots | length) == 4
  and ([.screenshots[].viewport] | sort) == ["1365x900","320x844","500x844","768x900"]
  and .control_ui_320_reflow_ready == true
  and .control_ui_preferred_touch_targets_ready == true
  and .control_ui_horizontal_overflow_free == true
  and .control_ui_native_popover_interaction_ready == true
  and .control_ui_native_popover_compatibility_source == "native_actual_click_single_submenu_audit"
  and .control_ui_shallow_light_glass_ready == true
  and .control_ui_light_theme_semantics_ready == true
  and .control_ui_stable_content_surface_ready == true
  and .control_ui_shallow_floating_surface_ready == true
  and .control_ui_restrained_optics_ready == true
  and .control_ui_restrained_mobile_metadata_ready == true
  and .control_ui_key_touch_controls_ready == true
  and .control_ui_visible_text_integrity_ready == true
  and .control_ui_active_chat_readability_ready == true
  and .control_ui_menu_surfaces_ready == true
  and .control_ui_menu_surface_viewport_guard_ready == true
  and .control_ui_browser_error_page_absent == true
  and .subresource_error_count == 0
  and .subresource_requests_clean == true
  and .density_qa.status == "ready"
  and .density_qa.viewport_count == 4
  and (.density_qa.results | length) == 4
  and (.density_qa.results | all(shallow_contract and .horizontal_overflow_free == true and (.errors | length) == 0))
' "$CONTROL_REPORT" >/dev/null

jq -e '
  .status == "ready"
  and .v6_ready == true
  and .real_click_ready == true
  and .control_real_click_activation.status == "ready"
  and .control_real_click_activation.viewport_count == 4
  and .control_real_click_activation.target_count == 26
  and .control_real_click_activation.screenshot_count == 32
  and .control_real_click_activation.failure_count == 0
  and .control_real_click_activation.mobile_route_viewport_count == 2
  and .control_real_click_activation.mobile_route_count == 6
  and .control_real_click_activation.mobile_route_screenshot_count == 6
  and .control_real_click_activation.mobile_routes_ready == true
  and .control_real_click_activation.popover_switch_sequence_ready == true
  and .control_real_click_activation.popover_switch_step_count == 26
  and (.control_real_click_activation.viewports | all(
    .ready == true
    and (.targets | all(
      .ready == true
      and .default_closed.ready == true
      and .click.ready == true
      and .toggle_cycle.ready == true
      and .audit.ready == true
      and .light_dismiss.ready == true
      and .escape_close.ready == true
    ))
    and .popover_switch_sequence.ready == true
    and .popover_switch_sequence.escape.ready == true
  ))
' "$CONTROL_V7_REPORT" >/dev/null

jq -e '
  .status == "ready"
  and .screenshot_count == 41
  and .native_top_design_referee_ready == true
  and .native_320_reflow_ready == true
  and .native_mobile_touch_target_preferred_ready == true
  and .native_readability_contrast_clip_ready == true
  and .native_tempered_glass_visual_contract_ready == true
  and .native_secondary_product_surfaces_ready == true
  and .native_secondary_harsh_action_matrix_ready == true
  and .native_telegram_mobile_safe_area_keyboard_ready == true
  and .selected_row_variant_count == 18
  and .selected_row_unique_count == 18
  and .route_variant_unique_count == 4
  and .mobile_route_variant_unique_count == 4
  and .tempered_glass_visual_contract.status == "ready"
  and .tempered_glass_visual_contract.min_contrast_ratio >= 4.8
  and .tempered_glass_visual_contract.readability_failure_count == 0
  and .tempered_glass_visual_contract.visible_audit_failure_count == 0
  and .secondary_product_surfaces.status == "ready"
  and .secondary_product_surfaces.case_count == 15
  and .secondary_product_surfaces.total_action_instance_count == 57
  and .secondary_product_surfaces.harsh_action_failure_count == 0
  and .secondary_product_surfaces.text_clipping_failure_count == 0
  and .secondary_product_surfaces.content_edge_failure_count == 0
  and .mobile_safe_area_keyboard.status == "ready"
  and .mobile_safe_area_keyboard.evidence_scope == "html_fixture_simulation"
  and .mobile_safe_area_keyboard.content_clipping_failure_count == 0
' "$NATIVE_FIXTURE_REPORT" >/dev/null

jq -e '
  .status == "ready"
  and .local_packaging_gate_ready == true
  and .local_unsigned_app_bundle_probe_ready == true
  and .local_unsigned_app_bundle.ready == true
  and .local_unsigned_app_bundle.bundle_identifier == "ai.hepta.nativeapp"
  and .local_unsigned_app_bundle.mach_o_binary_ready == true
  and .local_unsigned_app_bundle.distribution_signed == false
  and .local_unsigned_app_bundle.distribution_notarized == false
  and .local_unsigned_app_bundle.distribution_stapled == false
  and .local_unsigned_app_bundle.public_distribution_artifact_written == false
  and .signing_notarization_deferred == true
  and .public_distribution_artifact_written == false
  and .public_ga_ready == false
' "$PACKAGING_REPORT" >/dev/null

jq -e '
  .status == "ready"
  and .distribution_preflight_gate_ready == true
  and .distribution_static_contract_ready == true
  and .local_distribution_tooling_ready == true
  and .public_distribution_ready == false
  and .app_signed == false
  and .app_notarized == false
  and .app_stapled == false
  and .credential_values_read == false
  and .network_call_performed == false
  and .notary_submission_performed == false
  and .public_distribution_artifact_written == false
  and .side_effects.filesystem_written == false
  and .side_effects.credential_value_read == false
  and .side_effects.network_call_performed == false
  and .side_effects.notary_submission_performed == false
  and .side_effects.app_signed == false
  and .side_effects.app_notarized == false
  and .side_effects.app_stapled == false
  and .side_effects.public_distribution_artifact_written == false
  and .side_effects.external_send_performed == false
  and .side_effects.provider_invoked == false
  and .side_effects.channel_send_performed == false
  and .side_effects.gateway_mutation_performed == false
' "$DISTRIBUTION_REPORT" >/dev/null

jq -e '
  .status == "ready"
  and .release_signing_capability_gate_ready == true
  and .claim_boundary.local_release_signing_capability_audit_ready == true
  and .claim_boundary.release_artifact_claim_ready == false
  and .claim_boundary.release_execution_ready == false
  and .claim_boundary.public_distribution_claim_ready == false
  and .claim_boundary.release_claim_ready == false
  and .side_effects.credential_value_captured == false
  and .side_effects.network_call_performed == false
  and .side_effects.notary_submission_performed == false
  and .side_effects.app_signed == false
  and .side_effects.app_notarized == false
  and .side_effects.app_stapled == false
  and .side_effects.public_distribution_artifact_written == false
  and .side_effects.external_mutation == false
' "$SIGNING_CAPABILITY_REPORT" >/dev/null

jq -e '
  .screenshot_manifest_ready == true
  and .screenshot_count.control_ui == 4
  and .screenshot_count.native == 41
  and .screenshot_count.native_true_window == 2
  and .screenshot_count.native_true_window_route == 4
  and .screenshot_count.native_true_window_route_mobile == 4
  and .screenshot_count.native_true_window_secondary == 5
  and .screenshot_count.native_true_window_secondary_mobile == 5
  and .screenshot_count.total == 65
  and .key_screenshot_count == 24
  and (.key_screenshots | length) == 24
' "$SCREENSHOT_MANIFEST" >/dev/null

jq -e '
  .status == "ready"
  and .enabled == true
  and .true_window_capture_performed == true
  and (.screenshots | length) == 2
  and (.screenshots | all(.visual_probe.ready == true and .viewport_contract.host_window_usable_ready == true))
  and .native_app_log_error_free == true
' "$WINDOW_BASE_REPORT" >/dev/null

jq -e '
  .status == "ready"
  and .enabled == true
  and .true_window_capture_performed == true
  and .route_count == 4
  and .route_screenshot_unique_count == 4
  and .route_screenshot_unique_ready == true
  and .route_top_design_referee_ready == true
  and .route_content_probe_ready == true
  and (.screenshots | length) == 4
  and (.screenshots | all(
    .visual_probe.ready == true
    and .visual_probe.route_content_ready == true
    and .viewport_contract.host_window_usable_ready == true
  ))
  and .native_app_log_error_free == true
  and .blocked_allowed == false
' "$WINDOW_ROUTE_REPORT" >/dev/null

jq -e '
  .status == "ready"
  and .enabled == true
  and .true_window_capture_performed == true
  and .route_count == 4
  and .route_screenshot_unique_count == 4
  and .route_screenshot_unique_ready == true
  and .native_makepad_mobile_route_variants_ready == true
  and .mobile_host_window_ready == true
  and .host_constrained_count == 4
  and .exact_390x844_ready == false
  and (.screenshots | length) == 4
  and (.screenshots | all(
    .visual_probe.ready == true
    and .visual_probe.mobile_route_content_ready == true
    and .viewport_contract.host_window_usable_ready == true
    and .viewport_contract.host_constrained == true
  ))
  and .native_app_log_error_free == true
  and .blocked_allowed == false
' "$WINDOW_ROUTE_MOBILE_REPORT" >/dev/null

jq -e '
  .status == "ready"
  and .enabled == true
  and .true_window_capture_performed == true
  and .surface_count == 5
  and .surface_screenshot_unique_count == 5
  and .surface_screenshot_unique_ready == true
  and (.screenshots | length) == 5
  and (.screenshots | all(.visual_probe.ready == true and .viewport_contract.host_window_usable_ready == true))
  and .native_app_log_error_free == true
  and .blocked_allowed == false
' "$WINDOW_SECONDARY_REPORT" >/dev/null

jq -e '
  .status == "ready"
  and .enabled == true
  and .true_window_capture_performed == true
  and .surface_count == 5
  and .surface_screenshot_unique_count == 5
  and .surface_screenshot_unique_ready == true
  and .native_makepad_secondary_mobile_surfaces_ready == true
  and .mobile_secondary_content_probe_ready == true
  and .mobile_secondary_content_visible_count >= 10
  and .mobile_host_window_ready == true
  and .host_constrained_count == 5
  and .exact_390x844_ready == false
  and (.screenshots | length) == 5
  and (.screenshots | all(
    .visual_probe.ready == true
    and .visual_probe.mobile_secondary_content_ready == true
    and .viewport_contract.host_window_usable_ready == true
    and .viewport_contract.host_constrained == true
  ))
  and .native_app_log_error_free == true
  and .blocked_allowed == false
' "$WINDOW_SECONDARY_MOBILE_REPORT" >/dev/null

control_screenshots_json='[]'
while IFS=$'\t' read -r name viewport expected_sha; do
  path="$READINESS_DIR/control-ui-browser/$name.png"
  verify_artifact "$path" "" "$expected_sha"
  bytes="$(file_bytes "$path")"
  control_screenshots_json="$(jq -nc \
    --argjson current "$control_screenshots_json" \
    --arg name "$name" \
    --arg viewport "$viewport" \
    --arg path "$path" \
    --argjson bytes "$bytes" \
    --arg sha256 "$expected_sha" \
    '$current + [{name:$name,viewport:$viewport,path:$path,bytes:$bytes,sha256:$sha256}]')"
done < <(jq -r '.screenshots[] | [.name,.viewport,.sha256] | @tsv' "$CONTROL_REPORT")

if ! jq -e 'length == 4 and ([.[].name] | sort) == ["desktop","mobile","narrow","phone320"]' <<<"$control_screenshots_json" >/dev/null; then
  echo "Control screenshot evidence set is incomplete" >&2
  exit 1
fi

v7_screenshots_json='[]'
while IFS=$'\t' read -r name path expected_bytes expected_sha; do
  verify_artifact "$path" "$expected_bytes" "$expected_sha"
  v7_screenshots_json="$(append_file_record "$v7_screenshots_json" "$name" "$path" "$expected_bytes" "$expected_sha")"
done < <(jq -r '
  [
    .control_real_click_activation.viewports[].targets[].screenshot,
    .control_real_click_activation.viewports[].mobile_pane_routes.screenshots[]
  ][] | [.name,.path,.bytes,.sha256] | @tsv
' "$CONTROL_V7_REPORT")

if ! jq -e 'length == 32 and ([.[].path] | unique | length) == 32 and ([.[].sha256] | unique | length) == 32' <<<"$v7_screenshots_json" >/dev/null; then
  echo "Control v7 screenshot evidence set is incomplete or duplicated" >&2
  exit 1
fi

native_fixture_screenshots_json='[]'
native_fixture_count=0
while IFS= read -r path; do
  assert_evidence_path "$path"
  bytes="$(file_bytes "$path")"
  if [[ "$bytes" -lt 10000 ]]; then
    echo "Native fixture screenshot is too small: $path" >&2
    exit 1
  fi
  sha="$(file_sha256 "$path")"
  name="$(basename "$path" .png)"
  native_fixture_screenshots_json="$(append_file_record "$native_fixture_screenshots_json" "$name" "$path" "$bytes" "$sha")"
  native_fixture_count=$((native_fixture_count + 1))
done < <(find "$READINESS_DIR/native-fixture" -maxdepth 1 -type f -name '*.png' | LC_ALL=C sort)

if [[ "$native_fixture_count" != "41" ]]; then
  echo "Native fixture screenshot count mismatch: expected 41, found $native_fixture_count" >&2
  exit 1
fi

key_screenshots_json='[]'
while IFS=$'\t' read -r name path expected_bytes expected_sha; do
  verify_artifact "$path" "$expected_bytes" "$expected_sha"
  key_screenshots_json="$(append_file_record "$key_screenshots_json" "$name" "$path" "$expected_bytes" "$expected_sha")"
done < <(jq -r '.key_screenshots[] | [.name,.path,.bytes,.sha256] | @tsv' "$SCREENSHOT_MANIFEST")

if ! jq -e 'length == 24' <<<"$key_screenshots_json" >/dev/null; then
  echo "key screenshot evidence count mismatch" >&2
  exit 1
fi

window_screenshots_json='[]'
while IFS=$'\t' read -r name path expected_bytes expected_sha; do
  verify_artifact "$path" "$expected_bytes" "$expected_sha"
  window_screenshots_json="$(append_file_record "$window_screenshots_json" "$name" "$path" "$expected_bytes" "$expected_sha")"
done < <(jq -sr '.[] | .screenshots[] | [.name,.path,.bytes,.sha256] | @tsv' \
  "$WINDOW_BASE_REPORT" \
  "$WINDOW_ROUTE_REPORT" \
  "$WINDOW_ROUTE_MOBILE_REPORT" \
  "$WINDOW_SECONDARY_REPORT" \
  "$WINDOW_SECONDARY_MOBILE_REPORT")

if ! jq -e '
  length == 20
  and ([.[].path] | unique | length) == 20
  and ([.[].sha256] | unique | length) == 19
  and (
    group_by(.sha256)
    | map(select(length > 1))
    | length == 1
  )
  and (
    group_by(.sha256)
    | map(select(length > 1))[0]
    | ([.[].name] | sort) == ["mobile-route-home","mobile-window"]
    and ([.[].bytes] | unique | length) == 1
  )
' <<<"$window_screenshots_json" >/dev/null; then
  echo "true-window capture uniqueness contract failed" >&2
  exit 1
fi

source_files=(
  design-tokens/hepta-light-glass.tokens.json
  apps/hepta-control-ui/light-glass-tokens.generated.css
  apps/hepta-control-ui/index.html
  apps/hepta-control-ui/styles.css
  apps/hepta-control-ui/styles.foundation.css
  apps/hepta-control-ui/styles.components.css
  apps/hepta-control-ui/styles.responsive.css
  apps/hepta-control-ui/styles.accessibility.css
  apps/hepta-native/src/shared/light_glass_tokens.rs
  apps/hepta-native/src/home/hepta_fixture_cockpit.rs
  codex-rs/hepta-core/src/control_ui.rs
  scripts/hepta-ui-design-system-gate.sh
  scripts/hepta-ui-harsh-top-design-referee-v7-real-click-gate.sh
  scripts/hepta-native-window-smoke.sh
  scripts/hepta-native-window-route-smoke.sh
  scripts/hepta-native-window-route-mobile-smoke.sh
  scripts/hepta-native-window-secondary-smoke.sh
  scripts/hepta-native-window-secondary-mobile-smoke.sh
  scripts/hepta-ui-current-source-readiness-gate.sh
)

source_fingerprint_json='[]'
for path in "${source_files[@]}"; do
  if [[ ! -f "$path" ]]; then
    echo "missing current-source fingerprint input: $path" >&2
    exit 1
  fi
  bytes="$(file_bytes "$path")"
  sha="$(file_sha256 "$path")"
  source_fingerprint_json="$(append_file_record "$source_fingerprint_json" "$path" "$path" "$bytes" "$sha")"
done

source_commit="$(git rev-parse HEAD)"
source_branch="$(git branch --show-current)"
if [[ -z "$(git status --short)" ]]; then
  source_worktree_clean=true
else
  source_worktree_clean=false
fi
generated_at="$(date -u +%Y-%m-%dT%H:%M:%SZ)"

report_tmp="$REPORT_PATH.tmp.$$"
trap 'rm -f "$report_tmp"' EXIT

jq -n \
  --arg schema_version "hepta-ui-current-source-readiness/v1" \
  --arg generated_at "$generated_at" \
  --arg readiness_dir "$READINESS_DIR" \
  --arg report_path "$REPORT_PATH" \
  --arg source_commit "$source_commit" \
  --arg source_branch "$source_branch" \
  --argjson source_worktree_clean "$source_worktree_clean" \
  --argjson report_inputs "$report_inputs_json" \
  --argjson source_fingerprint "$source_fingerprint_json" \
  --argjson control_screenshots "$control_screenshots_json" \
  --argjson v7_screenshots "$v7_screenshots_json" \
  --argjson native_fixture_screenshots "$native_fixture_screenshots_json" \
  --argjson key_screenshots "$key_screenshots_json" \
  --argjson window_screenshots "$window_screenshots_json" \
  --slurpfile static "$STATIC_REPORT" \
  --slurpfile design "$DESIGN_REPORT" \
  --slurpfile control "$CONTROL_REPORT" \
  --slurpfile v7 "$CONTROL_V7_REPORT" \
  --slurpfile native "$NATIVE_FIXTURE_REPORT" \
  --slurpfile packaging "$PACKAGING_REPORT" \
  --slurpfile distribution "$DISTRIBUTION_REPORT" \
  --slurpfile signing "$SIGNING_CAPABILITY_REPORT" \
  --slurpfile manifest "$SCREENSHOT_MANIFEST" \
  --slurpfile route_mobile "$WINDOW_ROUTE_MOBILE_REPORT" \
  --slurpfile secondary_mobile "$WINDOW_SECONDARY_MOBILE_REPORT" \
  '
  ($static[0]) as $static
  | ($design[0]) as $design
  | ($control[0]) as $control
  | ($v7[0]) as $v7
  | ($native[0]) as $native
  | ($packaging[0]) as $packaging
  | ($distribution[0]) as $distribution
  | ($signing[0]) as $signing
  | ($manifest[0]) as $manifest
  | ($route_mobile[0]) as $route_mobile
  | ($secondary_mobile[0]) as $secondary_mobile
  | {
      schema_version:$schema_version,
      generated_at:$generated_at,
      product:"Hepta UI",
      status:"ready",
      readiness_kind:"ui_lane_current_source",
      readiness_dir:$readiness_dir,
      report_path:$report_path,
      source:{
        branch:$source_branch,
        commit:$source_commit,
        worktree_clean:$source_worktree_clean,
        fingerprint:$source_fingerprint
      },
      ui_lane_ready:true,
      control_ui_ready:true,
      native_ui_ready:true,
      current_source_evidence_ready:true,
      full_product_ready:false,
      backend_live_adapter_ready:false,
      public_ga_ready:false,
      design_system:{
        status:$design.status,
        token_source:$design.token_source,
        generated_token_sync_ready:$design.generated_token_sync_ready,
        minimum_dim_text_contrast:$design.dim_text_contrast.minimum,
        css_layer_count:$design.control.css_layer_count,
        runtime_css_bytes:$design.control.runtime_css_bytes,
        important_count:$design.control.important_count,
        accessibility_media_queries_ready:$design.control.accessibility_media_queries_ready,
        robrix_upstream_commit:$design.robrix.upstream_commit,
        robrix_selective_module_count:$design.robrix.selective_module_count,
        robrix_license:$design.robrix.license
      },
      control:{
        status:$control.status,
        viewport_count:($control.screenshots | length),
        viewports:[$control.screenshots[].viewport],
        shallow_contract:{
          shallow_light_glass_ready:$control.control_ui_shallow_light_glass_ready,
          light_theme_semantics_ready:$control.control_ui_light_theme_semantics_ready,
          stable_content_surface_ready:$control.control_ui_stable_content_surface_ready,
          native_popover_interaction_ready:$control.control_ui_native_popover_interaction_ready,
          shallow_floating_surface_ready:$control.control_ui_shallow_floating_surface_ready,
          restrained_optics_ready:$control.control_ui_restrained_optics_ready,
          restrained_mobile_metadata_ready:$control.control_ui_restrained_mobile_metadata_ready,
          key_touch_controls_ready:$control.control_ui_key_touch_controls_ready
        },
        real_click:{
          status:$v7.status,
          target_count:$v7.control_real_click_activation.target_count,
          screenshot_count:$v7.control_real_click_activation.screenshot_count,
          failure_count:$v7.control_real_click_activation.failure_count,
          mobile_route_count:$v7.control_real_click_activation.mobile_route_count,
          popover_switch_step_count:$v7.control_real_click_activation.popover_switch_step_count
        },
        screenshots:$control_screenshots,
        real_click_screenshots:$v7_screenshots
      },
      native:{
        fixture:{
          status:$native.status,
          screenshot_count:$native.screenshot_count,
          selected_row_variant_count:$native.selected_row_variant_count,
          selected_row_unique_count:$native.selected_row_unique_count,
          route_variant_unique_count:$native.route_variant_unique_count,
          mobile_route_variant_unique_count:$native.mobile_route_variant_unique_count,
          secondary_surface_case_count:$native.secondary_product_surfaces.case_count,
          secondary_action_instance_count:$native.secondary_product_surfaces.total_action_instance_count,
          minimum_contrast:$native.tempered_glass_visual_contract.min_contrast_ratio,
          mobile_safe_area_keyboard_scope:$native.mobile_safe_area_keyboard.evidence_scope,
          screenshots:$native_fixture_screenshots
        },
        true_windows:{
          status:"ready",
          capture_count:($window_screenshots | length),
          unique_path_count:($window_screenshots | map(.path) | unique | length),
          unique_sha256_count:($window_screenshots | map(.sha256) | unique | length),
          expected_duplicate:{
            names:["mobile-route-home","mobile-window"],
            reason:"The base mobile capture and the Home mobile-route capture intentionally select the same 390x844-requested state."
          },
          groups:{
            base:2,
            desktop_routes:4,
            mobile_routes:4,
            desktop_secondary:5,
            mobile_secondary:5
          },
          requested_mobile_bounds:$route_mobile.requested_mobile_bounds,
          mobile_actual_contract:"390x820 visible Makepad window inside a 390x844 macOS host request",
          mobile_host_constrained:true,
          mobile_route_host_constrained_count:$route_mobile.host_constrained_count,
          mobile_secondary_host_constrained_count:$secondary_mobile.host_constrained_count,
          real_device_claim_ready:false,
          screenshots:$window_screenshots
        }
      },
      screenshot_census:{
        manifest:$manifest.screenshot_count,
        total:$manifest.screenshot_count.total,
        key_screenshot_count:$manifest.key_screenshot_count,
        key_screenshots:$key_screenshots
      },
      packaging:{
        local_unsigned_app_ready:$packaging.local_unsigned_app_bundle.ready,
        bundle_identifier:$packaging.local_unsigned_app_bundle.bundle_identifier,
        bundle_bytes:$packaging.local_unsigned_app_bundle.bundle_bytes,
        distribution_signed:$packaging.local_unsigned_app_bundle.distribution_signed,
        distribution_notarized:$packaging.local_unsigned_app_bundle.distribution_notarized,
        distribution_stapled:$packaging.local_unsigned_app_bundle.distribution_stapled,
        public_distribution_ready:$distribution.public_distribution_ready,
        public_distribution_artifact_written:$distribution.public_distribution_artifact_written,
        release_execution_ready:$signing.claim_boundary.release_execution_ready
      },
      source_contract:{
        declared_marker_count:$static.declared_marker_count,
        verified_ui_marker_count:$static.marker_count,
        deferred_backend_marker_count:$static.deferred_backend_marker_count,
        deferred_backend_marker_files:$static.deferred_backend_marker_files,
        backend_live_adapter_source_ready:$static.backend_live_adapter_source_ready
      },
      legacy_full_root_chain:{
        status:"not_ready",
        last_blocking_gate:"hepta_ui_top_design_referee_refresh_gate",
        reason:"The legacy refresh still requires retired extreme prismatic, caustic, pill, and micro-glass diagnostics. Current-source readiness uses the frozen shallow/restrained 2026 contract and does not relabel those retired fields as true."
      },
      open_boundaries:[
        "search_upload_media_voice_account_live_backend_adapters",
        "voiceover_talkback_platform_nodes",
        "system_dynamic_type",
        "rtl_makepad_mirroring",
        "real_mobile_safe_area_and_keyboard",
        "low_power_device_gpu_memory_frame_time_and_battery",
        "apple_signing_notarization_stapling_and_public_distribution",
        "independent_third_party_aesthetic_certification"
      ],
      claim_boundary:{
        ui_lane_current_source_ready:true,
        local_unsigned_app_ready:true,
        live_backend_functionality_ready:false,
        real_device_lab_ready:false,
        signed_notarized_stapled_ready:false,
        public_distribution_ready:false,
        public_ga_ready:false,
        external_mutation_performed:false
      },
      input_reports:$report_inputs
    }
  ' >"$report_tmp"

jq -e '
  .status == "ready"
  and .readiness_kind == "ui_lane_current_source"
  and .ui_lane_ready == true
  and .control_ui_ready == true
  and .native_ui_ready == true
  and .current_source_evidence_ready == true
  and .full_product_ready == false
  and .backend_live_adapter_ready == false
  and .public_ga_ready == false
  and .design_system.minimum_dim_text_contrast >= 4.8
  and .design_system.robrix_selective_module_count == 6
  and .control.viewport_count == 4
  and .control.real_click.target_count == 26
  and .control.real_click.screenshot_count == 32
  and .control.real_click.failure_count == 0
  and .native.fixture.screenshot_count == 41
  and .native.fixture.secondary_surface_case_count == 15
  and .native.fixture.secondary_action_instance_count == 57
  and .native.true_windows.capture_count == 20
  and .native.true_windows.unique_path_count == 20
  and .native.true_windows.unique_sha256_count == 19
  and .native.true_windows.real_device_claim_ready == false
  and .screenshot_census.total == 65
  and .screenshot_census.key_screenshot_count == 24
  and .packaging.local_unsigned_app_ready == true
  and .packaging.distribution_signed == false
  and .packaging.distribution_notarized == false
  and .packaging.distribution_stapled == false
  and .packaging.public_distribution_ready == false
  and .source_contract.verified_ui_marker_count >= 3642
  and .source_contract.deferred_backend_marker_count == 357
  and .source_contract.backend_live_adapter_source_ready == false
  and .legacy_full_root_chain.status == "not_ready"
  and .claim_boundary.ui_lane_current_source_ready == true
  and .claim_boundary.live_backend_functionality_ready == false
  and .claim_boundary.real_device_lab_ready == false
  and .claim_boundary.signed_notarized_stapled_ready == false
  and .claim_boundary.public_distribution_ready == false
  and .claim_boundary.public_ga_ready == false
  and .claim_boundary.external_mutation_performed == false
  and (.input_reports | length) == 14
' "$report_tmp" >/dev/null

mv "$report_tmp" "$REPORT_PATH"
trap - EXIT
printf 'Hepta UI current-source readiness gate passed: %s\n' "$REPORT_PATH" >&2
printf '%s\n' "$REPORT_PATH"
