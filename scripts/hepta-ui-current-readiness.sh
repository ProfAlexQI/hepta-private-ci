#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
cd "$ROOT_DIR"

EVIDENCE_DIR=""
REPORT_PATH=""
REQUIRE="source"
VERIFY_FEATURES=0
VERIFY_PACKAGE=0
VERIFY_BROWSER=0
VERIFY_WINDOW=0
TARGET_DIR="${HEPTA_NATIVE_CARGO_TARGET_DIR:-${TMPDIR:-/tmp}/hepta-ui-current-readiness-target}"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --evidence-dir) EVIDENCE_DIR="${2:-}"; shift 2 ;;
    --output) REPORT_PATH="${2:-}"; shift 2 ;;
    --verify) VERIFY_FEATURES=1; VERIFY_PACKAGE=1; shift ;;
    --verify-local) VERIFY_FEATURES=1; VERIFY_PACKAGE=1; VERIFY_BROWSER=1; VERIFY_WINDOW=1; shift ;;
    --verify-features) VERIFY_FEATURES=1; shift ;;
    --verify-package) VERIFY_PACKAGE=1; shift ;;
    --verify-browser) VERIFY_BROWSER=1; shift ;;
    --verify-window) VERIFY_WINDOW=1; shift ;;
    --target-dir) TARGET_DIR="${2:-}"; shift 2 ;;
    --require) REQUIRE="${2:-}"; shift 2 ;;
    --help|-h)
      cat <<'EOF'
usage: scripts/hepta-ui-current-readiness.sh [options]

  --evidence-dir DIR       write all current-run receipts under DIR
  --output FILE            write the orchestrator report (default stdout)
  --verify                 run Native feature matrix and local unsigned package
  --verify-local           run every source/local-demo verifier
  --verify-features        run no-default/default/all-features Cargo checks
  --verify-package         build and stage a current-source unsigned Hepta.app
  --verify-browser         run the Control four-viewport browser smoke
  --verify-window          run the independent current-package Native window verifier
  --target-dir DIR         shared Native Cargo target directory
  --require LEVEL          none|source|local|full|ga (default source)

The gate is fail-closed. Missing live Matrix, live Hepta adapter, real-device,
accessibility, signing, notarization, stapling, or distribution receipts remain
false. Historical evidence is never consumed.
EOF
      exit 0
      ;;
    *) echo "unknown argument: $1" >&2; exit 64 ;;
  esac
done

case "$REQUIRE" in none|source|local|full|ga) ;; *) echo "invalid --require level: $REQUIRE" >&2; exit 64 ;; esac
case "$REQUIRE" in
  local|full|ga)
    VERIFY_FEATURES=1
    VERIFY_PACKAGE=1
    VERIFY_BROWSER=1
    VERIFY_WINDOW=1
    ;;
esac
for command in git jq ruby shasum uuidgen; do command -v "$command" >/dev/null 2>&1 || { echo "$command is required" >&2; exit 2; }; done

normalize_future_path() {
  ruby -e '
    cursor = File.expand_path(ARGV.fetch(0))
    suffix = []
    until File.exist?(cursor) || File.dirname(cursor) == cursor
      suffix.unshift(File.basename(cursor))
      cursor = File.dirname(cursor)
    end
    base = File.realpath(cursor)
    print File.join(base, *suffix)
  ' "$1"
}

path_is_within() {
  ruby -e '
    path = File.expand_path(ARGV.fetch(0))
    root = File.expand_path(ARGV.fetch(1))
    exit(path.start_with?(root + File::SEPARATOR) ? 0 : 1)
  ' "$1" "$2"
}

if [[ -z "$EVIDENCE_DIR" ]]; then
  EVIDENCE_DIR="$(mktemp -d "${TMPDIR:-/tmp}/hepta-ui-current-readiness.XXXXXX")"
elif [[ -L "$EVIDENCE_DIR" || ( -e "$EVIDENCE_DIR" && ! -d "$EVIDENCE_DIR" ) ]]; then
  echo "--evidence-dir must be a real directory, not a symlink or non-directory" >&2
  exit 64
fi
if [[ -L "$TARGET_DIR" || ( -e "$TARGET_DIR" && ! -d "$TARGET_DIR" ) ]]; then
  echo "--target-dir must be a real directory, not a symlink or non-directory" >&2
  exit 64
fi
EVIDENCE_DIR="$(normalize_future_path "$EVIDENCE_DIR")"
TARGET_DIR="$(normalize_future_path "$TARGET_DIR")"
if [[ "$EVIDENCE_DIR" == "$TARGET_DIR" ]] || path_is_within "$EVIDENCE_DIR" "$TARGET_DIR" || path_is_within "$TARGET_DIR" "$EVIDENCE_DIR"; then
  echo "--evidence-dir and --target-dir must not overlap" >&2
  exit 64
