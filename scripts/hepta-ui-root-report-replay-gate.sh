#!/bin/bash -p
set +x
PS4='+ '
set -euo pipefail
unset BASH_ENV ENV CDPATH GLOBIGNORE RUBYOPT RUBYLIB GEM_HOME GEM_PATH BUNDLE_GEMFILE BUNDLE_PATH
SYSTEM_PATH="/usr/bin:/bin:/usr/sbin:/sbin"
PATH="$SYSTEM_PATH"
export PATH

cd "$(/usr/bin/dirname "$0")/.."
REPO_ROOT="$(pwd -P)"
. "$REPO_ROOT/scripts/lib/hepta-safe-managed-output-v1.sh"

READINESS_DIR="${HEPTA_UI_PRODUCT_READINESS_DIR:-/Users/qianqi/.openclaw/tmp/hepta-ui-product-readiness.mention-taxonomy-20260615}"
REPORT_PATH="${HEPTA_UI_ROOT_REPORT_REPLAY_REPORT_PATH:-$READINESS_DIR/ui-root-report-replay-gate.json}"
READINESS_DIR="$(hepta_safe_normalize_path readiness "$READINESS_DIR")"
REPORT_PATH="$(hepta_safe_normalize_path report "$REPORT_PATH")"
REPORT_PARENT="$(hepta_safe_normalize_path report_parent "$(/usr/bin/dirname "$REPORT_PATH")")"
hepta_safe_require_directory_target readiness "$READINESS_DIR"
hepta_safe_require_directory_target report_parent "$REPORT_PARENT"
hepta_safe_require_regular_target report "$REPORT_PATH"
if hepta_safe_paths_overlap "$READINESS_DIR" "$REPO_ROOT"; then
  printf 'root-report readiness must not overlap the repository\n' >&2
  exit 64
fi
if [[ "$REPORT_PATH" != "$READINESS_DIR/ui-root-report-replay-gate.json" \
  || "$REPORT_PARENT" != "$READINESS_DIR" ]]; then
  printf 'root-report output must be the canonical fixed readiness leaf\n' >&2
  exit 64
fi
mkdir -p "$REPORT_PARENT"
hepta_safe_revalidate_directory report_parent "$REPORT_PARENT"

# Clear stale evidence only when the canonical leaf proves that this gate owns
# it. Unknown, malformed, hard-linked, or differently-owned files are retained
# and fail closed instead of being treated as disposable gate output.
hepta_safe_unlink_owned_json_target_if_present "$REPORT_PATH" root_report \
  gate hepta_ui_root_report_replay_gate \
  report_path "$REPORT_PATH" \
  product "Hepta UI" \
  runtime hepta

STATIC_CONTRACT_PATH="$READINESS_DIR/static-contract.json"
DESIGN_SYSTEM_REPORT_PATH="$READINESS_DIR/ui-design-system-gate.json"
CONTROL_BROWSER_REPORT_PATH="$READINESS_DIR/control-ui-browser-smoke.json"
CONTROL_REAL_CLICK_V7_REPORT_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v7-real-click-gate.json"
NATIVE_FIXTURE_REPORT_PATH="$READINESS_DIR/native-fixture/native-fixture-visual-smoke.json"
NATIVE_PACKAGING_REPORT_PATH="$READINESS_DIR/native-packaging-gate.json"
NATIVE_DISTRIBUTION_PREFLIGHT_REPORT_PATH="$READINESS_DIR/native-distribution-preflight-gate.json"
NATIVE_WINDOW_REPORT_PATH="$READINESS_DIR/native-window-smoke.json"
NATIVE_WINDOW_ROUTE_REPORT_PATH="$READINESS_DIR/native-window-routes-smoke.json"
NATIVE_WINDOW_ROUTE_MOBILE_REPORT_PATH="$READINESS_DIR/native-window-routes-mobile-smoke.json"
NATIVE_WINDOW_SECONDARY_REPORT_PATH="$READINESS_DIR/native-window-secondary-smoke.json"
NATIVE_WINDOW_SECONDARY_MOBILE_REPORT_PATH="$READINESS_DIR/native-window-secondary-mobile-smoke.json"
SCREENSHOT_MANIFEST_PATH="$READINESS_DIR/screenshot-manifest.json"
BASE_GAP_DRILLDOWN_PATH="$READINESS_DIR/native-base-gap-drilldown.json"
BASE_GAP_WORK_QUEUE_PATH="$READINESS_DIR/native-base-gap-work-queue.json"
BASE_GAP_BACKEND_HANDOFF_PATH="$READINESS_DIR/native-base-gap-backend-handoff.json"
BACKEND_CONTRACT_GATES_REPORT_PATH="$READINESS_DIR/native-backend-contract-gates.json"
NON_BASE_EDGE_GATES_REPORT_PATH="$READINESS_DIR/native-non-base-edge-gates.json"
PRODUCTIZATION_ROLLUP_REPORT_PATH="$READINESS_DIR/native-productization-blocker-rollup.json"
PLAN_BOUNDARY_REPORT_PATH="$READINESS_DIR/ui-plan-boundary-gate.json"
DEMO_EVIDENCE_REPORT_PATH="$READINESS_DIR/ui-demo-evidence-gate.json"
EVIDENCE_BUNDLE_REPORT_PATH="$READINESS_DIR/ui-evidence-bundle-gate.json"
EVIDENCE_ARCHIVE_REPORT_PATH="$READINESS_DIR/ui-evidence-archive-gate.json"
RELEASE_OPERATOR_DRY_RUN_REPORT_PATH="$READINESS_DIR/ui-release-operator-dry-run-gate.json"
OPERATOR_BRIEFING_REPORT_PATH="$READINESS_DIR/ui-operator-briefing-gate.json"
BACKEND_PROMOTION_PACKET_REPORT_PATH="$READINESS_DIR/ui-backend-promotion-packet-gate.json"
BACKEND_ALIGNMENT_EVIDENCE_REPORT_PATH="$READINESS_DIR/ui-backend-alignment-evidence-gate.json"
CRITICAL_PATH_PLAN_REPORT_PATH="$READINESS_DIR/ui-critical-path-plan-gate.json"
BACKEND_CONTRACT_ACCEPTANCE_REPORT_PATH="$READINESS_DIR/ui-backend-contract-acceptance-gate.json"
BACKEND_HANDOFF_EXPORT_REPORT_PATH="$READINESS_DIR/ui-backend-handoff-export-gate.json"
BACKEND_DISPATCH_PACKET_REPORT_PATH="$READINESS_DIR/ui-backend-dispatch-packet-gate.json"
BACKEND_RECEIPT_INTAKE_REPORT_PATH="$READINESS_DIR/ui-backend-receipt-intake-gate.json"
BACKEND_RECEIPT_ROUNDTRIP_REPORT_PATH="$READINESS_DIR/ui-backend-receipt-roundtrip-gate.json"
BACKEND_RECEIPT_REFRESH_LOCK_REPORT_PATH="$READINESS_DIR/ui-backend-receipt-refresh-lock-gate.json"
FUTURE_PLAN_REFRESH_REPORT_PATH="$READINESS_DIR/ui-future-plan-refresh-gate.json"
OPERATOR_BRIEFING_REFRESH_REPORT_PATH="$READINESS_DIR/ui-operator-briefing-refresh-gate.json"
RELEASE_APPROVAL_INTAKE_REPORT_PATH="$READINESS_DIR/ui-release-approval-intake-gate.json"
TOP_DESIGN_REFEREE_REFRESH_REPORT_PATH="$READINESS_DIR/ui-top-design-referee-refresh-gate.json"
RELEASE_ARTIFACT_BOUNDARY_REPORT_PATH="$READINESS_DIR/ui-release-artifact-boundary-gate.json"
RELEASE_ARTIFACT_INTAKE_REPORT_PATH="$READINESS_DIR/ui-release-artifact-intake-gate.json"
RELEASE_ARTIFACT_ROUNDTRIP_REPORT_PATH="$READINESS_DIR/ui-release-artifact-roundtrip-gate.json"
CURRENT_PLAN_REFRESH_REPORT_PATH="$READINESS_DIR/ui-current-plan-refresh-gate.json"
BLOCKER_CLOSURE_REPORT_PATH="$READINESS_DIR/ui-blocker-closure-gate.json"
BACKEND_DELIVERY_AUDIT_REPORT_PATH="$READINESS_DIR/ui-backend-delivery-audit-gate.json"
BACKEND_DELIVERY_RECEIPT_ROUNDTRIP_REPORT_PATH="$READINESS_DIR/ui-backend-delivery-receipt-roundtrip-gate.json"
RISK_FUTURE_PLAN_REPORT_PATH="$READINESS_DIR/ui-risk-future-plan-gate.json"

STRICT_CURRENT_SOURCE_RAW="${HEPTA_UI_PRODUCT_READINESS_STRICT_CURRENT_SOURCE:-0}"
case "$STRICT_CURRENT_SOURCE_RAW" in
  1 | true | TRUE | yes | YES | on | ON)
    STRICT_CURRENT_SOURCE_MODE=1
    ;;
  0 | false | FALSE | no | NO | off | OFF | "")
    STRICT_CURRENT_SOURCE_MODE=0
    ;;
  *)
    printf 'Invalid HEPTA_UI_PRODUCT_READINESS_STRICT_CURRENT_SOURCE value: %s\n' "$STRICT_CURRENT_SOURCE_RAW" >&2
    exit 2
    ;;
esac

if [[ "$STRICT_CURRENT_SOURCE_MODE" == "1" ]]; then
  TRUE_WINDOW_REPORT_KIND="strict_current_source_true_window"
else
  TRUE_WINDOW_REPORT_KIND="optional_true_window"
fi

HEPTA_UI_GATE_REQUIREMENT_CONTEXT="the Hepta UI root-report replay gate"
HEPTA_UI_REPORT_INPUT_LABEL="root-report replay"
source scripts/lib/hepta-ui-gate-common-v1.sh