fi
mkdir -p "$EVIDENCE_DIR" "$TARGET_DIR"
if [[ -z "$REPORT_PATH" ]]; then REPORT_PATH="$EVIDENCE_DIR/current-readiness.json"; fi
if [[ -L "$REPORT_PATH" || ( -e "$REPORT_PATH" && ! -f "$REPORT_PATH" ) ]]; then
  echo "--output must be a regular file path, not a symlink or special file" >&2
  exit 64
fi
REPORT_PATH="$(normalize_future_path "$REPORT_PATH")"
CANONICAL_CURRENT_REPORT="$EVIDENCE_DIR/current-readiness.json"
if [[ "$REPORT_PATH" == "$EVIDENCE_DIR" || "$REPORT_PATH" == "$TARGET_DIR" ]] \
  || path_is_within "$REPORT_PATH" "$TARGET_DIR" || path_is_within "$TARGET_DIR" "$REPORT_PATH"; then
  echo "--output must not overlap --target-dir or equal --evidence-dir" >&2
  exit 64
fi
if path_is_within "$REPORT_PATH" "$EVIDENCE_DIR" \
  && [[ "$REPORT_PATH" != "$CANONICAL_CURRENT_REPORT" ]]; then
  echo "--output must not be placed inside the producer-owned evidence tree (except current-readiness.json)" >&2
  exit 64
fi
mkdir -p "$(dirname "$REPORT_PATH")"
RUN_NONCE="$(uuidgen | tr '[:upper:]' '[:lower:]')"

BINDING_BEFORE="$EVIDENCE_DIR/source-binding-before.json"
BINDING_AFTER="$EVIDENCE_DIR/source-binding-after.json"
BINDING_FINAL="$EVIDENCE_DIR/source-binding-final.json"
SYNC_REPORT="$EVIDENCE_DIR/native-upstream-sync.json"
PRODUCT_REPORT="$EVIDENCE_DIR/native-product-shell.json"
TOKEN_REPORT="$EVIDENCE_DIR/token-sync.json"
FEATURE_REPORT="$EVIDENCE_DIR/native-feature-matrix.json"
PACKAGE_REPORT="$EVIDENCE_DIR/native-current-package.json"
BROWSER_REPORT="$EVIDENCE_DIR/control-browser-smoke.json"
MOBILE_REPORT="$EVIDENCE_DIR/native-mobile-readiness.json"
WINDOW_REPORT="$EVIDENCE_DIR/native-window-current.json"
BROWSER_LOG="$EVIDENCE_DIR/control-browser-smoke.log"
CURRENT_REPORT="$CANONICAL_CURRENT_REPORT"

MATRIX_LIVE_RECEIPT="$(normalize_future_path "${HEPTA_UI_MATRIX_LIVE_RECEIPT:-$EVIDENCE_DIR/matrix-live.json}")"
BRIDGE_LIVE_RECEIPT="$(normalize_future_path "${HEPTA_UI_BRIDGE_LIVE_RECEIPT:-$EVIDENCE_DIR/hepta-bridge-live.json}")"
DEVICE_RECEIPT="$(normalize_future_path "${HEPTA_UI_DEVICE_LAB_RECEIPT:-$EVIDENCE_DIR/device-lab.json}")"
ACCESSIBILITY_RECEIPT="$(normalize_future_path "${HEPTA_UI_ACCESSIBILITY_RECEIPT:-$EVIDENCE_DIR/accessibility.json}")"
RELEASE_RECEIPT="$(normalize_future_path "${HEPTA_UI_RELEASE_RECEIPT:-$EVIDENCE_DIR/release.json}")"

writable_paths=(
  "$BINDING_BEFORE" "$BINDING_AFTER" "$BINDING_FINAL" "$SYNC_REPORT" "$PRODUCT_REPORT"
  "$TOKEN_REPORT" "$FEATURE_REPORT" "$PACKAGE_REPORT" "$BROWSER_REPORT"
  "$MOBILE_REPORT" "$WINDOW_REPORT" "$BROWSER_LOG" "$CURRENT_REPORT" "$REPORT_PATH"
)
for writable_path in "${writable_paths[@]}"; do
  if [[ -L "$writable_path" || ( -e "$writable_path" && ! -f "$writable_path" ) ]]; then
    echo "current-readiness output path must be a regular file and not a symlink: $writable_path" >&2
    exit 64
  fi
done
for input_path in "$MATRIX_LIVE_RECEIPT" "$BRIDGE_LIVE_RECEIPT" "$DEVICE_RECEIPT" "$ACCESSIBILITY_RECEIPT" "$RELEASE_RECEIPT"; do
  for writable_path in "${writable_paths[@]}"; do
    if [[ "$input_path" == "$writable_path" ]]; then
      echo "current-readiness input receipt collides with an output path: $input_path" >&2
      exit 64
    fi
  done
done
for child_report in "$SYNC_REPORT" "$PRODUCT_REPORT" "$TOKEN_REPORT" "$FEATURE_REPORT" "$PACKAGE_REPORT" "$BROWSER_REPORT" "$MOBILE_REPORT" "$WINDOW_REPORT"; do
  if [[ "$REPORT_PATH" == "$child_report" ]]; then
    echo "--output must not replace a child receipt: $child_report" >&2
    exit 64
  fi
done

# Evidence directories are reusable, but child receipts never are. Atomically
# replace each receipt with a fail-closed sentinel before invoking a producer;
# this neither follows a pre-positioned symlink nor exposes a prior ready row.
for current_run_report in "$SYNC_REPORT" "$PRODUCT_REPORT" "$TOKEN_REPORT" "$FEATURE_REPORT" "$PACKAGE_REPORT" "$BROWSER_REPORT" "$MOBILE_REPORT" "$WINDOW_REPORT"; do
  sentinel="$(mktemp "$EVIDENCE_DIR/.current-run-receipt.XXXXXX")"
  printf '{"schema_version":0,"status":"not_ready","blockers":["current_run_producer_not_completed"]}\n' >"$sentinel"
  mv -f -- "$sentinel" "$current_run_report"
done

binding_tmp="$(mktemp "$EVIDENCE_DIR/.source-binding-before.XXXXXX")"
scripts/hepta-ui-source-fingerprint >"$binding_tmp"
mv -f -- "$binding_tmp" "$BINDING_BEFORE"

sync_rc=0
scripts/hepta-native-robrix-upstream-sync-check-v2.sh --json --strict >"$SYNC_REPORT" || sync_rc=$?
product_rc=0
scripts/hepta-native-product-shell-gate-v2.sh --json --output "$PRODUCT_REPORT" || product_rc=$?
token_rc=0
scripts/hepta-ui-light-glass-token-sync.rb --check >"$TOKEN_REPORT" || token_rc=$?

mobile_rc=0
scripts/hepta-native-mobile-readiness-gate.sh --output "$MOBILE_REPORT" >/dev/null || mobile_rc=$?

feature_rc=0
if [[ "$VERIFY_FEATURES" == "1" ]]; then
  scripts/hepta-native-feature-matrix-gate.sh --output "$FEATURE_REPORT" --target-dir "$TARGET_DIR" || feature_rc=$?
else
  jq -n '{schema_version:1,kind:"hepta-native-feature-matrix-gate",status:"not_run",feature_matrix_ready:false,blockers:["feature_matrix_not_run_for_current_source"]}' >"$FEATURE_REPORT"
fi

package_rc=0
package_args=(--output "$PACKAGE_REPORT" --target-dir "$TARGET_DIR")
if [[ "$VERIFY_PACKAGE" == "1" ]]; then package_args+=(--build --stage-dir "$EVIDENCE_DIR/native-current-package"); fi
scripts/hepta-native-current-package-gate.sh "${package_args[@]}" || package_rc=$?

window_rc=125
window_verifier_executed=false
if [[ "$VERIFY_WINDOW" == "1" ]]; then
  window_rc=0
  window_verifier_executed=true
  scripts/hepta-ui-native-window-verifier-v1 \
    --package-report "$PACKAGE_REPORT" \
    --run-nonce "$RUN_NONCE" \
    --evidence-dir "$EVIDENCE_DIR/native-window" \
    --output "$WINDOW_REPORT" || window_rc=$?
fi

browser_rc=0
if [[ "$VERIFY_BROWSER" == "1" ]]; then
  browser_log_tmp="$(mktemp "$EVIDENCE_DIR/.control-browser-smoke-log.XXXXXX")"
  HEPTA_CONTROL_UI_BROWSER_SMOKE_REPORT_PATH="$BROWSER_REPORT" \
    scripts/hepta-control-ui-browser-smoke.sh >"$browser_log_tmp" 2>&1 || browser_rc=$?
  mv -f -- "$browser_log_tmp" "$BROWSER_LOG"
else
  jq -n '{schema_version:1,kind:"hepta-control-ui-browser-smoke-current-wrapper",status:"not_run",browser_smoke_ready:false,blockers:["control_browser_smoke_not_run_for_current_source"]}' >"$BROWSER_REPORT"
fi

normalize_child_report() {
  local path="$1" schema="$2" kind="$3" reason="$4"
  if ! jq -e 'type == "object"' "$path" >/dev/null 2>&1; then
    jq -n --argjson schema "$schema" --arg kind "$kind" --arg reason "$reason" \
      '{schema_version:$schema,kind:$kind,status:"not_ready",source_stable_during_run:false,normalized_failure:true,blockers:[$reason]}' >"$path"
  fi
}
normalize_child_report "$SYNC_REPORT" 3 hepta-native-robrix-upstream-sync-check upstream_sync_report_invalid
normalize_child_report "$PRODUCT_REPORT" 3 hepta-native-upstream-first-product-shell-gate native_product_report_invalid
normalize_child_report "$TOKEN_REPORT" 2 hepta-ui-light-glass-token-sync token_sync_report_invalid
normalize_child_report "$FEATURE_REPORT" 1 hepta-native-feature-matrix-gate native_feature_report_invalid
normalize_child_report "$PACKAGE_REPORT" 1 hepta-native-current-package-gate native_package_report_invalid
normalize_child_report "$BROWSER_REPORT" 1 hepta-control-ui-browser-smoke-current-wrapper control_browser_report_invalid
normalize_child_report "$MOBILE_REPORT" 1 hepta-native-mobile-readiness-gate native_mobile_report_invalid