validate_true_window_screenshot_file_set() {
  local report_path="$1"
  local expected_count="$2"
  local evidence_label="$3"
  local readiness_real_path
  local screenshot_count
  local screenshot_index
  local screenshot_path
  local screenshot_real_path
  local reported_bytes
  local actual_bytes
  local reported_sha
  local actual_sha

  if ! readiness_real_path="$(realpath "$READINESS_DIR" 2>/dev/null)"; then
    printf 'Strict current-source screenshot root is not resolvable: %s\n' "$READINESS_DIR" >&2
    return 1
  fi

  screenshot_count="$(jq -r '(.screenshots // []) | length' "$report_path")"
  if [[ "$screenshot_count" != "$expected_count" ]]; then
    printf 'Strict current-source %s screenshot count mismatch: expected %s, found %s\n' \
      "$evidence_label" "$expected_count" "$screenshot_count" >&2
    return 1
  fi

  for ((screenshot_index = 0; screenshot_index < expected_count; screenshot_index += 1)); do
    if ! screenshot_path="$(jq -er --argjson index "$screenshot_index" \
      '.screenshots[$index].path | select(type == "string" and length > 0)' "$report_path")"; then
      printf 'Strict current-source %s screenshot %s has no valid path\n' \
        "$evidence_label" "$screenshot_index" >&2
      return 1
    fi
    if ! reported_bytes="$(jq -er --argjson index "$screenshot_index" \
      '.screenshots[$index].bytes | select(type == "number" and . >= 0 and . == floor) | tostring' "$report_path")"; then
      printf 'Strict current-source %s screenshot %s has no valid byte count\n' \
        "$evidence_label" "$screenshot_index" >&2
      return 1
    fi
    if ! reported_sha="$(jq -er --argjson index "$screenshot_index" \
      '.screenshots[$index].sha256 | select(type == "string" and test("^[0-9a-f]{64}$"))' "$report_path")"; then
      printf 'Strict current-source %s screenshot %s has no valid SHA-256\n' \
        "$evidence_label" "$screenshot_index" >&2
      return 1
    fi

    if [[ ! -f "$screenshot_path" || ! -s "$screenshot_path" ]]; then
      printf 'Strict current-source %s screenshot is missing or empty: %s\n' \
        "$evidence_label" "$screenshot_path" >&2
      return 1
    fi
    if ! screenshot_real_path="$(realpath "$screenshot_path" 2>/dev/null)"; then
      printf 'Strict current-source %s screenshot path is not resolvable: %s\n' \
        "$evidence_label" "$screenshot_path" >&2
      return 1
    fi
    case "$screenshot_real_path" in
      "$readiness_real_path"/*)
        ;;
      *)
        printf 'Strict current-source %s screenshot escapes readiness directory: %s\n' \
          "$evidence_label" "$screenshot_real_path" >&2
        return 1
        ;;
    esac

    actual_bytes="$(wc -c <"$screenshot_real_path" | tr -d ' ')"
    if [[ "$actual_bytes" != "$reported_bytes" || "$actual_bytes" -lt 10000 ]]; then
      printf 'Strict current-source %s screenshot byte mismatch: %s (reported %s, actual %s)\n' \
        "$evidence_label" "$screenshot_real_path" "$reported_bytes" "$actual_bytes" >&2
      return 1
    fi

    actual_sha="$(file_sha256 "$screenshot_real_path")"
    if [[ "$actual_sha" != "$reported_sha" ]]; then
      printf 'Strict current-source %s screenshot SHA-256 mismatch: %s\n' \
        "$evidence_label" "$screenshot_real_path" >&2
      return 1
    fi
  done
}

validate_strict_current_source_inputs() {
  if [[ "$STRICT_CURRENT_SOURCE_MODE" != "1" ]]; then
    return
  fi

  require_report "$DESIGN_SYSTEM_REPORT_PATH"
  require_report "$NATIVE_WINDOW_REPORT_PATH"
  require_report "$NATIVE_WINDOW_ROUTE_REPORT_PATH"
  require_report "$NATIVE_WINDOW_ROUTE_MOBILE_REPORT_PATH"
  require_report "$NATIVE_WINDOW_SECONDARY_REPORT_PATH"
  require_report "$NATIVE_WINDOW_SECONDARY_MOBILE_REPORT_PATH"

  if ! jq -e -n \
    --slurpfile design_system_file "$DESIGN_SYSTEM_REPORT_PATH" \
    --slurpfile native_window_file "$NATIVE_WINDOW_REPORT_PATH" \
    --slurpfile native_window_route_file "$NATIVE_WINDOW_ROUTE_REPORT_PATH" \
    --slurpfile native_window_route_mobile_file "$NATIVE_WINDOW_ROUTE_MOBILE_REPORT_PATH" \
    --slurpfile native_window_secondary_file "$NATIVE_WINDOW_SECONDARY_REPORT_PATH" \
    --slurpfile native_window_secondary_mobile_file "$NATIVE_WINDOW_SECONDARY_MOBILE_REPORT_PATH" \
    '
    ($design_system_file[0]) as $design_system
    | ($native_window_file[0]) as $native_window
    | ($native_window_route_file[0]) as $native_window_route
    | ($native_window_route_mobile_file[0]) as $native_window_route_mobile
    | ($native_window_secondary_file[0]) as $native_window_secondary
    | ($native_window_secondary_mobile_file[0]) as $native_window_secondary_mobile
    | def screenshot_files_ready($items; $expected):
        ($items | length) == $expected
        and ($items | all(
          (.bytes // 0) >= 10000
          and ((.sha256 // "") | test("^[0-9a-f]{64}$"))
          and ((.path // "") | length) > 0
          and .visual_probe.ready == true
        ))
        and ($items | map(.sha256 // "") | unique | length) == $expected;
      ($design_system.status == "ready")
      and ($design_system.rust_toolchain | test("^rustc 1\\.95\\.0([[:space:]]|$)"))
      and $native_window.enabled == true
      and $native_window.status == "ready"
      and ($native_window.blocked_allowed // false) != true
      and $native_window.true_window_capture_performed == true
      and $native_window.fixture_product_shell_selected_ready == true
      and $native_window.fixture_matrix_composer_hidden_ready == true
      and $native_window.fixture_desktop_product_layout_ready == true
      and $native_window.fixture_mobile_task_first_layout_ready == true
      and $native_window.native_makepad_fixture_script_error_free == true
      and $native_window.native_app_log_error_free == true
      and screenshot_files_ready(($native_window.screenshots // []); 2)
      and $native_window.side_effects.external_mutation == false
      and $native_window_route.enabled == true
      and $native_window_route.status == "ready"
      and ($native_window_route.blocked_allowed // false) != true
      and $native_window_route.true_window_capture_performed == true
      and $native_window_route.native_makepad_route_variants_ready == true
      and $native_window_route.route_top_design_referee_ready == true
      and $native_window_route.route_content_probe_ready == true
      and $native_window_route.route_count == 4
      and $native_window_route.screenshot_count == 4
      and $native_window_route.route_screenshot_unique_count == 4
      and $native_window_route.route_screenshot_unique_ready == true
      and $native_window_route.native_app_log_error_free == true
      and screenshot_files_ready(($native_window_route.screenshots // []); 4)
      and (($native_window_route.screenshots // []) | all(.visual_probe.route_content_ready == true))
      and $native_window_route.side_effects.external_mutation == false
      and $native_window_route_mobile.enabled == true
      and $native_window_route_mobile.status == "ready"
      and ($native_window_route_mobile.blocked_allowed // false) != true
      and $native_window_route_mobile.true_window_capture_performed == true
      and $native_window_route_mobile.native_makepad_mobile_route_variants_ready == true
      and $native_window_route_mobile.route_count == 4
      and $native_window_route_mobile.screenshot_count == 4
      and $native_window_route_mobile.route_screenshot_unique_count == 4
      and $native_window_route_mobile.route_screenshot_unique_ready == true
      and $native_window_route_mobile.non_home_content_log_signature_count >= 3
      and $native_window_route_mobile.mobile_host_window_ready == true
      and $native_window_route_mobile.native_app_log_error_free == true
      and screenshot_files_ready(($native_window_route_mobile.screenshots // []); 4)
      and (($native_window_route_mobile.screenshots // []) | all(
        .viewport_contract.expected_width == 390
        and .viewport_contract.expected_height == 844
        and .viewport_contract.host_window_usable_ready == true
        and .visual_probe.mobile_route_content_ready == true
      ))
      and $native_window_route_mobile.side_effects.external_mutation == false
      and $native_window_secondary.enabled == true
      and $native_window_secondary.status == "ready"
      and ($native_window_secondary.blocked_allowed // false) != true
      and $native_window_secondary.true_window_capture_performed == true
      and $native_window_secondary.native_makepad_secondary_surfaces_ready == true
      and $native_window_secondary.surface_count == 5
      and $native_window_secondary.screenshot_count == 5
      and $native_window_secondary.surface_screenshot_unique_count == 5
      and $native_window_secondary.surface_screenshot_unique_ready == true
      and $native_window_secondary.native_app_log_error_free == true
      and screenshot_files_ready(($native_window_secondary.screenshots // []); 5)
      and $native_window_secondary.side_effects.external_mutation == false
      and $native_window_secondary_mobile.enabled == true
      and $native_window_secondary_mobile.status == "ready"
      and ($native_window_secondary_mobile.blocked_allowed // false) != true
      and $native_window_secondary_mobile.true_window_capture_performed == true
      and $native_window_secondary_mobile.native_makepad_secondary_mobile_surfaces_ready == true
      and $native_window_secondary_mobile.mobile_secondary_content_probe_ready == true
      and $native_window_secondary_mobile.mobile_secondary_content_visible_count >= 5
      and $native_window_secondary_mobile.mobile_host_window_ready == true
      and $native_window_secondary_mobile.surface_count == 5
      and $native_window_secondary_mobile.screenshot_count == 5
      and $native_window_secondary_mobile.surface_screenshot_unique_count == 5
      and $native_window_secondary_mobile.surface_screenshot_unique_ready == true
      and $native_window_secondary_mobile.native_app_log_error_free == true
      and screenshot_files_ready(($native_window_secondary_mobile.screenshots // []); 5)
      and (($native_window_secondary_mobile.screenshots // []) | all(
        .viewport_contract.expected_width == 390
        and .viewport_contract.expected_height == 844
        and .viewport_contract.host_window_usable_ready == true
        and .visual_probe.mobile_secondary_content_ready == true
      ))
      and $native_window_secondary_mobile.side_effects.external_mutation == false
    ' >/dev/null; then
    printf 'Strict current-source root replay rejected the Rust/toolchain or five-report true-window evidence matrix\n' >&2
    exit 1
  fi

  validate_true_window_screenshot_file_set "$NATIVE_WINDOW_REPORT_PATH" 2 "main-window"
  validate_true_window_screenshot_file_set "$NATIVE_WINDOW_ROUTE_REPORT_PATH" 4 "desktop-route"
  validate_true_window_screenshot_file_set "$NATIVE_WINDOW_ROUTE_MOBILE_REPORT_PATH" 4 "mobile-route"
  validate_true_window_screenshot_file_set "$NATIVE_WINDOW_SECONDARY_REPORT_PATH" 5 "desktop-secondary"
  validate_true_window_screenshot_file_set "$NATIVE_WINDOW_SECONDARY_MOBILE_REPORT_PATH" 5 "mobile-secondary"
}

require_command jq
require_command shasum
require_command realpath
validate_strict_current_source_inputs
require_report "$CONTROL_REAL_CLICK_V7_REPORT_PATH"

TMP_DIR="$(mktemp -d "${TMPDIR:-/tmp}/hepta-ui-root-report-replay.XXXXXX")"
ROOT_REPORTS_NDJSON="$TMP_DIR/root-reports.ndjson"
ROOT_REPORTS_JSON="$TMP_DIR/root-reports.json"
REPORT_TMP="$TMP_DIR/root-report-replay-report.json"
trap 'rm -rf "$TMP_DIR"' EXIT

: >"$ROOT_REPORTS_NDJSON"

append_report() {
  local name="$1"
  local path="$2"
  local kind="$3"
  local bytes
  local sha

  require_report "$path"
  bytes="$(wc -c <"$path" | tr -d ' ')"
  sha="$(file_sha256 "$path")"

  jq -n \
    --arg name "$name" \
    --arg kind "$kind" \
    --arg path "$path" \
    --arg sha "$sha" \
    --argjson bytes "$bytes" \
    '{
      name:$name,
      kind:$kind,
      path:$path,
      bytes:$bytes,
      sha256:$sha,
      json_valid:true,
      ready:(
        $bytes > 0
        and ($sha | test("^[0-9a-f]{64}$"))
      )
    }' >>"$ROOT_REPORTS_NDJSON"
}

append_report "static_contract" "$STATIC_CONTRACT_PATH" "contract"
append_report "ui_design_system_gate" "$DESIGN_SYSTEM_REPORT_PATH" "design_system"
append_report "control_ui_browser_smoke" "$CONTROL_BROWSER_REPORT_PATH" "surface_smoke"
append_report "native_fixture_visual_smoke" "$NATIVE_FIXTURE_REPORT_PATH" "surface_smoke"
append_report "native_packaging_gate" "$NATIVE_PACKAGING_REPORT_PATH" "packaging"
append_report "native_distribution_preflight_gate" "$NATIVE_DISTRIBUTION_PREFLIGHT_REPORT_PATH" "distribution"
append_report "native_window_smoke" "$NATIVE_WINDOW_REPORT_PATH" "$TRUE_WINDOW_REPORT_KIND"
append_report "native_window_route_smoke" "$NATIVE_WINDOW_ROUTE_REPORT_PATH" "$TRUE_WINDOW_REPORT_KIND"
append_report "native_window_route_mobile_smoke" "$NATIVE_WINDOW_ROUTE_MOBILE_REPORT_PATH" "$TRUE_WINDOW_REPORT_KIND"
append_report "native_window_secondary_smoke" "$NATIVE_WINDOW_SECONDARY_REPORT_PATH" "$TRUE_WINDOW_REPORT_KIND"
append_report "native_window_secondary_mobile_smoke" "$NATIVE_WINDOW_SECONDARY_MOBILE_REPORT_PATH" "$TRUE_WINDOW_REPORT_KIND"
append_report "screenshot_manifest" "$SCREENSHOT_MANIFEST_PATH" "manifest"
append_report "native_base_gap_drilldown" "$BASE_GAP_DRILLDOWN_PATH" "backend_handoff"
append_report "native_base_gap_work_queue" "$BASE_GAP_WORK_QUEUE_PATH" "backend_handoff"
append_report "native_base_gap_backend_handoff" "$BASE_GAP_BACKEND_HANDOFF_PATH" "backend_handoff"
append_report "native_backend_contract_gates" "$BACKEND_CONTRACT_GATES_REPORT_PATH" "backend_handoff"
append_report "native_non_base_edge_gates" "$NON_BASE_EDGE_GATES_REPORT_PATH" "backend_handoff"
append_report "native_productization_blocker_rollup" "$PRODUCTIZATION_ROLLUP_REPORT_PATH" "blocker_rollup"
append_report "ui_plan_boundary_gate" "$PLAN_BOUNDARY_REPORT_PATH" "claim_boundary"
append_report "ui_demo_evidence_gate" "$DEMO_EVIDENCE_REPORT_PATH" "evidence"
append_report "ui_evidence_bundle_gate" "$EVIDENCE_BUNDLE_REPORT_PATH" "evidence"
append_report "ui_evidence_archive_gate" "$EVIDENCE_ARCHIVE_REPORT_PATH" "evidence"
append_report "ui_release_operator_dry_run_gate" "$RELEASE_OPERATOR_DRY_RUN_REPORT_PATH" "release_operator"
append_report "ui_operator_briefing_gate" "$OPERATOR_BRIEFING_REPORT_PATH" "operator_briefing"
append_report "ui_backend_promotion_packet_gate" "$BACKEND_PROMOTION_PACKET_REPORT_PATH" "backend_promotion"
append_report "ui_backend_alignment_evidence_gate" "$BACKEND_ALIGNMENT_EVIDENCE_REPORT_PATH" "backend_alignment"
append_report "ui_critical_path_plan_gate" "$CRITICAL_PATH_PLAN_REPORT_PATH" "critical_path_plan"
append_report "ui_backend_contract_acceptance_gate" "$BACKEND_CONTRACT_ACCEPTANCE_REPORT_PATH" "backend_acceptance"
append_report "ui_backend_handoff_export_gate" "$BACKEND_HANDOFF_EXPORT_REPORT_PATH" "backend_handoff_export"
append_report "ui_backend_dispatch_packet_gate" "$BACKEND_DISPATCH_PACKET_REPORT_PATH" "backend_dispatch_packet"
append_report "ui_backend_receipt_intake_gate" "$BACKEND_RECEIPT_INTAKE_REPORT_PATH" "backend_receipt_intake"
append_report "ui_backend_receipt_roundtrip_gate" "$BACKEND_RECEIPT_ROUNDTRIP_REPORT_PATH" "backend_receipt_roundtrip"
append_report "ui_backend_receipt_refresh_lock_gate" "$BACKEND_RECEIPT_REFRESH_LOCK_REPORT_PATH" "backend_receipt_refresh_lock"
append_report "ui_future_plan_refresh_gate" "$FUTURE_PLAN_REFRESH_REPORT_PATH" "future_plan_refresh"
append_report "ui_operator_briefing_refresh_gate" "$OPERATOR_BRIEFING_REFRESH_REPORT_PATH" "operator_briefing_refresh"
append_report "ui_release_approval_intake_gate" "$RELEASE_APPROVAL_INTAKE_REPORT_PATH" "release_approval_intake"
append_report "ui_top_design_referee_refresh_gate" "$TOP_DESIGN_REFEREE_REFRESH_REPORT_PATH" "top_design_referee_refresh"
append_report "ui_release_artifact_boundary_gate" "$RELEASE_ARTIFACT_BOUNDARY_REPORT_PATH" "release_artifact_boundary"
append_report "ui_release_artifact_intake_gate" "$RELEASE_ARTIFACT_INTAKE_REPORT_PATH" "release_artifact_intake"
append_report "ui_release_artifact_roundtrip_gate" "$RELEASE_ARTIFACT_ROUNDTRIP_REPORT_PATH" "release_artifact_roundtrip"
append_report "ui_current_plan_refresh_gate" "$CURRENT_PLAN_REFRESH_REPORT_PATH" "current_plan_refresh"
append_report "ui_blocker_closure_gate" "$BLOCKER_CLOSURE_REPORT_PATH" "blocker_closure"
append_report "ui_backend_delivery_audit_gate" "$BACKEND_DELIVERY_AUDIT_REPORT_PATH" "backend_delivery_audit"
append_report "ui_backend_delivery_receipt_roundtrip_gate" "$BACKEND_DELIVERY_RECEIPT_ROUNDTRIP_REPORT_PATH" "backend_delivery_receipt_roundtrip"
append_report "ui_risk_future_plan_gate" "$RISK_FUTURE_PLAN_REPORT_PATH" "risk_future_plan"

jq -s '.' "$ROOT_REPORTS_NDJSON" >"$ROOT_REPORTS_JSON"

jq -n \
  --arg product "Hepta UI" \
  --arg runtime "hepta" \
  --arg gate "hepta_ui_root_report_replay_gate" \
  --arg readiness_dir "$READINESS_DIR" \
  --arg report_path "$REPORT_PATH" \
  --argjson strict_current_source_mode "$STRICT_CURRENT_SOURCE_MODE" \
  --slurpfile root_reports_file "$ROOT_REPORTS_JSON" \
  --slurpfile static_contract_file "$STATIC_CONTRACT_PATH" \
  --slurpfile design_system_file "$DESIGN_SYSTEM_REPORT_PATH" \
  --slurpfile control_browser_file "$CONTROL_BROWSER_REPORT_PATH" \
  --slurpfile control_real_click_v7_file "$CONTROL_REAL_CLICK_V7_REPORT_PATH" \
  --slurpfile native_fixture_file "$NATIVE_FIXTURE_REPORT_PATH" \
  --slurpfile native_packaging_file "$NATIVE_PACKAGING_REPORT_PATH" \
  --slurpfile native_distribution_file "$NATIVE_DISTRIBUTION_PREFLIGHT_REPORT_PATH" \
  --slurpfile native_window_file "$NATIVE_WINDOW_REPORT_PATH" \
  --slurpfile native_window_route_file "$NATIVE_WINDOW_ROUTE_REPORT_PATH" \
  --slurpfile native_window_route_mobile_file "$NATIVE_WINDOW_ROUTE_MOBILE_REPORT_PATH" \
  --slurpfile native_window_secondary_file "$NATIVE_WINDOW_SECONDARY_REPORT_PATH" \
  --slurpfile native_window_secondary_mobile_file "$NATIVE_WINDOW_SECONDARY_MOBILE_REPORT_PATH" \
  --slurpfile screenshot_manifest_file "$SCREENSHOT_MANIFEST_PATH" \
  --slurpfile drilldown_file "$BASE_GAP_DRILLDOWN_PATH" \
  --slurpfile work_queue_file "$BASE_GAP_WORK_QUEUE_PATH" \
  --slurpfile handoff_file "$BASE_GAP_BACKEND_HANDOFF_PATH" \
  --slurpfile backend_contract_file "$BACKEND_CONTRACT_GATES_REPORT_PATH" \
  --slurpfile non_base_edge_file "$NON_BASE_EDGE_GATES_REPORT_PATH" \
  --slurpfile rollup_file "$PRODUCTIZATION_ROLLUP_REPORT_PATH" \
  --slurpfile plan_boundary_file "$PLAN_BOUNDARY_REPORT_PATH" \
  --slurpfile demo_evidence_file "$DEMO_EVIDENCE_REPORT_PATH" \
  --slurpfile evidence_bundle_file "$EVIDENCE_BUNDLE_REPORT_PATH" \
  --slurpfile evidence_archive_file "$EVIDENCE_ARCHIVE_REPORT_PATH" \
  --slurpfile release_operator_dry_run_file "$RELEASE_OPERATOR_DRY_RUN_REPORT_PATH" \
  --slurpfile operator_briefing_file "$OPERATOR_BRIEFING_REPORT_PATH" \
  --slurpfile backend_promotion_packet_file "$BACKEND_PROMOTION_PACKET_REPORT_PATH" \
  --slurpfile backend_alignment_evidence_file "$BACKEND_ALIGNMENT_EVIDENCE_REPORT_PATH" \
  --slurpfile critical_path_plan_file "$CRITICAL_PATH_PLAN_REPORT_PATH" \
  --slurpfile backend_contract_acceptance_file "$BACKEND_CONTRACT_ACCEPTANCE_REPORT_PATH" \
  --slurpfile backend_handoff_export_file "$BACKEND_HANDOFF_EXPORT_REPORT_PATH" \
  --slurpfile backend_dispatch_packet_file "$BACKEND_DISPATCH_PACKET_REPORT_PATH" \
  --slurpfile backend_receipt_intake_file "$BACKEND_RECEIPT_INTAKE_REPORT_PATH" \
  --slurpfile backend_receipt_roundtrip_file "$BACKEND_RECEIPT_ROUNDTRIP_REPORT_PATH" \
  --slurpfile backend_receipt_refresh_lock_file "$BACKEND_RECEIPT_REFRESH_LOCK_REPORT_PATH" \
  --slurpfile future_plan_refresh_file "$FUTURE_PLAN_REFRESH_REPORT_PATH" \
  --slurpfile operator_briefing_refresh_file "$OPERATOR_BRIEFING_REFRESH_REPORT_PATH" \
  --slurpfile release_approval_intake_file "$RELEASE_APPROVAL_INTAKE_REPORT_PATH" \
  --slurpfile top_design_referee_refresh_file "$TOP_DESIGN_REFEREE_REFRESH_REPORT_PATH" \
  --slurpfile release_artifact_boundary_file "$RELEASE_ARTIFACT_BOUNDARY_REPORT_PATH" \
  --slurpfile release_artifact_intake_file "$RELEASE_ARTIFACT_INTAKE_REPORT_PATH" \
  --slurpfile release_artifact_roundtrip_file "$RELEASE_ARTIFACT_ROUNDTRIP_REPORT_PATH" \
  --slurpfile current_plan_refresh_file "$CURRENT_PLAN_REFRESH_REPORT_PATH" \
  --slurpfile blocker_closure_file "$BLOCKER_CLOSURE_REPORT_PATH" \
  --slurpfile backend_delivery_audit_file "$BACKEND_DELIVERY_AUDIT_REPORT_PATH" \
  --slurpfile backend_delivery_receipt_roundtrip_file "$BACKEND_DELIVERY_RECEIPT_ROUNDTRIP_REPORT_PATH" \
  --slurpfile risk_future_plan_file "$RISK_FUTURE_PLAN_REPORT_PATH" \
  '
  ($root_reports_file[0]) as $root_reports
  | ($static_contract_file[0]) as $static_contract
  | ($design_system_file[0]) as $design_system
  | ($control_browser_file[0]) as $control_browser
  | ($control_real_click_v7_file[0]) as $control_real_click_v7
  | ($native_fixture_file[0]) as $native_fixture
  | ($native_packaging_file[0]) as $native_packaging
  | ($native_distribution_file[0]) as $native_distribution
  | ($native_window_file[0]) as $native_window
  | ($native_window_route_file[0]) as $native_window_route
  | ($native_window_route_mobile_file[0]) as $native_window_route_mobile
  | ($native_window_secondary_file[0]) as $native_window_secondary
  | ($native_window_secondary_mobile_file[0]) as $native_window_secondary_mobile
  | ($screenshot_manifest_file[0]) as $screenshot_manifest
  | ($drilldown_file[0]) as $drilldown
  | ($work_queue_file[0]) as $work_queue
  | ($handoff_file[0]) as $handoff
  | ($backend_contract_file[0]) as $backend_contract
  | ($non_base_edge_file[0]) as $non_base_edge
  | ($rollup_file[0]) as $rollup
  | ($plan_boundary_file[0]) as $plan_boundary
  | ($demo_evidence_file[0]) as $demo_evidence
  | ($evidence_bundle_file[0]) as $evidence_bundle
  | ($evidence_archive_file[0]) as $evidence_archive
  | ($release_operator_dry_run_file[0]) as $release_operator_dry_run
  | ($operator_briefing_file[0]) as $operator_briefing
  | ($backend_promotion_packet_file[0]) as $backend_promotion_packet
  | ($backend_alignment_evidence_file[0]) as $backend_alignment_evidence
  | ($critical_path_plan_file[0]) as $critical_path_plan
  | ($backend_contract_acceptance_file[0]) as $backend_contract_acceptance
  | ($backend_handoff_export_file[0]) as $backend_handoff_export
  | ($backend_dispatch_packet_file[0]) as $backend_dispatch_packet
  | ($backend_receipt_intake_file[0]) as $backend_receipt_intake
  | ($backend_receipt_roundtrip_file[0]) as $backend_receipt_roundtrip
  | ($backend_receipt_refresh_lock_file[0]) as $backend_receipt_refresh_lock
  | ($future_plan_refresh_file[0]) as $future_plan_refresh
  | ($operator_briefing_refresh_file[0]) as $operator_briefing_refresh
  | ($release_approval_intake_file[0]) as $release_approval_intake
  | ($top_design_referee_refresh_file[0]) as $top_design_referee_refresh
  | ($release_artifact_boundary_file[0]) as $release_artifact_boundary
  | ($release_artifact_intake_file[0]) as $release_artifact_intake
  | ($release_artifact_roundtrip_file[0]) as $release_artifact_roundtrip
  | ($current_plan_refresh_file[0]) as $current_plan_refresh
  | ($blocker_closure_file[0]) as $blocker_closure
  | ($backend_delivery_audit_file[0]) as $backend_delivery_audit
  | ($backend_delivery_receipt_roundtrip_file[0]) as $backend_delivery_receipt_roundtrip
  | ($risk_future_plan_file[0]) as $risk_future_plan
  | def expected_gap_ids: [
      "account_avatar_upload",
      "account_management",
      "file_upload_send",
      "matrix_link_resolution",
      "media_download_playback",
      "mention_picker_send",
      "message_edit_history",
      "message_report_send",
      "message_search",
      "notifications",
      "room_settings",
      "voice_message_send"
    ] | sort;
  def report_names: ($root_reports | map(.name) | sort);
  def required_report_names: [
    "static_contract",
    "ui_design_system_gate",
    "control_ui_browser_smoke",
    "native_fixture_visual_smoke",
    "native_packaging_gate",
    "native_distribution_preflight_gate",
    "native_window_smoke",
    "native_window_route_smoke",
    "native_window_route_mobile_smoke",
    "native_window_secondary_smoke",
    "native_window_secondary_mobile_smoke",
    "screenshot_manifest",
    "native_base_gap_drilldown",
    "native_base_gap_work_queue",
    "native_base_gap_backend_handoff",
    "native_backend_contract_gates",
    "native_non_base_edge_gates",
    "native_productization_blocker_rollup",
    "ui_plan_boundary_gate",
    "ui_demo_evidence_gate",
    "ui_evidence_bundle_gate",
    "ui_evidence_archive_gate",
    "ui_release_operator_dry_run_gate",
    "ui_operator_briefing_gate",
    "ui_backend_promotion_packet_gate",
    "ui_backend_alignment_evidence_gate",
    "ui_critical_path_plan_gate",
    "ui_backend_contract_acceptance_gate",
    "ui_backend_handoff_export_gate",
    "ui_backend_dispatch_packet_gate",
    "ui_backend_receipt_intake_gate",
    "ui_backend_receipt_roundtrip_gate",
    "ui_backend_receipt_refresh_lock_gate",
    "ui_future_plan_refresh_gate",
    "ui_operator_briefing_refresh_gate",
    "ui_release_approval_intake_gate",
    "ui_top_design_referee_refresh_gate",
    "ui_release_artifact_boundary_gate",
    "ui_release_artifact_intake_gate",
    "ui_release_artifact_roundtrip_gate",
    "ui_current_plan_refresh_gate",
    "ui_blocker_closure_gate",
    "ui_backend_delivery_audit_gate",
    "ui_backend_delivery_receipt_roundtrip_gate",
    "ui_risk_future_plan_gate"
  ] | sort;
  def root_reports_ready:
    ($root_reports | length) == 45
    and report_names == required_report_names
    and ($root_reports | all(.ready == true and .json_valid == true and .bytes > 0 and (.sha256 | test("^[0-9a-f]{64}$"))));
  def control_real_click_v7_ready:
    $control_real_click_v7.status == "ready"
    and $control_real_click_v7.real_click_ready == true
    and $control_real_click_v7.summary.control_real_click_activation.viewport_count == 4
    and $control_real_click_v7.summary.control_real_click_activation.target_count == 26
    and $control_real_click_v7.summary.control_real_click_activation.failure_count == 0
    and $control_real_click_v7.summary.control_real_click_activation.mobile_routes_ready == true
    and $control_real_click_v7.summary.control_real_click_activation.popover_switch_sequence_ready == true
    and $control_real_click_v7.summary.control_real_click_activation.popover_switch_step_count == 26
    and ($control_real_click_v7.control_real_click_activation.viewports | all(
      .ready == true
      and .mobile_pane_routes.ready == true
      and .popover_switch_sequence.ready == true
      and (.targets | all(
        .light_dismiss.ready == true
        and .escape_close.ready == true
        and .escape_close.focus_returned_to_trigger == true
      ))
    ));
  def base_gap_alignment_ready:
    $drilldown.native_base_gap_drilldown_ready == true
    and $drilldown.schema_version == 2
    and ($drilldown.gaps | length) == 12
    and ($drilldown.gaps | map(.id) | sort) == expected_gap_ids
    and $work_queue.native_base_gap_work_queue_ready == true
    and $work_queue.schema_version == 2
    and $work_queue.source_drilldown_schema == 2
    and ($work_queue.items | length) == 12
    and ($work_queue.items | map(.id) | sort) == expected_gap_ids
    and ([$work_queue.items[].priority] | sort) == [range(1;13)]
    and ($work_queue.items | all(.status == "ui_contract_ready"))
    and ($work_queue.items | all(.ui_lane_state == "complete"))
    and ($work_queue.items | all(.next_owner_lane == "backend_contract"))
    and $handoff.native_base_gap_backend_handoff_ready == true
    and $handoff.schema_version == 1
    and $handoff.source_drilldown_schema == 2
    and $handoff.source_work_queue_schema == 2
    and $handoff.handoff_count == 12
    and ($handoff.items | length) == 12
    and ($handoff.items | map(.id) | sort) == expected_gap_ids
    and ([$handoff.items[].priority] | sort) == [range(1;13)]
    and ($handoff.items | all(.status == "partial_live_backend_contract_remaining"))
    and ($handoff.items | all(.ui_lane_state == "complete"))
    and ($handoff.items | all(.next_owner_lane == "backend_contract"))
    and ($handoff.items | all((.required_backend_contracts | length) >= 5));
  def evidence_chain_ready:
    $plan_boundary.plan_boundary_gate_ready == true
    and $plan_boundary.claim_boundary.local_fixture_demo_ready == true
    and $plan_boundary.claim_boundary.live_product_claim_ready == false
    and $plan_boundary.claim_boundary.public_distribution_claim_ready == false
    and $plan_boundary.claim_boundary.release_claim_ready == false
    and ($plan_boundary.next_plan | length) == 3
    and $demo_evidence.demo_evidence_gate_ready == true
    and $demo_evidence.claim_boundary.local_fixture_demo_evidence_ready == true
    and $evidence_bundle.evidence_bundle_gate_ready == true
    and $evidence_bundle.claim_boundary.local_evidence_bundle_ready == true
    and $evidence_bundle.all_bundle_items_sha256_match == true
    and $evidence_archive.evidence_archive_gate_ready == true
    and $evidence_archive.claim_boundary.local_evidence_archive_ready == true
    and $evidence_archive.all_extracted_items_sha256_match == true
    and ($evidence_archive.archive_sha256 | test("^[0-9a-f]{64}$"))
    and $evidence_archive.archive_bytes > 0;
  def release_operator_dry_run_ready:
    $release_operator_dry_run.release_operator_dry_run_gate_ready == true
    and $release_operator_dry_run.status == "ready"
    and $release_operator_dry_run.claim_boundary.local_release_operator_dry_run_ready == true
    and ($release_operator_dry_run.dry_run_manifest_sha256 | test("^[0-9a-f]{64}$"))
    and $release_operator_dry_run.dry_run_manifest_bytes > 0
    and $release_operator_dry_run.denial_case_count == 4
    and $release_operator_dry_run.allowed_dry_run_case_count == 1
    and $release_operator_dry_run.operator_packet.dry_run_only == true
    and $release_operator_dry_run.operator_packet.operator_approval_recorded == false
    and $release_operator_dry_run.operator_packet.credential_values_read == false
    and $release_operator_dry_run.operator_packet.notary_submission_performed == false
    and $release_operator_dry_run.operator_packet.public_distribution_artifact_written == false
    and $release_operator_dry_run.claim_boundary.release_execution_ready == false
    and $release_operator_dry_run.claim_boundary.release_claim_ready == false
    and $release_operator_dry_run.claim_boundary.public_distribution_claim_ready == false
    and $release_operator_dry_run.side_effects.external_mutation == false;
  def operator_briefing_ready:
    $operator_briefing.operator_briefing_gate_ready == true
    and $operator_briefing.status == "ready"
    and $operator_briefing.claim_boundary.local_operator_briefing_ready == true
    and $operator_briefing.current_position.local_fixture_demo_ready == true
    and $operator_briefing.current_position.live_product_claim_ready == false
    and $operator_briefing.current_position.public_distribution_claim_ready == false
    and $operator_briefing.current_position.release_claim_ready == false
    and $operator_briefing.critical_risk_count == 3
    and $operator_briefing.backend_remaining_contract_count == 12
    and ($operator_briefing.backend_priority_ids | length) == 12
    and $operator_briefing.backend_priority_ids[0] == "message_search"
    and $operator_briefing.backend_priority_ids[1] == "file_upload_send"
    and $operator_briefing.backend_priority_ids[2] == "media_download_playback"
    and ($operator_briefing.answer_guardrail.forbidden_claims | index("live_product_ready") != null)
    and ($operator_briefing.answer_guardrail.forbidden_claims | index("public_distribution_ready") != null)
    and ($operator_briefing.answer_guardrail.forbidden_claims | index("release_ready") != null)
    and $operator_briefing.side_effects.external_mutation == false;
  def backend_promotion_packet_ready:
    $backend_promotion_packet.backend_promotion_packet_gate_ready == true
    and $backend_promotion_packet.status == "ready"
    and $backend_promotion_packet.packet_kind == "local_backend_contract_promotion_packet"
    and $backend_promotion_packet.total_contract_count == 12
    and $backend_promotion_packet.backend_remaining_contract_count == 12
    and $backend_promotion_packet.priority_packet_count == 5
    and $backend_promotion_packet.selected_priority_ids == ["message_search","file_upload_send","media_download_playback","notifications","room_settings"]
    and ($backend_promotion_packet.priority_packets | length) == 5
    and ($backend_promotion_packet.backlog_packets | length) == 12
    and ($backend_promotion_packet.priority_packets | all(.next_owner_lane == "backend_contract"))
    and ($backend_promotion_packet.priority_packets | all(.promote_requires_backend_adapter == true))
    and ($backend_promotion_packet.priority_packets | all(.active_promotion_performed == false))
    and $backend_promotion_packet.acceptance_guardrail.active_backend_promotion_allowed == false
    and $backend_promotion_packet.claim_boundary.local_backend_promotion_packet_ready == true
    and $backend_promotion_packet.claim_boundary.active_backend_promotion_performed == false
    and $backend_promotion_packet.claim_boundary.live_product_claim_ready == false
    and $backend_promotion_packet.claim_boundary.public_distribution_claim_ready == false
    and $backend_promotion_packet.claim_boundary.release_claim_ready == false
    and $backend_promotion_packet.claim_boundary.external_actions_allowed == false
    and $backend_promotion_packet.side_effects.backend_adapter_promoted == false
    and $backend_promotion_packet.side_effects.live_runtime_mutation == false
    and $backend_promotion_packet.side_effects.external_mutation == false;
  def backend_alignment_evidence_ready:
    $backend_alignment_evidence.backend_alignment_evidence_gate_ready == true
    and $backend_alignment_evidence.status == "ready"
    and $backend_alignment_evidence.alignment_kind == "local_backend_handoff_alignment_evidence"
    and $backend_alignment_evidence.alignment_version == 1
    and $backend_alignment_evidence.selected_alignment_ids == ["message_search","file_upload_send","media_download_playback","notifications","room_settings"]
    and $backend_alignment_evidence.alignment_item_count == 5
    and ($backend_alignment_evidence.alignment_items | length) == 5
    and ($backend_alignment_evidence.alignment_items | all(.alignment_ready == true))
    and ($backend_alignment_evidence.alignment_items | all(.next_owner_lane == "backend_contract"))
    and ($backend_alignment_evidence.alignment_items | all(.status == "partial_live_backend_contract_remaining"))
    and ($backend_alignment_evidence.alignment_items | all(.required_backend_contract_count >= 5))
    and ($backend_alignment_evidence.alignment_items | all(.fixture_source_marker_count >= 4))
    and $backend_alignment_evidence.visual_evidence.local_fixture_ready == true
    and $backend_alignment_evidence.visual_evidence.local_fixture_case_count == 15
    and $backend_alignment_evidence.visual_evidence.demo_evidence_ready == true
    and $backend_alignment_evidence.visual_evidence.required_report_count >= 13
    and $backend_alignment_evidence.visual_evidence.required_screenshot_count >= 24
    and $backend_alignment_evidence.visual_evidence.evidence_archive_ready == true
    and ($backend_alignment_evidence.visual_evidence.evidence_archive_sha256 | test("^[0-9a-f]{64}$"))
    and $backend_alignment_evidence.future_plan_alignment.backend_contract_remaining_count == 12
    and $backend_alignment_evidence.future_plan_alignment.next_owner_lane == "backend_contract"
    and $backend_alignment_evidence.future_plan_alignment.selected_alignment_ids == ["message_search","file_upload_send","media_download_playback","notifications","room_settings"]
    and $backend_alignment_evidence.acceptance_guardrail.active_backend_promotion_allowed == false
    and $backend_alignment_evidence.claim_boundary.local_backend_alignment_evidence_ready == true
    and $backend_alignment_evidence.claim_boundary.local_backend_promotion_packet_ready == true
    and $backend_alignment_evidence.claim_boundary.active_backend_promotion_performed == false
    and $backend_alignment_evidence.claim_boundary.backend_adapter_promoted == false
    and $backend_alignment_evidence.claim_boundary.live_runtime_mutation == false
    and $backend_alignment_evidence.claim_boundary.live_product_claim_ready == false
    and $backend_alignment_evidence.claim_boundary.public_distribution_claim_ready == false
    and $backend_alignment_evidence.claim_boundary.release_claim_ready == false
    and $backend_alignment_evidence.claim_boundary.external_actions_allowed == false
    and $backend_alignment_evidence.side_effects.backend_adapter_promoted == false
    and $backend_alignment_evidence.side_effects.live_runtime_mutation == false
    and $backend_alignment_evidence.side_effects.external_mutation == false;
  def critical_path_plan_ready:
    $critical_path_plan.critical_path_plan_gate_ready == true
    and $critical_path_plan.status == "ready"
    and $critical_path_plan.plan_kind == "local_ui_critical_path_plan"
    and $critical_path_plan.plan_version == 1
    and $critical_path_plan.critical_blocker_count == 3
    and $critical_path_plan.current_backend_selected_ids == ["message_search","file_upload_send","media_download_playback","notifications","room_settings"]
    and ($critical_path_plan.backend_priority_ids | length) == 12
    and $critical_path_plan.backend_priority_ids[0:5] == ["message_search","file_upload_send","media_download_playback","notifications","room_settings"]
    and ($critical_path_plan.backend_alignment_items | length) == 5
    and ($critical_path_plan.backend_alignment_items | all(.alignment_ready == true))
    and ($critical_path_plan.backend_alignment_items | all(.next_owner_lane == "backend_contract"))
    and ($critical_path_plan.backend_alignment_items | all(.required_backend_contract_count >= 5))
    and $critical_path_plan.future_plan_count == 3
    and $critical_path_plan.future_plan[0].id == "backend_contract_first_five"
    and $critical_path_plan.future_plan[1].id == "hepta_ui_hard_evidence_refresh"
    and $critical_path_plan.future_plan[2].id == "release_operator_after_approval"
    and $critical_path_plan.current_state.local_fixture_demo_ready == true
    and $critical_path_plan.current_state.local_evidence_archive_ready == true
    and ($critical_path_plan.current_state.evidence_archive_sha256 | test("^[0-9a-f]{64}$"))
    and $critical_path_plan.current_state.local_release_operator_dry_run_ready == true
    and $critical_path_plan.current_state.backend_alignment_evidence_ready == true
    and $critical_path_plan.current_state.live_product_claim_ready == false
    and $critical_path_plan.current_state.public_distribution_claim_ready == false
    and $critical_path_plan.current_state.release_claim_ready == false
    and $critical_path_plan.claim_boundary.local_critical_path_plan_ready == true
    and $critical_path_plan.claim_boundary.active_backend_promotion_performed == false
    and $critical_path_plan.claim_boundary.backend_adapter_promoted == false
    and $critical_path_plan.claim_boundary.live_runtime_mutation == false
    and $critical_path_plan.claim_boundary.live_product_claim_ready == false
    and $critical_path_plan.claim_boundary.public_distribution_claim_ready == false
    and $critical_path_plan.claim_boundary.release_claim_ready == false
    and $critical_path_plan.claim_boundary.external_actions_allowed == false
    and $critical_path_plan.side_effects.backend_adapter_promoted == false
    and $critical_path_plan.side_effects.live_runtime_mutation == false
    and $critical_path_plan.side_effects.external_mutation == false;
  def backend_contract_acceptance_ready:
    $backend_contract_acceptance.backend_contract_acceptance_gate_ready == true
    and $backend_contract_acceptance.status == "ready"
    and $backend_contract_acceptance.acceptance_kind == "local_backend_contract_acceptance_handoff"
    and $backend_contract_acceptance.acceptance_version == 1
    and $backend_contract_acceptance.backend_remaining_contract_count == 12
    and $backend_contract_acceptance.selected_acceptance_ids == ["message_search","file_upload_send","media_download_playback","notifications","room_settings"]
    and $backend_contract_acceptance.acceptance_item_count == 5
    and $backend_contract_acceptance.acceptance_ready_count == 5
    and ($backend_contract_acceptance.acceptance_items | length) == 5
    and ($backend_contract_acceptance.acceptance_items | all(.owner_lane == "backend_contract"))
    and ($backend_contract_acceptance.acceptance_items | all(.next_owner_lane == "backend_contract"))
    and ($backend_contract_acceptance.acceptance_items | all(.handoff_acceptance_ready == true))
    and ($backend_contract_acceptance.acceptance_items | all(.required_backend_contract_count >= 5))
    and ($backend_contract_acceptance.acceptance_items | all(.fixture_source_marker_count >= 4))
    and ($backend_contract_acceptance.acceptance_items | all((.acceptance_required_evidence | length) == 8))
    and ($backend_contract_acceptance.acceptance_items | all(.current_backend_completion.backend_adapter_promoted == false))
    and ($backend_contract_acceptance.acceptance_items | all(.current_backend_completion.readback_evidence_recorded == false))
    and $backend_contract_acceptance.future_plan_link.critical_path_plan_id == "backend_contract_first_five"
    and $backend_contract_acceptance.future_plan_link.hepta_ui_after_backend_refresh == "hepta_ui_hard_evidence_refresh"
    and $backend_contract_acceptance.promotion_exit_guard.active_backend_promotion_allowed == false
    and $backend_contract_acceptance.promotion_exit_guard.backend_adapter_promoted == false
    and $backend_contract_acceptance.promotion_exit_guard.live_runtime_mutation == false
    and $backend_contract_acceptance.promotion_exit_guard.live_product_claim_ready == false
    and $backend_contract_acceptance.claim_boundary.local_backend_contract_acceptance_ready == true
    and $backend_contract_acceptance.claim_boundary.local_backend_promotion_packet_ready == true
    and $backend_contract_acceptance.claim_boundary.local_backend_alignment_evidence_ready == true
    and $backend_contract_acceptance.claim_boundary.local_critical_path_plan_ready == true
    and $backend_contract_acceptance.claim_boundary.active_backend_promotion_performed == false
    and $backend_contract_acceptance.claim_boundary.backend_adapter_promoted == false
    and $backend_contract_acceptance.claim_boundary.live_runtime_mutation == false
    and $backend_contract_acceptance.claim_boundary.live_product_claim_ready == false
    and $backend_contract_acceptance.claim_boundary.public_distribution_claim_ready == false
    and $backend_contract_acceptance.claim_boundary.release_claim_ready == false
    and $backend_contract_acceptance.claim_boundary.external_actions_allowed == false
    and $backend_contract_acceptance.side_effects.backend_adapter_promoted == false
    and $backend_contract_acceptance.side_effects.live_runtime_mutation == false
    and $backend_contract_acceptance.side_effects.external_mutation == false;
  def backend_handoff_export_ready:
    $backend_handoff_export.backend_handoff_export_gate_ready == true
    and $backend_handoff_export.status == "ready"
    and $backend_handoff_export.export_kind == "local_backend_lane_execution_export"
    and $backend_handoff_export.export_version == 1
    and $backend_handoff_export.selected_export_ids == ["message_search","file_upload_send","media_download_playback","notifications","room_settings"]
    and $backend_handoff_export.export_item_count == 5
    and $backend_handoff_export.export_ready_count == 5
    and ($backend_handoff_export.export_items | length) == 5
    and ($backend_handoff_export.export_items | all(.owner_lane == "backend_contract"))
    and ($backend_handoff_export.export_items | all(.status == "partial_live_backend_contract_remaining"))
    and ($backend_handoff_export.export_items | all((.acceptance_required_evidence | length) == 8))
    and ($backend_handoff_export.export_items | all(.current_backend_completion.backend_adapter_promoted == false))
    and ($backend_handoff_export.export_items | all(.current_backend_completion.readback_evidence_recorded == false))
    and $backend_handoff_export.backend_lane_target.owner_lane == "backend_contract"
    and $backend_handoff_export.backend_lane_target.selected_export_ids == ["message_search","file_upload_send","media_download_playback","notifications","room_settings"]
    and ($backend_handoff_export.hepta_ui_after_backend.required_refresh_commands | length) == 2
    and $backend_handoff_export.dispatch_guardrail.local_export_ready == true
    and $backend_handoff_export.dispatch_guardrail.external_dispatch_performed == false
    and $backend_handoff_export.dispatch_guardrail.backend_adapter_promoted == false
    and $backend_handoff_export.dispatch_guardrail.live_runtime_mutation == false
    and $backend_handoff_export.dispatch_guardrail.live_product_claim_ready == false
    and $backend_handoff_export.claim_boundary.local_backend_handoff_export_ready == true
    and $backend_handoff_export.claim_boundary.backend_adapter_promoted == false
    and $backend_handoff_export.claim_boundary.readback_evidence_recorded == false
    and $backend_handoff_export.claim_boundary.live_runtime_mutation == false
    and $backend_handoff_export.claim_boundary.live_product_claim_ready == false
    and $backend_handoff_export.claim_boundary.public_distribution_claim_ready == false
    and $backend_handoff_export.claim_boundary.release_claim_ready == false
    and $backend_handoff_export.claim_boundary.external_actions_allowed == false
    and $backend_handoff_export.side_effects.local_markdown_export_written == true
    and $backend_handoff_export.side_effects.backend_adapter_promoted == false
    and $backend_handoff_export.side_effects.live_runtime_mutation == false
    and $backend_handoff_export.side_effects.external_mutation == false;
  def backend_dispatch_packet_ready:
    $backend_dispatch_packet.backend_dispatch_packet_gate_ready == true
    and $backend_dispatch_packet.status == "ready"
    and $backend_dispatch_packet.packet_kind == "local_backend_dispatch_packet"
    and $backend_dispatch_packet.packet_version == 1
    and $backend_dispatch_packet.selected_packet_ids == ["message_search","file_upload_send","media_download_playback","notifications","room_settings"]
    and $backend_dispatch_packet.packet_item_count == 5
    and $backend_dispatch_packet.packet_ready_count == 5
    and $backend_dispatch_packet.payload_file_count == 8
    and $backend_dispatch_packet.extracted_file_count == 8
    and $backend_dispatch_packet.all_extracted_files_sha256_match == true
    and ($backend_dispatch_packet.archive_sha256 | test("^[0-9a-f]{64}$"))
    and $backend_dispatch_packet.archive_bytes > 0
    and ($backend_dispatch_packet.manifest_sha256 | test("^[0-9a-f]{64}$"))
    and $backend_dispatch_packet.manifest_bytes > 0
    and ($backend_dispatch_packet.packet_markdown_sha256 | test("^[0-9a-f]{64}$"))
    and $backend_dispatch_packet.packet_markdown_bytes > 0
    and $backend_dispatch_packet.backend_lane_target.target_repo == "/Users/qianqi/.openclaw/workspace/Hepta"
    and $backend_dispatch_packet.backend_lane_target.owner_lane == "backend_contract"
    and ($backend_dispatch_packet.hepta_ui_after_backend.required_refresh_commands | length) == 2
    and $backend_dispatch_packet.dispatch_guardrail.local_dispatch_packet_ready == true
    and $backend_dispatch_packet.dispatch_guardrail.external_dispatch_performed == false
    and ($backend_dispatch_packet.dispatch_guardrail.backend_agent_available | type) == "boolean"
    and $backend_dispatch_packet.dispatch_guardrail.backend_adapter_promoted == false
    and $backend_dispatch_packet.dispatch_guardrail.live_runtime_mutation == false
    and $backend_dispatch_packet.dispatch_guardrail.live_product_claim_ready == false
    and $backend_dispatch_packet.claim_boundary.local_backend_dispatch_packet_ready == true
    and $backend_dispatch_packet.claim_boundary.local_backend_handoff_export_ready == true
    and $backend_dispatch_packet.claim_boundary.local_backend_contract_acceptance_ready == true
    and $backend_dispatch_packet.claim_boundary.backend_adapter_promoted == false
    and $backend_dispatch_packet.claim_boundary.readback_evidence_recorded == false
    and $backend_dispatch_packet.claim_boundary.live_runtime_mutation == false
    and $backend_dispatch_packet.claim_boundary.live_product_claim_ready == false
    and $backend_dispatch_packet.claim_boundary.public_distribution_claim_ready == false
    and $backend_dispatch_packet.claim_boundary.release_claim_ready == false
    and $backend_dispatch_packet.claim_boundary.external_actions_allowed == false
    and $backend_dispatch_packet.side_effects.local_archive_written == true
    and $backend_dispatch_packet.side_effects.local_extract_verification == true
    and $backend_dispatch_packet.side_effects.backend_adapter_promoted == false
    and $backend_dispatch_packet.side_effects.live_runtime_mutation == false
    and $backend_dispatch_packet.side_effects.external_mutation == false;
  def backend_receipt_intake_ready:
    $backend_receipt_intake.backend_receipt_intake_gate_ready == true
    and $backend_receipt_intake.status == "ready"
    and $backend_receipt_intake.intake_kind == "local_backend_receipt_intake_contract"
    and $backend_receipt_intake.intake_version == 1
    and $backend_receipt_intake.selected_receipt_ids == ["message_search","file_upload_send","media_download_playback","notifications","room_settings"]
    and ($backend_receipt_intake.template_sha256 | test("^[0-9a-f]{64}$"))
    and $backend_receipt_intake.template_bytes > 0
    and ($backend_receipt_intake.markdown_sha256 | test("^[0-9a-f]{64}$"))
    and $backend_receipt_intake.markdown_bytes > 0
    and $backend_receipt_intake.dispatch_packet_archive_sha256 == $backend_dispatch_packet.archive_sha256
    and $backend_receipt_intake.dispatch_packet_archive_bytes == $backend_dispatch_packet.archive_bytes
    and ($backend_receipt_intake.required_receipt_item_fields | length) == 8
    and ($backend_receipt_intake.required_ui_refresh_commands | length) == 2
    and (
      (
        $backend_receipt_intake.backend_receipt_present == false
        and $backend_receipt_intake.waiting_for_backend_receipt == true
        and $backend_receipt_intake.backend_receipt_valid == false
        and $backend_receipt_intake.receipt_item_count == 0
        and $backend_receipt_intake.receipt_ready_count == 0
      )
      or (
        $backend_receipt_intake.backend_receipt_present == true
        and $backend_receipt_intake.waiting_for_backend_receipt == false
        and $backend_receipt_intake.backend_receipt_valid == true
        and $backend_receipt_intake.receipt_item_count == 5
        and $backend_receipt_intake.receipt_ready_count == 5
      )
    )
    and $backend_receipt_intake.claim_boundary.local_backend_receipt_intake_ready == true
    and $backend_receipt_intake.claim_boundary.local_backend_dispatch_packet_ready == true
    and $backend_receipt_intake.claim_boundary.live_product_claim_ready == false
    and $backend_receipt_intake.claim_boundary.public_distribution_claim_ready == false
    and $backend_receipt_intake.claim_boundary.release_claim_ready == false
    and $backend_receipt_intake.claim_boundary.external_actions_allowed == false
    and $backend_receipt_intake.side_effects.local_template_written == true
    and $backend_receipt_intake.side_effects.local_markdown_written == true
    and $backend_receipt_intake.side_effects.live_runtime_mutation == false
    and $backend_receipt_intake.side_effects.external_mutation == false;
  def backend_receipt_roundtrip_ready:
    $backend_receipt_roundtrip.backend_receipt_roundtrip_gate_ready == true
    and $backend_receipt_roundtrip.status == "ready"
    and $backend_receipt_roundtrip.roundtrip_kind == "local_backend_receipt_valid_branch_replay"
    and $backend_receipt_roundtrip.roundtrip_version == 1
    and $backend_receipt_roundtrip.selected_roundtrip_ids == ["message_search","file_upload_send","media_download_playback","notifications","room_settings"]
    and $backend_receipt_roundtrip.roundtrip_item_count == 5
    and $backend_receipt_roundtrip.roundtrip_ready_count == 5
    and $backend_receipt_roundtrip.dispatch_packet_archive_sha256 == $backend_dispatch_packet.archive_sha256
    and $backend_receipt_roundtrip.dispatch_packet_archive_bytes == $backend_dispatch_packet.archive_bytes
    and (
      (
        $backend_receipt_roundtrip.waiting_receipt_state.backend_receipt_present == false
        and $backend_receipt_roundtrip.waiting_receipt_state.waiting_for_backend_receipt == true
        and $backend_receipt_roundtrip.waiting_receipt_state.backend_receipt_valid == false
      )
      or (
        $backend_receipt_roundtrip.waiting_receipt_state.backend_receipt_present == true
        and $backend_receipt_roundtrip.waiting_receipt_state.waiting_for_backend_receipt == false
        and $backend_receipt_roundtrip.waiting_receipt_state.backend_receipt_valid == true
      )
    )
    and $backend_receipt_roundtrip.simulated_receipt_state.receipt_mode == "local_simulated_receipt_roundtrip_only"
    and $backend_receipt_roundtrip.simulated_receipt_state.backend_receipt_present == true
    and $backend_receipt_roundtrip.simulated_receipt_state.waiting_for_backend_receipt == false
    and $backend_receipt_roundtrip.simulated_receipt_state.backend_receipt_valid == true
    and $backend_receipt_roundtrip.simulated_receipt_state.receipt_item_count == 5
    and $backend_receipt_roundtrip.simulated_receipt_state.receipt_ready_count == 5
    and $backend_receipt_roundtrip.source_alignment.backend_dispatch_packet_ready == true
    and $backend_receipt_roundtrip.source_alignment.backend_receipt_waiting_branch_ready == true
    and $backend_receipt_roundtrip.source_alignment.backend_receipt_present_branch_ready == true
    and $backend_receipt_roundtrip.source_alignment.simulated_receipt_ready == true
    and $backend_receipt_roundtrip.source_alignment.selected_ids_match == true
    and $backend_receipt_roundtrip.source_alignment.dispatch_archive_match == true
    and $backend_receipt_roundtrip.claim_boundary.local_backend_receipt_roundtrip_ready == true
    and $backend_receipt_roundtrip.claim_boundary.local_backend_receipt_intake_ready == true
    and $backend_receipt_roundtrip.claim_boundary.simulated_backend_receipt_branch_ready == true
    and $backend_receipt_roundtrip.claim_boundary.backend_receipt_claim_ready == false
    and $backend_receipt_roundtrip.claim_boundary.live_runtime_mutation == false
    and $backend_receipt_roundtrip.claim_boundary.live_product_claim_ready == false
    and $backend_receipt_roundtrip.claim_boundary.public_distribution_claim_ready == false
    and $backend_receipt_roundtrip.claim_boundary.release_claim_ready == false
    and $backend_receipt_roundtrip.claim_boundary.external_actions_allowed == false
    and $backend_receipt_roundtrip.side_effects.local_simulated_receipt_written == true
    and $backend_receipt_roundtrip.side_effects.local_simulated_intake_written == true
    and $backend_receipt_roundtrip.side_effects.live_runtime_mutation == false
    and $backend_receipt_roundtrip.side_effects.external_mutation == false;
  def backend_receipt_refresh_lock_ready:
    $backend_receipt_refresh_lock.backend_receipt_refresh_lock_gate_ready == true
    and $backend_receipt_refresh_lock.status == "ready"
    and $backend_receipt_refresh_lock.refresh_lock_kind == "local_backend_receipt_refresh_and_misclaim_lock"
    and $backend_receipt_refresh_lock.refresh_lock_version == 1
    and $backend_receipt_refresh_lock.selected_refresh_ids == ["message_search","file_upload_send","media_download_playback","notifications","room_settings"]
    and $backend_receipt_refresh_lock.receipt_state.backend_receipt_present == $backend_receipt_intake.backend_receipt_present
    and $backend_receipt_refresh_lock.receipt_state.backend_receipt_valid == $backend_receipt_intake.backend_receipt_valid
    and $backend_receipt_refresh_lock.receipt_state.waiting_for_backend_receipt == $backend_receipt_intake.waiting_for_backend_receipt
    and $backend_receipt_refresh_lock.receipt_state.dispatch_archive_match == true
    and $backend_receipt_refresh_lock.source_alignment.backend_dispatch_packet_ready == true
    and $backend_receipt_refresh_lock.source_alignment.backend_receipt_intake_ready == true
    and $backend_receipt_refresh_lock.source_alignment.backend_receipt_roundtrip_ready == true
    and $backend_receipt_refresh_lock.source_alignment.selected_ids_match == true
    and $backend_receipt_refresh_lock.source_alignment.dispatch_archive_match == true
    and $backend_receipt_refresh_lock.source_alignment.receipt_state_ready == true
    and $backend_receipt_refresh_lock.misclaim_lock.simulated_receipt_branch_available == true
    and $backend_receipt_refresh_lock.misclaim_lock.roundtrip_backend_receipt_claim_ready == false
    and $backend_receipt_refresh_lock.claim_boundary.local_backend_receipt_refresh_lock_ready == true
    and $backend_receipt_refresh_lock.claim_boundary.local_backend_receipt_intake_ready == true
    and $backend_receipt_refresh_lock.claim_boundary.local_backend_receipt_roundtrip_ready == true
    and $backend_receipt_refresh_lock.claim_boundary.simulated_backend_receipt_branch_ready == true
    and $backend_receipt_refresh_lock.claim_boundary.live_runtime_mutation == false
    and $backend_receipt_refresh_lock.claim_boundary.live_product_claim_ready == false
    and $backend_receipt_refresh_lock.claim_boundary.public_distribution_claim_ready == false
    and $backend_receipt_refresh_lock.claim_boundary.release_claim_ready == false
    and $backend_receipt_refresh_lock.claim_boundary.external_actions_allowed == false
    and $backend_receipt_refresh_lock.side_effects.local_markdown_written == true
    and $backend_receipt_refresh_lock.side_effects.live_runtime_mutation == false
    and $backend_receipt_refresh_lock.side_effects.external_mutation == false;
  def future_plan_refresh_ready:
    $future_plan_refresh.future_plan_refresh_gate_ready == true
    and $future_plan_refresh.status == "ready"
    and $future_plan_refresh.plan_kind == "local_ui_future_plan_refresh_after_backend_receipt_lock"
    and $future_plan_refresh.plan_version == 1
    and $future_plan_refresh.r52_minimum_gate.defined == true
    and $future_plan_refresh.r52_minimum_gate.root_report_replay_required_count == 32
    and $future_plan_refresh.r52_minimum_gate.backend_receipt_roundtrip_required == true
    and $future_plan_refresh.r52_minimum_gate.backend_receipt_refresh_lock_required == true
    and $future_plan_refresh.backend_receipt_refresh_contract.selected_ids == ["message_search","file_upload_send","media_download_playback","notifications","room_settings"]
    and $future_plan_refresh.backend_receipt_refresh_contract.real_backend_receipt_present == $backend_receipt_refresh_lock.receipt_state.real_backend_receipt_present
    and $future_plan_refresh.backend_receipt_refresh_contract.backend_receipt_claim_ready == $backend_receipt_refresh_lock.claim_boundary.backend_receipt_claim_ready
    and $future_plan_refresh.backend_receipt_refresh_contract.simulated_branch_not_promoted == true
    and ($future_plan_refresh.backend_receipt_refresh_contract.required_ui_refresh_commands | length) == 2
    and $future_plan_refresh.future_plan_count == 3
    and ($future_plan_refresh.future_plan | map(.id)) == ["r52_minimum_ui_demo_gate","backend_real_receipt_return","ui_refresh_after_real_receipt"]
    and $future_plan_refresh.future_plan[1].selected_ids == ["message_search","file_upload_send","media_download_playback","notifications","room_settings"]
    and $future_plan_refresh.future_plan[1].target_repo == "/Users/qianqi/.openclaw/workspace/Hepta"
    and ($future_plan_refresh.future_plan[2].required_commands | length) == 2
    and $future_plan_refresh.stale_plan_lock.refreshed_minimum_id == "r52_minimum_ui_demo_gate"
    and $future_plan_refresh.stale_plan_lock.refreshed_plan_supersedes_plan_boundary_next_plan == true
    and $future_plan_refresh.claim_boundary.local_future_plan_refresh_ready == true
    and $future_plan_refresh.claim_boundary.local_backend_receipt_refresh_lock_ready == true
    and ($future_plan_refresh.claim_boundary.real_backend_receipt_claim_ready | type) == "boolean"
    and ($future_plan_refresh.claim_boundary.backend_receipt_claim_ready | type) == "boolean"
    and $future_plan_refresh.claim_boundary.backend_receipt_claim_ready == $backend_receipt_refresh_lock.claim_boundary.backend_receipt_claim_ready
    and $future_plan_refresh.claim_boundary.live_product_claim_ready == false
    and $future_plan_refresh.claim_boundary.public_distribution_claim_ready == false
    and $future_plan_refresh.claim_boundary.release_claim_ready == false
    and $future_plan_refresh.claim_boundary.backend_adapter_promoted == false
    and $future_plan_refresh.claim_boundary.live_runtime_mutation == false
    and $future_plan_refresh.claim_boundary.external_actions_allowed == false
    and $future_plan_refresh.claim_boundary.public_upload_performed == false
    and $future_plan_refresh.claim_boundary.signing_notarization_performed == false
    and $future_plan_refresh.side_effects.external_mutation == false;
  def operator_briefing_refresh_ready:
    $operator_briefing_refresh.operator_briefing_refresh_gate_ready == true
    and $operator_briefing_refresh.status == "ready"
    and $operator_briefing_refresh.briefing_refresh_kind == "local_ui_operator_briefing_after_future_plan_refresh"
    and $operator_briefing_refresh.briefing_refresh_version == 1
    and $operator_briefing_refresh.current_state.prior_operator_briefing_risk_count == 3
    and $operator_briefing_refresh.current_state.source_future_plan_root_report_required_count == 32
    and $operator_briefing_refresh.current_state.root_report_replay_required_count_after_refresh == 33
    and $operator_briefing_refresh.current_state.real_backend_receipt_present == $backend_receipt_refresh_lock.receipt_state.real_backend_receipt_present
    and $operator_briefing_refresh.current_state.backend_receipt_claim_ready == $backend_receipt_refresh_lock.claim_boundary.backend_receipt_claim_ready
    and ($operator_briefing_refresh.updated_critical_risk_count >= 1 and $operator_briefing_refresh.updated_critical_risk_count <= 4)
    and $operator_briefing_refresh.current_next_plan_ids == ["r52_minimum_ui_demo_gate","backend_real_receipt_return","ui_refresh_after_real_receipt"]
    and $operator_briefing_refresh.backend_dispatch_pointer.selected_ids == ["message_search","file_upload_send","media_download_playback","notifications","room_settings"]
    and $operator_briefing_refresh.backend_dispatch_pointer.archive_sha256 == $backend_dispatch_packet.archive_sha256
    and $operator_briefing_refresh.backend_dispatch_pointer.archive_bytes == $backend_dispatch_packet.archive_bytes
    and $operator_briefing_refresh.backend_dispatch_pointer.backend_adapter_promoted == false
    and $operator_briefing_refresh.backend_dispatch_pointer.readback_evidence_recorded == false
    and $operator_briefing_refresh.receipt_refresh_pointer.real_backend_receipt_present == $backend_receipt_refresh_lock.receipt_state.real_backend_receipt_present
    and $operator_briefing_refresh.receipt_refresh_pointer.simulated_branch_not_promoted == true
    and ($operator_briefing_refresh.receipt_refresh_pointer.required_ui_refresh_commands | length) == 2
    and $operator_briefing_refresh.source_alignment.operator_briefing_ready == true
    and $operator_briefing_refresh.source_alignment.future_plan_refresh_ready == true
    and $operator_briefing_refresh.source_alignment.backend_dispatch_packet_ready == true
    and $operator_briefing_refresh.source_alignment.backend_receipt_refresh_lock_ready == true
    and $operator_briefing_refresh.source_alignment.selected_ids_match == true
    and $operator_briefing_refresh.source_alignment.plan_ids_match == true
    and $operator_briefing_refresh.source_alignment.real_receipt_state_match == true
    and (
      (
        $operator_briefing_refresh.claim_boundary.backend_receipt_claim_ready == true
        and ($operator_briefing_refresh.answer_guardrail.forbidden_claims | index("real_backend_receipt_ready") == null)
      )
      or (
        $operator_briefing_refresh.claim_boundary.backend_receipt_claim_ready == false
        and ($operator_briefing_refresh.answer_guardrail.forbidden_claims | index("real_backend_receipt_ready") != null)
      )
    )
    and ($operator_briefing_refresh.answer_guardrail.forbidden_claims | index("live_product_ready") != null)
    and ($operator_briefing_refresh.refresh_markdown_sha256 | test("^[0-9a-f]{64}$"))
    and $operator_briefing_refresh.refresh_markdown_bytes > 0
    and $operator_briefing_refresh.claim_boundary.local_operator_briefing_refresh_ready == true
    and ($operator_briefing_refresh.claim_boundary.real_backend_receipt_claim_ready | type) == "boolean"
    and ($operator_briefing_refresh.claim_boundary.backend_receipt_claim_ready | type) == "boolean"
    and $operator_briefing_refresh.claim_boundary.backend_receipt_claim_ready == $backend_receipt_refresh_lock.claim_boundary.backend_receipt_claim_ready
    and $operator_briefing_refresh.claim_boundary.backend_adapter_promoted == false
    and $operator_briefing_refresh.claim_boundary.readback_evidence_recorded == false
    and $operator_briefing_refresh.claim_boundary.live_runtime_mutation == false
    and $operator_briefing_refresh.claim_boundary.live_product_claim_ready == false
    and $operator_briefing_refresh.claim_boundary.public_distribution_claim_ready == false
    and $operator_briefing_refresh.claim_boundary.release_claim_ready == false
    and $operator_briefing_refresh.claim_boundary.external_actions_allowed == false
    and $operator_briefing_refresh.side_effects.local_markdown_written == true
    and $operator_briefing_refresh.side_effects.external_mutation == false;
  def release_approval_intake_ready:
    $release_approval_intake.release_approval_intake_gate_ready == true
    and $release_approval_intake.status == "ready"
    and $release_approval_intake.intake_kind == "local_release_approval_intake_contract"
    and $release_approval_intake.intake_version == 1
    and ($release_approval_intake.template_sha256 | test("^[0-9a-f]{64}$"))
    and $release_approval_intake.template_bytes > 0
    and ($release_approval_intake.markdown_sha256 | test("^[0-9a-f]{64}$"))
    and $release_approval_intake.markdown_bytes > 0
    and $release_approval_intake.release_approval_state.waiting_for_release_approval == true
    and $release_approval_intake.release_approval_state.release_approval_present == false
    and $release_approval_intake.release_approval_state.release_approval_valid == false
    and $release_approval_intake.release_approval_state.independent_approval_verifier_ready == false
    and $release_approval_intake.release_approval_state.self_reported_approval_can_authorize_release == false
    and ($release_approval_intake.approval_blockers | index("independent_release_approval_verifier_unavailable")) != null
    and $release_approval_intake.claim_boundary.release_approval_claim_ready == false
    and $release_approval_intake.release_approval_state.approval_only_can_make_release_claim == false
    and $release_approval_intake.release_approval_state.signed_notarized_stapled_artifact_present == false
    and $release_approval_intake.release_approval_state.public_distribution_artifact_written == false
    and $release_approval_intake.release_approval_state.root_report_replay_required_count_after_intake == 34
    and $release_approval_intake.source_alignment.native_distribution_preflight_ready == true
    and $release_approval_intake.source_alignment.release_operator_dry_run_ready == true
    and $release_approval_intake.source_alignment.operator_briefing_refresh_ready == true
    and $release_approval_intake.source_alignment.evidence_archive_ready == true
    and $release_approval_intake.source_alignment.release_public_distribution_not_approved_risk_present == true
    and $release_approval_intake.source_alignment.approval_valid_branch_supported == false
    and $release_approval_intake.claim_boundary.local_release_approval_intake_ready == true
    and $release_approval_intake.claim_boundary.release_execution_ready == false
    and $release_approval_intake.claim_boundary.live_product_claim_ready == false
    and $release_approval_intake.claim_boundary.public_distribution_claim_ready == false
    and $release_approval_intake.claim_boundary.release_claim_ready == false
    and $release_approval_intake.claim_boundary.external_actions_allowed == false
    and $release_approval_intake.claim_boundary.public_upload_performed == false
    and $release_approval_intake.claim_boundary.signing_notarization_performed == false
    and $release_approval_intake.side_effects.local_template_written == true
    and $release_approval_intake.side_effects.local_markdown_written == true
    and $release_approval_intake.side_effects.credential_value_read == false
    and $release_approval_intake.side_effects.keychain_identity_lookup_performed == false
    and $release_approval_intake.side_effects.network_call_performed == false
    and $release_approval_intake.side_effects.notary_submission_performed == false
    and $release_approval_intake.side_effects.app_signed == false
    and $release_approval_intake.side_effects.app_notarized == false
    and $release_approval_intake.side_effects.app_stapled == false
    and $release_approval_intake.side_effects.public_distribution_artifact_written == false
    and $release_approval_intake.side_effects.external_mutation == false;
  def top_design_referee_refresh_hard_ready:
    $top_design_referee_refresh.true_window_evidence_mode == "full_hard_true_window"
    and $top_design_referee_refresh.hard_true_window_evidence_ready == true
    and $top_design_referee_refresh.referee_matrix.true_window_routes.hard_ready == true
    and $top_design_referee_refresh.referee_matrix.true_window_routes.content_probe_ready == true
    and $top_design_referee_refresh.referee_matrix.true_window_secondary_desktop.hard_ready == true
    and $top_design_referee_refresh.referee_matrix.true_window_secondary_mobile.hard_ready == true
    and $top_design_referee_refresh.referee_matrix.true_window_secondary_mobile.content_probe_ready == true
    and $top_design_referee_refresh.referee_matrix.true_window_secondary_mobile.content_visible_count >= 10
    and $top_design_referee_refresh.screenshot_manifest.hard_ready == true
    and $top_design_referee_refresh.screenshot_manifest.counts.native_true_window == 2
    and $top_design_referee_refresh.screenshot_manifest.counts.native_true_window_route == 4
    and $top_design_referee_refresh.screenshot_manifest.counts.native_true_window_secondary == 5
    and $top_design_referee_refresh.screenshot_manifest.counts.native_true_window_secondary_mobile == 5
    and $top_design_referee_refresh.screenshot_manifest.counts.total >= 60;
  def top_design_referee_refresh_no_window_ready:
    $top_design_referee_refresh.true_window_evidence_mode == "no_window_fixture"
    and $top_design_referee_refresh.no_window_evidence_accepted == true
    and $top_design_referee_refresh.referee_matrix.true_window_routes.no_window_accepted == true
    and $top_design_referee_refresh.referee_matrix.true_window_secondary_desktop.no_window_accepted == true
    and $top_design_referee_refresh.referee_matrix.true_window_secondary_mobile.no_window_accepted == true
    and $top_design_referee_refresh.screenshot_manifest.no_window_ready == true
    and $top_design_referee_refresh.screenshot_manifest.counts.native_true_window == 0
    and $top_design_referee_refresh.screenshot_manifest.counts.native_true_window_route == 0
    and $top_design_referee_refresh.screenshot_manifest.counts.native_true_window_secondary == 0
    and $top_design_referee_refresh.screenshot_manifest.counts.native_true_window_secondary_mobile == 0
    and $top_design_referee_refresh.screenshot_manifest.counts.total >= 44;
  def top_design_referee_refresh_ready:
    $top_design_referee_refresh.top_design_referee_refresh_gate_ready == true
    and $top_design_referee_refresh.top_design_harsh_2026_referee_ready == true
    and $top_design_referee_refresh.control_ui_harsh_2026_ready == true
    and $top_design_referee_refresh.native_secondary_harsh_action_matrix_ready == true
    and $top_design_referee_refresh.status == "ready"
    and $top_design_referee_refresh.refresh_kind == "local_ui_top_design_referee_2026_refresh"
	    and $top_design_referee_refresh.refresh_version == 46
	    and $top_design_referee_refresh.referee_matrix.control_ui.rail_action_icon_ready == true
	    and $top_design_referee_refresh.referee_matrix.control_ui.microcopy_word_split_guard_ready == true
	    and $top_design_referee_refresh.referee_matrix.control_ui.logo_clip_guard_ready == true
	    and $top_design_referee_refresh.referee_matrix.control_ui.active_chat_readability_ready == true
	    and $top_design_referee_refresh.referee_matrix.control_ui.folder_chip_touch_ready == true
	    and $top_design_referee_refresh.referee_matrix.control_ui.row_menu_touch_ready == true
	    and $top_design_referee_refresh.referee_matrix.control_ui.row_menu_all_rows_ready == true
	    and $top_design_referee_refresh.referee_matrix.control_ui.row_menu_light_glass_ready == true
	    and $top_design_referee_refresh.referee_matrix.control_ui.thread_tools_menu_ready == true
	    and $top_design_referee_refresh.referee_matrix.control_ui.composer_tools_menu_ready == true
	    and $top_design_referee_refresh.referee_matrix.control_ui.composer_popover_ready == true
	    and $top_design_referee_refresh.referee_matrix.control_ui.composer_popover_search_light_glass_ready == true
	    and $top_design_referee_refresh.referee_matrix.control_ui.micro_surface_light_glass_ready == true
	    and $top_design_referee_refresh.referee_matrix.control_ui.message_routing_badge_light_glass_ready == true
	    and $top_design_referee_refresh.referee_matrix.control_ui.visible_text_integrity_ready == true
	    and $top_design_referee_refresh.referee_matrix.control_level.ready == true
    and $top_design_referee_refresh.referee_matrix.control_level.requested_scope == "desktop_mobile_all_modules_buttons_submenus"
    and $top_design_referee_refresh.referee_matrix.tempered_glass_2026.ready == true
    and $top_design_referee_refresh.referee_matrix.tempered_glass_2026.aesthetic_standard == "2026_tempered_glass_liquid_glass"
    and $top_design_referee_refresh.referee_matrix.tempered_glass_2026.control_visible_text_integrity_ready == true
    and $top_design_referee_refresh.referee_matrix.tempered_glass_2026.control_micro_surface_light_glass_ready == true
    and $top_design_referee_refresh.referee_matrix.tempered_glass_2026.control_composer_popover_search_light_glass_ready == true
    and $top_design_referee_refresh.referee_matrix.tempered_glass_2026.control_message_routing_badge_light_glass_ready == true
    and $top_design_referee_refresh.referee_matrix.tempered_glass_2026.clipping_failure_count == 0
    and $top_design_referee_refresh.referee_matrix.tempered_glass_2026.min_contrast_ratio >= 4.5
    and $top_design_referee_refresh.referee_matrix.control_level.selected_row_variant_count == 18
    and $top_design_referee_refresh.referee_matrix.control_level.secondary_surface_case_count == 15
    and $top_design_referee_refresh.referee_matrix.control_level.secondary_surface_total_action_count == 57
    and $top_design_referee_refresh.referee_matrix.control_level.secondary_surface_action_matrix_ready == true
    and $top_design_referee_refresh.referee_matrix.control_level.secondary_surface_action_matrix_case_count == 15
    and $top_design_referee_refresh.referee_matrix.control_level.secondary_surface_harsh_action_matrix_ready == true
    and $top_design_referee_refresh.referee_matrix.control_level.secondary_surface_harsh_action_failure_count == 0
    and $top_design_referee_refresh.referee_matrix.control_level.secondary_surface_title_tooltip_ready == true
    and $top_design_referee_refresh.referee_matrix.control_level.secondary_surface_title_tooltip_failure_count == 0
    and $top_design_referee_refresh.referee_matrix.control_level.true_window_submenu_coverage_ready == true
    and $top_design_referee_refresh.referee_matrix.control_ui.ready == true
    and $top_design_referee_refresh.referee_matrix.control_ui.harsh_2026_ready == true
    and $top_design_referee_refresh.referee_matrix.control_ui.icon_buttons_ready == true
    and $top_design_referee_refresh.referee_matrix.control_ui.icon_button_title_match_ready == true
    and $top_design_referee_refresh.referee_matrix.control_ui.menu_triggers_ready == true
    and $top_design_referee_refresh.referee_matrix.control_ui.menu_trigger_title_match_ready == true
    and $top_design_referee_refresh.referee_matrix.control_ui.menu_item_icons_ready == true
    and $top_design_referee_refresh.referee_matrix.control_ui.menu_surfaces_ready == true
    and $top_design_referee_refresh.referee_matrix.control_ui.menu_surface_viewport_guard_ready == true
    and $top_design_referee_refresh.referee_matrix.control_ui.navigation_icons_ready == true
    and $top_design_referee_refresh.referee_matrix.control_ui.scroll_edge_ready == true
    and $top_design_referee_refresh.referee_matrix.control_ui.persisted_phone320_screenshot_ready == true
    and $top_design_referee_refresh.referee_matrix.control_ui.persisted_phone320_screenshot.viewport == "320x844"
    and ($top_design_referee_refresh.referee_matrix.control_ui.persisted_phone320_screenshot.sha256 | test("^[0-9a-f]{64}$"))
    and $top_design_referee_refresh.referee_matrix.control_ui.persisted_phone320_screenshot.bytes >= 50000
    and $top_design_referee_refresh.referee_matrix.native_fixture.ready == true
    and $top_design_referee_refresh.referee_matrix.native_fixture.secondary_surface_harsh_action_matrix_ready == true
    and $top_design_referee_refresh.referee_matrix.native_fixture.secondary_surface_harsh_action_failure_count == 0
    and $top_design_referee_refresh.referee_matrix.native_fixture.secondary_surface_title_tooltip_ready == true
    and $top_design_referee_refresh.referee_matrix.native_fixture.secondary_surface_title_tooltip_failure_count == 0
    and $top_design_referee_refresh.referee_matrix.true_window_routes.ready == true
    and $top_design_referee_refresh.referee_matrix.true_window_secondary_desktop.ready == true
    and $top_design_referee_refresh.referee_matrix.true_window_secondary_mobile.ready == true
    and (top_design_referee_refresh_hard_ready or top_design_referee_refresh_no_window_ready)
    and $top_design_referee_refresh.screenshot_manifest.ready == true
    and $top_design_referee_refresh.screenshot_manifest.base_ready == true
    and $top_design_referee_refresh.screenshot_manifest.counts.control_ui == 4
    and $top_design_referee_refresh.source_alignment.operator_briefing_refresh_ready == true
    and (
      $top_design_referee_refresh.source_alignment.operator_briefing_refresh_critical_risk_count >= 1
      and $top_design_referee_refresh.source_alignment.operator_briefing_refresh_critical_risk_count <= 4
    )
    and $top_design_referee_refresh.source_alignment.current_plan_ids == ["r52_minimum_ui_demo_gate","backend_real_receipt_return","ui_refresh_after_real_receipt"]
    and $top_design_referee_refresh.source_alignment.current_roundtrip_plan_ids == ["r62_minimum_ui_demo_gate","backend_real_receipt_return","ui_refresh_after_real_receipt","release_artifact_roundtrip_and_signed_artifact_gate"]
    and ($top_design_referee_refresh.source_alignment.real_backend_receipt_present | type) == "boolean"
    and $top_design_referee_refresh.source_alignment.control_phone320_screenshot_ready == true
    and $top_design_referee_refresh.current_referee_alignment.current_minimum_gate_id == "r62_minimum_ui_demo_gate"
    and $top_design_referee_refresh.current_referee_alignment.current_plan_ids == ["r62_minimum_ui_demo_gate","backend_real_receipt_return","ui_refresh_after_real_receipt","release_artifact_roundtrip_and_signed_artifact_gate"]
    and $top_design_referee_refresh.current_referee_alignment.release_artifact_roundtrip_required == true
    and $top_design_referee_refresh.current_referee_alignment.release_artifact_roundtrip_present_branch_required == true
    and $top_design_referee_refresh.current_referee_alignment.signed_notarized_stapled_artifact_required_for_release == true
    and $top_design_referee_refresh.current_referee_alignment.root_report_replay_required_count_after_roundtrip == 41
    and (
      $top_design_referee_refresh.current_referee_alignment.blocker_closure_critical_blocker_count_expected >= 2
      and $top_design_referee_refresh.current_referee_alignment.blocker_closure_critical_blocker_count_expected <= 5
    )
    and (
      $top_design_referee_refresh.current_referee_alignment.backend_delivery_audit_critical_blocker_count_expected >= 2
      and $top_design_referee_refresh.current_referee_alignment.backend_delivery_audit_critical_blocker_count_expected <= 6
    )
    and $top_design_referee_refresh.current_state.root_report_replay_required_count_after_top_design_refresh == 35
    and $top_design_referee_refresh.current_state.downstream_minimum_gate_expected == "r62_minimum_ui_demo_gate"
    and $top_design_referee_refresh.current_state.downstream_root_report_replay_required_count_after_release_artifact_roundtrip == 41
    and $top_design_referee_refresh.current_state.downstream_current_plan_ids == ["r62_minimum_ui_demo_gate","backend_real_receipt_return","ui_refresh_after_real_receipt","release_artifact_roundtrip_and_signed_artifact_gate"]
    and (
      $top_design_referee_refresh.current_state.downstream_blocker_closure_critical_blocker_count_expected >= 2
      and $top_design_referee_refresh.current_state.downstream_blocker_closure_critical_blocker_count_expected <= 5
    )
    and (
      $top_design_referee_refresh.current_state.downstream_backend_delivery_audit_critical_blocker_count_expected >= 2
      and $top_design_referee_refresh.current_state.downstream_backend_delivery_audit_critical_blocker_count_expected <= 6
    )
    and ($top_design_referee_refresh.refresh_markdown_sha256 | test("^[0-9a-f]{64}$"))
    and $top_design_referee_refresh.refresh_markdown_bytes > 0
    and $top_design_referee_refresh.claim_boundary.local_top_design_referee_refresh_ready == true
    and $top_design_referee_refresh.claim_boundary.desktop_mobile_design_claim_ready == true
    and ($top_design_referee_refresh.claim_boundary.real_backend_receipt_claim_ready | type) == "boolean"
    and ($top_design_referee_refresh.claim_boundary.backend_receipt_claim_ready | type) == "boolean"
    and $top_design_referee_refresh.claim_boundary.backend_receipt_claim_ready == $operator_briefing_refresh.claim_boundary.backend_receipt_claim_ready
    and $top_design_referee_refresh.claim_boundary.backend_adapter_promoted == false
    and $top_design_referee_refresh.claim_boundary.live_runtime_mutation == false
    and $top_design_referee_refresh.claim_boundary.live_product_claim_ready == false
    and $top_design_referee_refresh.claim_boundary.public_distribution_claim_ready == false
    and $top_design_referee_refresh.claim_boundary.release_claim_ready == false
    and $top_design_referee_refresh.claim_boundary.external_actions_allowed == false
    and $top_design_referee_refresh.side_effects.external_mutation == false;
  def release_artifact_boundary_ready:
    $release_artifact_boundary.release_artifact_boundary_gate_ready == true
    and $release_artifact_boundary.status == "ready"
    and $release_artifact_boundary.boundary_kind == "local_release_artifact_boundary_lock"
    and $release_artifact_boundary.boundary_version == 1
    and ($release_artifact_boundary.boundary_markdown_sha256 | test("^[0-9a-f]{64}$"))
    and $release_artifact_boundary.boundary_markdown_bytes > 0
    and $release_artifact_boundary.release_artifact_boundary.unsigned_app_bundle_probe_ready == true
    and $release_artifact_boundary.release_artifact_boundary.unsigned_app_bundle_codesign_status == "unsigned_expected"
    and $release_artifact_boundary.release_artifact_boundary.release_approval_waiting_for_approval == true
    and $release_artifact_boundary.release_artifact_boundary.release_approval_present == false
    and $release_artifact_boundary.release_artifact_boundary.release_approval_valid == false
    and $release_artifact_boundary.release_artifact_boundary.independent_approval_verifier_ready == false
    and $release_artifact_boundary.release_artifact_boundary.self_reported_approval_can_authorize_release == false
    and ($release_artifact_boundary.release_blockers | index("independent_release_approval_verifier_unavailable")) != null
    and $release_artifact_boundary.claim_boundary.release_approval_claim_ready == false
    and $release_artifact_boundary.release_artifact_boundary.approval_only_can_make_release_claim == false
    and $release_artifact_boundary.release_artifact_boundary.signed_app_artifact_present == false
    and $release_artifact_boundary.release_artifact_boundary.notarized_app_artifact_present == false
    and $release_artifact_boundary.release_artifact_boundary.stapled_app_artifact_present == false
    and $release_artifact_boundary.release_artifact_boundary.signed_notarized_stapled_artifact_present == false
    and $release_artifact_boundary.release_artifact_boundary.public_distribution_artifact_present == false
    and $release_artifact_boundary.release_artifact_boundary.public_distribution_artifact_written == false
    and $release_artifact_boundary.release_artifact_boundary.next_required_artifact_gate == "signed_notarized_stapled_artifact_gate"
    and $release_artifact_boundary.release_artifact_boundary.root_report_replay_required_count_after_boundary == 36
    and $release_artifact_boundary.source_alignment.native_packaging_ready == true
    and $release_artifact_boundary.source_alignment.native_distribution_preflight_ready == true
    and $release_artifact_boundary.source_alignment.release_operator_dry_run_ready == true
    and $release_artifact_boundary.source_alignment.release_approval_intake_ready == true
    and $release_artifact_boundary.source_alignment.top_design_referee_refresh_ready == true
    and $release_artifact_boundary.source_alignment.evidence_archive_ready == true
    and $release_artifact_boundary.source_alignment.desktop_mobile_design_claim_ready == true
    and $release_artifact_boundary.source_alignment.approval_only_can_make_release_claim == false
    and $release_artifact_boundary.source_alignment.signed_notarized_stapled_artifact_present == false
    and $release_artifact_boundary.source_alignment.public_distribution_artifact_written == false
    and $release_artifact_boundary.source_alignment.root_report_replay_required_count_after_top_design_refresh == 35
    and $release_artifact_boundary.source_alignment.root_report_replay_required_count_after_boundary == 36
    and ($release_artifact_boundary.release_blockers | index("signed_notarized_stapled_artifact_missing") != null)
    and ($release_artifact_boundary.release_blockers | index("public_distribution_artifact_not_written") != null)
    and $release_artifact_boundary.claim_boundary.local_release_artifact_boundary_ready == true
    and $release_artifact_boundary.claim_boundary.desktop_mobile_design_claim_ready == true
    and $release_artifact_boundary.claim_boundary.release_artifact_claim_ready == false
    and $release_artifact_boundary.claim_boundary.release_execution_ready == false
    and $release_artifact_boundary.claim_boundary.live_product_claim_ready == false
    and $release_artifact_boundary.claim_boundary.public_distribution_claim_ready == false
    and $release_artifact_boundary.claim_boundary.release_claim_ready == false
    and $release_artifact_boundary.claim_boundary.external_actions_allowed == false
    and $release_artifact_boundary.claim_boundary.public_upload_performed == false
    and $release_artifact_boundary.claim_boundary.signing_notarization_performed == false
    and $release_artifact_boundary.side_effects.credential_value_read == false
    and $release_artifact_boundary.side_effects.keychain_identity_lookup_performed == false
    and $release_artifact_boundary.side_effects.network_call_performed == false
    and $release_artifact_boundary.side_effects.notary_submission_performed == false
    and $release_artifact_boundary.side_effects.app_signed == false
    and $release_artifact_boundary.side_effects.app_notarized == false
    and $release_artifact_boundary.side_effects.app_stapled == false
    and $release_artifact_boundary.side_effects.public_distribution_artifact_written == false
    and $release_artifact_boundary.side_effects.external_mutation == false;
  def release_artifact_intake_ready:
    $release_artifact_intake.release_artifact_intake_gate_ready == true
    and $release_artifact_intake.status == "ready"
    and $release_artifact_intake.intake_kind == "local_signed_notarized_stapled_artifact_intake_contract"
    and $release_artifact_intake.intake_version == 3
    and ($release_artifact_intake.template_sha256 | test("^[0-9a-f]{64}$"))
    and $release_artifact_intake.template_bytes > 0
    and ($release_artifact_intake.markdown_sha256 | test("^[0-9a-f]{64}$"))
    and $release_artifact_intake.markdown_bytes > 0
    and ($release_artifact_intake.readback_sha256 | test("^[0-9a-f]{64}$"))
    and $release_artifact_intake.readback_bytes > 0
    and $release_artifact_intake.root_report_replay_required_count_after_intake == 37
    and $release_artifact_intake.release_artifact_state.waiting_for_release_artifact == true
    and $release_artifact_intake.release_artifact_state.release_artifact_present == false
    and $release_artifact_intake.release_artifact_state.release_artifact_valid == false
    and $release_artifact_intake.release_artifact_state.receipt_contract_version == 0
    and $release_artifact_intake.release_artifact_state.evidence_readback_valid == false
    and $release_artifact_intake.release_artifact_state.referenced_paths_absolute_and_unique == false
    and $release_artifact_intake.release_artifact_state.signed_app_artifact_present == false
    and $release_artifact_intake.release_artifact_state.notarized_app_artifact_present == false
    and $release_artifact_intake.release_artifact_state.stapled_app_artifact_present == false
    and $release_artifact_intake.release_artifact_state.signed_notarized_stapled_artifact_present == false
    and $release_artifact_intake.release_artifact_state.local_distribution_artifact_written == false
    and $release_artifact_intake.release_artifact_state.public_distribution_artifact_written == false
    and $release_artifact_intake.release_artifact_state.public_upload_performed == false
    and $release_artifact_intake.source_alignment.present_artifact_branch_supported == false
    and $release_artifact_intake.source_alignment.independent_approval_verifier_contract_ready == false
    and ($release_artifact_intake.release_artifact_blockers | index("signed_notarized_stapled_artifact_missing") != null)
    and ($release_artifact_intake.release_artifact_blockers | index("release_artifact_v3_readback_not_verified") != null)
    and ($release_artifact_intake.release_artifact_blockers | index("public_distribution_artifact_not_written") != null)
    and ($release_artifact_intake.release_artifact_blockers | index("release_artifact_present_branch_unsupported_without_independent_approval_verifier") != null)
    and $release_artifact_intake.release_artifact_state.next_required_step == "post_artifact_ui_readiness_refresh"
    and $release_artifact_intake.source_alignment.release_approval_intake_ready == true
    and $release_artifact_intake.source_alignment.release_approval_waiting_for_approval == true
    and $release_artifact_intake.source_alignment.release_approval_present == false
    and $release_artifact_intake.source_alignment.release_approval_valid == false
    and $release_artifact_intake.source_alignment.independent_approval_verifier_ready == false
    and $release_artifact_intake.source_alignment.self_reported_approval_can_authorize_release == false
    and ($release_artifact_intake.release_artifact_blockers | index("operator_release_approval_required")) != null
    and ($release_artifact_intake.release_artifact_blockers | index("independent_release_approval_verifier_unavailable")) != null
    and $release_artifact_intake.claim_boundary.release_approval_claim_ready == false
    and $release_artifact_intake.source_alignment.release_artifact_boundary_ready == true
    and $release_artifact_intake.source_alignment.release_artifact_boundary_root_report_required_count == 36
    and $release_artifact_intake.source_alignment.release_artifact_boundary_next_required_artifact_gate == "signed_notarized_stapled_artifact_gate"
    and $release_artifact_intake.source_alignment.approval_only_can_make_release_claim == false
    and $release_artifact_intake.source_alignment.boundary_signed_notarized_stapled_artifact_present == false
    and $release_artifact_intake.source_alignment.boundary_public_distribution_artifact_written == false
    and $release_artifact_intake.release_artifact_state.signed_notarized_stapled_artifact_present == false
    and ($release_artifact_intake.release_artifact_blockers | index("signed_notarized_stapled_artifact_missing") != null)
    and ($release_artifact_intake.release_artifact_blockers | index("public_distribution_artifact_not_written") != null)
    and $release_artifact_intake.claim_boundary.local_release_artifact_intake_ready == true
    and $release_artifact_intake.claim_boundary.release_artifact_claim_ready == false
    and $release_artifact_intake.claim_boundary.release_execution_ready == false
    and $release_artifact_intake.claim_boundary.live_product_claim_ready == false
    and $release_artifact_intake.claim_boundary.public_distribution_claim_ready == false
    and $release_artifact_intake.claim_boundary.release_claim_ready == false
    and $release_artifact_intake.claim_boundary.external_actions_allowed == false
    and $release_artifact_intake.side_effects.local_template_written == true
    and $release_artifact_intake.side_effects.local_markdown_written == true
    and $release_artifact_intake.side_effects.local_report_written == true
    and $release_artifact_intake.side_effects.external_mutation == false;
  def release_artifact_roundtrip_ready:
    $release_artifact_roundtrip.release_artifact_roundtrip_gate_ready == true
    and $release_artifact_roundtrip.status == "ready"
    and $release_artifact_roundtrip.roundtrip_kind == "release_artifact_v3_fail_closed_contract_replay"
    and $release_artifact_roundtrip.roundtrip_version == 3
    and $release_artifact_roundtrip.roundtrip_ready_count == 2
    and $release_artifact_roundtrip.source_alignment.waiting_branch_ready == true
    and $release_artifact_roundtrip.source_alignment.present_branch_ready == false
    and $release_artifact_roundtrip.source_alignment.present_artifact_branch_supported == false
    and $release_artifact_roundtrip.source_alignment.independent_approval_verifier_contract_ready == false
    and $release_artifact_roundtrip.source_alignment.simulated_artifact_ready == false
    and $release_artifact_roundtrip.source_alignment.legacy_simulated_artifact_rejected == true
    and $release_artifact_roundtrip.source_alignment.v3_valid_branch_selftest_ready == true
    and $release_artifact_roundtrip.source_alignment.waiting_branch_release_artifact_present == false
    and $release_artifact_roundtrip.source_alignment.waiting_branch_release_artifact_valid == false
    and $release_artifact_roundtrip.source_alignment.present_branch_release_artifact_present == false
    and $release_artifact_roundtrip.source_alignment.present_branch_release_artifact_valid == false
    and $release_artifact_roundtrip.source_alignment.present_branch_signed_notarized_stapled_artifact_present == false
    and $release_artifact_roundtrip.source_alignment.present_branch_public_distribution_artifact_written == false
    and $release_artifact_roundtrip.source_alignment.present_branch_post_artifact_refresh_required == true
    and ($release_artifact_roundtrip.source_alignment.present_branch_real_backend_receipt_missing | type) == "boolean"
    and $release_artifact_roundtrip.source_alignment.present_branch_release_artifact_claim_ready == false
    and $release_artifact_roundtrip.source_alignment.present_branch_release_claim_ready == false
    and $release_artifact_roundtrip.source_alignment.root_report_replay_required_count_after_roundtrip == 41
    and $release_artifact_roundtrip.claim_boundary.local_release_artifact_roundtrip_ready == true
    and $release_artifact_roundtrip.claim_boundary.release_artifact_claim_ready == false
    and $release_artifact_roundtrip.claim_boundary.release_execution_ready == false
    and $release_artifact_roundtrip.claim_boundary.live_product_claim_ready == false
    and $release_artifact_roundtrip.claim_boundary.public_distribution_claim_ready == false
    and $release_artifact_roundtrip.claim_boundary.release_claim_ready == false
    and $release_artifact_roundtrip.claim_boundary.external_actions_allowed == false
    and $release_artifact_roundtrip.claim_boundary.signing_notarization_performed == false
    and ($release_artifact_roundtrip.source_report_sha256.legacy_v1_simulated_artifact | test("^[0-9a-f]{64}$"))
    and ($release_artifact_roundtrip.source_report_sha256.legacy_v1_rejection_intake | test("^[0-9a-f]{64}$"))
    and ($release_artifact_roundtrip.source_report_sha256.v3_intake_selftest_log | test("^[0-9a-f]{64}$"))
    and $release_artifact_roundtrip.side_effects.local_legacy_fixture_written == true
    and $release_artifact_roundtrip.side_effects.local_rejection_report_written == true
    and $release_artifact_roundtrip.side_effects.local_v3_selftest_executed == true
    and $release_artifact_roundtrip.side_effects.external_mutation == false;
  def current_plan_refresh_ready:
    $current_plan_refresh.current_plan_refresh_gate_ready == true
    and $current_plan_refresh.status == "ready"
    and $current_plan_refresh.plan_kind == "local_ui_current_plan_refresh_after_release_artifact_roundtrip"
    and $current_plan_refresh.plan_version == 1
    and $current_plan_refresh.legacy_plan_snapshot.legacy_plan_ids == ["r52_minimum_ui_demo_gate","backend_real_receipt_return","ui_refresh_after_real_receipt"]
    and $current_plan_refresh.legacy_plan_snapshot.legacy_root_report_replay_required_count == 32
    and $current_plan_refresh.current_minimum_gate.gate_id == "r62_minimum_ui_demo_gate"
    and $current_plan_refresh.current_minimum_gate.root_report_replay_required_count_after_current_plan_refresh == 41
    and $current_plan_refresh.current_minimum_gate.control_phone320_ready == true
    and $current_plan_refresh.current_minimum_gate.release_approval_intake_required == true
    and $current_plan_refresh.current_minimum_gate.top_design_referee_refresh_required == true
    and $current_plan_refresh.current_minimum_gate.release_artifact_boundary_required == true
    and $current_plan_refresh.current_minimum_gate.release_artifact_intake_required == true
    and $current_plan_refresh.current_minimum_gate.release_artifact_roundtrip_required == true
    and $current_plan_refresh.current_minimum_gate.signed_notarized_stapled_artifact_required_for_release == true
    and $current_plan_refresh.current_plan_count == 4
    and $current_plan_refresh.current_plan_ids == ["r62_minimum_ui_demo_gate","backend_real_receipt_return","ui_refresh_after_real_receipt","release_artifact_roundtrip_and_signed_artifact_gate"]
    and $current_plan_refresh.current_plan[1].selected_ids == ["message_search","file_upload_send","media_download_playback","notifications","room_settings"]
    and $current_plan_refresh.current_plan[1].target_repo == "/Users/qianqi/.openclaw/workspace/Hepta"
    and $current_plan_refresh.current_plan[1].dispatch_archive_sha256 == $backend_dispatch_packet.archive_sha256
    and ($current_plan_refresh.current_plan[2].required_commands | length) == 2
    and $current_plan_refresh.current_plan[3].next_required_artifact_gate == "signed_notarized_stapled_artifact_gate"
    and ($current_plan_refresh.current_plan[3].waiting_for_release_artifact | type) == "boolean"
    and ($current_plan_refresh.current_plan[3].release_artifact_present | type) == "boolean"
    and ($current_plan_refresh.current_plan[3].release_artifact_valid | type) == "boolean"
    and $current_plan_refresh.current_plan[3].local_roundtrip_ready == true
    and $current_plan_refresh.current_plan[3].roundtrip_present_branch_ready == false
    and $current_plan_refresh.current_plan[3].roundtrip_present_branch_valid == false
    and $current_plan_refresh.current_plan[3].roundtrip_legacy_simulated_rejected == true
    and $current_plan_refresh.current_plan[3].roundtrip_v3_valid_branch_selftest_ready == true
    and $current_plan_refresh.source_alignment.release_approval_waiting_for_approval == true
    and $current_plan_refresh.source_alignment.release_approval_present == false
    and $current_plan_refresh.source_alignment.release_approval_valid == false
    and $current_plan_refresh.source_alignment.independent_approval_verifier_ready == false
    and $current_plan_refresh.source_alignment.self_reported_approval_can_authorize_release == false
    and ($current_plan_refresh.current_plan[3].blockers | index("operator_release_approval_required")) != null
    and ($current_plan_refresh.current_plan[3].blockers | index("independent_release_approval_verifier_unavailable")) != null
    and ($current_plan_refresh.current_plan[3].blockers | index("release_artifact_present_branch_unsupported_without_independent_approval_verifier")) != null
    and $current_plan_refresh.claim_boundary.release_approval_claim_ready == false
    and $current_plan_refresh.current_plan[3].waiting_for_release_artifact == true
    and $current_plan_refresh.current_plan[3].release_artifact_present == false
    and $current_plan_refresh.current_plan[3].release_artifact_valid == false
    and $current_plan_refresh.current_plan[3].signed_notarized_stapled_artifact_present == false
    and $current_plan_refresh.current_plan[3].local_distribution_artifact_written == false
    and $current_plan_refresh.current_plan[3].public_distribution_artifact_written == false
    and $current_plan_refresh.current_plan[3].public_upload_performed == false
    and ($current_plan_refresh.current_plan[3].blockers | index("signed_notarized_stapled_artifact_missing") != null)
    and ($current_plan_refresh.current_plan[3].blockers | index("public_distribution_artifact_not_written") != null)
    and (
      (
        $current_plan_refresh.source_alignment.real_backend_receipt_present == true
        and ($current_plan_refresh.current_plan[3].blockers | index("real_backend_receipt_missing") == null)
      )
      or (
        $current_plan_refresh.source_alignment.real_backend_receipt_present == false
        and ($current_plan_refresh.current_plan[3].blockers | index("real_backend_receipt_missing") != null)
      )
    )
    and $current_plan_refresh.source_alignment.current_plan_supersedes_legacy_plan == true
    and $current_plan_refresh.source_alignment.release_artifact_boundary_ready == true
    and $current_plan_refresh.source_alignment.release_artifact_boundary_root_report_required_count == 36
    and $current_plan_refresh.source_alignment.release_artifact_intake_ready == true
    and $current_plan_refresh.source_alignment.release_artifact_intake_root_report_required_count == 37
    and $current_plan_refresh.source_alignment.release_artifact_intake_waiting_for_artifact == $current_plan_refresh.current_plan[3].waiting_for_release_artifact
    and $current_plan_refresh.source_alignment.release_artifact_intake_artifact_present == $current_plan_refresh.current_plan[3].release_artifact_present
    and $current_plan_refresh.source_alignment.release_artifact_intake_artifact_valid == $current_plan_refresh.current_plan[3].release_artifact_valid
    and $current_plan_refresh.source_alignment.release_artifact_intake_artifact_present == false
    and $current_plan_refresh.source_alignment.release_artifact_intake_artifact_valid == false
    and $current_plan_refresh.source_alignment.release_artifact_intake_receipt_contract_version == 0
    and $current_plan_refresh.source_alignment.release_artifact_intake_evidence_readback_valid == false
    and $current_plan_refresh.source_alignment.release_artifact_intake_present_artifact_branch_supported == false
    and $current_plan_refresh.source_alignment.release_artifact_intake_independent_approval_verifier_contract_ready == false
    and $current_plan_refresh.source_alignment.release_artifact_roundtrip_ready == true
    and $current_plan_refresh.source_alignment.release_artifact_roundtrip_root_report_required_count == 41
    and $current_plan_refresh.source_alignment.release_artifact_roundtrip_waiting_branch_ready == true
    and $current_plan_refresh.source_alignment.release_artifact_roundtrip_present_branch_ready == false
    and $current_plan_refresh.source_alignment.release_artifact_roundtrip_present_artifact_present == false
    and $current_plan_refresh.source_alignment.release_artifact_roundtrip_present_artifact_valid == false
    and $current_plan_refresh.source_alignment.release_artifact_roundtrip_present_artifact_branch_supported == false
    and $current_plan_refresh.source_alignment.release_artifact_roundtrip_independent_approval_verifier_contract_ready == false
    and $current_plan_refresh.source_alignment.release_artifact_roundtrip_legacy_simulated_rejected == true
    and $current_plan_refresh.source_alignment.release_artifact_roundtrip_v3_valid_branch_selftest_ready == true
    and $current_plan_refresh.source_alignment.selected_ids_match == true
    and ($current_plan_refresh.source_alignment.real_backend_receipt_present | type) == "boolean"
    and ($current_plan_refresh.refresh_markdown_sha256 | test("^[0-9a-f]{64}$"))
    and $current_plan_refresh.refresh_markdown_bytes > 0
    and $current_plan_refresh.claim_boundary.local_current_plan_refresh_ready == true
    and ($current_plan_refresh.claim_boundary.real_backend_receipt_claim_ready | type) == "boolean"
    and ($current_plan_refresh.claim_boundary.backend_receipt_claim_ready | type) == "boolean"
    and $current_plan_refresh.claim_boundary.backend_adapter_promoted == false
    and $current_plan_refresh.claim_boundary.release_artifact_claim_ready == false
    and $current_plan_refresh.claim_boundary.local_release_artifact_roundtrip_ready == true
    and $current_plan_refresh.claim_boundary.release_execution_ready == false
    and $current_plan_refresh.claim_boundary.live_runtime_mutation == false
    and $current_plan_refresh.claim_boundary.live_product_claim_ready == false
    and $current_plan_refresh.claim_boundary.public_distribution_claim_ready == false
    and $current_plan_refresh.claim_boundary.release_claim_ready == false
    and $current_plan_refresh.claim_boundary.external_actions_allowed == false
    and $current_plan_refresh.claim_boundary.public_upload_performed == false
    and $current_plan_refresh.claim_boundary.signing_notarization_performed == false
    and $current_plan_refresh.side_effects.local_markdown_written == true
    and $current_plan_refresh.side_effects.external_mutation == false;
  def blocker_closure_ready:
    $blocker_closure.blocker_closure_gate_ready == true
    and $blocker_closure.status == "ready"
    and $blocker_closure.closure_kind == "local_ui_blocker_closure_after_current_plan_refresh"
    and $blocker_closure.closure_version == 1
    and $blocker_closure.closure_state.current_minimum_gate_id == "r62_minimum_ui_demo_gate"
    and $blocker_closure.closure_state.prior_current_plan_root_report_required_count == 41
    and $blocker_closure.closure_state.root_report_replay_required_count_after_blocker_closure == 41
    and ($blocker_closure.closure_state.backend_agent_available | type) == "boolean"
    and $blocker_closure.closure_state.external_dispatch_performed == false
    and ($blocker_closure.closure_state.real_backend_receipt_present | type) == "boolean"
    and ($blocker_closure.closure_state.backend_receipt_valid | type) == "boolean"
    and ($blocker_closure.closure_state.backend_receipt_claim_ready | type) == "boolean"
    and $blocker_closure.closure_state.backend_adapter_promoted == false
    and $blocker_closure.closure_state.readback_evidence_recorded == false
    and $blocker_closure.closure_state.release_approval_present == false
    and $blocker_closure.closure_state.release_approval_valid == false
    and $blocker_closure.closure_state.independent_approval_verifier_ready == false
    and $blocker_closure.closure_state.self_reported_approval_can_authorize_release == false
    and ($blocker_closure.critical_blockers | map(.id) | index("release_approval_missing")) != null
    and ($blocker_closure.critical_blockers | map(.id) | index("independent_release_approval_verifier_unavailable")) != null
    and $blocker_closure.claim_boundary.release_approval_claim_ready == false
    and $blocker_closure.closure_state.release_artifact_present == false
    and $blocker_closure.closure_state.release_artifact_valid == false
    and $blocker_closure.closure_state.release_artifact_receipt_contract_version == 0
    and $blocker_closure.closure_state.release_artifact_evidence_readback_valid == false
    and $blocker_closure.closure_state.release_artifact_present_artifact_branch_supported == false
    and $blocker_closure.closure_state.release_artifact_independent_approval_verifier_contract_ready == false
    and $blocker_closure.closure_state.signed_notarized_stapled_artifact_present == false
    and $blocker_closure.closure_state.public_distribution_artifact_written == false
    and $blocker_closure.closure_state.public_upload_performed == false
    and $blocker_closure.closure_state.local_release_artifact_roundtrip_ready == true
    and $blocker_closure.closure_state.release_artifact_roundtrip_present_branch_ready == false
    and $blocker_closure.closure_state.release_artifact_roundtrip_present_artifact_present == false
    and $blocker_closure.closure_state.release_artifact_roundtrip_present_artifact_valid == false
    and $blocker_closure.closure_state.release_artifact_roundtrip_present_artifact_branch_supported == false
    and $blocker_closure.closure_state.release_artifact_roundtrip_independent_approval_verifier_contract_ready == false
    and $blocker_closure.closure_state.release_artifact_roundtrip_legacy_simulated_rejected == true
    and $blocker_closure.closure_state.release_artifact_roundtrip_v3_valid_branch_selftest_ready == true
    and $blocker_closure.closure_state.next_required_artifact_gate == "signed_notarized_stapled_artifact_gate"
    and $blocker_closure.closure_state.selected_ids == ["message_search","file_upload_send","media_download_playback","notifications","room_settings"]
    and $blocker_closure.critical_blocker_count == ($blocker_closure.critical_blockers | length)
    and ($blocker_closure.critical_blocker_count >= 0 and $blocker_closure.critical_blocker_count <= 10)
    and (
      (
        $blocker_closure.closure_state.backend_agent_available == true
        and ($blocker_closure.critical_blockers | map(.id) | index("backend_agent_dispatch_unavailable_in_this_session")) == null
      )
      or (
        $blocker_closure.closure_state.backend_agent_available == false
        and ($blocker_closure.critical_blockers | map(.id) | index("backend_agent_dispatch_unavailable_in_this_session")) != null
      )
    )
    and (
      (
        $blocker_closure.closure_state.real_backend_receipt_present == true
        and ($blocker_closure.critical_blockers | map(.id) | index("real_backend_receipt_missing")) == null
        and ($blocker_closure.critical_blockers | map(.id) | index("backend_contract_first_five_not_executed")) == null
        and (
          (
            $blocker_closure.closure_state.backend_receipt_claim_ready == true
            and ($blocker_closure.critical_blockers | map(.id) | index("backend_receipt_full_hard_refresh_required")) == null
          )
          or (
            $blocker_closure.closure_state.backend_receipt_claim_ready == false
            and ($blocker_closure.critical_blockers | map(.id) | index("backend_receipt_full_hard_refresh_required")) != null
          )
        )
      )
      or (
        $blocker_closure.closure_state.real_backend_receipt_present == false
        and ($blocker_closure.critical_blockers | map(.id) | index("real_backend_receipt_missing")) != null
        and ($blocker_closure.critical_blockers | map(.id) | index("backend_contract_first_five_not_executed")) != null
      )
    )
    and (
      (
        $blocker_closure.closure_state.signed_notarized_stapled_artifact_present == false
        and ($blocker_closure.critical_blockers | map(.id) | index("signed_notarized_stapled_artifact_missing")) != null
        and ($blocker_closure.critical_blockers | map(.id) | index("public_distribution_artifact_not_written")) != null
      )
      or
      (
        $blocker_closure.closure_state.signed_notarized_stapled_artifact_present == true
        and ($blocker_closure.critical_blockers | map(.id) | index("signed_notarized_stapled_artifact_missing")) == null
        and ($blocker_closure.critical_blockers | map(.id) | index("public_distribution_artifact_not_written")) == null
      )
    )
    and ($blocker_closure.future_plan | map(.id)) == ["r62_minimum_ui_demo_gate","backend_real_receipt_return","ui_refresh_after_real_receipt","release_artifact_roundtrip_and_signed_artifact_gate"]
    and (($blocker_closure.next_unblock_sequence | length) >= 1 and ($blocker_closure.next_unblock_sequence | length) <= 7)
    and $blocker_closure.source_alignment.current_plan_refresh_ready == true
    and $blocker_closure.source_alignment.current_plan_ids_match == true
    and $blocker_closure.source_alignment.current_plan_root_report_required_count == 41
    and $blocker_closure.source_alignment.backend_dispatch_packet_ready == true
    and ($blocker_closure.source_alignment.backend_agent_available | type) == "boolean"
    and $blocker_closure.source_alignment.external_dispatch_performed == false
    and $blocker_closure.source_alignment.backend_receipt_refresh_lock_ready == true
    and ($blocker_closure.source_alignment.real_backend_receipt_present | type) == "boolean"
    and ($blocker_closure.source_alignment.backend_receipt_claim_ready | type) == "boolean"
    and $blocker_closure.source_alignment.backend_receipt_claim_ready == $blocker_closure.closure_state.backend_receipt_claim_ready
    and $blocker_closure.source_alignment.release_approval_intake_ready == true
    and $blocker_closure.source_alignment.release_approval_present == $blocker_closure.closure_state.release_approval_present
    and $blocker_closure.source_alignment.release_approval_valid == $blocker_closure.closure_state.release_approval_valid
    and $blocker_closure.source_alignment.independent_approval_verifier_ready == $blocker_closure.closure_state.independent_approval_verifier_ready
    and $blocker_closure.source_alignment.self_reported_approval_can_authorize_release == $blocker_closure.closure_state.self_reported_approval_can_authorize_release
    and $blocker_closure.source_alignment.release_artifact_boundary_ready == true
    and $blocker_closure.source_alignment.release_artifact_intake_ready == true
    and $blocker_closure.source_alignment.release_artifact_intake_root_report_required_count == 37
    and $blocker_closure.source_alignment.release_artifact_present == $blocker_closure.closure_state.release_artifact_present
    and $blocker_closure.source_alignment.release_artifact_valid == $blocker_closure.closure_state.release_artifact_valid
    and $blocker_closure.source_alignment.release_artifact_intake_receipt_contract_version == $blocker_closure.closure_state.release_artifact_receipt_contract_version
    and $blocker_closure.source_alignment.release_artifact_intake_evidence_readback_valid == $blocker_closure.closure_state.release_artifact_evidence_readback_valid
    and $blocker_closure.source_alignment.release_artifact_intake_signed_notarized_stapled_artifact_present == $blocker_closure.closure_state.signed_notarized_stapled_artifact_present
    and $blocker_closure.source_alignment.release_artifact_roundtrip_ready == true
    and $blocker_closure.source_alignment.release_artifact_roundtrip_root_report_required_count == 41
    and $blocker_closure.source_alignment.release_artifact_roundtrip_waiting_branch_ready == true
    and $blocker_closure.source_alignment.release_artifact_present == false
    and $blocker_closure.source_alignment.release_artifact_valid == false
    and $blocker_closure.source_alignment.release_artifact_present_artifact_branch_supported == false
    and $blocker_closure.source_alignment.release_artifact_independent_approval_verifier_contract_ready == false
    and $blocker_closure.source_alignment.release_artifact_roundtrip_present_branch_ready == false
    and $blocker_closure.source_alignment.release_artifact_roundtrip_present_artifact_present == false
    and $blocker_closure.source_alignment.release_artifact_roundtrip_present_artifact_valid == false
    and $blocker_closure.source_alignment.release_artifact_roundtrip_present_artifact_branch_supported == false
    and $blocker_closure.source_alignment.release_artifact_roundtrip_independent_approval_verifier_contract_ready == false
    and $blocker_closure.source_alignment.release_artifact_roundtrip_legacy_simulated_rejected == true
    and $blocker_closure.source_alignment.release_artifact_roundtrip_v3_valid_branch_selftest_ready == true
    and $blocker_closure.source_alignment.root_report_replay_required_count_after_blocker_closure == 41
    and $blocker_closure.source_alignment.selected_ids_match == true
    and $blocker_closure.claim_boundary.local_blocker_closure_ready == true
    and ($blocker_closure.claim_boundary.real_backend_receipt_claim_ready | type) == "boolean"
    and ($blocker_closure.claim_boundary.backend_receipt_claim_ready | type) == "boolean"
    and $blocker_closure.claim_boundary.backend_receipt_claim_ready == $blocker_closure.closure_state.backend_receipt_claim_ready
    and $blocker_closure.claim_boundary.backend_adapter_promoted == false
    and $blocker_closure.claim_boundary.readback_evidence_recorded == false
    and $blocker_closure.claim_boundary.release_artifact_claim_ready == false
    and $blocker_closure.claim_boundary.local_release_artifact_roundtrip_ready == true
    and $blocker_closure.claim_boundary.release_execution_ready == false
    and $blocker_closure.claim_boundary.live_runtime_mutation == false
    and $blocker_closure.claim_boundary.live_product_claim_ready == false
    and $blocker_closure.claim_boundary.public_distribution_claim_ready == false
    and $blocker_closure.claim_boundary.release_claim_ready == false
    and $blocker_closure.side_effects.backend_agent_spawned == false
    and $blocker_closure.side_effects.backend_repo_write == false
    and $blocker_closure.side_effects.external_mutation == false
    and ($blocker_closure.closure_markdown_sha256 | test("^[0-9a-f]{64}$"))
    and $blocker_closure.closure_markdown_bytes > 0;
  def backend_delivery_audit_ready:
    $backend_delivery_audit.backend_delivery_audit_gate_ready == true
    and $backend_delivery_audit.status == "ready"
    and $backend_delivery_audit.audit_kind == "local_backend_dispatch_delivery_boundary"
    and $backend_delivery_audit.audit_version == 1
    and $backend_delivery_audit.delivery_state.local_dispatch_packet_ready == true
    and $backend_delivery_audit.delivery_state.selected_ids == ["message_search","file_upload_send","media_download_playback","notifications","room_settings"]
    and $backend_delivery_audit.delivery_state.dispatch_archive_sha256 == $backend_dispatch_packet.archive_sha256
    and $backend_delivery_audit.delivery_state.manifest_sha256 == $backend_dispatch_packet.manifest_sha256
    and ($backend_delivery_audit.delivery_state.backend_agent_available | type) == "boolean"
    and $backend_delivery_audit.delivery_state.external_dispatch_performed == false
    and (
      (
        $backend_delivery_audit.delivery_state.delivery_receipt_present == false
        and $backend_delivery_audit.delivery_state.delivery_receipt_valid == false
        and $backend_delivery_audit.delivery_state.waiting_for_delivery_receipt == true
        and $backend_delivery_audit.delivery_state.backend_delivery_claim_ready == false
        and $backend_delivery_audit.claim_boundary.backend_delivery_claim_ready == false
      )
      or
      (
        $backend_delivery_audit.delivery_state.delivery_receipt_present == true
        and $backend_delivery_audit.delivery_state.delivery_receipt_valid == true
        and $backend_delivery_audit.delivery_state.waiting_for_delivery_receipt == false
        and $backend_delivery_audit.delivery_state.backend_delivery_claim_ready == true
        and $backend_delivery_audit.claim_boundary.backend_delivery_claim_ready == true
      )
    )
    and ($backend_delivery_audit.delivery_state.real_backend_receipt_present | type) == "boolean"
    and ($backend_delivery_audit.delivery_state.backend_receipt_valid | type) == "boolean"
    and $backend_delivery_audit.delivery_state.root_report_replay_required_count_after_delivery_audit == 41
    and ($backend_delivery_audit.critical_blockers | length) >= $backend_delivery_audit.critical_blocker_count
    and ($backend_delivery_audit.critical_blockers | length) <= ($backend_delivery_audit.critical_blocker_count + 1)
    and ($backend_delivery_audit.critical_blocker_count >= 0 and $backend_delivery_audit.critical_blocker_count <= 11)
    and (
      (
        $backend_delivery_audit.delivery_state.delivery_receipt_valid == true
        and ($backend_delivery_audit.critical_blockers | map(.id) | index("backend_dispatch_delivery_receipt_missing")) == null
      )
      or (
        $backend_delivery_audit.delivery_state.delivery_receipt_valid == false
        and ($backend_delivery_audit.critical_blockers | map(.id) | index("backend_dispatch_delivery_receipt_missing")) != null
      )
    )
    and (($backend_delivery_audit.next_unblock_sequence | length) >= 1 and ($backend_delivery_audit.next_unblock_sequence | length) <= 6)
    and $backend_delivery_audit.source_alignment.backend_dispatch_packet_ready == true
    and $backend_delivery_audit.source_alignment.backend_receipt_refresh_lock_ready == true
    and $backend_delivery_audit.source_alignment.blocker_closure_ready == true
    and ($backend_delivery_audit.source_alignment.blocker_closure_critical_blocker_count >= 0 and $backend_delivery_audit.source_alignment.blocker_closure_critical_blocker_count <= 10)
    and $backend_delivery_audit.source_alignment.selected_ids_match == true
    and $backend_delivery_audit.source_alignment.dispatch_archive_match == true
    and $backend_delivery_audit.source_alignment.root_report_replay_required_count_after_blocker_closure == 41
    and $backend_delivery_audit.source_alignment.blocker_closure_local_release_artifact_roundtrip_ready == true
    and $backend_delivery_audit.source_alignment.blocker_closure_release_artifact_present == false
    and $backend_delivery_audit.source_alignment.blocker_closure_release_artifact_valid == false
    and $backend_delivery_audit.source_alignment.blocker_closure_release_artifact_roundtrip_present_artifact_present == false
    and $backend_delivery_audit.source_alignment.blocker_closure_release_artifact_roundtrip_present_artifact_valid == false
    and $backend_delivery_audit.source_alignment.blocker_closure_release_artifact_receipt_contract_version == 0
    and $backend_delivery_audit.source_alignment.blocker_closure_release_artifact_evidence_readback_valid == false
    and $backend_delivery_audit.source_alignment.blocker_closure_release_artifact_roundtrip_legacy_simulated_rejected == true
    and $backend_delivery_audit.source_alignment.blocker_closure_release_artifact_roundtrip_v3_valid_branch_selftest_ready == true
    and $backend_delivery_audit.source_alignment.root_report_replay_required_count_after_delivery_audit == 41
    and $backend_delivery_audit.claim_boundary.local_backend_delivery_audit_ready == true
    and ($backend_delivery_audit.claim_boundary.real_backend_receipt_claim_ready | type) == "boolean"
    and ($backend_delivery_audit.claim_boundary.backend_receipt_claim_ready | type) == "boolean"
    and $backend_delivery_audit.claim_boundary.backend_adapter_promoted == false
    and $backend_delivery_audit.claim_boundary.readback_evidence_recorded == false
    and $backend_delivery_audit.claim_boundary.live_product_claim_ready == false
    and $backend_delivery_audit.claim_boundary.public_distribution_claim_ready == false
    and $backend_delivery_audit.claim_boundary.release_claim_ready == false
    and $backend_delivery_audit.side_effects.external_mutation == false
    and ($backend_delivery_audit.delivery_receipt_template_sha256 | test("^[0-9a-f]{64}$"))
    and ($backend_delivery_audit.audit_markdown_sha256 | test("^[0-9a-f]{64}$"));
  def backend_delivery_receipt_roundtrip_ready:
    $backend_delivery_receipt_roundtrip.backend_delivery_receipt_roundtrip_gate_ready == true
    and $backend_delivery_receipt_roundtrip.status == "ready"
    and $backend_delivery_receipt_roundtrip.roundtrip_kind == "local_backend_delivery_receipt_valid_branch_replay"
    and $backend_delivery_receipt_roundtrip.roundtrip_version == 1
    and $backend_delivery_receipt_roundtrip.selected_ids == ["message_search","file_upload_send","media_download_playback","notifications","room_settings"]
    and $backend_delivery_receipt_roundtrip.roundtrip_ready_count == 3
    and $backend_delivery_receipt_roundtrip.source_alignment.waiting_branch_ready == true
    and $backend_delivery_receipt_roundtrip.source_alignment.simulated_receipt_ready == true
    and $backend_delivery_receipt_roundtrip.source_alignment.present_branch_ready == true
    and $backend_delivery_receipt_roundtrip.source_alignment.waiting_branch_delivery_receipt_present == false
    and $backend_delivery_receipt_roundtrip.source_alignment.waiting_branch_delivery_receipt_valid == false
    and $backend_delivery_receipt_roundtrip.source_alignment.waiting_branch_backend_delivery_claim_ready == false
    and $backend_delivery_receipt_roundtrip.source_alignment.present_branch_delivery_receipt_present == true
    and $backend_delivery_receipt_roundtrip.source_alignment.present_branch_delivery_receipt_valid == true
    and $backend_delivery_receipt_roundtrip.source_alignment.present_branch_backend_delivery_claim_ready == true
    and (
      $backend_delivery_receipt_roundtrip.source_alignment.present_branch_critical_blocker_count >= 0
      and $backend_delivery_receipt_roundtrip.source_alignment.present_branch_critical_blocker_count <= 10
    )
    and ($backend_delivery_receipt_roundtrip.source_alignment.present_branch_real_backend_receipt_present | type) == "boolean"
    and ($backend_delivery_receipt_roundtrip.source_alignment.present_branch_backend_receipt_valid | type) == "boolean"
    and $backend_delivery_receipt_roundtrip.source_alignment.dispatch_archive_match == true
    and $backend_delivery_receipt_roundtrip.source_alignment.payload_manifest_match == true
    and $backend_delivery_receipt_roundtrip.source_alignment.root_report_replay_required_count_after_roundtrip == 45
    and $backend_delivery_receipt_roundtrip.claim_boundary.local_backend_delivery_receipt_roundtrip_ready == true
    and $backend_delivery_receipt_roundtrip.claim_boundary.local_backend_delivery_audit_ready == true
    and $backend_delivery_receipt_roundtrip.claim_boundary.backend_delivery_claim_ready == false
    and $backend_delivery_receipt_roundtrip.claim_boundary.real_backend_receipt_claim_ready == false
    and $backend_delivery_receipt_roundtrip.claim_boundary.backend_receipt_claim_ready == false
    and $backend_delivery_receipt_roundtrip.claim_boundary.backend_adapter_promoted == false
    and $backend_delivery_receipt_roundtrip.claim_boundary.readback_evidence_recorded == false
    and $backend_delivery_receipt_roundtrip.claim_boundary.live_runtime_mutation == false
    and $backend_delivery_receipt_roundtrip.claim_boundary.live_product_claim_ready == false
    and $backend_delivery_receipt_roundtrip.claim_boundary.public_distribution_claim_ready == false
    and $backend_delivery_receipt_roundtrip.claim_boundary.release_claim_ready == false
    and ($backend_delivery_receipt_roundtrip.source_report_sha256.simulated_delivery_receipt | test("^[0-9a-f]{64}$"))
    and ($backend_delivery_receipt_roundtrip.source_report_sha256.simulated_delivery_audit | test("^[0-9a-f]{64}$"))
    and ($backend_delivery_receipt_roundtrip.roundtrip_markdown_sha256 | test("^[0-9a-f]{64}$"))
    and $backend_delivery_receipt_roundtrip.side_effects.backend_agent_spawned == false
    and $backend_delivery_receipt_roundtrip.side_effects.backend_repo_write == false
    and $backend_delivery_receipt_roundtrip.side_effects.gateway_call == false
    and $backend_delivery_receipt_roundtrip.side_effects.channel_delivery == false
    and $backend_delivery_receipt_roundtrip.side_effects.external_mutation == false;
  def risk_future_plan_ready:
    $risk_future_plan.risk_future_plan_gate_ready == true
    and $risk_future_plan.status == "ready"
    and $risk_future_plan.plan_kind == "local_ui_post_r151_harsh_top_design_v46_badge_micro_surface_light_glass_risk_future_plan_refresh"
    and $risk_future_plan.plan_version == 1
    and $risk_future_plan.latest_minimum_gate.gate_id == "r151_harsh_top_design_v46_badge_micro_surface_light_glass_minimum_ui_demo_gate"
    and $risk_future_plan.latest_minimum_gate.current_artifact_evidence_ready == true
    and ($risk_future_plan.latest_minimum_gate.current_evidence_mode == "full_hard_true_window" or $risk_future_plan.latest_minimum_gate.current_evidence_mode == "no_window_fixture")
    and $risk_future_plan.latest_minimum_gate.top_design_refresh_version == 46
	    and $risk_future_plan.latest_minimum_gate.top_design_harsh_2026_referee_ready == true
	    and $risk_future_plan.latest_minimum_gate.control_ui_harsh_2026_ready == true
	    and $risk_future_plan.latest_minimum_gate.control_ui_microcopy_word_split_guard_ready == true
	    and $risk_future_plan.latest_minimum_gate.control_ui_logo_clip_guard_ready == true
	    and $risk_future_plan.latest_minimum_gate.control_ui_active_chat_readability_ready == true
	    and $risk_future_plan.latest_minimum_gate.control_ui_placeholder_readability_ready == true
	    and $risk_future_plan.latest_minimum_gate.control_ui_small_control_readability_ready == true
	    and $risk_future_plan.latest_minimum_gate.control_ui_rail_action_icon_ready == true
	    and $risk_future_plan.latest_minimum_gate.control_ui_folder_chip_touch_ready == true
		    and $risk_future_plan.latest_minimum_gate.control_ui_row_menu_touch_ready == true
		    and $risk_future_plan.latest_minimum_gate.control_ui_row_menu_all_rows_ready == true
		    and $risk_future_plan.latest_minimum_gate.control_ui_row_menu_light_glass_ready == true
		    and $risk_future_plan.latest_minimum_gate.control_ui_chat_row_option_semantic_touch_ready == true
		    and $risk_future_plan.latest_minimum_gate.control_ui_thread_tools_menu_ready == true
		    and $risk_future_plan.latest_minimum_gate.control_ui_composer_tools_menu_ready == true
		    and $risk_future_plan.latest_minimum_gate.control_ui_composer_popover_ready == true
	    and $risk_future_plan.latest_minimum_gate.control_ui_composer_popover_search_light_glass_ready == true
		    and $risk_future_plan.latest_minimum_gate.control_ui_micro_surface_light_glass_ready == true
		    and $risk_future_plan.latest_minimum_gate.control_ui_message_routing_badge_light_glass_ready == true
		    and $risk_future_plan.latest_minimum_gate.control_ui_visible_text_integrity_ready == true
	    and $risk_future_plan.latest_minimum_gate.native_secondary_harsh_action_matrix_ready == true
	    and $risk_future_plan.latest_minimum_gate.native_secondary_title_tooltip_ready == true
	    and $risk_future_plan.latest_minimum_gate.native_secondary_title_tooltip_failure_count == 0
    and $risk_future_plan.latest_minimum_gate.tempered_glass_2026_ready == true
    and $risk_future_plan.latest_minimum_gate.tempered_glass_min_contrast_ratio >= 4.5
    and $risk_future_plan.latest_minimum_gate.tempered_glass_clipping_failure_count == 0
    and $risk_future_plan.latest_minimum_gate.requested_scope == "desktop_mobile_all_modules_buttons_submenus"
    and $risk_future_plan.latest_minimum_gate.root_report_replay_required_count_after_risk_future_plan == 45
    and $risk_future_plan.latest_minimum_gate.current_plan_root_report_required_count == 41
    and $risk_future_plan.latest_minimum_gate.selected_row_variant_count == 18
    and $risk_future_plan.latest_minimum_gate.secondary_surface_case_count == 15
    and $risk_future_plan.latest_minimum_gate.secondary_surface_total_action_count == 57
    and $risk_future_plan.latest_minimum_gate.secondary_surface_action_matrix_ready == true
    and $risk_future_plan.latest_minimum_gate.secondary_surface_action_matrix_case_count == 15
    and $risk_future_plan.latest_minimum_gate.secondary_surface_harsh_action_matrix_ready == true
    and $risk_future_plan.latest_minimum_gate.secondary_surface_harsh_action_failure_count == 0
    and $risk_future_plan.latest_minimum_gate.secondary_surface_title_tooltip_ready == true
    and $risk_future_plan.latest_minimum_gate.secondary_surface_title_tooltip_failure_count == 0
    and $risk_future_plan.latest_minimum_gate.true_window_submenu_coverage_ready == true
    and $risk_future_plan.latest_plan_count == 5
    and $risk_future_plan.latest_plan_ids == ["r151_harsh_top_design_v46_badge_micro_surface_light_glass_minimum_ui_demo_gate","backend_delivery_receipt_return","backend_real_receipt_return","ui_refresh_after_real_receipt","release_artifact_roundtrip_and_signed_artifact_gate"]
    and ($risk_future_plan.critical_blockers | length) >= $risk_future_plan.critical_blocker_count
    and ($risk_future_plan.critical_blockers | length) <= ($risk_future_plan.critical_blocker_count + 1)
    and ($risk_future_plan.critical_blocker_count >= 0 and $risk_future_plan.critical_blocker_count <= 11)
    and (
      (
        $risk_future_plan.source_alignment.backend_delivery_receipt_present == false
        and $risk_future_plan.source_alignment.backend_delivery_receipt_valid == false
        and ($risk_future_plan.critical_blockers | map(.id) | index("backend_dispatch_delivery_receipt_missing")) != null
        and $risk_future_plan.claim_boundary.backend_delivery_claim_ready == false
      )
      or
      (
        $risk_future_plan.source_alignment.backend_delivery_receipt_present == true
        and $risk_future_plan.source_alignment.backend_delivery_receipt_valid == true
        and $risk_future_plan.claim_boundary.backend_delivery_claim_ready == true
      )
    )
    and ($risk_future_plan.next_unblock_sequence | length) >= 1
    and $risk_future_plan.source_alignment.top_design_referee_refresh_ready == true
    and $risk_future_plan.source_alignment.tempered_glass_2026_ready == true
    and $risk_future_plan.source_alignment.tempered_glass_min_contrast_ratio >= 4.5
    and $risk_future_plan.source_alignment.tempered_glass_clipping_failure_count == 0
    and $risk_future_plan.source_alignment.current_plan_refresh_ready == true
    and $risk_future_plan.source_alignment.blocker_closure_ready == true
    and $risk_future_plan.source_alignment.backend_delivery_audit_ready == true
    and $risk_future_plan.source_alignment.backend_delivery_receipt_roundtrip_ready == true
    and $risk_future_plan.source_alignment.backend_delivery_receipt_roundtrip_present_branch_ready == true
    and $risk_future_plan.source_alignment.backend_delivery_receipt_roundtrip_present_branch_valid == true
    and $risk_future_plan.source_alignment.backend_delivery_receipt_roundtrip_root_report_required_count == 45
    and ($risk_future_plan.source_alignment.real_backend_receipt_present | type) == "boolean"
    and ($risk_future_plan.source_alignment.backend_receipt_valid | type) == "boolean"
    and $risk_future_plan.source_alignment.root_report_replay_required_count_after_risk_future_plan == 45
    and $risk_future_plan.claim_boundary.local_risk_future_plan_ready == true
    and $risk_future_plan.claim_boundary.local_backend_delivery_receipt_roundtrip_ready == true
    and ($risk_future_plan.claim_boundary.real_backend_receipt_claim_ready | type) == "boolean"
    and ($risk_future_plan.claim_boundary.backend_receipt_claim_ready | type) == "boolean"
    and $risk_future_plan.claim_boundary.live_product_claim_ready == false
    and $risk_future_plan.claim_boundary.public_distribution_claim_ready == false
    and $risk_future_plan.claim_boundary.release_claim_ready == false
    and $risk_future_plan.side_effects.backend_agent_spawned == false
    and $risk_future_plan.side_effects.backend_repo_write == false
    and $risk_future_plan.side_effects.external_mutation == false
    and ($risk_future_plan.risk_plan_markdown_sha256 | test("^[0-9a-f]{64}$"))
    and $risk_future_plan.risk_plan_markdown_bytes > 0;
  def screenshot_files_ready($items; $expected):
    ($items | length) == $expected
    and ($items | all(
      (.bytes // 0) >= 10000
      and ((.sha256 // "") | test("^[0-9a-f]{64}$"))
      and ((.path // "") | length) > 0
      and .visual_probe.ready == true
    ))
    and ($items | map(.sha256 // "") | unique | length) == $expected;
  def native_window_current_source_ready:
    $native_window.enabled == true
    and $native_window.status == "ready"
    and ($native_window.blocked_allowed // false) != true
    and $native_window.true_window_capture_performed == true
    and $native_window.fixture_product_shell_selected_ready == true
    and $native_window.fixture_matrix_composer_hidden_ready == true
    and $native_window.fixture_desktop_product_layout_ready == true
    and $native_window.fixture_mobile_task_first_layout_ready == true
    and $native_window.native_makepad_fixture_script_error_free == true
    and $native_window.native_app_log_error_free == true
    and screenshot_files_ready(($native_window.screenshots // []); 2)
    and $native_window.side_effects.external_mutation == false;
  def native_window_route_current_source_ready:
    $native_window_route.enabled == true
    and $native_window_route.status == "ready"
    and ($native_window_route.blocked_allowed // false) != true
    and $native_window_route.true_window_capture_performed == true
    and $native_window_route.native_makepad_route_variants_ready == true
    and $native_window_route.route_top_design_referee_ready == true
    and $native_window_route.route_content_probe_ready == true
    and $native_window_route.route_count == 4
    and $native_window_route.screenshot_count == 4
    and $native_window_route.route_screenshot_unique_count == 4
    and $native_window_route.route_screenshot_unique_ready == true
    and $native_window_route.native_app_log_error_free == true
    and screenshot_files_ready(($native_window_route.screenshots // []); 4)
    and (($native_window_route.screenshots // []) | all(.visual_probe.route_content_ready == true))
    and $native_window_route.side_effects.external_mutation == false;
  def native_window_route_mobile_current_source_ready:
    $native_window_route_mobile.enabled == true
    and $native_window_route_mobile.status == "ready"
    and ($native_window_route_mobile.blocked_allowed // false) != true
    and $native_window_route_mobile.true_window_capture_performed == true
    and $native_window_route_mobile.native_makepad_mobile_route_variants_ready == true
    and $native_window_route_mobile.route_count == 4
    and $native_window_route_mobile.screenshot_count == 4
    and $native_window_route_mobile.route_screenshot_unique_count == 4
    and $native_window_route_mobile.route_screenshot_unique_ready == true
    and $native_window_route_mobile.non_home_content_log_signature_count >= 3
    and $native_window_route_mobile.mobile_host_window_ready == true
    and $native_window_route_mobile.native_app_log_error_free == true
    and screenshot_files_ready(($native_window_route_mobile.screenshots // []); 4)
    and (($native_window_route_mobile.screenshots // []) | all(
      .viewport_contract.expected_width == 390
      and .viewport_contract.expected_height == 844
      and .viewport_contract.host_window_usable_ready == true
      and .visual_probe.mobile_route_content_ready == true
    ))
    and $native_window_route_mobile.side_effects.external_mutation == false;
  def native_window_secondary_current_source_ready:
    $native_window_secondary.enabled == true
    and $native_window_secondary.status == "ready"
    and ($native_window_secondary.blocked_allowed // false) != true
    and $native_window_secondary.true_window_capture_performed == true
    and $native_window_secondary.native_makepad_secondary_surfaces_ready == true
    and $native_window_secondary.surface_count == 5
    and $native_window_secondary.screenshot_count == 5
    and $native_window_secondary.surface_screenshot_unique_count == 5
    and $native_window_secondary.surface_screenshot_unique_ready == true
    and $native_window_secondary.native_app_log_error_free == true
    and screenshot_files_ready(($native_window_secondary.screenshots // []); 5)
    and $native_window_secondary.side_effects.external_mutation == false;
  def native_window_secondary_mobile_current_source_ready:
    $native_window_secondary_mobile.enabled == true
    and $native_window_secondary_mobile.status == "ready"
    and ($native_window_secondary_mobile.blocked_allowed // false) != true
    and $native_window_secondary_mobile.true_window_capture_performed == true
    and $native_window_secondary_mobile.native_makepad_secondary_mobile_surfaces_ready == true
    and $native_window_secondary_mobile.mobile_secondary_content_probe_ready == true
    and $native_window_secondary_mobile.mobile_secondary_content_visible_count >= 5
    and $native_window_secondary_mobile.mobile_host_window_ready == true
    and $native_window_secondary_mobile.surface_count == 5
    and $native_window_secondary_mobile.screenshot_count == 5
    and $native_window_secondary_mobile.surface_screenshot_unique_count == 5
    and $native_window_secondary_mobile.surface_screenshot_unique_ready == true
    and $native_window_secondary_mobile.native_app_log_error_free == true
    and screenshot_files_ready(($native_window_secondary_mobile.screenshots // []); 5)
    and (($native_window_secondary_mobile.screenshots // []) | all(
      .viewport_contract.expected_width == 390
      and .viewport_contract.expected_height == 844
      and .viewport_contract.host_window_usable_ready == true
      and .visual_probe.mobile_secondary_content_ready == true
    ))
    and $native_window_secondary_mobile.side_effects.external_mutation == false;
  def current_source_true_window_matrix_ready:
    native_window_current_source_ready
    and native_window_route_current_source_ready
    and native_window_route_mobile_current_source_ready
    and native_window_secondary_current_source_ready
    and native_window_secondary_mobile_current_source_ready;
  def true_window_alignment_ready:
    (($native_window.enabled != true) or $native_window.status == "ready" or (($native_window.blocked_allowed // false) == true))
    and (($native_window_route.enabled != true) or ($native_window_route.status == "ready" and $native_window_route.route_content_probe_ready == true) or (($native_window_route.blocked_allowed // false) == true))
    and (($native_window_route_mobile.enabled != true) or ($native_window_route_mobile.status == "ready" and $native_window_route_mobile.native_makepad_mobile_route_variants_ready == true and $native_window_route_mobile.route_count == 4 and $native_window_route_mobile.mobile_host_window_ready == true) or (($native_window_route_mobile.blocked_allowed // false) == true))
    and (($native_window_secondary.enabled != true) or ($native_window_secondary.status == "ready" and $native_window_secondary.surface_count == 5) or (($native_window_secondary.blocked_allowed // false) == true))
    and (($native_window_secondary_mobile.enabled != true) or ($native_window_secondary_mobile.status == "ready" and $native_window_secondary_mobile.mobile_secondary_content_probe_ready == true) or (($native_window_secondary_mobile.blocked_allowed // false) == true));
  def claim_boundary_locked:
    $plan_boundary.live_product_claim.ready == false
    and $plan_boundary.claim_boundary.public_distribution_claim_ready == false
    and $plan_boundary.release_claim.ready == false
    and $native_distribution.public_distribution_ready == false
    and $native_distribution.credential_values_read == false
    and $native_distribution.network_call_performed == false
    and $native_distribution.notary_submission_performed == false
    and $native_distribution.public_distribution_artifact_written == false
    and $native_packaging.local_unsigned_app_bundle.public_distribution_artifact_written == false
    and $evidence_archive.claim_boundary.live_product_claim_ready == false
    and $evidence_archive.claim_boundary.public_distribution_claim_ready == false
    and $evidence_archive.claim_boundary.release_claim_ready == false
    and $release_operator_dry_run.claim_boundary.release_execution_ready == false
    and $release_operator_dry_run.claim_boundary.release_claim_ready == false
    and $release_operator_dry_run.claim_boundary.public_distribution_claim_ready == false
    and $release_operator_dry_run.operator_packet.public_distribution_artifact_written == false;
  (
    root_reports_ready
    and $static_contract.static_contract_ready == true
    and $static_contract.marker_count >= 3642
    and $design_system.status == "ready"
    and $design_system.generated_token_sync_ready == true
    and $design_system.documentation_token_sync_ready == true
    and $design_system.control.css_layer_count == 6
    and $design_system.control.runtime_css_bytes < 300000
    and $design_system.control.important_count <= $design_system.control.important_budget
    and $design_system.control.important_budget == 2100
    and $design_system.control.accessibility_media_queries_ready == true
    and $design_system.control.static_light_theme_ready == true
    and $design_system.control.renderer_light_theme_ready == true
    and $design_system.control.document_direction_source_ready == true
    and $design_system.control.legacy_texture_asset_reference_count == 0
    and $design_system.control.retired_texture_asset_free == true
    and $design_system.native.generated_tokens_registered == true
    and $design_system.native.fixture_generated_tokens_consumed == true
    and $design_system.native.fixture_unified_radius_scale_ready == true
    and $design_system.native.fixture_key_surface_shadows_ready == true
    and $design_system.robrix.selective_module_count == 6
    and (
      $strict_current_source_mode != 1
      or (
        ($design_system.rust_toolchain | test("^rustc 1\\.95\\.0([[:space:]]|$)"))
        and current_source_true_window_matrix_ready
      )
    )
    and $control_browser.status == "ready"
    and control_real_click_v7_ready
    and $native_fixture.status == "ready"
    and $native_fixture.native_secondary_product_surfaces_ready == true
    and $screenshot_manifest.screenshot_manifest_ready == true
    and $native_packaging.local_packaging_gate_ready == true
    and $native_packaging.local_unsigned_app_bundle_probe_ready == true
    and $native_distribution.distribution_preflight_gate_ready == true
    and $backend_contract.backend_contract_waves_ready == true
    and $backend_contract.verified_gap_count == 12
    and $non_base_edge.non_base_edge_gates_ready == true
    and $non_base_edge.verified_edge_count == 4
    and $rollup.productization_blocker_rollup_ready == true
    and base_gap_alignment_ready
    and evidence_chain_ready
    and release_operator_dry_run_ready
    and operator_briefing_ready
    and backend_promotion_packet_ready
    and backend_alignment_evidence_ready
    and critical_path_plan_ready
    and backend_contract_acceptance_ready
    and backend_handoff_export_ready
    and backend_dispatch_packet_ready
    and backend_receipt_intake_ready
    and backend_receipt_roundtrip_ready
    and backend_receipt_refresh_lock_ready
    and future_plan_refresh_ready
    and operator_briefing_refresh_ready
    and release_approval_intake_ready
    and top_design_referee_refresh_ready
    and release_artifact_boundary_ready
    and release_artifact_intake_ready
    and release_artifact_roundtrip_ready
    and current_plan_refresh_ready
    and blocker_closure_ready
    and backend_delivery_audit_ready
    and backend_delivery_receipt_roundtrip_ready
    and risk_future_plan_ready
    and true_window_alignment_ready
    and claim_boundary_locked
    and ($root_reports | all(.ready == true))
    and ($drilldown.side_effects.external_mutation == false)
    and ($work_queue.side_effects.external_mutation == false)
    and ($handoff.side_effects.external_mutation == false)
    and ($plan_boundary.side_effects.external_mutation == false)
    and ($demo_evidence.side_effects.external_mutation == false)
    and ($evidence_archive.side_effects.external_mutation == false)
    and ($release_operator_dry_run.side_effects.external_mutation == false)
    and ($operator_briefing.side_effects.external_mutation == false)
    and ($backend_promotion_packet.side_effects.external_mutation == false)
    and ($backend_alignment_evidence.side_effects.external_mutation == false)
    and ($critical_path_plan.side_effects.external_mutation == false)
    and ($backend_contract_acceptance.side_effects.external_mutation == false)
    and ($backend_handoff_export.side_effects.external_mutation == false)
    and ($backend_dispatch_packet.side_effects.external_mutation == false)
    and ($backend_receipt_roundtrip.side_effects.external_mutation == false)
    and ($backend_receipt_refresh_lock.side_effects.external_mutation == false)
    and ($future_plan_refresh.side_effects.external_mutation == false)
    and ($operator_briefing_refresh.side_effects.external_mutation == false)
    and ($release_approval_intake.side_effects.external_mutation == false)
    and ($top_design_referee_refresh.side_effects.external_mutation == false)
    and ($release_artifact_boundary.side_effects.external_mutation == false)
    and ($release_artifact_intake.side_effects.external_mutation == false)
    and ($release_artifact_roundtrip.side_effects.external_mutation == false)
    and ($current_plan_refresh.side_effects.external_mutation == false)
    and ($blocker_closure.side_effects.external_mutation == false)
    and ($backend_delivery_audit.side_effects.external_mutation == false)
    and ($backend_delivery_receipt_roundtrip.side_effects.external_mutation == false)
    and ($risk_future_plan.side_effects.external_mutation == false)
  ) as $ready
  | {
      product:$product,
      runtime:$runtime,
      gate:$gate,
      status:(if $ready then "ready" else "failed" end),
      root_report_replay_gate_ready:$ready,
      strict_current_source_mode:($strict_current_source_mode == 1),
      current_source_true_window_matrix_ready:current_source_true_window_matrix_ready,
      current_source_true_window_matrix:{
        expected_screenshot_counts:{
          main:2,
          desktop_routes:4,
          mobile_routes:4,
          desktop_secondary:5,
          mobile_secondary:5
        },
        main:{
          enabled:($native_window.enabled == true),
          status:($native_window.status // "not_run"),
          blocked_allowed:($native_window.blocked_allowed // false),
          true_window_capture_performed:($native_window.true_window_capture_performed // false),
          screenshot_count:(($native_window.screenshots // []) | length),
          app_log_error_free:($native_window.native_app_log_error_free // false),
          evidence_ready:native_window_current_source_ready
        },
        desktop_routes:{
          enabled:($native_window_route.enabled == true),
          status:($native_window_route.status // "not_run"),
          blocked_allowed:($native_window_route.blocked_allowed // false),
          true_window_capture_performed:($native_window_route.true_window_capture_performed // false),
          screenshot_count:(($native_window_route.screenshots // []) | length),
          unique_screenshot_count:($native_window_route.route_screenshot_unique_count // 0),
          content_probe_ready:($native_window_route.route_content_probe_ready // false),
          app_log_error_free:($native_window_route.native_app_log_error_free // false),
          evidence_ready:native_window_route_current_source_ready
        },
        mobile_routes:{
          enabled:($native_window_route_mobile.enabled == true),
          status:($native_window_route_mobile.status // "not_run"),
          blocked_allowed:($native_window_route_mobile.blocked_allowed // false),
          true_window_capture_performed:($native_window_route_mobile.true_window_capture_performed // false),
          screenshot_count:(($native_window_route_mobile.screenshots // []) | length),
          unique_screenshot_count:($native_window_route_mobile.route_screenshot_unique_count // 0),
          content_log_signature_count:($native_window_route_mobile.non_home_content_log_signature_count // 0),
          host_window_ready:($native_window_route_mobile.mobile_host_window_ready // false),
          exact_390x844_ready:($native_window_route_mobile.exact_390x844_ready // false),
          app_log_error_free:($native_window_route_mobile.native_app_log_error_free // false),
          evidence_ready:native_window_route_mobile_current_source_ready
        },
        desktop_secondary:{
          enabled:($native_window_secondary.enabled == true),
          status:($native_window_secondary.status // "not_run"),
          blocked_allowed:($native_window_secondary.blocked_allowed // false),
          true_window_capture_performed:($native_window_secondary.true_window_capture_performed // false),
          screenshot_count:(($native_window_secondary.screenshots // []) | length),
          unique_screenshot_count:($native_window_secondary.surface_screenshot_unique_count // 0),
          app_log_error_free:($native_window_secondary.native_app_log_error_free // false),
          evidence_ready:native_window_secondary_current_source_ready
        },
        mobile_secondary:{
          enabled:($native_window_secondary_mobile.enabled == true),
          status:($native_window_secondary_mobile.status // "not_run"),
          blocked_allowed:($native_window_secondary_mobile.blocked_allowed // false),
          true_window_capture_performed:($native_window_secondary_mobile.true_window_capture_performed // false),
          screenshot_count:(($native_window_secondary_mobile.screenshots // []) | length),
          unique_screenshot_count:($native_window_secondary_mobile.surface_screenshot_unique_count // 0),
          content_probe_ready:($native_window_secondary_mobile.mobile_secondary_content_probe_ready // false),
          content_visible_count:($native_window_secondary_mobile.mobile_secondary_content_visible_count // 0),
          host_window_ready:($native_window_secondary_mobile.mobile_host_window_ready // false),
          exact_390x844_ready:($native_window_secondary_mobile.exact_390x844_ready // false),
          app_log_error_free:($native_window_secondary_mobile.native_app_log_error_free // false),
          evidence_ready:native_window_secondary_mobile_current_source_ready
        }
      },
      readiness_dir:$readiness_dir,
      report_path:$report_path,
      root_report_count:($root_reports | length),
      root_json_report_count:($root_reports | map(select(.json_valid == true)) | length),
      root_report_sha256_ready:($root_reports | all(.ready == true)),
      source_reports:$root_reports,
      source_alignment:{
        static_contract_ready:$static_contract.static_contract_ready,
        static_marker_count:$static_contract.marker_count,
        ui_design_system_ready:($design_system.status == "ready" and $design_system.generated_token_sync_ready == true),
        ui_design_system_rust_toolchain:($design_system.rust_toolchain // ""),
        ui_design_system_css_layer_count:$design_system.control.css_layer_count,
        ui_design_system_runtime_css_bytes:$design_system.control.runtime_css_bytes,
        ui_design_system_important_count:$design_system.control.important_count,
        ui_design_system_important_budget:$design_system.control.important_budget,
        ui_design_system_static_light_theme_ready:$design_system.control.static_light_theme_ready,
        ui_design_system_renderer_light_theme_ready:$design_system.control.renderer_light_theme_ready,
        ui_design_system_document_direction_source_ready:$design_system.control.document_direction_source_ready,
        ui_design_system_legacy_texture_asset_reference_count:$design_system.control.legacy_texture_asset_reference_count,
        ui_design_system_retired_texture_asset_free:$design_system.control.retired_texture_asset_free,
        ui_design_system_native_fixture_tokens_ready:$design_system.native.fixture_generated_tokens_consumed,
        native_window_route_mobile_status:($native_window_route_mobile.status // "not_run"),
        native_window_route_mobile_ready:($native_window_route_mobile.native_makepad_mobile_route_variants_ready // false),
        native_window_route_mobile_screenshot_count:(($native_window_route_mobile.screenshots // []) | length),
        native_window_route_mobile_exact_390x844_ready:($native_window_route_mobile.exact_390x844_ready // false),
        native_window_route_mobile_host_window_ready:($native_window_route_mobile.mobile_host_window_ready // false),
        screenshot_manifest_ready:$screenshot_manifest.screenshot_manifest_ready,
        control_ui_ready:($control_browser.status == "ready"),
        control_ui_real_click_v7_ready:control_real_click_v7_ready,
        control_ui_real_click_v7_viewport_count:$control_real_click_v7.summary.control_real_click_activation.viewport_count,
        control_ui_real_click_v7_target_count:$control_real_click_v7.summary.control_real_click_activation.target_count,
        control_ui_real_click_v7_failure_count:$control_real_click_v7.summary.control_real_click_activation.failure_count,
        control_ui_real_click_v7_mobile_routes_ready:$control_real_click_v7.summary.control_real_click_activation.mobile_routes_ready,
        control_ui_real_click_v7_popover_switch_sequence_ready:$control_real_click_v7.summary.control_real_click_activation.popover_switch_sequence_ready,
        control_ui_real_click_v7_popover_switch_step_count:$control_real_click_v7.summary.control_real_click_activation.popover_switch_step_count,
        native_fixture_ready:($native_fixture.status == "ready"),
        native_packaging_ready:$native_packaging.local_packaging_gate_ready,
        native_distribution_preflight_ready:$native_distribution.distribution_preflight_gate_ready,
        base_gap_alignment_ready:base_gap_alignment_ready,
        base_gap_count:$handoff.handoff_count,
        backend_contract_waves_ready:$backend_contract.backend_contract_waves_ready,
        backend_contract_gap_count:$backend_contract.verified_gap_count,
        non_base_edge_gates_ready:$non_base_edge.non_base_edge_gates_ready,
        non_base_edge_count:$non_base_edge.verified_edge_count,
        productization_blocker_rollup_ready:$rollup.productization_blocker_rollup_ready,
        plan_boundary_ready:$plan_boundary.plan_boundary_gate_ready,
        demo_evidence_ready:$demo_evidence.demo_evidence_gate_ready,
        evidence_bundle_ready:$evidence_bundle.evidence_bundle_gate_ready,
        evidence_archive_ready:$evidence_archive.evidence_archive_gate_ready,
        evidence_archive_sha256:$evidence_archive.archive_sha256,
        evidence_archive_bytes:$evidence_archive.archive_bytes,
        release_operator_dry_run_ready:$release_operator_dry_run.release_operator_dry_run_gate_ready,
        release_operator_dry_run_manifest_sha256:$release_operator_dry_run.dry_run_manifest_sha256,
        release_operator_dry_run_manifest_bytes:$release_operator_dry_run.dry_run_manifest_bytes,
        operator_briefing_ready:$operator_briefing.operator_briefing_gate_ready,
        operator_briefing_critical_risk_count:$operator_briefing.critical_risk_count,
        backend_promotion_packet_ready:$backend_promotion_packet.backend_promotion_packet_gate_ready,
        backend_promotion_packet_priority_count:$backend_promotion_packet.priority_packet_count,
        backend_promotion_packet_selected_ids:$backend_promotion_packet.selected_priority_ids,
        backend_alignment_evidence_ready:$backend_alignment_evidence.backend_alignment_evidence_gate_ready,
        backend_alignment_evidence_item_count:$backend_alignment_evidence.alignment_item_count,
        backend_alignment_evidence_selected_ids:$backend_alignment_evidence.selected_alignment_ids,
        critical_path_plan_ready:$critical_path_plan.critical_path_plan_gate_ready,
        critical_path_plan_blocker_count:$critical_path_plan.critical_blocker_count,
        critical_path_plan_selected_ids:$critical_path_plan.current_backend_selected_ids,
        backend_contract_acceptance_ready:$backend_contract_acceptance.backend_contract_acceptance_gate_ready,
        backend_contract_acceptance_item_count:$backend_contract_acceptance.acceptance_item_count,
        backend_contract_acceptance_selected_ids:$backend_contract_acceptance.selected_acceptance_ids,
        backend_handoff_export_ready:$backend_handoff_export.backend_handoff_export_gate_ready,
        backend_handoff_export_item_count:$backend_handoff_export.export_item_count,
        backend_handoff_export_selected_ids:$backend_handoff_export.selected_export_ids,
        backend_dispatch_packet_ready:$backend_dispatch_packet.backend_dispatch_packet_gate_ready,
        backend_dispatch_packet_item_count:$backend_dispatch_packet.packet_item_count,
        backend_dispatch_packet_selected_ids:$backend_dispatch_packet.selected_packet_ids,
        backend_dispatch_packet_archive_sha256:$backend_dispatch_packet.archive_sha256,
        backend_dispatch_packet_archive_bytes:$backend_dispatch_packet.archive_bytes,
        backend_receipt_intake_ready:$backend_receipt_intake.backend_receipt_intake_gate_ready,
        backend_receipt_intake_selected_ids:$backend_receipt_intake.selected_receipt_ids,
        backend_receipt_intake_template_sha256:$backend_receipt_intake.template_sha256,
        backend_receipt_intake_template_bytes:$backend_receipt_intake.template_bytes,
        backend_receipt_intake_waiting_for_receipt:$backend_receipt_intake.waiting_for_backend_receipt,
        backend_receipt_present:$backend_receipt_intake.backend_receipt_present,
        backend_receipt_valid:$backend_receipt_intake.backend_receipt_valid,
        backend_receipt_roundtrip_ready:$backend_receipt_roundtrip.backend_receipt_roundtrip_gate_ready,
        backend_receipt_roundtrip_selected_ids:$backend_receipt_roundtrip.selected_roundtrip_ids,
        backend_receipt_roundtrip_ready_count:$backend_receipt_roundtrip.roundtrip_ready_count,
        backend_receipt_roundtrip_waiting_branch_ready:$backend_receipt_roundtrip.source_alignment.backend_receipt_waiting_branch_ready,
        backend_receipt_roundtrip_present_branch_ready:$backend_receipt_roundtrip.source_alignment.backend_receipt_present_branch_ready,
        backend_receipt_roundtrip_simulated_receipt_ready:$backend_receipt_roundtrip.source_alignment.simulated_receipt_ready,
        backend_receipt_refresh_lock_ready:$backend_receipt_refresh_lock.backend_receipt_refresh_lock_gate_ready,
        backend_receipt_refresh_lock_selected_ids:$backend_receipt_refresh_lock.selected_refresh_ids,
        backend_receipt_refresh_lock_real_receipt_present:$backend_receipt_refresh_lock.receipt_state.real_backend_receipt_present,
        backend_receipt_refresh_lock_simulated_input_present:$backend_receipt_refresh_lock.receipt_state.simulated_receipt_input_present,
        backend_receipt_refresh_lock_hard_true_window_ready:$backend_receipt_refresh_lock.refresh_requirements.hard_true_window_refresh_ready,
        backend_receipt_refresh_lock_full_hard_required:$backend_receipt_refresh_lock.refresh_requirements.full_hard_refresh_required,
        future_plan_refresh_ready:$future_plan_refresh.future_plan_refresh_gate_ready,
        future_plan_refresh_minimum_gate_id:$future_plan_refresh.future_plan[0].id,
        future_plan_refresh_current_full_hard_evidence_ready:$future_plan_refresh.r52_minimum_gate.current_full_hard_evidence_ready,
        future_plan_refresh_required_root_report_count:$future_plan_refresh.r52_minimum_gate.root_report_replay_required_count,
        future_plan_refresh_ids:($future_plan_refresh.future_plan | map(.id)),
        operator_briefing_refresh_ready:$operator_briefing_refresh.operator_briefing_refresh_gate_ready,
        operator_briefing_refresh_critical_risk_count:$operator_briefing_refresh.updated_critical_risk_count,
        operator_briefing_refresh_current_plan_ids:$operator_briefing_refresh.current_next_plan_ids,
        operator_briefing_refresh_root_report_required_count:$operator_briefing_refresh.current_state.root_report_replay_required_count_after_refresh,
        operator_briefing_refresh_dispatch_archive_sha256:$operator_briefing_refresh.backend_dispatch_pointer.archive_sha256,
        operator_briefing_refresh_markdown_sha256:$operator_briefing_refresh.refresh_markdown_sha256,
        release_approval_intake_ready:$release_approval_intake.release_approval_intake_gate_ready,
        release_approval_intake_waiting_for_approval:$release_approval_intake.release_approval_state.waiting_for_release_approval,
        release_approval_present:$release_approval_intake.release_approval_state.release_approval_present,
        release_approval_valid:$release_approval_intake.release_approval_state.release_approval_valid,
        independent_approval_verifier_ready:$release_approval_intake.release_approval_state.independent_approval_verifier_ready,
        self_reported_approval_can_authorize_release:$release_approval_intake.release_approval_state.self_reported_approval_can_authorize_release,
        approval_valid_branch_supported:$release_approval_intake.source_alignment.approval_valid_branch_supported,
        release_approval_intake_template_sha256:$release_approval_intake.template_sha256,
        release_approval_intake_template_bytes:$release_approval_intake.template_bytes,
        release_approval_intake_root_report_required_count:$release_approval_intake.release_approval_state.root_report_replay_required_count_after_intake,
        top_design_referee_refresh_ready:$top_design_referee_refresh.top_design_referee_refresh_gate_ready,
        top_design_referee_refresh_version:$top_design_referee_refresh.refresh_version,
	        top_design_harsh_2026_referee_ready:$top_design_referee_refresh.top_design_harsh_2026_referee_ready,
	        top_design_control_ui_harsh_2026_ready:$top_design_referee_refresh.control_ui_harsh_2026_ready,
	        top_design_control_ui_microcopy_word_split_guard_ready:$top_design_referee_refresh.referee_matrix.control_ui.microcopy_word_split_guard_ready,
	        top_design_control_ui_logo_clip_guard_ready:$top_design_referee_refresh.referee_matrix.control_ui.logo_clip_guard_ready,
	        top_design_control_ui_active_chat_readability_ready:$top_design_referee_refresh.referee_matrix.control_ui.active_chat_readability_ready,
	        top_design_control_ui_placeholder_readability_ready:$top_design_referee_refresh.referee_matrix.control_ui.placeholder_readability_ready,
	        top_design_control_ui_small_control_readability_ready:$top_design_referee_refresh.referee_matrix.control_ui.small_control_readability_ready,
	        top_design_control_ui_rail_action_icon_ready:$top_design_referee_refresh.referee_matrix.control_ui.rail_action_icon_ready,
	        top_design_control_ui_folder_chip_touch_ready:$top_design_referee_refresh.referee_matrix.control_ui.folder_chip_touch_ready,
	        top_design_control_ui_row_menu_touch_ready:$top_design_referee_refresh.referee_matrix.control_ui.row_menu_touch_ready,
	        top_design_control_ui_row_menu_all_rows_ready:$top_design_referee_refresh.referee_matrix.control_ui.row_menu_all_rows_ready,
	        top_design_control_ui_row_menu_light_glass_ready:$top_design_referee_refresh.referee_matrix.control_ui.row_menu_light_glass_ready,
	        top_design_control_ui_command_palette_ready:$top_design_referee_refresh.referee_matrix.control_ui.command_palette_ready,
	        top_design_control_ui_command_palette_surface_light_glass_ready:$top_design_referee_refresh.referee_matrix.control_ui.command_palette_surface_light_glass_ready,
	        top_design_control_ui_command_palette_trigger_light_glass_ready:$top_design_referee_refresh.referee_matrix.control_ui.command_palette_trigger_light_glass_ready,
	        top_design_control_ui_command_palette_close_light_glass_ready:$top_design_referee_refresh.referee_matrix.control_ui.command_palette_close_light_glass_ready,
	        top_design_control_ui_command_palette_input_light_glass_ready:$top_design_referee_refresh.referee_matrix.control_ui.command_palette_input_light_glass_ready,
	        top_design_control_ui_command_palette_item_light_glass_ready:$top_design_referee_refresh.referee_matrix.control_ui.command_palette_item_light_glass_ready,
	        top_design_control_ui_form_control_title_touch_ready:$top_design_referee_refresh.referee_matrix.control_ui.form_control_title_touch_ready,
	        top_design_control_ui_chat_row_option_semantic_touch_ready:$top_design_referee_refresh.referee_matrix.control_ui.chat_row_option_semantic_touch_ready,
	        top_design_control_ui_thread_tools_menu_ready:$top_design_referee_refresh.referee_matrix.control_ui.thread_tools_menu_ready,
	        top_design_control_ui_composer_tools_menu_ready:$top_design_referee_refresh.referee_matrix.control_ui.composer_tools_menu_ready,
	        top_design_control_ui_composer_popover_ready:$top_design_referee_refresh.referee_matrix.control_ui.composer_popover_ready,
	        top_design_control_ui_composer_popover_search_light_glass_ready:$top_design_referee_refresh.referee_matrix.control_ui.composer_popover_search_light_glass_ready,
	        top_design_control_ui_rail_search_light_glass_ready:$top_design_referee_refresh.referee_matrix.control_ui.rail_search_light_glass_ready,
	        top_design_control_ui_micro_surface_light_glass_ready:$top_design_referee_refresh.referee_matrix.control_ui.micro_surface_light_glass_ready,
	        top_design_control_ui_message_routing_badge_light_glass_ready:$top_design_referee_refresh.referee_matrix.control_ui.message_routing_badge_light_glass_ready,
	        top_design_control_ui_thread_intro_badge_light_glass_ready:$top_design_referee_refresh.referee_matrix.control_ui.thread_intro_badge_light_glass_ready,
	        top_design_control_ui_status_trust_strip_light_glass_ready:$top_design_referee_refresh.referee_matrix.control_ui.status_trust_strip_light_glass_ready,
	        top_design_control_ui_visible_text_integrity_ready:$top_design_referee_refresh.referee_matrix.control_ui.visible_text_integrity_ready,
	        top_design_control_ui_icon_button_title_match_ready:$top_design_referee_refresh.referee_matrix.control_ui.icon_button_title_match_ready,
	        top_design_control_ui_menu_trigger_title_match_ready:$top_design_referee_refresh.referee_matrix.control_ui.menu_trigger_title_match_ready,
	        top_design_native_secondary_harsh_action_matrix_ready:$top_design_referee_refresh.native_secondary_harsh_action_matrix_ready,
	        top_design_native_secondary_title_tooltip_ready:$top_design_referee_refresh.referee_matrix.control_level.secondary_surface_title_tooltip_ready,
	        top_design_native_secondary_title_tooltip_failure_count:$top_design_referee_refresh.referee_matrix.control_level.secondary_surface_title_tooltip_failure_count,
        top_design_tempered_glass_2026_ready:$top_design_referee_refresh.referee_matrix.tempered_glass_2026.ready,
        top_design_tempered_glass_min_contrast_ratio:$top_design_referee_refresh.referee_matrix.tempered_glass_2026.min_contrast_ratio,
        top_design_tempered_glass_clipping_failure_count:$top_design_referee_refresh.referee_matrix.tempered_glass_2026.clipping_failure_count,
        top_design_referee_refresh_action_matrix_ready:$top_design_referee_refresh.referee_matrix.control_level.secondary_surface_action_matrix_ready,
        top_design_referee_refresh_action_matrix_case_count:$top_design_referee_refresh.referee_matrix.control_level.secondary_surface_action_matrix_case_count,
        top_design_referee_refresh_harsh_action_matrix_ready:$top_design_referee_refresh.referee_matrix.control_level.secondary_surface_harsh_action_matrix_ready,
        top_design_referee_refresh_harsh_action_failure_count:$top_design_referee_refresh.referee_matrix.control_level.secondary_surface_harsh_action_failure_count,
        top_design_referee_refresh_control_phone320_ready:$top_design_referee_refresh.referee_matrix.control_ui.persisted_phone320_screenshot_ready,
        top_design_referee_refresh_current_plan_ids:$top_design_referee_refresh.source_alignment.current_plan_ids,
        top_design_referee_refresh_current_roundtrip_plan_ids:$top_design_referee_refresh.source_alignment.current_roundtrip_plan_ids,
        top_design_referee_refresh_current_minimum_gate_id:$top_design_referee_refresh.current_referee_alignment.current_minimum_gate_id,
        top_design_referee_refresh_root_report_required_count:$top_design_referee_refresh.current_state.root_report_replay_required_count_after_top_design_refresh,
        top_design_referee_refresh_downstream_root_report_required_count:$top_design_referee_refresh.current_state.downstream_root_report_replay_required_count_after_release_artifact_roundtrip,
        top_design_referee_refresh_markdown_sha256:$top_design_referee_refresh.refresh_markdown_sha256,
        release_artifact_boundary_ready:$release_artifact_boundary.release_artifact_boundary_gate_ready,
        release_artifact_boundary_root_report_required_count:$release_artifact_boundary.release_artifact_boundary.root_report_replay_required_count_after_boundary,
        release_artifact_boundary_unsigned_app_codesign_status:$release_artifact_boundary.release_artifact_boundary.unsigned_app_bundle_codesign_status,
        release_artifact_boundary_next_required_artifact_gate:$release_artifact_boundary.release_artifact_boundary.next_required_artifact_gate,
        release_artifact_boundary_signed_notarized_stapled_artifact_present:$release_artifact_boundary.release_artifact_boundary.signed_notarized_stapled_artifact_present,
        release_artifact_boundary_public_distribution_artifact_written:$release_artifact_boundary.release_artifact_boundary.public_distribution_artifact_written,
        release_artifact_boundary_markdown_sha256:$release_artifact_boundary.boundary_markdown_sha256,
        release_artifact_intake_ready:$release_artifact_intake.release_artifact_intake_gate_ready,
        release_artifact_intake_root_report_required_count:$release_artifact_intake.root_report_replay_required_count_after_intake,
        release_artifact_intake_waiting_for_artifact:$release_artifact_intake.release_artifact_state.waiting_for_release_artifact,
        release_artifact_present:$release_artifact_intake.release_artifact_state.release_artifact_present,
        release_artifact_valid:$release_artifact_intake.release_artifact_state.release_artifact_valid,
        release_artifact_intake_artifact_present:$release_artifact_intake.release_artifact_state.release_artifact_present,
        release_artifact_intake_artifact_valid:$release_artifact_intake.release_artifact_state.release_artifact_valid,
        release_artifact_intake_present_artifact_branch_supported:$release_artifact_intake.source_alignment.present_artifact_branch_supported,
        release_artifact_intake_independent_approval_verifier_contract_ready:$release_artifact_intake.source_alignment.independent_approval_verifier_contract_ready,
        release_artifact_intake_signed_notarized_stapled_artifact_present:$release_artifact_intake.release_artifact_state.signed_notarized_stapled_artifact_present,
        release_artifact_intake_public_distribution_artifact_written:$release_artifact_intake.release_artifact_state.public_distribution_artifact_written,
        release_artifact_intake_template_sha256:$release_artifact_intake.template_sha256,
        release_artifact_intake_markdown_sha256:$release_artifact_intake.markdown_sha256,
        release_artifact_roundtrip_ready:$release_artifact_roundtrip.release_artifact_roundtrip_gate_ready,
        release_artifact_roundtrip_ready_count:$release_artifact_roundtrip.roundtrip_ready_count,
        release_artifact_roundtrip_present_branch_ready:$release_artifact_roundtrip.source_alignment.present_branch_ready,
        release_artifact_roundtrip_waiting_branch_ready:$release_artifact_roundtrip.source_alignment.waiting_branch_ready,
        release_artifact_roundtrip_present_artifact_valid:$release_artifact_roundtrip.source_alignment.present_branch_release_artifact_valid,
        release_artifact_roundtrip_present_artifact_branch_supported:$release_artifact_roundtrip.source_alignment.present_artifact_branch_supported,
        release_artifact_roundtrip_independent_approval_verifier_contract_ready:$release_artifact_roundtrip.source_alignment.independent_approval_verifier_contract_ready,
        release_artifact_roundtrip_root_report_required_count:$release_artifact_roundtrip.source_alignment.root_report_replay_required_count_after_roundtrip,
        release_artifact_roundtrip_legacy_simulated_rejected:$release_artifact_roundtrip.source_alignment.legacy_simulated_artifact_rejected,
        release_artifact_roundtrip_v3_valid_branch_selftest_ready:$release_artifact_roundtrip.source_alignment.v3_valid_branch_selftest_ready,
        release_artifact_roundtrip_legacy_artifact_sha256:$release_artifact_roundtrip.source_report_sha256.legacy_v1_simulated_artifact,
        release_artifact_roundtrip_legacy_rejection_report_sha256:$release_artifact_roundtrip.source_report_sha256.legacy_v1_rejection_intake,
        release_artifact_roundtrip_v3_selftest_log_sha256:$release_artifact_roundtrip.source_report_sha256.v3_intake_selftest_log,
        current_plan_refresh_ready:$current_plan_refresh.current_plan_refresh_gate_ready,
        current_plan_refresh_minimum_gate_id:$current_plan_refresh.current_minimum_gate.gate_id,
        current_plan_refresh_root_report_required_count:$current_plan_refresh.current_minimum_gate.root_report_replay_required_count_after_current_plan_refresh,
        current_plan_refresh_current_plan_ids:$current_plan_refresh.current_plan_ids,
        current_plan_refresh_markdown_sha256:$current_plan_refresh.refresh_markdown_sha256,
        blocker_closure_ready:$blocker_closure.blocker_closure_gate_ready,
        blocker_closure_root_report_required_count:$blocker_closure.closure_state.root_report_replay_required_count_after_blocker_closure,
        blocker_closure_critical_blocker_count:$blocker_closure.critical_blocker_count,
        blocker_closure_backend_agent_available:$blocker_closure.closure_state.backend_agent_available,
        blocker_closure_real_backend_receipt_present:$blocker_closure.closure_state.real_backend_receipt_present,
        blocker_closure_release_artifact_valid:$blocker_closure.closure_state.release_artifact_valid,
        blocker_closure_markdown_sha256:$blocker_closure.closure_markdown_sha256,
        backend_delivery_audit_ready:$backend_delivery_audit.backend_delivery_audit_gate_ready,
        backend_delivery_audit_root_report_required_count:$backend_delivery_audit.delivery_state.root_report_replay_required_count_after_delivery_audit,
        backend_delivery_audit_delivery_receipt_present:$backend_delivery_audit.delivery_state.delivery_receipt_present,
        backend_delivery_audit_delivery_receipt_valid:$backend_delivery_audit.delivery_state.delivery_receipt_valid,
        backend_delivery_audit_waiting_for_delivery_receipt:$backend_delivery_audit.delivery_state.waiting_for_delivery_receipt,
        backend_delivery_audit_critical_blocker_count:$backend_delivery_audit.critical_blocker_count,
        backend_delivery_audit_backend_delivery_claim_ready:$backend_delivery_audit.delivery_state.backend_delivery_claim_ready,
        backend_delivery_audit_markdown_sha256:$backend_delivery_audit.audit_markdown_sha256,
        backend_delivery_receipt_roundtrip_ready:$backend_delivery_receipt_roundtrip.backend_delivery_receipt_roundtrip_gate_ready,
        backend_delivery_receipt_roundtrip_ready_count:$backend_delivery_receipt_roundtrip.roundtrip_ready_count,
        backend_delivery_receipt_roundtrip_waiting_branch_ready:$backend_delivery_receipt_roundtrip.source_alignment.waiting_branch_ready,
        backend_delivery_receipt_roundtrip_present_branch_ready:$backend_delivery_receipt_roundtrip.source_alignment.present_branch_ready,
        backend_delivery_receipt_roundtrip_simulated_receipt_ready:$backend_delivery_receipt_roundtrip.source_alignment.simulated_receipt_ready,
        backend_delivery_receipt_roundtrip_present_branch_valid:$backend_delivery_receipt_roundtrip.source_alignment.present_branch_delivery_receipt_valid,
        backend_delivery_receipt_roundtrip_present_branch_claim_ready:$backend_delivery_receipt_roundtrip.source_alignment.present_branch_backend_delivery_claim_ready,
        backend_delivery_receipt_roundtrip_root_report_required_count:$backend_delivery_receipt_roundtrip.source_alignment.root_report_replay_required_count_after_roundtrip,
        backend_delivery_receipt_roundtrip_simulated_receipt_sha256:$backend_delivery_receipt_roundtrip.source_report_sha256.simulated_delivery_receipt,
        backend_delivery_receipt_roundtrip_present_report_sha256:$backend_delivery_receipt_roundtrip.source_report_sha256.simulated_delivery_audit,
        risk_future_plan_ready:$risk_future_plan.risk_future_plan_gate_ready,
        risk_future_plan_latest_minimum_gate_id:$risk_future_plan.latest_minimum_gate.gate_id,
        risk_future_plan_latest_plan_ids:$risk_future_plan.latest_plan_ids,
        risk_future_plan_top_design_refresh_version:$risk_future_plan.latest_minimum_gate.top_design_refresh_version,
	        risk_future_plan_top_design_harsh_2026_referee_ready:$risk_future_plan.latest_minimum_gate.top_design_harsh_2026_referee_ready,
	        risk_future_plan_control_ui_harsh_2026_ready:$risk_future_plan.latest_minimum_gate.control_ui_harsh_2026_ready,
	        risk_future_plan_control_ui_microcopy_word_split_guard_ready:$risk_future_plan.latest_minimum_gate.control_ui_microcopy_word_split_guard_ready,
	        risk_future_plan_control_ui_logo_clip_guard_ready:$risk_future_plan.latest_minimum_gate.control_ui_logo_clip_guard_ready,
	        risk_future_plan_control_ui_active_chat_readability_ready:$risk_future_plan.latest_minimum_gate.control_ui_active_chat_readability_ready,
	        risk_future_plan_control_ui_placeholder_readability_ready:$risk_future_plan.latest_minimum_gate.control_ui_placeholder_readability_ready,
	        risk_future_plan_control_ui_small_control_readability_ready:$risk_future_plan.latest_minimum_gate.control_ui_small_control_readability_ready,
	        risk_future_plan_control_ui_rail_action_icon_ready:$risk_future_plan.latest_minimum_gate.control_ui_rail_action_icon_ready,
	        risk_future_plan_control_ui_folder_chip_touch_ready:$risk_future_plan.latest_minimum_gate.control_ui_folder_chip_touch_ready,
	        risk_future_plan_control_ui_row_menu_touch_ready:$risk_future_plan.latest_minimum_gate.control_ui_row_menu_touch_ready,
	        risk_future_plan_control_ui_row_menu_all_rows_ready:$risk_future_plan.latest_minimum_gate.control_ui_row_menu_all_rows_ready,
	        risk_future_plan_control_ui_row_menu_light_glass_ready:$risk_future_plan.latest_minimum_gate.control_ui_row_menu_light_glass_ready,
	        risk_future_plan_control_ui_command_palette_ready:$risk_future_plan.latest_minimum_gate.control_ui_command_palette_ready,
	        risk_future_plan_control_ui_command_palette_surface_light_glass_ready:$risk_future_plan.latest_minimum_gate.control_ui_command_palette_surface_light_glass_ready,
	        risk_future_plan_control_ui_command_palette_trigger_light_glass_ready:$risk_future_plan.latest_minimum_gate.control_ui_command_palette_trigger_light_glass_ready,
	        risk_future_plan_control_ui_command_palette_close_light_glass_ready:$risk_future_plan.latest_minimum_gate.control_ui_command_palette_close_light_glass_ready,
	        risk_future_plan_control_ui_command_palette_input_light_glass_ready:$risk_future_plan.latest_minimum_gate.control_ui_command_palette_input_light_glass_ready,
	        risk_future_plan_control_ui_command_palette_item_light_glass_ready:$risk_future_plan.latest_minimum_gate.control_ui_command_palette_item_light_glass_ready,
	        risk_future_plan_control_ui_form_control_title_touch_ready:$risk_future_plan.latest_minimum_gate.control_ui_form_control_title_touch_ready,
	        risk_future_plan_control_ui_chat_row_option_semantic_touch_ready:$risk_future_plan.latest_minimum_gate.control_ui_chat_row_option_semantic_touch_ready,
	        risk_future_plan_control_ui_thread_tools_menu_ready:$risk_future_plan.latest_minimum_gate.control_ui_thread_tools_menu_ready,
	        risk_future_plan_control_ui_composer_tools_menu_ready:$risk_future_plan.latest_minimum_gate.control_ui_composer_tools_menu_ready,
	        risk_future_plan_control_ui_composer_popover_ready:$risk_future_plan.latest_minimum_gate.control_ui_composer_popover_ready,
	        risk_future_plan_control_ui_composer_popover_search_light_glass_ready:$risk_future_plan.latest_minimum_gate.control_ui_composer_popover_search_light_glass_ready,
	        risk_future_plan_control_ui_rail_search_light_glass_ready:$risk_future_plan.latest_minimum_gate.control_ui_rail_search_light_glass_ready,
	        risk_future_plan_control_ui_micro_surface_light_glass_ready:$risk_future_plan.latest_minimum_gate.control_ui_micro_surface_light_glass_ready,
	        risk_future_plan_control_ui_message_routing_badge_light_glass_ready:$risk_future_plan.latest_minimum_gate.control_ui_message_routing_badge_light_glass_ready,
	        risk_future_plan_control_ui_thread_intro_badge_light_glass_ready:$risk_future_plan.latest_minimum_gate.control_ui_thread_intro_badge_light_glass_ready,
	        risk_future_plan_control_ui_status_trust_strip_light_glass_ready:$risk_future_plan.latest_minimum_gate.control_ui_status_trust_strip_light_glass_ready,
	        risk_future_plan_control_ui_visible_text_integrity_ready:$risk_future_plan.latest_minimum_gate.control_ui_visible_text_integrity_ready,
	        risk_future_plan_control_ui_icon_button_title_match_ready:$risk_future_plan.latest_minimum_gate.control_ui_icon_button_title_match_ready,
	        risk_future_plan_control_ui_menu_trigger_title_match_ready:$risk_future_plan.latest_minimum_gate.control_ui_menu_trigger_title_match_ready,
	        risk_future_plan_native_secondary_harsh_action_matrix_ready:$risk_future_plan.latest_minimum_gate.native_secondary_harsh_action_matrix_ready,
	        risk_future_plan_native_secondary_title_tooltip_ready:$risk_future_plan.latest_minimum_gate.native_secondary_title_tooltip_ready,
	        risk_future_plan_native_secondary_title_tooltip_failure_count:$risk_future_plan.latest_minimum_gate.native_secondary_title_tooltip_failure_count,
        risk_future_plan_tempered_glass_2026_ready:$risk_future_plan.latest_minimum_gate.tempered_glass_2026_ready,
        risk_future_plan_tempered_glass_min_contrast_ratio:$risk_future_plan.latest_minimum_gate.tempered_glass_min_contrast_ratio,
        risk_future_plan_tempered_glass_clipping_failure_count:$risk_future_plan.latest_minimum_gate.tempered_glass_clipping_failure_count,
        risk_future_plan_action_matrix_ready:$risk_future_plan.latest_minimum_gate.secondary_surface_action_matrix_ready,
        risk_future_plan_action_matrix_case_count:$risk_future_plan.latest_minimum_gate.secondary_surface_action_matrix_case_count,
        risk_future_plan_harsh_action_matrix_ready:$risk_future_plan.latest_minimum_gate.secondary_surface_harsh_action_matrix_ready,
        risk_future_plan_harsh_action_failure_count:$risk_future_plan.latest_minimum_gate.secondary_surface_harsh_action_failure_count,
        risk_future_plan_critical_blocker_count:$risk_future_plan.critical_blocker_count,
        risk_future_plan_root_report_required_count:$risk_future_plan.latest_minimum_gate.root_report_replay_required_count_after_risk_future_plan,
        risk_future_plan_markdown_sha256:$risk_future_plan.risk_plan_markdown_sha256,
        true_window_alignment_ready:true_window_alignment_ready
      },
      claim_boundary:{
        local_root_report_replay_ready:$ready,
        local_fixture_demo_ready:$plan_boundary.claim_boundary.local_fixture_demo_ready,
        local_evidence_bundle_ready:$evidence_bundle.claim_boundary.local_evidence_bundle_ready,
        local_evidence_archive_ready:$evidence_archive.claim_boundary.local_evidence_archive_ready,
        local_release_operator_dry_run_ready:$release_operator_dry_run.claim_boundary.local_release_operator_dry_run_ready,
        local_operator_briefing_ready:$operator_briefing.claim_boundary.local_operator_briefing_ready,
        local_backend_promotion_packet_ready:$backend_promotion_packet.claim_boundary.local_backend_promotion_packet_ready,
        local_backend_alignment_evidence_ready:$backend_alignment_evidence.claim_boundary.local_backend_alignment_evidence_ready,
        local_critical_path_plan_ready:$critical_path_plan.claim_boundary.local_critical_path_plan_ready,
        local_backend_contract_acceptance_ready:$backend_contract_acceptance.claim_boundary.local_backend_contract_acceptance_ready,
        local_backend_handoff_export_ready:$backend_handoff_export.claim_boundary.local_backend_handoff_export_ready,
        local_backend_dispatch_packet_ready:$backend_dispatch_packet.claim_boundary.local_backend_dispatch_packet_ready,
        local_backend_receipt_intake_ready:$backend_receipt_intake.claim_boundary.local_backend_receipt_intake_ready,
        local_backend_receipt_roundtrip_ready:$backend_receipt_roundtrip.claim_boundary.local_backend_receipt_roundtrip_ready,
        local_backend_receipt_refresh_lock_ready:$backend_receipt_refresh_lock.claim_boundary.local_backend_receipt_refresh_lock_ready,
        local_future_plan_refresh_ready:$future_plan_refresh.claim_boundary.local_future_plan_refresh_ready,
        local_operator_briefing_refresh_ready:$operator_briefing_refresh.claim_boundary.local_operator_briefing_refresh_ready,
        local_release_approval_intake_ready:$release_approval_intake.claim_boundary.local_release_approval_intake_ready,
        local_top_design_referee_refresh_ready:$top_design_referee_refresh.claim_boundary.local_top_design_referee_refresh_ready,
        local_release_artifact_boundary_ready:$release_artifact_boundary.claim_boundary.local_release_artifact_boundary_ready,
        local_release_artifact_intake_ready:$release_artifact_intake.claim_boundary.local_release_artifact_intake_ready,
        local_release_artifact_roundtrip_ready:$release_artifact_roundtrip.claim_boundary.local_release_artifact_roundtrip_ready,
        local_current_plan_refresh_ready:$current_plan_refresh.claim_boundary.local_current_plan_refresh_ready,
        local_blocker_closure_ready:$blocker_closure.claim_boundary.local_blocker_closure_ready,
        local_backend_delivery_audit_ready:$backend_delivery_audit.claim_boundary.local_backend_delivery_audit_ready,
        local_backend_delivery_receipt_roundtrip_ready:$backend_delivery_receipt_roundtrip.claim_boundary.local_backend_delivery_receipt_roundtrip_ready,
        local_risk_future_plan_ready:$risk_future_plan.claim_boundary.local_risk_future_plan_ready,
        desktop_mobile_design_claim_ready:$top_design_referee_refresh.claim_boundary.desktop_mobile_design_claim_ready,
        release_approval_claim_ready:$release_approval_intake.claim_boundary.release_approval_claim_ready,
        backend_delivery_claim_ready:$backend_delivery_audit.claim_boundary.backend_delivery_claim_ready,
        release_artifact_claim_ready:$release_artifact_boundary.claim_boundary.release_artifact_claim_ready,
        backend_receipt_claim_ready:$backend_receipt_refresh_lock.claim_boundary.backend_receipt_claim_ready,
        real_backend_receipt_claim_ready:$backend_receipt_refresh_lock.claim_boundary.real_backend_receipt_claim_ready,
        simulated_backend_receipt_branch_ready:$backend_receipt_roundtrip.claim_boundary.simulated_backend_receipt_branch_ready,
        hard_true_window_required:$evidence_archive.hard_true_window_required,
        r33_hard_demo_evidence_ready:$evidence_archive.r33_hard_demo_evidence_ready,
        live_product_claim_ready:false,
        public_distribution_claim_ready:false,
        release_claim_ready:false,
        external_actions_allowed:false,
        public_upload_performed:false,
        signing_notarization_performed:false
      },
      future_plan_replay:{
        next_plan_count:($plan_boundary.next_plan | length),
        next_plan:$plan_boundary.next_plan,
        backend_contract_remaining_count:$plan_boundary.live_product_claim.remaining_backend_contract_count,
        backend_priority_ids:($handoff.items | sort_by(.priority) | map(.id)),
        backend_promotion_packet_selected_ids:$backend_promotion_packet.selected_priority_ids,
        backend_alignment_evidence_selected_ids:$backend_alignment_evidence.selected_alignment_ids,
        critical_path_plan_selected_ids:$critical_path_plan.current_backend_selected_ids,
        backend_contract_acceptance_selected_ids:$backend_contract_acceptance.selected_acceptance_ids,
        backend_contract_acceptance_future_plan_link:$backend_contract_acceptance.future_plan_link,
        backend_handoff_export_selected_ids:$backend_handoff_export.selected_export_ids,
        backend_handoff_export_target_repo:$backend_handoff_export.backend_lane_target.target_repo,
        backend_dispatch_packet_selected_ids:$backend_dispatch_packet.selected_packet_ids,
        backend_dispatch_packet_target_repo:$backend_dispatch_packet.backend_lane_target.target_repo,
        backend_receipt_intake_selected_ids:$backend_receipt_intake.selected_receipt_ids,
        backend_receipt_intake_waiting_for_receipt:$backend_receipt_intake.waiting_for_backend_receipt,
        backend_receipt_present:$backend_receipt_intake.backend_receipt_present,
        backend_receipt_valid:$backend_receipt_intake.backend_receipt_valid,
        backend_receipt_roundtrip_selected_ids:$backend_receipt_roundtrip.selected_roundtrip_ids,
        backend_receipt_roundtrip_present_branch_ready:$backend_receipt_roundtrip.source_alignment.backend_receipt_present_branch_ready,
        backend_receipt_roundtrip_simulated_receipt_ready:$backend_receipt_roundtrip.source_alignment.simulated_receipt_ready,
        backend_receipt_refresh_lock_selected_ids:$backend_receipt_refresh_lock.selected_refresh_ids,
        backend_receipt_refresh_lock_real_receipt_present:$backend_receipt_refresh_lock.receipt_state.real_backend_receipt_present,
        backend_receipt_refresh_lock_simulated_input_present:$backend_receipt_refresh_lock.receipt_state.simulated_receipt_input_present,
        backend_receipt_refresh_lock_hard_true_window_ready:$backend_receipt_refresh_lock.refresh_requirements.hard_true_window_refresh_ready,
        refreshed_future_plan_ids:($future_plan_refresh.future_plan | map(.id)),
        refreshed_future_plan_minimum_gate_id:$future_plan_refresh.future_plan[0].id,
        refreshed_future_plan:$future_plan_refresh.future_plan,
        operator_briefing_refresh_plan_ids:$operator_briefing_refresh.current_next_plan_ids,
        operator_briefing_refresh_updated_critical_risk_count:$operator_briefing_refresh.updated_critical_risk_count,
        operator_briefing_refresh_root_report_required_count:$operator_briefing_refresh.current_state.root_report_replay_required_count_after_refresh,
        release_approval_intake_waiting_for_approval:$release_approval_intake.release_approval_state.waiting_for_release_approval,
        release_approval_intake_root_report_required_count:$release_approval_intake.release_approval_state.root_report_replay_required_count_after_intake,
        release_approval_intake_next_required_artifact_gate:"signed_notarized_stapled_artifact_gate",
        top_design_referee_refresh_ready:$top_design_referee_refresh.top_design_referee_refresh_gate_ready,
        top_design_referee_refresh_version:$top_design_referee_refresh.refresh_version,
	        top_design_harsh_2026_referee_ready:$top_design_referee_refresh.top_design_harsh_2026_referee_ready,
	        top_design_control_ui_harsh_2026_ready:$top_design_referee_refresh.control_ui_harsh_2026_ready,
	        top_design_control_ui_microcopy_word_split_guard_ready:$top_design_referee_refresh.referee_matrix.control_ui.microcopy_word_split_guard_ready,
	        top_design_control_ui_logo_clip_guard_ready:$top_design_referee_refresh.referee_matrix.control_ui.logo_clip_guard_ready,
	        top_design_control_ui_active_chat_readability_ready:$top_design_referee_refresh.referee_matrix.control_ui.active_chat_readability_ready,
	        top_design_control_ui_placeholder_readability_ready:$top_design_referee_refresh.referee_matrix.control_ui.placeholder_readability_ready,
	        top_design_control_ui_small_control_readability_ready:$top_design_referee_refresh.referee_matrix.control_ui.small_control_readability_ready,
	        top_design_control_ui_rail_action_icon_ready:$top_design_referee_refresh.referee_matrix.control_ui.rail_action_icon_ready,
	        top_design_control_ui_folder_chip_touch_ready:$top_design_referee_refresh.referee_matrix.control_ui.folder_chip_touch_ready,
	        top_design_control_ui_row_menu_touch_ready:$top_design_referee_refresh.referee_matrix.control_ui.row_menu_touch_ready,
	        top_design_control_ui_row_menu_all_rows_ready:$top_design_referee_refresh.referee_matrix.control_ui.row_menu_all_rows_ready,
	        top_design_control_ui_row_menu_light_glass_ready:$top_design_referee_refresh.referee_matrix.control_ui.row_menu_light_glass_ready,
	        top_design_control_ui_command_palette_ready:$top_design_referee_refresh.referee_matrix.control_ui.command_palette_ready,
	        top_design_control_ui_command_palette_surface_light_glass_ready:$top_design_referee_refresh.referee_matrix.control_ui.command_palette_surface_light_glass_ready,
	        top_design_control_ui_command_palette_trigger_light_glass_ready:$top_design_referee_refresh.referee_matrix.control_ui.command_palette_trigger_light_glass_ready,
	        top_design_control_ui_command_palette_close_light_glass_ready:$top_design_referee_refresh.referee_matrix.control_ui.command_palette_close_light_glass_ready,
	        top_design_control_ui_command_palette_input_light_glass_ready:$top_design_referee_refresh.referee_matrix.control_ui.command_palette_input_light_glass_ready,
	        top_design_control_ui_command_palette_item_light_glass_ready:$top_design_referee_refresh.referee_matrix.control_ui.command_palette_item_light_glass_ready,
	        top_design_control_ui_form_control_title_touch_ready:$top_design_referee_refresh.referee_matrix.control_ui.form_control_title_touch_ready,
	        top_design_control_ui_chat_row_option_semantic_touch_ready:$top_design_referee_refresh.referee_matrix.control_ui.chat_row_option_semantic_touch_ready,
	        top_design_control_ui_thread_tools_menu_ready:$top_design_referee_refresh.referee_matrix.control_ui.thread_tools_menu_ready,
	        top_design_control_ui_composer_tools_menu_ready:$top_design_referee_refresh.referee_matrix.control_ui.composer_tools_menu_ready,
	        top_design_control_ui_composer_popover_ready:$top_design_referee_refresh.referee_matrix.control_ui.composer_popover_ready,
	        top_design_control_ui_composer_popover_search_light_glass_ready:$top_design_referee_refresh.referee_matrix.control_ui.composer_popover_search_light_glass_ready,
	        top_design_control_ui_rail_search_light_glass_ready:$top_design_referee_refresh.referee_matrix.control_ui.rail_search_light_glass_ready,
	        top_design_control_ui_micro_surface_light_glass_ready:$top_design_referee_refresh.referee_matrix.control_ui.micro_surface_light_glass_ready,
	        top_design_control_ui_message_routing_badge_light_glass_ready:$top_design_referee_refresh.referee_matrix.control_ui.message_routing_badge_light_glass_ready,
	        top_design_control_ui_thread_intro_badge_light_glass_ready:$top_design_referee_refresh.referee_matrix.control_ui.thread_intro_badge_light_glass_ready,
	        top_design_control_ui_status_trust_strip_light_glass_ready:$top_design_referee_refresh.referee_matrix.control_ui.status_trust_strip_light_glass_ready,
	        top_design_control_ui_visible_text_integrity_ready:$top_design_referee_refresh.referee_matrix.control_ui.visible_text_integrity_ready,
	        top_design_control_ui_icon_button_title_match_ready:$top_design_referee_refresh.referee_matrix.control_ui.icon_button_title_match_ready,
	        top_design_control_ui_menu_trigger_title_match_ready:$top_design_referee_refresh.referee_matrix.control_ui.menu_trigger_title_match_ready,
	        top_design_native_secondary_harsh_action_matrix_ready:$top_design_referee_refresh.native_secondary_harsh_action_matrix_ready,
	        top_design_native_secondary_title_tooltip_ready:$top_design_referee_refresh.referee_matrix.control_level.secondary_surface_title_tooltip_ready,
	        top_design_native_secondary_title_tooltip_failure_count:$top_design_referee_refresh.referee_matrix.control_level.secondary_surface_title_tooltip_failure_count,
        top_design_tempered_glass_2026_ready:$top_design_referee_refresh.referee_matrix.tempered_glass_2026.ready,
        top_design_tempered_glass_min_contrast_ratio:$top_design_referee_refresh.referee_matrix.tempered_glass_2026.min_contrast_ratio,
        top_design_tempered_glass_clipping_failure_count:$top_design_referee_refresh.referee_matrix.tempered_glass_2026.clipping_failure_count,
        top_design_referee_refresh_action_matrix_ready:$top_design_referee_refresh.referee_matrix.control_level.secondary_surface_action_matrix_ready,
        top_design_referee_refresh_action_matrix_case_count:$top_design_referee_refresh.referee_matrix.control_level.secondary_surface_action_matrix_case_count,
        top_design_referee_refresh_harsh_action_matrix_ready:$top_design_referee_refresh.referee_matrix.control_level.secondary_surface_harsh_action_matrix_ready,
        top_design_referee_refresh_harsh_action_failure_count:$top_design_referee_refresh.referee_matrix.control_level.secondary_surface_harsh_action_failure_count,
        top_design_referee_refresh_control_phone320_ready:$top_design_referee_refresh.referee_matrix.control_ui.persisted_phone320_screenshot_ready,
        top_design_referee_refresh_current_roundtrip_plan_ids:$top_design_referee_refresh.source_alignment.current_roundtrip_plan_ids,
        top_design_referee_refresh_current_minimum_gate_id:$top_design_referee_refresh.current_referee_alignment.current_minimum_gate_id,
        top_design_referee_refresh_root_report_required_count:$top_design_referee_refresh.current_state.root_report_replay_required_count_after_top_design_refresh,
        top_design_referee_refresh_downstream_root_report_required_count:$top_design_referee_refresh.current_state.downstream_root_report_replay_required_count_after_release_artifact_roundtrip,
        release_artifact_boundary_ready:$release_artifact_boundary.release_artifact_boundary_gate_ready,
        release_artifact_boundary_root_report_required_count:$release_artifact_boundary.release_artifact_boundary.root_report_replay_required_count_after_boundary,
        release_artifact_boundary_next_required_artifact_gate:$release_artifact_boundary.release_artifact_boundary.next_required_artifact_gate,
        release_artifact_boundary_signed_notarized_stapled_artifact_present:$release_artifact_boundary.release_artifact_boundary.signed_notarized_stapled_artifact_present,
        release_artifact_boundary_public_distribution_artifact_written:$release_artifact_boundary.release_artifact_boundary.public_distribution_artifact_written,
        release_artifact_intake_ready:$release_artifact_intake.release_artifact_intake_gate_ready,
        release_artifact_intake_root_report_required_count:$release_artifact_intake.root_report_replay_required_count_after_intake,
        release_artifact_intake_waiting_for_artifact:$release_artifact_intake.release_artifact_state.waiting_for_release_artifact,
        release_artifact_intake_release_artifact_valid:$release_artifact_intake.release_artifact_state.release_artifact_valid,
        release_artifact_intake_present_artifact_branch_supported:$release_artifact_intake.source_alignment.present_artifact_branch_supported,
        release_artifact_intake_independent_approval_verifier_contract_ready:$release_artifact_intake.source_alignment.independent_approval_verifier_contract_ready,
        release_artifact_roundtrip_ready:$release_artifact_roundtrip.release_artifact_roundtrip_gate_ready,
        release_artifact_roundtrip_root_report_required_count:$release_artifact_roundtrip.source_alignment.root_report_replay_required_count_after_roundtrip,
        release_artifact_roundtrip_present_branch_ready:$release_artifact_roundtrip.source_alignment.present_branch_ready,
        release_artifact_roundtrip_present_artifact_valid:$release_artifact_roundtrip.source_alignment.present_branch_release_artifact_valid,
        release_artifact_roundtrip_present_artifact_branch_supported:$release_artifact_roundtrip.source_alignment.present_artifact_branch_supported,
        release_artifact_roundtrip_independent_approval_verifier_contract_ready:$release_artifact_roundtrip.source_alignment.independent_approval_verifier_contract_ready,
        release_artifact_roundtrip_legacy_simulated_rejected:$release_artifact_roundtrip.source_alignment.legacy_simulated_artifact_rejected,
        release_artifact_roundtrip_v3_valid_branch_selftest_ready:$release_artifact_roundtrip.source_alignment.v3_valid_branch_selftest_ready,
        current_plan_refresh_ids:$current_plan_refresh.current_plan_ids,
        current_plan_refresh_minimum_gate_id:$current_plan_refresh.current_minimum_gate.gate_id,
        current_plan_refresh_root_report_required_count:$current_plan_refresh.current_minimum_gate.root_report_replay_required_count_after_current_plan_refresh,
        current_plan_refresh_legacy_plan_ids:$current_plan_refresh.legacy_plan_snapshot.legacy_plan_ids,
        current_plan_refresh_next_required_artifact_gate:$current_plan_refresh.current_plan[3].next_required_artifact_gate,
        blocker_closure_ready:$blocker_closure.blocker_closure_gate_ready,
        blocker_closure_root_report_required_count:$blocker_closure.closure_state.root_report_replay_required_count_after_blocker_closure,
        blocker_closure_critical_blocker_count:$blocker_closure.critical_blocker_count,
        blocker_closure_next_unblock_sequence:$blocker_closure.next_unblock_sequence,
        backend_delivery_audit_ready:$backend_delivery_audit.backend_delivery_audit_gate_ready,
        backend_delivery_audit_root_report_required_count:$backend_delivery_audit.delivery_state.root_report_replay_required_count_after_delivery_audit,
        backend_delivery_audit_waiting_for_delivery_receipt:$backend_delivery_audit.delivery_state.waiting_for_delivery_receipt,
        backend_delivery_audit_next_unblock_sequence:$backend_delivery_audit.next_unblock_sequence,
        backend_delivery_receipt_roundtrip_ready:$backend_delivery_receipt_roundtrip.backend_delivery_receipt_roundtrip_gate_ready,
        backend_delivery_receipt_roundtrip_present_branch_ready:$backend_delivery_receipt_roundtrip.source_alignment.present_branch_ready,
        backend_delivery_receipt_roundtrip_present_branch_valid:$backend_delivery_receipt_roundtrip.source_alignment.present_branch_delivery_receipt_valid,
        backend_delivery_receipt_roundtrip_root_report_required_count:$backend_delivery_receipt_roundtrip.source_alignment.root_report_replay_required_count_after_roundtrip,
        risk_future_plan_ready:$risk_future_plan.risk_future_plan_gate_ready,
        risk_future_plan_latest_minimum_gate_id:$risk_future_plan.latest_minimum_gate.gate_id,
        risk_future_plan_latest_plan_ids:$risk_future_plan.latest_plan_ids,
        risk_future_plan_top_design_refresh_version:$risk_future_plan.latest_minimum_gate.top_design_refresh_version,
	        risk_future_plan_top_design_harsh_2026_referee_ready:$risk_future_plan.latest_minimum_gate.top_design_harsh_2026_referee_ready,
	        risk_future_plan_control_ui_harsh_2026_ready:$risk_future_plan.latest_minimum_gate.control_ui_harsh_2026_ready,
	        risk_future_plan_control_ui_microcopy_word_split_guard_ready:$risk_future_plan.latest_minimum_gate.control_ui_microcopy_word_split_guard_ready,
	        risk_future_plan_control_ui_logo_clip_guard_ready:$risk_future_plan.latest_minimum_gate.control_ui_logo_clip_guard_ready,
	        risk_future_plan_control_ui_active_chat_readability_ready:$risk_future_plan.latest_minimum_gate.control_ui_active_chat_readability_ready,
	        risk_future_plan_control_ui_placeholder_readability_ready:$risk_future_plan.latest_minimum_gate.control_ui_placeholder_readability_ready,
	        risk_future_plan_control_ui_small_control_readability_ready:$risk_future_plan.latest_minimum_gate.control_ui_small_control_readability_ready,
	        risk_future_plan_control_ui_rail_action_icon_ready:$risk_future_plan.latest_minimum_gate.control_ui_rail_action_icon_ready,
	        risk_future_plan_control_ui_folder_chip_touch_ready:$risk_future_plan.latest_minimum_gate.control_ui_folder_chip_touch_ready,
	        risk_future_plan_control_ui_row_menu_touch_ready:$risk_future_plan.latest_minimum_gate.control_ui_row_menu_touch_ready,
	        risk_future_plan_control_ui_row_menu_all_rows_ready:$risk_future_plan.latest_minimum_gate.control_ui_row_menu_all_rows_ready,
	        risk_future_plan_control_ui_row_menu_light_glass_ready:$risk_future_plan.latest_minimum_gate.control_ui_row_menu_light_glass_ready,
	        risk_future_plan_control_ui_command_palette_ready:$risk_future_plan.latest_minimum_gate.control_ui_command_palette_ready,
	        risk_future_plan_control_ui_command_palette_surface_light_glass_ready:$risk_future_plan.latest_minimum_gate.control_ui_command_palette_surface_light_glass_ready,
	        risk_future_plan_control_ui_command_palette_trigger_light_glass_ready:$risk_future_plan.latest_minimum_gate.control_ui_command_palette_trigger_light_glass_ready,
	        risk_future_plan_control_ui_command_palette_close_light_glass_ready:$risk_future_plan.latest_minimum_gate.control_ui_command_palette_close_light_glass_ready,
	        risk_future_plan_control_ui_command_palette_input_light_glass_ready:$risk_future_plan.latest_minimum_gate.control_ui_command_palette_input_light_glass_ready,
	        risk_future_plan_control_ui_command_palette_item_light_glass_ready:$risk_future_plan.latest_minimum_gate.control_ui_command_palette_item_light_glass_ready,
	        risk_future_plan_control_ui_form_control_title_touch_ready:$risk_future_plan.latest_minimum_gate.control_ui_form_control_title_touch_ready,
	        risk_future_plan_control_ui_chat_row_option_semantic_touch_ready:$risk_future_plan.latest_minimum_gate.control_ui_chat_row_option_semantic_touch_ready,
	        risk_future_plan_control_ui_thread_tools_menu_ready:$risk_future_plan.latest_minimum_gate.control_ui_thread_tools_menu_ready,
	        risk_future_plan_control_ui_composer_tools_menu_ready:$risk_future_plan.latest_minimum_gate.control_ui_composer_tools_menu_ready,
	        risk_future_plan_control_ui_composer_popover_ready:$risk_future_plan.latest_minimum_gate.control_ui_composer_popover_ready,
	        risk_future_plan_control_ui_composer_popover_search_light_glass_ready:$risk_future_plan.latest_minimum_gate.control_ui_composer_popover_search_light_glass_ready,
	        risk_future_plan_control_ui_rail_search_light_glass_ready:$risk_future_plan.latest_minimum_gate.control_ui_rail_search_light_glass_ready,
	        risk_future_plan_control_ui_micro_surface_light_glass_ready:$risk_future_plan.latest_minimum_gate.control_ui_micro_surface_light_glass_ready,
	        risk_future_plan_control_ui_message_routing_badge_light_glass_ready:$risk_future_plan.latest_minimum_gate.control_ui_message_routing_badge_light_glass_ready,
	        risk_future_plan_control_ui_thread_intro_badge_light_glass_ready:$risk_future_plan.latest_minimum_gate.control_ui_thread_intro_badge_light_glass_ready,
	        risk_future_plan_control_ui_status_trust_strip_light_glass_ready:$risk_future_plan.latest_minimum_gate.control_ui_status_trust_strip_light_glass_ready,
	        risk_future_plan_control_ui_visible_text_integrity_ready:$risk_future_plan.latest_minimum_gate.control_ui_visible_text_integrity_ready,
	        risk_future_plan_control_ui_icon_button_title_match_ready:$risk_future_plan.latest_minimum_gate.control_ui_icon_button_title_match_ready,
	        risk_future_plan_control_ui_menu_trigger_title_match_ready:$risk_future_plan.latest_minimum_gate.control_ui_menu_trigger_title_match_ready,
	        risk_future_plan_native_secondary_harsh_action_matrix_ready:$risk_future_plan.latest_minimum_gate.native_secondary_harsh_action_matrix_ready,
	        risk_future_plan_native_secondary_title_tooltip_ready:$risk_future_plan.latest_minimum_gate.native_secondary_title_tooltip_ready,
	        risk_future_plan_native_secondary_title_tooltip_failure_count:$risk_future_plan.latest_minimum_gate.native_secondary_title_tooltip_failure_count,
        risk_future_plan_tempered_glass_2026_ready:$risk_future_plan.latest_minimum_gate.tempered_glass_2026_ready,
        risk_future_plan_tempered_glass_min_contrast_ratio:$risk_future_plan.latest_minimum_gate.tempered_glass_min_contrast_ratio,
        risk_future_plan_tempered_glass_clipping_failure_count:$risk_future_plan.latest_minimum_gate.tempered_glass_clipping_failure_count,
        risk_future_plan_action_matrix_ready:$risk_future_plan.latest_minimum_gate.secondary_surface_action_matrix_ready,
        risk_future_plan_action_matrix_case_count:$risk_future_plan.latest_minimum_gate.secondary_surface_action_matrix_case_count,
        risk_future_plan_harsh_action_matrix_ready:$risk_future_plan.latest_minimum_gate.secondary_surface_harsh_action_matrix_ready,
        risk_future_plan_harsh_action_failure_count:$risk_future_plan.latest_minimum_gate.secondary_surface_harsh_action_failure_count,
        risk_future_plan_root_report_required_count:$risk_future_plan.latest_minimum_gate.root_report_replay_required_count_after_risk_future_plan,
        risk_future_plan_critical_blocker_count:$risk_future_plan.critical_blocker_count,
        risk_future_plan_next_unblock_sequence:$risk_future_plan.next_unblock_sequence,
        critical_path_plan_future_plan:$critical_path_plan.future_plan,
        release_blockers:$current_plan_refresh.current_plan[3].blockers
      },
      side_effects:{
        filesystem_read:true,
        local_report_written:true,
        matrix_login:false,
        gateway_call:false,
        provider_invoked:false,
        channel_delivery:false,
        external_mutation:false
      }
    }' >"$REPORT_TMP"

jq -e '
  .status == "ready"
  and .root_report_replay_gate_ready == true
  and (
    .strict_current_source_mode != true
    or (
      .current_source_true_window_matrix_ready == true
      and (.source_alignment.ui_design_system_rust_toolchain | test("^rustc 1\\.95\\.0([[:space:]]|$)"))
      and ([.source_reports[] | select(.kind == "strict_current_source_true_window")] | length) == 5
      and .current_source_true_window_matrix.main.enabled == true
      and .current_source_true_window_matrix.main.status == "ready"
      and .current_source_true_window_matrix.main.blocked_allowed != true
      and .current_source_true_window_matrix.main.true_window_capture_performed == true
      and .current_source_true_window_matrix.main.screenshot_count == 2
      and .current_source_true_window_matrix.main.app_log_error_free == true
      and .current_source_true_window_matrix.main.evidence_ready == true
      and .current_source_true_window_matrix.desktop_routes.enabled == true
      and .current_source_true_window_matrix.desktop_routes.status == "ready"
      and .current_source_true_window_matrix.desktop_routes.blocked_allowed != true
      and .current_source_true_window_matrix.desktop_routes.true_window_capture_performed == true
      and .current_source_true_window_matrix.desktop_routes.screenshot_count == 4
      and .current_source_true_window_matrix.desktop_routes.unique_screenshot_count == 4
      and .current_source_true_window_matrix.desktop_routes.content_probe_ready == true
      and .current_source_true_window_matrix.desktop_routes.app_log_error_free == true
      and .current_source_true_window_matrix.desktop_routes.evidence_ready == true
      and .current_source_true_window_matrix.mobile_routes.enabled == true
      and .current_source_true_window_matrix.mobile_routes.status == "ready"
      and .current_source_true_window_matrix.mobile_routes.blocked_allowed != true
      and .current_source_true_window_matrix.mobile_routes.true_window_capture_performed == true
      and .current_source_true_window_matrix.mobile_routes.screenshot_count == 4
      and .current_source_true_window_matrix.mobile_routes.unique_screenshot_count == 4
      and .current_source_true_window_matrix.mobile_routes.content_log_signature_count >= 3
      and .current_source_true_window_matrix.mobile_routes.host_window_ready == true
      and .current_source_true_window_matrix.mobile_routes.app_log_error_free == true
      and .current_source_true_window_matrix.mobile_routes.evidence_ready == true
      and .current_source_true_window_matrix.desktop_secondary.enabled == true
      and .current_source_true_window_matrix.desktop_secondary.status == "ready"
      and .current_source_true_window_matrix.desktop_secondary.blocked_allowed != true
      and .current_source_true_window_matrix.desktop_secondary.true_window_capture_performed == true
      and .current_source_true_window_matrix.desktop_secondary.screenshot_count == 5
      and .current_source_true_window_matrix.desktop_secondary.unique_screenshot_count == 5
      and .current_source_true_window_matrix.desktop_secondary.app_log_error_free == true
      and .current_source_true_window_matrix.desktop_secondary.evidence_ready == true
      and .current_source_true_window_matrix.mobile_secondary.enabled == true
      and .current_source_true_window_matrix.mobile_secondary.status == "ready"
      and .current_source_true_window_matrix.mobile_secondary.blocked_allowed != true
      and .current_source_true_window_matrix.mobile_secondary.true_window_capture_performed == true
      and .current_source_true_window_matrix.mobile_secondary.screenshot_count == 5
      and .current_source_true_window_matrix.mobile_secondary.unique_screenshot_count == 5
      and .current_source_true_window_matrix.mobile_secondary.content_probe_ready == true
      and .current_source_true_window_matrix.mobile_secondary.content_visible_count >= 5
      and .current_source_true_window_matrix.mobile_secondary.host_window_ready == true
      and .current_source_true_window_matrix.mobile_secondary.app_log_error_free == true
      and .current_source_true_window_matrix.mobile_secondary.evidence_ready == true
    )
  )
  and .claim_boundary.local_root_report_replay_ready == true
  and .root_report_count == 45
  and .root_json_report_count == 45
  and .source_alignment.control_ui_real_click_v7_ready == true
  and .source_alignment.control_ui_real_click_v7_viewport_count == 4
  and .source_alignment.control_ui_real_click_v7_target_count == 26
  and .source_alignment.control_ui_real_click_v7_failure_count == 0
  and .source_alignment.control_ui_real_click_v7_mobile_routes_ready == true
  and .source_alignment.control_ui_real_click_v7_popover_switch_sequence_ready == true
  and .source_alignment.control_ui_real_click_v7_popover_switch_step_count == 26
  and .root_report_sha256_ready == true
  and .source_alignment.static_contract_ready == true
  and .source_alignment.static_marker_count >= 3642
  and .source_alignment.ui_design_system_ready == true
  and .source_alignment.ui_design_system_css_layer_count == 6
  and .source_alignment.ui_design_system_runtime_css_bytes < 300000
  and .source_alignment.ui_design_system_important_count <= .source_alignment.ui_design_system_important_budget
  and .source_alignment.ui_design_system_important_budget == 2100
  and .source_alignment.ui_design_system_static_light_theme_ready == true
  and .source_alignment.ui_design_system_renderer_light_theme_ready == true
  and .source_alignment.ui_design_system_document_direction_source_ready == true
  and .source_alignment.ui_design_system_legacy_texture_asset_reference_count == 0
  and .source_alignment.ui_design_system_retired_texture_asset_free == true
  and .source_alignment.ui_design_system_native_fixture_tokens_ready == true
  and (
    .source_alignment.native_window_route_mobile_status != "ready"
    or (
      .source_alignment.native_window_route_mobile_ready == true
      and .source_alignment.native_window_route_mobile_screenshot_count == 4
      and .source_alignment.native_window_route_mobile_host_window_ready == true
    )
  )
  and .source_alignment.screenshot_manifest_ready == true
  and .source_alignment.control_ui_ready == true
  and .source_alignment.native_fixture_ready == true
  and .source_alignment.native_packaging_ready == true
  and .source_alignment.native_distribution_preflight_ready == true
  and .source_alignment.base_gap_alignment_ready == true
  and .source_alignment.base_gap_count == 12
  and .source_alignment.backend_contract_waves_ready == true
  and .source_alignment.backend_contract_gap_count == 12
  and .source_alignment.non_base_edge_gates_ready == true
  and .source_alignment.non_base_edge_count == 4
  and .source_alignment.productization_blocker_rollup_ready == true
  and .source_alignment.plan_boundary_ready == true
  and .source_alignment.demo_evidence_ready == true
  and .source_alignment.evidence_bundle_ready == true
  and .source_alignment.evidence_archive_ready == true
  and .source_alignment.release_operator_dry_run_ready == true
  and .source_alignment.operator_briefing_ready == true
  and .source_alignment.operator_briefing_critical_risk_count == 3
  and .source_alignment.backend_promotion_packet_ready == true
  and .source_alignment.backend_promotion_packet_priority_count == 5
  and .source_alignment.backend_promotion_packet_selected_ids == ["message_search","file_upload_send","media_download_playback","notifications","room_settings"]
  and .source_alignment.backend_alignment_evidence_ready == true
  and .source_alignment.backend_alignment_evidence_item_count == 5
  and .source_alignment.backend_alignment_evidence_selected_ids == ["message_search","file_upload_send","media_download_playback","notifications","room_settings"]
  and .source_alignment.critical_path_plan_ready == true
  and .source_alignment.critical_path_plan_blocker_count == 3
  and .source_alignment.critical_path_plan_selected_ids == ["message_search","file_upload_send","media_download_playback","notifications","room_settings"]
  and .source_alignment.backend_contract_acceptance_ready == true
  and .source_alignment.backend_contract_acceptance_item_count == 5
  and .source_alignment.backend_contract_acceptance_selected_ids == ["message_search","file_upload_send","media_download_playback","notifications","room_settings"]
  and .source_alignment.backend_handoff_export_ready == true
  and .source_alignment.backend_handoff_export_item_count == 5
  and .source_alignment.backend_handoff_export_selected_ids == ["message_search","file_upload_send","media_download_playback","notifications","room_settings"]
  and .source_alignment.backend_dispatch_packet_ready == true
  and .source_alignment.backend_dispatch_packet_item_count == 5
  and .source_alignment.backend_dispatch_packet_selected_ids == ["message_search","file_upload_send","media_download_playback","notifications","room_settings"]
  and (.source_alignment.backend_dispatch_packet_archive_sha256 | test("^[0-9a-f]{64}$"))
  and .source_alignment.backend_dispatch_packet_archive_bytes > 0
  and .source_alignment.backend_receipt_intake_ready == true
  and .source_alignment.backend_receipt_intake_selected_ids == ["message_search","file_upload_send","media_download_playback","notifications","room_settings"]
  and (.source_alignment.backend_receipt_intake_template_sha256 | test("^[0-9a-f]{64}$"))
  and .source_alignment.backend_receipt_intake_template_bytes > 0
  and (
    (
      .source_alignment.backend_receipt_intake_waiting_for_receipt == true
      and .source_alignment.backend_receipt_present == false
      and .source_alignment.backend_receipt_valid == false
    )
    or (
      .source_alignment.backend_receipt_intake_waiting_for_receipt == false
      and .source_alignment.backend_receipt_present == true
      and .source_alignment.backend_receipt_valid == true
    )
  )
  and .source_alignment.backend_receipt_roundtrip_ready == true
  and .source_alignment.backend_receipt_roundtrip_selected_ids == ["message_search","file_upload_send","media_download_playback","notifications","room_settings"]
  and .source_alignment.backend_receipt_roundtrip_ready_count == 5
  and .source_alignment.backend_receipt_roundtrip_waiting_branch_ready == true
  and .source_alignment.backend_receipt_roundtrip_present_branch_ready == true
  and .source_alignment.backend_receipt_roundtrip_simulated_receipt_ready == true
  and .source_alignment.backend_receipt_refresh_lock_ready == true
  and .source_alignment.backend_receipt_refresh_lock_selected_ids == ["message_search","file_upload_send","media_download_playback","notifications","room_settings"]
  and .source_alignment.backend_receipt_refresh_lock_real_receipt_present == .future_plan_replay.backend_receipt_refresh_lock_real_receipt_present
  and .source_alignment.backend_receipt_refresh_lock_simulated_input_present == .future_plan_replay.backend_receipt_refresh_lock_simulated_input_present
  and .source_alignment.future_plan_refresh_ready == true
  and .source_alignment.future_plan_refresh_minimum_gate_id == "r52_minimum_ui_demo_gate"
  and .source_alignment.future_plan_refresh_required_root_report_count == 32
  and .source_alignment.future_plan_refresh_ids == ["r52_minimum_ui_demo_gate","backend_real_receipt_return","ui_refresh_after_real_receipt"]
  and .source_alignment.operator_briefing_refresh_ready == true
  and (
    .source_alignment.operator_briefing_refresh_critical_risk_count >= 1
    and .source_alignment.operator_briefing_refresh_critical_risk_count <= 4
  )
  and .source_alignment.operator_briefing_refresh_current_plan_ids == ["r52_minimum_ui_demo_gate","backend_real_receipt_return","ui_refresh_after_real_receipt"]
  and .source_alignment.operator_briefing_refresh_root_report_required_count == 33
  and .source_alignment.operator_briefing_refresh_dispatch_archive_sha256 == .source_alignment.backend_dispatch_packet_archive_sha256
  and (.source_alignment.operator_briefing_refresh_markdown_sha256 | test("^[0-9a-f]{64}$"))
  and .source_alignment.release_approval_intake_ready == true
  and .source_alignment.release_approval_intake_waiting_for_approval == true
  and .source_alignment.release_approval_present == false
  and .source_alignment.release_approval_valid == false
  and .source_alignment.independent_approval_verifier_ready == false
  and .source_alignment.self_reported_approval_can_authorize_release == false
  and .source_alignment.approval_valid_branch_supported == false
  and .claim_boundary.release_approval_claim_ready == false
  and (.source_alignment.release_approval_intake_template_sha256 | test("^[0-9a-f]{64}$"))
  and .source_alignment.release_approval_intake_template_bytes > 0
  and .source_alignment.release_approval_intake_root_report_required_count == 34
  and .source_alignment.top_design_referee_refresh_ready == true
	  and .source_alignment.top_design_referee_refresh_version == 46
		  and .source_alignment.top_design_harsh_2026_referee_ready == true
		  and .source_alignment.top_design_control_ui_harsh_2026_ready == true
	  and .source_alignment.top_design_control_ui_microcopy_word_split_guard_ready == true
	  and .source_alignment.top_design_control_ui_logo_clip_guard_ready == true
	  and .source_alignment.top_design_control_ui_active_chat_readability_ready == true
	  and .source_alignment.top_design_control_ui_placeholder_readability_ready == true
	  and .source_alignment.top_design_control_ui_small_control_readability_ready == true
	  and .source_alignment.top_design_control_ui_rail_action_icon_ready == true
	  and .source_alignment.top_design_control_ui_folder_chip_touch_ready == true
	  and .source_alignment.top_design_control_ui_row_menu_touch_ready == true
	  and .source_alignment.top_design_control_ui_row_menu_all_rows_ready == true
	  and .source_alignment.top_design_control_ui_row_menu_light_glass_ready == true
	  and .source_alignment.top_design_control_ui_command_palette_ready == true
	  and .source_alignment.top_design_control_ui_command_palette_surface_light_glass_ready == true
	  and .source_alignment.top_design_control_ui_command_palette_trigger_light_glass_ready == true
	  and .source_alignment.top_design_control_ui_command_palette_close_light_glass_ready == true
	  and .source_alignment.top_design_control_ui_command_palette_input_light_glass_ready == true
	  and .source_alignment.top_design_control_ui_command_palette_item_light_glass_ready == true
	  and .source_alignment.top_design_control_ui_form_control_title_touch_ready == true
	  and .source_alignment.top_design_control_ui_chat_row_option_semantic_touch_ready == true
	  and .source_alignment.top_design_control_ui_thread_tools_menu_ready == true
	  and .source_alignment.top_design_control_ui_composer_tools_menu_ready == true
	  and .source_alignment.top_design_control_ui_composer_popover_ready == true
	  and .source_alignment.top_design_control_ui_composer_popover_search_light_glass_ready == true
	  and .source_alignment.top_design_control_ui_rail_search_light_glass_ready == true
	  and .source_alignment.top_design_control_ui_micro_surface_light_glass_ready == true
	  and .source_alignment.top_design_control_ui_message_routing_badge_light_glass_ready == true
	    and .source_alignment.top_design_control_ui_thread_intro_badge_light_glass_ready == true
	  and .source_alignment.top_design_control_ui_status_trust_strip_light_glass_ready == true
	  and .source_alignment.top_design_control_ui_visible_text_integrity_ready == true
	  and .source_alignment.top_design_control_ui_icon_button_title_match_ready == true
	  and .source_alignment.top_design_control_ui_menu_trigger_title_match_ready == true
	  and .source_alignment.top_design_native_secondary_harsh_action_matrix_ready == true
	  and .source_alignment.top_design_native_secondary_title_tooltip_ready == true
	  and .source_alignment.top_design_native_secondary_title_tooltip_failure_count == 0
  and .source_alignment.top_design_tempered_glass_2026_ready == true
  and .source_alignment.top_design_tempered_glass_min_contrast_ratio >= 4.5
  and .source_alignment.top_design_tempered_glass_clipping_failure_count == 0
  and .source_alignment.top_design_referee_refresh_action_matrix_ready == true
  and .source_alignment.top_design_referee_refresh_action_matrix_case_count == 15
  and .source_alignment.top_design_referee_refresh_harsh_action_matrix_ready == true
  and .source_alignment.top_design_referee_refresh_harsh_action_failure_count == 0
  and .source_alignment.top_design_referee_refresh_control_phone320_ready == true
  and .source_alignment.top_design_referee_refresh_current_plan_ids == ["r52_minimum_ui_demo_gate","backend_real_receipt_return","ui_refresh_after_real_receipt"]
  and .source_alignment.top_design_referee_refresh_current_roundtrip_plan_ids == ["r62_minimum_ui_demo_gate","backend_real_receipt_return","ui_refresh_after_real_receipt","release_artifact_roundtrip_and_signed_artifact_gate"]
  and .source_alignment.top_design_referee_refresh_current_minimum_gate_id == "r62_minimum_ui_demo_gate"
  and .source_alignment.top_design_referee_refresh_root_report_required_count == 35
  and .source_alignment.top_design_referee_refresh_downstream_root_report_required_count == 41
  and (.source_alignment.top_design_referee_refresh_markdown_sha256 | test("^[0-9a-f]{64}$"))
  and .source_alignment.release_artifact_boundary_ready == true
  and .source_alignment.release_artifact_boundary_root_report_required_count == 36
  and .source_alignment.release_artifact_boundary_unsigned_app_codesign_status == "unsigned_expected"
  and .source_alignment.release_artifact_boundary_next_required_artifact_gate == "signed_notarized_stapled_artifact_gate"
  and .source_alignment.release_artifact_boundary_signed_notarized_stapled_artifact_present == false
  and .source_alignment.release_artifact_boundary_public_distribution_artifact_written == false
  and (.source_alignment.release_artifact_boundary_markdown_sha256 | test("^[0-9a-f]{64}$"))
  and .source_alignment.release_artifact_intake_ready == true
  and .source_alignment.release_artifact_intake_root_report_required_count == 37
  and .source_alignment.release_artifact_intake_waiting_for_artifact == true
  and .source_alignment.release_artifact_present == false
  and .source_alignment.release_artifact_valid == false
  and .source_alignment.release_artifact_intake_present_artifact_branch_supported == false
  and .source_alignment.release_artifact_intake_independent_approval_verifier_contract_ready == false
  and .source_alignment.release_artifact_intake_signed_notarized_stapled_artifact_present == false
  and .source_alignment.release_artifact_intake_public_distribution_artifact_written == false
  and (.source_alignment.release_artifact_intake_markdown_sha256 | test("^[0-9a-f]{64}$"))
  and .source_alignment.release_artifact_roundtrip_ready == true
  and .source_alignment.release_artifact_roundtrip_ready_count == 2
  and .source_alignment.release_artifact_roundtrip_present_branch_ready == false
  and .source_alignment.release_artifact_roundtrip_waiting_branch_ready == true
  and .source_alignment.release_artifact_roundtrip_present_artifact_valid == false
  and .source_alignment.release_artifact_roundtrip_present_artifact_branch_supported == false
  and .source_alignment.release_artifact_roundtrip_independent_approval_verifier_contract_ready == false
  and .source_alignment.release_artifact_roundtrip_legacy_simulated_rejected == true
  and .source_alignment.release_artifact_roundtrip_v3_valid_branch_selftest_ready == true
  and .source_alignment.release_artifact_roundtrip_root_report_required_count == 41
  and (.source_alignment.release_artifact_roundtrip_legacy_artifact_sha256 | test("^[0-9a-f]{64}$"))
  and (.source_alignment.release_artifact_roundtrip_legacy_rejection_report_sha256 | test("^[0-9a-f]{64}$"))
  and (.source_alignment.release_artifact_roundtrip_v3_selftest_log_sha256 | test("^[0-9a-f]{64}$"))
  and .source_alignment.current_plan_refresh_ready == true
  and .source_alignment.current_plan_refresh_minimum_gate_id == "r62_minimum_ui_demo_gate"
  and .source_alignment.current_plan_refresh_root_report_required_count == 41
  and .source_alignment.current_plan_refresh_current_plan_ids == ["r62_minimum_ui_demo_gate","backend_real_receipt_return","ui_refresh_after_real_receipt","release_artifact_roundtrip_and_signed_artifact_gate"]
  and (.source_alignment.current_plan_refresh_markdown_sha256 | test("^[0-9a-f]{64}$"))
  and .source_alignment.blocker_closure_ready == true
  and .source_alignment.blocker_closure_root_report_required_count == 41
  and (.source_alignment.blocker_closure_critical_blocker_count >= 0 and .source_alignment.blocker_closure_critical_blocker_count <= 10)
  and (.source_alignment.blocker_closure_backend_agent_available | type) == "boolean"
  and (.source_alignment.blocker_closure_real_backend_receipt_present | type) == "boolean"
  and .source_alignment.blocker_closure_release_artifact_valid == .source_alignment.release_artifact_valid
  and (.source_alignment.blocker_closure_markdown_sha256 | test("^[0-9a-f]{64}$"))
  and .source_alignment.backend_delivery_audit_ready == true
  and .source_alignment.backend_delivery_audit_root_report_required_count == 41
  and (
    (
      .source_alignment.backend_delivery_audit_delivery_receipt_present == false
      and .source_alignment.backend_delivery_audit_delivery_receipt_valid == false
      and .source_alignment.backend_delivery_audit_waiting_for_delivery_receipt == true
      and .source_alignment.backend_delivery_audit_critical_blocker_count >= 1
      and .source_alignment.backend_delivery_audit_critical_blocker_count <= 11
      and .source_alignment.backend_delivery_audit_backend_delivery_claim_ready == false
      and .claim_boundary.backend_delivery_claim_ready == false
    )
    or
    (
      .source_alignment.backend_delivery_audit_delivery_receipt_present == true
      and .source_alignment.backend_delivery_audit_delivery_receipt_valid == true
      and .source_alignment.backend_delivery_audit_waiting_for_delivery_receipt == false
      and (
        .source_alignment.backend_delivery_audit_critical_blocker_count >= 0
        and .source_alignment.backend_delivery_audit_critical_blocker_count <= 11
      )
      and .source_alignment.backend_delivery_audit_backend_delivery_claim_ready == true
      and .claim_boundary.backend_delivery_claim_ready == true
    )
  )
  and (.source_alignment.backend_delivery_audit_markdown_sha256 | test("^[0-9a-f]{64}$"))
  and .source_alignment.backend_delivery_receipt_roundtrip_ready == true
  and .source_alignment.backend_delivery_receipt_roundtrip_ready_count == 3
  and .source_alignment.backend_delivery_receipt_roundtrip_waiting_branch_ready == true
  and .source_alignment.backend_delivery_receipt_roundtrip_present_branch_ready == true
  and .source_alignment.backend_delivery_receipt_roundtrip_simulated_receipt_ready == true
  and .source_alignment.backend_delivery_receipt_roundtrip_present_branch_valid == true
  and .source_alignment.backend_delivery_receipt_roundtrip_present_branch_claim_ready == true
  and .source_alignment.backend_delivery_receipt_roundtrip_root_report_required_count == 45
  and (.source_alignment.backend_delivery_receipt_roundtrip_simulated_receipt_sha256 | test("^[0-9a-f]{64}$"))
  and (.source_alignment.backend_delivery_receipt_roundtrip_present_report_sha256 | test("^[0-9a-f]{64}$"))
  and .source_alignment.risk_future_plan_ready == true
  and .source_alignment.risk_future_plan_latest_minimum_gate_id == "r151_harsh_top_design_v46_badge_micro_surface_light_glass_minimum_ui_demo_gate"
  and .source_alignment.risk_future_plan_latest_plan_ids == ["r151_harsh_top_design_v46_badge_micro_surface_light_glass_minimum_ui_demo_gate","backend_delivery_receipt_return","backend_real_receipt_return","ui_refresh_after_real_receipt","release_artifact_roundtrip_and_signed_artifact_gate"]
	  and .source_alignment.risk_future_plan_top_design_refresh_version == 46
		  and .source_alignment.risk_future_plan_top_design_harsh_2026_referee_ready == true
		  and .source_alignment.risk_future_plan_control_ui_harsh_2026_ready == true
	  and .source_alignment.risk_future_plan_control_ui_microcopy_word_split_guard_ready == true
	  and .source_alignment.risk_future_plan_control_ui_logo_clip_guard_ready == true
	  and .source_alignment.risk_future_plan_control_ui_active_chat_readability_ready == true
	  and .source_alignment.risk_future_plan_control_ui_placeholder_readability_ready == true
	  and .source_alignment.risk_future_plan_control_ui_small_control_readability_ready == true
	  and .source_alignment.risk_future_plan_control_ui_rail_action_icon_ready == true
	  and .source_alignment.risk_future_plan_control_ui_folder_chip_touch_ready == true
	  and .source_alignment.risk_future_plan_control_ui_row_menu_touch_ready == true
	  and .source_alignment.risk_future_plan_control_ui_row_menu_all_rows_ready == true
	  and .source_alignment.risk_future_plan_control_ui_row_menu_light_glass_ready == true
	  and .source_alignment.risk_future_plan_control_ui_command_palette_ready == true
	  and .source_alignment.risk_future_plan_control_ui_command_palette_surface_light_glass_ready == true
	  and .source_alignment.risk_future_plan_control_ui_command_palette_trigger_light_glass_ready == true
	  and .source_alignment.risk_future_plan_control_ui_command_palette_close_light_glass_ready == true
	  and .source_alignment.risk_future_plan_control_ui_command_palette_input_light_glass_ready == true
	  and .source_alignment.risk_future_plan_control_ui_command_palette_item_light_glass_ready == true
	  and .source_alignment.risk_future_plan_control_ui_form_control_title_touch_ready == true
	  and .source_alignment.risk_future_plan_control_ui_chat_row_option_semantic_touch_ready == true
	  and .source_alignment.risk_future_plan_control_ui_thread_tools_menu_ready == true
	  and .source_alignment.risk_future_plan_control_ui_composer_tools_menu_ready == true
	  and .source_alignment.risk_future_plan_control_ui_composer_popover_ready == true
	  and .source_alignment.risk_future_plan_control_ui_composer_popover_search_light_glass_ready == true
	  and .source_alignment.risk_future_plan_control_ui_rail_search_light_glass_ready == true
	  and .source_alignment.risk_future_plan_control_ui_micro_surface_light_glass_ready == true
	  and .source_alignment.risk_future_plan_control_ui_message_routing_badge_light_glass_ready == true
	    and .source_alignment.risk_future_plan_control_ui_thread_intro_badge_light_glass_ready == true
	  and .source_alignment.risk_future_plan_control_ui_status_trust_strip_light_glass_ready == true
	  and .source_alignment.risk_future_plan_control_ui_visible_text_integrity_ready == true
	  and .source_alignment.risk_future_plan_control_ui_icon_button_title_match_ready == true
	  and .source_alignment.risk_future_plan_control_ui_menu_trigger_title_match_ready == true
	  and .source_alignment.risk_future_plan_native_secondary_harsh_action_matrix_ready == true
	  and .source_alignment.risk_future_plan_native_secondary_title_tooltip_ready == true
	  and .source_alignment.risk_future_plan_native_secondary_title_tooltip_failure_count == 0
  and .source_alignment.risk_future_plan_tempered_glass_2026_ready == true
  and .source_alignment.risk_future_plan_tempered_glass_min_contrast_ratio >= 4.5
  and .source_alignment.risk_future_plan_tempered_glass_clipping_failure_count == 0
  and .source_alignment.risk_future_plan_action_matrix_ready == true
  and .source_alignment.risk_future_plan_action_matrix_case_count == 15
  and .source_alignment.risk_future_plan_harsh_action_matrix_ready == true
  and .source_alignment.risk_future_plan_harsh_action_failure_count == 0
  and (.source_alignment.risk_future_plan_critical_blocker_count >= 0 and .source_alignment.risk_future_plan_critical_blocker_count <= 11)
  and .source_alignment.risk_future_plan_root_report_required_count == 45
  and (.source_alignment.risk_future_plan_markdown_sha256 | test("^[0-9a-f]{64}$"))
  and (.source_alignment.evidence_archive_sha256 | test("^[0-9a-f]{64}$"))
  and (.source_alignment.release_operator_dry_run_manifest_sha256 | test("^[0-9a-f]{64}$"))
  and .source_alignment.evidence_archive_bytes > 0
  and .source_alignment.release_operator_dry_run_manifest_bytes > 0
  and .source_alignment.true_window_alignment_ready == true
  and .future_plan_replay.next_plan_count == 3
  and .future_plan_replay.backend_contract_remaining_count == 12
  and .future_plan_replay.backend_alignment_evidence_selected_ids == ["message_search","file_upload_send","media_download_playback","notifications","room_settings"]
  and .future_plan_replay.critical_path_plan_selected_ids == ["message_search","file_upload_send","media_download_playback","notifications","room_settings"]
  and .future_plan_replay.backend_contract_acceptance_selected_ids == ["message_search","file_upload_send","media_download_playback","notifications","room_settings"]
  and .future_plan_replay.backend_contract_acceptance_future_plan_link.critical_path_plan_id == "backend_contract_first_five"
  and .future_plan_replay.backend_handoff_export_selected_ids == ["message_search","file_upload_send","media_download_playback","notifications","room_settings"]
  and .future_plan_replay.backend_handoff_export_target_repo == "/Users/qianqi/.openclaw/workspace/Hepta"
  and .future_plan_replay.backend_dispatch_packet_selected_ids == ["message_search","file_upload_send","media_download_playback","notifications","room_settings"]
  and .future_plan_replay.backend_dispatch_packet_target_repo == "/Users/qianqi/.openclaw/workspace/Hepta"
  and .future_plan_replay.backend_receipt_intake_selected_ids == ["message_search","file_upload_send","media_download_playback","notifications","room_settings"]
  and .future_plan_replay.backend_receipt_intake_waiting_for_receipt == .source_alignment.backend_receipt_intake_waiting_for_receipt
  and .future_plan_replay.backend_receipt_present == .source_alignment.backend_receipt_present
  and .future_plan_replay.backend_receipt_valid == .source_alignment.backend_receipt_valid
  and .future_plan_replay.backend_receipt_roundtrip_selected_ids == ["message_search","file_upload_send","media_download_playback","notifications","room_settings"]
  and .future_plan_replay.backend_receipt_roundtrip_present_branch_ready == true
  and .future_plan_replay.backend_receipt_roundtrip_simulated_receipt_ready == true
  and .future_plan_replay.backend_receipt_refresh_lock_selected_ids == ["message_search","file_upload_send","media_download_playback","notifications","room_settings"]
  and .future_plan_replay.refreshed_future_plan_ids == ["r52_minimum_ui_demo_gate","backend_real_receipt_return","ui_refresh_after_real_receipt"]
  and .future_plan_replay.refreshed_future_plan_minimum_gate_id == "r52_minimum_ui_demo_gate"
  and .future_plan_replay.operator_briefing_refresh_plan_ids == ["r52_minimum_ui_demo_gate","backend_real_receipt_return","ui_refresh_after_real_receipt"]
  and (
    .future_plan_replay.operator_briefing_refresh_updated_critical_risk_count >= 1
    and .future_plan_replay.operator_briefing_refresh_updated_critical_risk_count <= 4
  )
  and .future_plan_replay.operator_briefing_refresh_root_report_required_count == 33
  and .future_plan_replay.release_approval_intake_waiting_for_approval == .source_alignment.release_approval_intake_waiting_for_approval
  and .future_plan_replay.release_approval_intake_root_report_required_count == 34
  and .future_plan_replay.release_approval_intake_next_required_artifact_gate == "signed_notarized_stapled_artifact_gate"
  and .future_plan_replay.top_design_referee_refresh_ready == true
	  and .future_plan_replay.top_design_referee_refresh_version == 46
		  and .future_plan_replay.top_design_harsh_2026_referee_ready == true
		  and .future_plan_replay.top_design_control_ui_harsh_2026_ready == true
	  and .future_plan_replay.top_design_control_ui_microcopy_word_split_guard_ready == true
	  and .future_plan_replay.top_design_control_ui_logo_clip_guard_ready == true
	  and .future_plan_replay.top_design_control_ui_active_chat_readability_ready == true
	  and .future_plan_replay.top_design_control_ui_placeholder_readability_ready == true
	  and .future_plan_replay.top_design_control_ui_small_control_readability_ready == true
	  and .future_plan_replay.top_design_control_ui_rail_action_icon_ready == true
	  and .future_plan_replay.top_design_control_ui_folder_chip_touch_ready == true
	  and .future_plan_replay.top_design_control_ui_row_menu_touch_ready == true
	  and .future_plan_replay.top_design_control_ui_row_menu_all_rows_ready == true
	  and .future_plan_replay.top_design_control_ui_row_menu_light_glass_ready == true
	  and .future_plan_replay.top_design_control_ui_command_palette_ready == true
	  and .future_plan_replay.top_design_control_ui_command_palette_surface_light_glass_ready == true
	  and .future_plan_replay.top_design_control_ui_command_palette_trigger_light_glass_ready == true
	  and .future_plan_replay.top_design_control_ui_command_palette_close_light_glass_ready == true
	  and .source_alignment.top_design_control_ui_command_palette_ready == true
	  and .source_alignment.top_design_control_ui_command_palette_surface_light_glass_ready == true
	  and .source_alignment.top_design_control_ui_command_palette_trigger_light_glass_ready == true
	  and .source_alignment.top_design_control_ui_command_palette_close_light_glass_ready == true
	  and .future_plan_replay.top_design_control_ui_command_palette_input_light_glass_ready == true
	  and .source_alignment.top_design_control_ui_command_palette_input_light_glass_ready == true
	  and .future_plan_replay.top_design_control_ui_command_palette_item_light_glass_ready == true
	  and .source_alignment.top_design_control_ui_command_palette_item_light_glass_ready == true
	  and .future_plan_replay.top_design_control_ui_form_control_title_touch_ready == true
	  and .future_plan_replay.top_design_control_ui_chat_row_option_semantic_touch_ready == true
	  and .future_plan_replay.top_design_control_ui_thread_tools_menu_ready == true
	  and .future_plan_replay.top_design_control_ui_composer_tools_menu_ready == true
	  and .future_plan_replay.top_design_control_ui_composer_popover_ready == true
	  and .future_plan_replay.top_design_control_ui_composer_popover_search_light_glass_ready == true
	  and .future_plan_replay.top_design_control_ui_rail_search_light_glass_ready == true
	  and .future_plan_replay.top_design_control_ui_micro_surface_light_glass_ready == true
	  and .future_plan_replay.top_design_control_ui_message_routing_badge_light_glass_ready == true
	    and .future_plan_replay.top_design_control_ui_thread_intro_badge_light_glass_ready == true
	  and .future_plan_replay.top_design_control_ui_status_trust_strip_light_glass_ready == true
	  and .source_alignment.top_design_control_ui_form_control_title_touch_ready == true
	  and .source_alignment.top_design_control_ui_chat_row_option_semantic_touch_ready == true
	  and .source_alignment.top_design_control_ui_thread_tools_menu_ready == true
	  and .source_alignment.top_design_control_ui_composer_tools_menu_ready == true
	  and .source_alignment.top_design_control_ui_composer_popover_ready == true
	  and .source_alignment.top_design_control_ui_composer_popover_search_light_glass_ready == true
	  and .source_alignment.top_design_control_ui_rail_search_light_glass_ready == true
	  and .source_alignment.top_design_control_ui_micro_surface_light_glass_ready == true
	  and .source_alignment.top_design_control_ui_message_routing_badge_light_glass_ready == true
	    and .source_alignment.top_design_control_ui_thread_intro_badge_light_glass_ready == true
	  and .source_alignment.top_design_control_ui_status_trust_strip_light_glass_ready == true
	  and .future_plan_replay.top_design_control_ui_visible_text_integrity_ready == true
	  and .future_plan_replay.top_design_control_ui_icon_button_title_match_ready == true
	  and .future_plan_replay.top_design_control_ui_menu_trigger_title_match_ready == true
	  and .future_plan_replay.top_design_native_secondary_harsh_action_matrix_ready == true
	  and .future_plan_replay.top_design_native_secondary_title_tooltip_ready == true
	  and .future_plan_replay.top_design_native_secondary_title_tooltip_failure_count == 0
  and .future_plan_replay.top_design_tempered_glass_2026_ready == true
  and .future_plan_replay.top_design_tempered_glass_min_contrast_ratio >= 4.5
  and .future_plan_replay.top_design_tempered_glass_clipping_failure_count == 0
  and .future_plan_replay.top_design_referee_refresh_action_matrix_ready == true
  and .future_plan_replay.top_design_referee_refresh_action_matrix_case_count == 15
  and .future_plan_replay.top_design_referee_refresh_harsh_action_matrix_ready == true
  and .future_plan_replay.top_design_referee_refresh_harsh_action_failure_count == 0
  and .future_plan_replay.top_design_referee_refresh_control_phone320_ready == true
  and .future_plan_replay.top_design_referee_refresh_current_roundtrip_plan_ids == ["r62_minimum_ui_demo_gate","backend_real_receipt_return","ui_refresh_after_real_receipt","release_artifact_roundtrip_and_signed_artifact_gate"]
  and .future_plan_replay.top_design_referee_refresh_current_minimum_gate_id == "r62_minimum_ui_demo_gate"
  and .future_plan_replay.top_design_referee_refresh_root_report_required_count == 35
  and .future_plan_replay.top_design_referee_refresh_downstream_root_report_required_count == 41
  and .future_plan_replay.release_artifact_boundary_ready == true
  and .future_plan_replay.release_artifact_boundary_root_report_required_count == 36
  and .future_plan_replay.release_artifact_boundary_next_required_artifact_gate == "signed_notarized_stapled_artifact_gate"
  and .future_plan_replay.release_artifact_boundary_signed_notarized_stapled_artifact_present == false
  and .future_plan_replay.release_artifact_boundary_public_distribution_artifact_written == false
  and .future_plan_replay.release_artifact_intake_ready == true
  and .future_plan_replay.release_artifact_intake_root_report_required_count == 37
  and .future_plan_replay.release_artifact_intake_waiting_for_artifact == .source_alignment.release_artifact_intake_waiting_for_artifact
  and .future_plan_replay.release_artifact_intake_release_artifact_valid == .source_alignment.release_artifact_valid
  and .future_plan_replay.release_artifact_roundtrip_ready == true
  and .future_plan_replay.release_artifact_roundtrip_root_report_required_count == 41
  and .future_plan_replay.release_artifact_intake_release_artifact_valid == false
  and .future_plan_replay.release_artifact_intake_present_artifact_branch_supported == false
  and .future_plan_replay.release_artifact_intake_independent_approval_verifier_contract_ready == false
  and .future_plan_replay.release_artifact_roundtrip_present_branch_ready == false
  and .future_plan_replay.release_artifact_roundtrip_present_artifact_valid == false
  and .future_plan_replay.release_artifact_roundtrip_present_artifact_branch_supported == false
  and .future_plan_replay.release_artifact_roundtrip_independent_approval_verifier_contract_ready == false
  and .future_plan_replay.release_artifact_roundtrip_legacy_simulated_rejected == true
  and .future_plan_replay.release_artifact_roundtrip_v3_valid_branch_selftest_ready == true
  and .future_plan_replay.current_plan_refresh_ids == ["r62_minimum_ui_demo_gate","backend_real_receipt_return","ui_refresh_after_real_receipt","release_artifact_roundtrip_and_signed_artifact_gate"]
  and .future_plan_replay.current_plan_refresh_minimum_gate_id == "r62_minimum_ui_demo_gate"
  and .future_plan_replay.current_plan_refresh_root_report_required_count == 41
  and .future_plan_replay.current_plan_refresh_legacy_plan_ids == ["r52_minimum_ui_demo_gate","backend_real_receipt_return","ui_refresh_after_real_receipt"]
  and .future_plan_replay.current_plan_refresh_next_required_artifact_gate == "signed_notarized_stapled_artifact_gate"
  and .future_plan_replay.blocker_closure_ready == true
  and .future_plan_replay.blocker_closure_root_report_required_count == 41
  and (.future_plan_replay.blocker_closure_critical_blocker_count >= 0 and .future_plan_replay.blocker_closure_critical_blocker_count <= 10)
  and ((.future_plan_replay.blocker_closure_next_unblock_sequence | length) >= 1 and (.future_plan_replay.blocker_closure_next_unblock_sequence | length) <= 7)
  and .future_plan_replay.backend_delivery_audit_ready == true
  and .future_plan_replay.backend_delivery_audit_root_report_required_count == 41
  and .future_plan_replay.backend_delivery_audit_waiting_for_delivery_receipt == .source_alignment.backend_delivery_audit_waiting_for_delivery_receipt
  and (
    (.future_plan_replay.backend_delivery_audit_next_unblock_sequence | length) == 6
    or (.future_plan_replay.backend_delivery_audit_next_unblock_sequence | length) == 5
    or (.future_plan_replay.backend_delivery_audit_next_unblock_sequence | length) == 4
    or (.future_plan_replay.backend_delivery_audit_next_unblock_sequence | length) == 3
    or (.future_plan_replay.backend_delivery_audit_next_unblock_sequence | length) == 2
    or (.future_plan_replay.backend_delivery_audit_next_unblock_sequence | length) == 1
  )
  and .future_plan_replay.backend_delivery_receipt_roundtrip_ready == true
  and .future_plan_replay.backend_delivery_receipt_roundtrip_present_branch_ready == true
  and .future_plan_replay.backend_delivery_receipt_roundtrip_present_branch_valid == true
  and .future_plan_replay.backend_delivery_receipt_roundtrip_root_report_required_count == 45
  and .future_plan_replay.risk_future_plan_ready == true
  and .future_plan_replay.risk_future_plan_latest_minimum_gate_id == "r151_harsh_top_design_v46_badge_micro_surface_light_glass_minimum_ui_demo_gate"
  and .future_plan_replay.risk_future_plan_latest_plan_ids == ["r151_harsh_top_design_v46_badge_micro_surface_light_glass_minimum_ui_demo_gate","backend_delivery_receipt_return","backend_real_receipt_return","ui_refresh_after_real_receipt","release_artifact_roundtrip_and_signed_artifact_gate"]
	  and .future_plan_replay.risk_future_plan_top_design_refresh_version == 46
	  and .future_plan_replay.risk_future_plan_control_ui_microcopy_word_split_guard_ready == true
	  and .future_plan_replay.risk_future_plan_control_ui_logo_clip_guard_ready == true
	  and .future_plan_replay.risk_future_plan_control_ui_active_chat_readability_ready == true
	  and .future_plan_replay.risk_future_plan_control_ui_placeholder_readability_ready == true
	  and .future_plan_replay.risk_future_plan_control_ui_small_control_readability_ready == true
	  and .future_plan_replay.risk_future_plan_control_ui_rail_action_icon_ready == true
	  and .future_plan_replay.risk_future_plan_control_ui_folder_chip_touch_ready == true
	  and .future_plan_replay.risk_future_plan_control_ui_row_menu_touch_ready == true
	  and .future_plan_replay.risk_future_plan_control_ui_row_menu_all_rows_ready == true
	  and .future_plan_replay.risk_future_plan_control_ui_row_menu_light_glass_ready == true
	  and .future_plan_replay.risk_future_plan_control_ui_command_palette_ready == true
	  and .future_plan_replay.risk_future_plan_control_ui_command_palette_surface_light_glass_ready == true
	  and .future_plan_replay.risk_future_plan_control_ui_command_palette_trigger_light_glass_ready == true
	  and .future_plan_replay.risk_future_plan_control_ui_command_palette_close_light_glass_ready == true
	  and .source_alignment.risk_future_plan_control_ui_command_palette_ready == true
	  and .source_alignment.risk_future_plan_control_ui_command_palette_surface_light_glass_ready == true
	  and .source_alignment.risk_future_plan_control_ui_command_palette_trigger_light_glass_ready == true
	  and .source_alignment.risk_future_plan_control_ui_command_palette_close_light_glass_ready == true
	  and .future_plan_replay.risk_future_plan_control_ui_command_palette_input_light_glass_ready == true
	  and .source_alignment.risk_future_plan_control_ui_command_palette_input_light_glass_ready == true
	  and .future_plan_replay.risk_future_plan_control_ui_command_palette_item_light_glass_ready == true
	  and .source_alignment.risk_future_plan_control_ui_command_palette_item_light_glass_ready == true
	  and .future_plan_replay.risk_future_plan_control_ui_form_control_title_touch_ready == true
	  and .future_plan_replay.risk_future_plan_control_ui_chat_row_option_semantic_touch_ready == true
	  and .future_plan_replay.risk_future_plan_control_ui_thread_tools_menu_ready == true
	  and .future_plan_replay.risk_future_plan_control_ui_composer_tools_menu_ready == true
	  and .future_plan_replay.risk_future_plan_control_ui_composer_popover_ready == true
	  and .future_plan_replay.risk_future_plan_control_ui_composer_popover_search_light_glass_ready == true
	  and .future_plan_replay.risk_future_plan_control_ui_rail_search_light_glass_ready == true
	  and .future_plan_replay.risk_future_plan_control_ui_micro_surface_light_glass_ready == true
	  and .future_plan_replay.risk_future_plan_control_ui_message_routing_badge_light_glass_ready == true
	    and .future_plan_replay.risk_future_plan_control_ui_thread_intro_badge_light_glass_ready == true
	  and .future_plan_replay.risk_future_plan_control_ui_status_trust_strip_light_glass_ready == true
	  and .source_alignment.risk_future_plan_control_ui_form_control_title_touch_ready == true
	  and .source_alignment.risk_future_plan_control_ui_chat_row_option_semantic_touch_ready == true
	  and .source_alignment.risk_future_plan_control_ui_thread_tools_menu_ready == true
	  and .source_alignment.risk_future_plan_control_ui_composer_tools_menu_ready == true
	  and .source_alignment.risk_future_plan_control_ui_composer_popover_ready == true
	  and .source_alignment.risk_future_plan_control_ui_composer_popover_search_light_glass_ready == true
	  and .source_alignment.risk_future_plan_control_ui_rail_search_light_glass_ready == true
	  and .source_alignment.risk_future_plan_control_ui_micro_surface_light_glass_ready == true
	  and .source_alignment.risk_future_plan_control_ui_message_routing_badge_light_glass_ready == true
	    and .source_alignment.risk_future_plan_control_ui_thread_intro_badge_light_glass_ready == true
	  and .source_alignment.risk_future_plan_control_ui_status_trust_strip_light_glass_ready == true
	  and .future_plan_replay.risk_future_plan_control_ui_visible_text_integrity_ready == true
	  and .future_plan_replay.risk_future_plan_control_ui_icon_button_title_match_ready == true
	  and .future_plan_replay.risk_future_plan_control_ui_menu_trigger_title_match_ready == true
	  and .future_plan_replay.risk_future_plan_native_secondary_title_tooltip_ready == true
	  and .future_plan_replay.risk_future_plan_native_secondary_title_tooltip_failure_count == 0
	  and .future_plan_replay.risk_future_plan_tempered_glass_2026_ready == true
  and .future_plan_replay.risk_future_plan_tempered_glass_min_contrast_ratio >= 4.5
  and .future_plan_replay.risk_future_plan_tempered_glass_clipping_failure_count == 0
  and .future_plan_replay.risk_future_plan_action_matrix_ready == true
  and .future_plan_replay.risk_future_plan_action_matrix_case_count == 15
  and .future_plan_replay.risk_future_plan_root_report_required_count == 45
  and (.future_plan_replay.risk_future_plan_critical_blocker_count >= 0 and .future_plan_replay.risk_future_plan_critical_blocker_count <= 11)
  and ((.future_plan_replay.risk_future_plan_next_unblock_sequence | length) >= 1 and (.future_plan_replay.risk_future_plan_next_unblock_sequence | length) <= 6)
  and (.future_plan_replay.backend_priority_ids | length) == 12
  and (.future_plan_replay.backend_priority_ids[0] == "message_search")
  and (.future_plan_replay.backend_priority_ids[1] == "file_upload_send")
  and (.future_plan_replay.backend_priority_ids[2] == "media_download_playback")
  and .source_alignment.release_approval_valid == false
  and .source_alignment.independent_approval_verifier_ready == false
  and .source_alignment.self_reported_approval_can_authorize_release == false
  and (.future_plan_replay.release_blockers | index("operator_release_approval_required")) != null
  and (.future_plan_replay.release_blockers | index("independent_release_approval_verifier_unavailable")) != null
  and .source_alignment.release_artifact_intake_public_distribution_artifact_written == false
  and (.future_plan_replay.release_blockers | index("public_distribution_artifact_not_written")) != null
  and .claim_boundary.live_product_claim_ready == false
  and .claim_boundary.public_distribution_claim_ready == false
  and .claim_boundary.release_claim_ready == false
  and .claim_boundary.local_release_operator_dry_run_ready == true
  and .claim_boundary.local_operator_briefing_ready == true
  and .claim_boundary.local_backend_promotion_packet_ready == true
  and .claim_boundary.local_backend_alignment_evidence_ready == true
  and .claim_boundary.local_critical_path_plan_ready == true
  and .claim_boundary.local_backend_contract_acceptance_ready == true
  and .claim_boundary.local_backend_handoff_export_ready == true
  and .claim_boundary.local_backend_dispatch_packet_ready == true
  and .claim_boundary.local_backend_receipt_intake_ready == true
  and .claim_boundary.local_backend_receipt_roundtrip_ready == true
  and .claim_boundary.local_backend_receipt_refresh_lock_ready == true
  and .claim_boundary.local_future_plan_refresh_ready == true
  and .claim_boundary.local_operator_briefing_refresh_ready == true
  and .claim_boundary.local_release_approval_intake_ready == true
  and .claim_boundary.local_top_design_referee_refresh_ready == true
  and .claim_boundary.local_release_artifact_boundary_ready == true
  and .claim_boundary.local_release_artifact_intake_ready == true
  and .claim_boundary.local_release_artifact_roundtrip_ready == true
  and .claim_boundary.local_current_plan_refresh_ready == true
  and .claim_boundary.local_blocker_closure_ready == true
  and .claim_boundary.local_backend_delivery_audit_ready == true
  and .claim_boundary.local_backend_delivery_receipt_roundtrip_ready == true
  and .claim_boundary.local_risk_future_plan_ready == true
  and .claim_boundary.desktop_mobile_design_claim_ready == true
  and .claim_boundary.release_approval_claim_ready == .source_alignment.release_approval_valid
  and .claim_boundary.backend_delivery_claim_ready == .source_alignment.backend_delivery_audit_delivery_receipt_valid
  and .claim_boundary.release_artifact_claim_ready == false
  and .claim_boundary.simulated_backend_receipt_branch_ready == true
  and .claim_boundary.external_actions_allowed == false
  and .claim_boundary.public_upload_performed == false
  and .claim_boundary.signing_notarization_performed == false
  and .side_effects.external_mutation == false
' "$REPORT_TMP" >/dev/null

hepta_safe_atomic_replace_owned_json "$REPORT_TMP" "$REPORT_PATH" root_report \
  gate hepta_ui_root_report_replay_gate \
  report_path "$REPORT_PATH" \
  product "Hepta UI" \
  runtime hepta
cat "$REPORT_TMP"