binding_tmp="$(mktemp "$EVIDENCE_DIR/.source-binding-after.XXXXXX")"
scripts/hepta-ui-source-fingerprint >"$binding_tmp"
mv -f -- "$binding_tmp" "$BINDING_AFTER"
source_fingerprint="$(jq -r '.source_fingerprint' "$BINDING_AFTER")"
source_head="$(jq -r '.head' "$BINDING_AFTER")"
source_tree="$(jq -r '.head_tree' "$BINDING_AFTER")"

if [[ "$VERIFY_BROWSER" == "1" && -s "$BROWSER_REPORT" ]]; then
  tmp_browser="$(mktemp "$EVIDENCE_DIR/.control-browser-smoke.bound.XXXXXX")"
  browser_original_valid=false
  if [[ "$browser_rc" == "0" ]] && jq -e '
      .schema_version == 1
      and .kind == "hepta-browser-visual-smoke"
      and .producer == "scripts/hepta-browser-visual-smoke.sh"
      and .status == "ready"
      and .control_ui_browser_error_page_absent == true
      and .control_ui_horizontal_overflow_free == true
      and .control_ui_320_reflow_ready == true
      and .control_ui_native_popover_interaction_ready == true
    ' "$BROWSER_REPORT" >/dev/null 2>&1; then
    browser_original_valid=true
  fi
  if [[ "$browser_original_valid" == "true" ]]; then
    jq --slurpfile binding "$BINDING_AFTER" \
      '. + {schema_version:1,kind:"hepta-control-ui-browser-smoke-current-wrapper",producer:"scripts/hepta-ui-current-readiness.sh",original_receipt:{schema_version:1,kind:"hepta-browser-visual-smoke",producer:"scripts/hepta-browser-visual-smoke.sh"},original_receipt_valid:true,browser_child_exit_code:0,source_binding:$binding[0],browser_smoke_ready:true}' \
      "$BROWSER_REPORT" >"$tmp_browser"
  else
    jq -n --slurpfile binding "$BINDING_AFTER" --argjson exit_code "$browser_rc" \
      '{schema_version:1,kind:"hepta-control-ui-browser-smoke-current-wrapper",producer:"scripts/hepta-ui-current-readiness.sh",status:"not_ready",original_receipt_valid:false,browser_child_exit_code:$exit_code,source_binding:$binding[0],browser_smoke_ready:false,blockers:["browser_child_receipt_identity_or_contract_invalid"]}' >"$tmp_browser"
  fi
  mv "$tmp_browser" "$BROWSER_REPORT"
fi

binding_stable=false
if [[ "$(jq -r '.head' "$BINDING_BEFORE")" == "$source_head" \
  && "$(jq -r '.head_tree' "$BINDING_BEFORE")" == "$source_tree" \
  && "$(jq -r '.source_fingerprint' "$BINDING_BEFORE")" == "$source_fingerprint" ]]; then binding_stable=true; fi

gate_bound() {
  local path="$1" expected_schema="$2" expected_kind="$3"
  jq -e --arg head "$source_head" --arg tree "$source_tree" --arg fingerprint "$source_fingerprint" \
    --argjson schema "$expected_schema" --arg kind "$expected_kind" '
      .schema_version == $schema
      and .kind == $kind
      and .source_stable_during_run == true
      and .source_binding.head == $head
      and .source_binding.head_tree == $tree
      and .source_binding.source_fingerprint == $fingerprint
    ' "$path" >/dev/null 2>&1
}

sync_bound=false; gate_bound "$SYNC_REPORT" 3 hepta-native-robrix-upstream-sync-check && sync_bound=true
product_bound=false; gate_bound "$PRODUCT_REPORT" 3 hepta-native-upstream-first-product-shell-gate && product_bound=true
token_bound=false; gate_bound "$TOKEN_REPORT" 2 hepta-ui-light-glass-token-sync && token_bound=true
feature_bound=false; gate_bound "$FEATURE_REPORT" 1 hepta-native-feature-matrix-gate && feature_bound=true
package_bound=false; gate_bound "$PACKAGE_REPORT" 1 hepta-native-current-package-gate && package_bound=true
mobile_bound=false; gate_bound "$MOBILE_REPORT" 1 hepta-native-mobile-readiness-gate && mobile_bound=true

receipt_json() {
  local name="$1"
  local path="$2"
  local capability="$3"
  local expected_kind="$4"
  local expected_producer="$5"
  if [[ -L "$path" ]]; then
    jq -n --arg name "$name" --arg path "$path" --arg capability "$capability" \
      '{name:$name,path:$path,present:true,bound_to_current_source:false,capability:$capability,ready:false,reason:"receipt_symlink_rejected"}'
    return
  fi
  if [[ ! -s "$path" ]]; then
    jq -n --arg name "$name" --arg path "$path" --arg capability "$capability" \
      '{name:$name,path:$path,present:false,bound_to_current_source:false,capability:$capability,ready:false,reason:"receipt_missing"}'
    return
  fi
  if ! jq -e . "$path" >/dev/null 2>&1; then
    jq -n --arg name "$name" --arg path "$path" --arg capability "$capability" \
      '{name:$name,path:$path,present:true,bound_to_current_source:false,capability:$capability,ready:false,reason:"receipt_invalid_json"}'
    return
  fi
  local artifact_path expected_sha actual_sha="" artifact_hash_valid=false
  artifact_path="$(jq -r '.artifact.path // ""' "$path")"
  expected_sha="$(jq -r '.artifact.sha256 // ""' "$path")"
  if [[ -f "$artifact_path" && ! -L "$artifact_path" && "$expected_sha" =~ ^[0-9a-f]{64}$ ]]; then
    actual_sha="$(shasum -a 256 "$artifact_path" | awk '{print $1}')"
    [[ "$actual_sha" == "$expected_sha" ]] && artifact_hash_valid=true
  fi
  jq -n --arg name "$name" --arg path "$path" --arg capability "$capability" --arg head "$source_head" \
    --arg tree "$source_tree" --arg fingerprint "$source_fingerprint" --arg expected_kind "$expected_kind" \
    --arg expected_producer "$expected_producer" --arg artifact_path "$artifact_path" --arg expected_sha "$expected_sha" \
    --arg actual_sha "$actual_sha" --argjson artifact_hash_valid "$artifact_hash_valid" --slurpfile receipt "$path" '
      ($receipt[0]) as $r |
      (($r.schema_version // null) == 1 and ($r.kind // "") == $expected_kind and ($r.producer // "") == $expected_producer) as $identity |
      (($r.source_binding.head // "") == $head and ($r.source_binding.head_tree // "") == $tree and ($r.source_binding.source_fingerprint // "") == $fingerprint) as $bound |
      {name:$name,path:$path,present:true,schema_and_producer_valid:$identity,bound_to_current_source:$bound,
       artifact:{path:$artifact_path,expected_sha256:$expected_sha,actual_sha256:$actual_sha,hash_valid:$artifact_hash_valid},
       capability:$capability,ready:($identity and $bound and $artifact_hash_valid and ($r.status == "ready") and (($r[$capability] // false) == true)),
       reported_status:($r.status // "unknown"),source_stable_during_run:($r.source_stable_during_run // false),
       independent_verifier_ready:($r.independent_promotion_verifier_ready // false),package:($r.package // null),
       run_nonce:($r.run_nonce // null),host_window:($r.host_window // null),automation:($r.automation // null),isolation:($r.isolation // null),
       scope:($r.scope // null),signed:false,notarized:false,stapled:false}'
}

NATIVE_WINDOW_RECEIPT="$WINDOW_REPORT"

matrix_receipt="$(receipt_json matrix_live "$MATRIX_LIVE_RECEIPT" matrix_live_ready hepta-ui-matrix-live-receipt-v1 scripts/hepta-ui-matrix-live-verifier-v1)"
bridge_receipt="$(receipt_json hepta_bridge_live "$BRIDGE_LIVE_RECEIPT" hepta_live_bridge_ready hepta-ui-bridge-live-receipt-v1 scripts/hepta-ui-bridge-live-verifier-v1)"
window_receipt="$(receipt_json native_window "$NATIVE_WINDOW_RECEIPT" native_window_ready hepta-ui-native-window-receipt-v1 scripts/hepta-ui-native-window-verifier-v1)"
device_receipt="$(receipt_json device_lab "$DEVICE_RECEIPT" real_device_lab_ready hepta-ui-device-lab-receipt-v1 scripts/hepta-ui-device-lab-verifier-v1)"
accessibility_receipt="$(receipt_json accessibility "$ACCESSIBILITY_RECEIPT" accessibility_ready hepta-ui-accessibility-receipt-v1 scripts/hepta-ui-accessibility-verifier-v1)"
release_receipt="$(receipt_json release "$RELEASE_RECEIPT" public_distribution_ready hepta-ui-release-receipt-v1 scripts/hepta-ui-release-verifier-v1)"
package_report_sha256="$(shasum -a 256 "$PACKAGE_REPORT" | awk '{print $1}')"
package_app_path="$(jq -r '.artifact.path // ""' "$PACKAGE_REPORT")"
expected_package_app_path="$EVIDENCE_DIR/native-current-package/Hepta.app"
package_binary_path="$package_app_path/Contents/MacOS/hepta-native"
package_binary_actual_sha256=""
package_bundle_actual_sha256=""
package_artifact_hash_valid=false
if [[ -d "$package_app_path" && -x "$package_binary_path" ]]; then
  package_binary_actual_sha256="$(shasum -a 256 "$package_binary_path" | awk '{print $1}')"
  package_bundle_actual_sha256="$(scripts/hepta-ui-bundle-fingerprint --root "$package_app_path")"
  if [[ "$package_binary_actual_sha256" == "$(jq -r '.artifact.binary_sha256 // ""' "$PACKAGE_REPORT")" ]]; then
    package_artifact_hash_valid=true
  fi
fi

report="$(jq -L "$ROOT_DIR/scripts/lib" -n \
  --arg generated_at_utc "$(date -u +%Y-%m-%dT%H:%M:%SZ)" --arg require "$REQUIRE" --arg evidence_dir "$EVIDENCE_DIR" \
  --argjson binding_before "$(cat "$BINDING_BEFORE")" --argjson binding_after "$(cat "$BINDING_AFTER")" \
  --argjson binding_stable "$binding_stable" --argjson sync "$(cat "$SYNC_REPORT")" --argjson product "$(cat "$PRODUCT_REPORT")" \
  --argjson tokens "$(cat "$TOKEN_REPORT" 2>/dev/null || echo '{"status":"not_ready"}')" \
  --argjson feature "$(cat "$FEATURE_REPORT")" --argjson package "$(cat "$PACKAGE_REPORT")" --argjson mobile "$(cat "$MOBILE_REPORT")" --slurpfile browser_file "$BROWSER_REPORT" \
  --argjson matrix_receipt "$matrix_receipt" --argjson bridge_receipt "$bridge_receipt" --argjson window_receipt "$window_receipt" \
  --argjson device_receipt "$device_receipt" --argjson accessibility_receipt "$accessibility_receipt" --argjson release_receipt "$release_receipt" \
  --argjson sync_bound "$sync_bound" --argjson product_bound "$product_bound" --argjson token_bound "$token_bound" \
  --argjson feature_bound "$feature_bound" --argjson package_bound "$package_bound" --argjson mobile_bound "$mobile_bound" \
  --argjson sync_exit_code "$sync_rc" --argjson product_exit_code "$product_rc" --argjson token_exit_code "$token_rc" \
  --argjson feature_exit_code "$feature_rc" --argjson package_exit_code "$package_rc" --argjson browser_exit_code "$browser_rc" --argjson mobile_exit_code "$mobile_rc" \
  --argjson window_exit_code "$window_rc" --argjson window_verifier_executed "$window_verifier_executed" \
  --arg run_nonce "$RUN_NONCE" --arg package_report_path "$PACKAGE_REPORT" --arg package_report_sha256 "$package_report_sha256" \
  --arg expected_package_app_path "$expected_package_app_path" \
  --arg package_binary_path "$package_binary_path" --arg package_binary_actual_sha256 "$package_binary_actual_sha256" \
  --arg package_bundle_actual_sha256 "$package_bundle_actual_sha256" --argjson package_artifact_hash_valid "$package_artifact_hash_valid" '
    include "hepta-ui-current-readiness-v1";
    ($browser_file[0]) as $browser |
    {
      binding_stable:$binding_stable,binding_after:$binding_after,
      sync_exit_code:$sync_exit_code,product_exit_code:$product_exit_code,token_exit_code:$token_exit_code,
      feature_exit_code:$feature_exit_code,package_exit_code:$package_exit_code,mobile_exit_code:$mobile_exit_code,
      sync_bound:$sync_bound,product_bound:$product_bound,token_bound:$token_bound,
      feature_bound:$feature_bound,package_bound:$package_bound,mobile_bound:$mobile_bound,
      sync:$sync,product:$product,tokens:$tokens,feature:$feature,package:$package,mobile:$mobile,browser:$browser,
      window_verifier_executed:$window_verifier_executed,window_exit_code:$window_exit_code,
      window_receipt:$window_receipt,run_nonce:$run_nonce,
      package_report_path:$package_report_path,package_report_sha256:$package_report_sha256,
      expected_package_app_path:$expected_package_app_path,package_binary_path:$package_binary_path,
      package_binary_actual_sha256:$package_binary_actual_sha256,
      package_bundle_actual_sha256:$package_bundle_actual_sha256,
      package_artifact_hash_valid:$package_artifact_hash_valid
    } as $truth_context |
    ($truth_context | hepta_ui_readiness_truth) as $truth |
    $truth.source as $source_ready |
    $truth.browser as $browser_ready |
    $truth.promotion as $promotion_independent_verifiers_ready |
    $truth.local as $local_ready |
    false as $full_ready |
    false as $release_independent_verification_ready |
    ($full_ready and $release_receipt.ready and $release_independent_verification_ready) as $ga_ready |
    ({none:true,source:$source_ready,local:$local_ready,full:$full_ready,ga:$ga_ready}[$require]) as $required_ready |
    {
      schema_version:1,
      kind:"hepta-ui-current-readiness",
      generated_at_utc:$generated_at_utc,
      status:(if $require == "none" then "report_complete" elif $required_ready then "ready" else "not_ready" end),
      report_only:($require == "none"),
      required_level:$require,
      evidence_dir:$evidence_dir,
      historical_evidence_consumed:false,
      source_binding:$binding_after,
      source_stable_during_run:$binding_stable,
      current_head_active_truth_ready:($binding_stable and $binding_after.repository_worktree_clean),
      readiness:{source:$source_ready,local_demo:$local_ready,full_product:$full_ready,public_ga:$ga_ready},
      gates:{
        upstream_sync:{status:$sync.status,exit_code:$sync_exit_code,bound_to_current_source:$sync_bound,path_ledger_ready:($sync.path_ledger_ready // false),report:"native-upstream-sync.json"},
        native_product_shell:{status:$product.status,exit_code:$product_exit_code,bound_to_current_source:$product_bound,ready:($product.native_ui_ready // false),report:"native-product-shell.json"},
        token_sync:{status:($tokens.status // "not_ready"),exit_code:$token_exit_code,bound_to_current_source:$token_bound,schema_version:($tokens.schema_version // null),report:"token-sync.json"},
        native_feature_matrix:{status:$feature.status,exit_code:$feature_exit_code,bound_to_current_source:$feature_bound,ready:($feature.feature_matrix_ready // false),report:"native-feature-matrix.json"},
        native_package:{status:$package.status,exit_code:$package_exit_code,bound_to_current_source:$package_bound,static_ready:($package.static_package_contract_ready // false),local_package_ready:($package.local_package_ready // false),report:"native-current-package.json"},
        native_window:{status:$window_receipt.reported_status,verifier_executed:$window_verifier_executed,exit_code:$window_exit_code,bound_to_current_source:$window_receipt.bound_to_current_source,receipt_ready:$window_receipt.ready,independent_promotion_ready:$promotion_independent_verifiers_ready,scope:$window_receipt.scope,exact_packaged_executable_verified:(($window_receipt.package.visual_capture_binary_is_exact_packaged_executable // false) == true),local_host_forced:(($window_receipt.automation.no_remote // false) == true and ($window_receipt.automation.host_kind // "") == "local"),package_artifact_hash_valid:$package_artifact_hash_valid,report:"native-window-current.json"},
        native_mobile:{status:$mobile.status,exit_code:$mobile_exit_code,bound_to_current_source:$mobile_bound,source_contract_ready:($mobile.mobile_source_contract_ready // false),android_unauthenticated_login_surface:{visual_verified:($mobile.hard_boundaries.android_emulator_unauthenticated_login_surface_visual_verified // false),rotation_verified:($mobile.hard_boundaries.android_emulator_unauthenticated_login_surface_rotation_verified // false),ime_verified:($mobile.hard_boundaries.android_emulator_unauthenticated_login_surface_ime_verified // false)},generic_android_visual_rotation_ime_claims_hard_false:(($mobile.hard_boundaries.android_emulator_visual_verified // false) == false and ($mobile.hard_boundaries.android_emulator_rotation_verified // false) == false and ($mobile.hard_boundaries.android_emulator_ime_verified // false) == false),full_product_ready:($mobile.hard_boundaries.mobile_full_product_ready // false),report:"native-mobile-readiness.json"},
        control_browser:{status:($browser.status // "not_run"),exit_code:$browser_exit_code,ready:$browser_ready,report:"control-browser-smoke.json"}
      },
      promotion_receipts:[$window_receipt,$matrix_receipt,$bridge_receipt,$device_receipt,$accessibility_receipt,$release_receipt],
      hard_boundaries:{promotion_independent_verifiers_ready:$promotion_independent_verifiers_ready,matrix_live_ready:false,hepta_live_bridge_ready:false,real_device_lab_ready:false,accessibility_ready:false,ios_accessibility_update_consumed:($mobile.hard_boundaries.ios_accessibility_update_consumed // false),android_accessibility_update_consumed:($mobile.hard_boundaries.android_accessibility_update_consumed // false),android_secure_session_persistence_ready:($mobile.hard_boundaries.android_secure_session_persistence_ready // false),android_emulator_visual_verified:false,android_emulator_rotation_verified:false,android_emulator_ime_verified:false,mobile_full_product_ready:($mobile.hard_boundaries.mobile_full_product_ready // false),release_independent_verification_ready:$release_independent_verification_ready,signed:false,notarized:false,stapled:false,public_distribution_ready:false},
      external_side_effects_performed:false,
      blockers:([if $binding_stable then empty else "source_changed_during_gate" end,if $binding_after.repository_worktree_clean then empty else "repository_worktree_dirty" end,if $sync_exit_code == 0 then empty else "upstream_sync_child_failed" end,if $product_exit_code == 0 then empty else "native_product_child_failed" end,if $token_exit_code == 0 then empty else "token_sync_child_failed" end,if $feature_exit_code == 0 then empty else "feature_matrix_child_failed" end,if (($package.build_requested // false) == false or $package_exit_code == 0) then empty else "native_package_child_failed" end,if $mobile_exit_code == 0 then empty else "native_mobile_child_failed" end,if $window_exit_code == 0 then empty else "native_window_verifier_failed" end,if $sync_bound then empty else "upstream_sync_receipt_not_bound" end,if $product_bound then empty else "native_product_receipt_not_bound" end,if $token_bound then empty else "token_receipt_not_bound" end,if $feature_bound then empty else "feature_receipt_not_bound" end,if $package_bound then empty else "package_receipt_not_bound" end,if $mobile_bound then empty else "native_mobile_receipt_not_bound" end,if $sync.status == "ready" then empty else "upstream_sync_or_path_ledger_not_ready" end,if $product.status == "ready" then empty else "native_product_shell_not_ready" end,if $tokens.status == "ready" then empty else "token_sync_not_ready" end,if $feature.feature_matrix_ready == true then empty else "native_feature_matrix_not_ready" end,if (($package.build_requested // false) == false or $package.status == "ready") then empty else "native_package_status_not_ready" end,if $package.static_package_contract_ready == true then empty else "package_metadata_not_ready" end,if $mobile.mobile_source_contract_ready == true then empty else "native_mobile_source_contract_not_ready" end,if $package.local_package_ready == true then empty else "current_source_local_package_not_ready" end,if $browser_ready then empty else "control_browser_current_receipt_not_ready" end,if $promotion_independent_verifiers_ready then empty else "native_window_independent_promotion_not_ready" end,if $window_receipt.ready then empty else "native_window_current_receipt_not_ready" end,if $matrix_receipt.ready then empty else "matrix_live_not_ready" end,if $bridge_receipt.ready then empty else "hepta_live_bridge_not_ready" end,if $device_receipt.ready then empty else "real_device_lab_not_ready" end,if $accessibility_receipt.ready then empty else "accessibility_not_ready" end,"pinned_makepad_mobile_accessibility_backend_not_implemented","android_secure_credential_backend_not_supported","independent_release_verifier_not_implemented",if $release_receipt.ready then empty else "public_release_not_ready" end])
    }')"

write_report_atomically() {
  local destination="$1" temporary
  temporary="$(mktemp "$(dirname "$destination")/.hepta-current-readiness.XXXXXX")"
  printf '%s\n' "$report" >"$temporary"
  mv -f -- "$temporary" "$destination"
}

# The child receipts and report assembly above can be expensive. Re-capture the
# source at the publication boundary so a concurrent HEAD/tree/source change
# cannot publish a stale ready receipt merely because the earlier snapshot was
# clean. The final receipt remains structured and fail-closed.
binding_tmp="$(mktemp "$EVIDENCE_DIR/.source-binding-final.XXXXXX")"
scripts/hepta-ui-source-fingerprint >"$binding_tmp"
mv -f -- "$binding_tmp" "$BINDING_FINAL"
publish_source_stable=false
if jq -e --slurpfile evaluated "$BINDING_AFTER" '
    .head == $evaluated[0].head
    and .head_tree == $evaluated[0].head_tree
    and .source_fingerprint == $evaluated[0].source_fingerprint
    and .repository_worktree_clean == true
    and $evaluated[0].repository_worktree_clean == true
  ' "$BINDING_FINAL" >/dev/null; then
  publish_source_stable=true
fi
report="$(jq -c \
  --slurpfile final_binding "$BINDING_FINAL" \
  --argjson publish_source_stable "$publish_source_stable" '
    . + {
      source_binding_at_publish:$final_binding[0],
      source_publish_boundary_stable:$publish_source_stable
    }
    | if $publish_source_stable then . else
        .source_binding = $final_binding[0]
        | .source_stable_during_run = false
        | .current_head_active_truth_ready = false
        | .readiness.source = false
        | .readiness.local_demo = false
        | .readiness.full_product = false
        | .readiness.public_ga = false
        | .status = (if .report_only then "report_complete" else "not_ready" end)
        | .blockers = ((.blockers + ["source_changed_or_became_dirty_before_atomic_publish"]) | unique)
      end
  ' <<<"$report")"
write_report_atomically "$REPORT_PATH"
if [[ "$REPORT_PATH" != "$CURRENT_REPORT" ]]; then write_report_atomically "$CURRENT_REPORT"; fi
printf '%s\n' "$report"
[[ "$REQUIRE" == "none" ]] && exit 0
jq -e '.status == "ready"' <<<"$report" >/dev/null
