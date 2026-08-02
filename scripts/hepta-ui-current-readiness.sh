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
TARGET_DIR="${HEPTA_NATIVE_CARGO_TARGET_DIR:-${TMPDIR:-/tmp}/hepta-ui-current-readiness-target}"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --evidence-dir) EVIDENCE_DIR="${2:-}"; shift 2 ;;
    --output) REPORT_PATH="${2:-}"; shift 2 ;;
    --verify) VERIFY_FEATURES=1; VERIFY_PACKAGE=1; shift ;;
    --verify-features) VERIFY_FEATURES=1; shift ;;
    --verify-package) VERIFY_PACKAGE=1; shift ;;
    --verify-browser) VERIFY_BROWSER=1; shift ;;
    --target-dir) TARGET_DIR="${2:-}"; shift 2 ;;
    --require) REQUIRE="${2:-}"; shift 2 ;;
    --help|-h)
      cat <<'EOF'
usage: scripts/hepta-ui-current-readiness.sh [options]

  --evidence-dir DIR       write all current-run receipts under DIR
  --output FILE            write the orchestrator report (default stdout)
  --verify                 run Native feature matrix and local unsigned package
  --verify-features        run no-default/default/all-features Cargo checks
  --verify-package         build and stage a current-source unsigned Hepta.app
  --verify-browser         run the Control four-viewport browser smoke
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
if [[ -z "$EVIDENCE_DIR" ]]; then EVIDENCE_DIR="$(mktemp -d "${TMPDIR:-/tmp}/hepta-ui-current-readiness.XXXXXX")"; fi
mkdir -p "$EVIDENCE_DIR" "$TARGET_DIR"
EVIDENCE_DIR="$(cd "$EVIDENCE_DIR" && pwd -P)"
if [[ -z "$REPORT_PATH" ]]; then REPORT_PATH="$EVIDENCE_DIR/current-readiness.json"; fi
mkdir -p "$(dirname "$REPORT_PATH")"

for command in git jq shasum; do command -v "$command" >/dev/null 2>&1 || { echo "$command is required" >&2; exit 2; }; done

BINDING_BEFORE="$EVIDENCE_DIR/source-binding-before.json"
BINDING_AFTER="$EVIDENCE_DIR/source-binding-after.json"
SYNC_REPORT="$EVIDENCE_DIR/native-upstream-sync.json"
PRODUCT_REPORT="$EVIDENCE_DIR/native-product-shell.json"
TOKEN_REPORT="$EVIDENCE_DIR/token-sync.json"
FEATURE_REPORT="$EVIDENCE_DIR/native-feature-matrix.json"
PACKAGE_REPORT="$EVIDENCE_DIR/native-current-package.json"
BROWSER_REPORT="$EVIDENCE_DIR/control-browser-smoke.json"

# Evidence directories are reusable, but child receipts never are. Truncate
# every current-run output before invoking a producer so a crash cannot expose
# a prior ready receipt at the same path.
for current_run_report in "$SYNC_REPORT" "$PRODUCT_REPORT" "$TOKEN_REPORT" "$FEATURE_REPORT" "$PACKAGE_REPORT" "$BROWSER_REPORT"; do
  : >"$current_run_report"
done

scripts/hepta-ui-source-fingerprint >"$BINDING_BEFORE"

sync_rc=0
scripts/hepta-native-robrix-upstream-sync-check-v2.sh --json --strict >"$SYNC_REPORT" || sync_rc=$?
product_rc=0
scripts/hepta-native-product-shell-gate-v2.sh --json --output "$PRODUCT_REPORT" || product_rc=$?
token_rc=0
scripts/hepta-ui-light-glass-token-sync.rb --check >"$TOKEN_REPORT" || token_rc=$?

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

browser_rc=0
if [[ "$VERIFY_BROWSER" == "1" ]]; then
  HEPTA_CONTROL_UI_BROWSER_SMOKE_REPORT_PATH="$BROWSER_REPORT" \
    scripts/hepta-control-ui-browser-smoke.sh >"$EVIDENCE_DIR/control-browser-smoke.log" 2>&1 || browser_rc=$?
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

scripts/hepta-ui-source-fingerprint >"$BINDING_AFTER"
source_fingerprint="$(jq -r '.source_fingerprint' "$BINDING_AFTER")"
source_head="$(jq -r '.head' "$BINDING_AFTER")"
source_tree="$(jq -r '.head_tree' "$BINDING_AFTER")"

if [[ "$VERIFY_BROWSER" == "1" && -s "$BROWSER_REPORT" ]]; then
  tmp_browser="$EVIDENCE_DIR/.control-browser-smoke.bound.$$"
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

receipt_json() {
  local name="$1"
  local path="$2"
  local capability="$3"
  local expected_kind="$4"
  local expected_producer="$5"
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
  if [[ -f "$artifact_path" && "$expected_sha" =~ ^[0-9a-f]{64}$ ]]; then
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
       reported_status:($r.status // "unknown"),signed:false,notarized:false,stapled:false}'
}

MATRIX_LIVE_RECEIPT="${HEPTA_UI_MATRIX_LIVE_RECEIPT:-$EVIDENCE_DIR/matrix-live.json}"
BRIDGE_LIVE_RECEIPT="${HEPTA_UI_BRIDGE_LIVE_RECEIPT:-$EVIDENCE_DIR/hepta-bridge-live.json}"
NATIVE_WINDOW_RECEIPT="${HEPTA_UI_NATIVE_WINDOW_RECEIPT:-$EVIDENCE_DIR/native-window-current.json}"
DEVICE_RECEIPT="${HEPTA_UI_DEVICE_LAB_RECEIPT:-$EVIDENCE_DIR/device-lab.json}"
ACCESSIBILITY_RECEIPT="${HEPTA_UI_ACCESSIBILITY_RECEIPT:-$EVIDENCE_DIR/accessibility.json}"
RELEASE_RECEIPT="${HEPTA_UI_RELEASE_RECEIPT:-$EVIDENCE_DIR/release.json}"

matrix_receipt="$(receipt_json matrix_live "$MATRIX_LIVE_RECEIPT" matrix_live_ready hepta-ui-matrix-live-receipt-v1 scripts/hepta-ui-matrix-live-verifier-v1)"
bridge_receipt="$(receipt_json hepta_bridge_live "$BRIDGE_LIVE_RECEIPT" hepta_live_bridge_ready hepta-ui-bridge-live-receipt-v1 scripts/hepta-ui-bridge-live-verifier-v1)"
window_receipt="$(receipt_json native_window "$NATIVE_WINDOW_RECEIPT" native_window_ready hepta-ui-native-window-receipt-v1 scripts/hepta-ui-native-window-verifier-v1)"
device_receipt="$(receipt_json device_lab "$DEVICE_RECEIPT" real_device_lab_ready hepta-ui-device-lab-receipt-v1 scripts/hepta-ui-device-lab-verifier-v1)"
accessibility_receipt="$(receipt_json accessibility "$ACCESSIBILITY_RECEIPT" accessibility_ready hepta-ui-accessibility-receipt-v1 scripts/hepta-ui-accessibility-verifier-v1)"
release_receipt="$(receipt_json release "$RELEASE_RECEIPT" public_distribution_ready hepta-ui-release-receipt-v1 scripts/hepta-ui-release-verifier-v1)"

report="$(jq -n \
  --arg generated_at_utc "$(date -u +%Y-%m-%dT%H:%M:%SZ)" --arg require "$REQUIRE" --arg evidence_dir "$EVIDENCE_DIR" \
  --argjson binding_before "$(cat "$BINDING_BEFORE")" --argjson binding_after "$(cat "$BINDING_AFTER")" \
  --argjson binding_stable "$binding_stable" --argjson sync "$(cat "$SYNC_REPORT")" --argjson product "$(cat "$PRODUCT_REPORT")" \
  --argjson tokens "$(cat "$TOKEN_REPORT" 2>/dev/null || echo '{"status":"not_ready"}')" \
  --argjson feature "$(cat "$FEATURE_REPORT")" --argjson package "$(cat "$PACKAGE_REPORT")" --slurpfile browser_file "$BROWSER_REPORT" \
  --argjson matrix_receipt "$matrix_receipt" --argjson bridge_receipt "$bridge_receipt" --argjson window_receipt "$window_receipt" \
  --argjson device_receipt "$device_receipt" --argjson accessibility_receipt "$accessibility_receipt" --argjson release_receipt "$release_receipt" \
  --argjson sync_bound "$sync_bound" --argjson product_bound "$product_bound" --argjson token_bound "$token_bound" \
  --argjson feature_bound "$feature_bound" --argjson package_bound "$package_bound" \
  --argjson sync_exit_code "$sync_rc" --argjson product_exit_code "$product_rc" --argjson token_exit_code "$token_rc" \
  --argjson feature_exit_code "$feature_rc" --argjson package_exit_code "$package_rc" --argjson browser_exit_code "$browser_rc" '
    ($browser_file[0]) as $browser |
    ($binding_stable and $binding_after.repository_worktree_clean and $sync_exit_code == 0 and $product_exit_code == 0 and $token_exit_code == 0 and $feature_exit_code == 0 and $sync_bound and $product_bound and $token_bound and $feature_bound and $package_bound and $sync.status == "ready" and $sync.path_ledger_ready == true and $product.status == "ready" and $tokens.status == "ready" and $feature.feature_matrix_ready == true and $package.static_package_contract_ready == true) as $source_ready |
    (($browser.schema_version // null) == 1 and ($browser.kind // "") == "hepta-control-ui-browser-smoke-current-wrapper" and ($browser.producer // "") == "scripts/hepta-ui-current-readiness.sh" and ($browser.original_receipt_valid // false) == true and ($browser.browser_child_exit_code // -1) == 0 and ($browser.source_binding.head // "") == $binding_after.head and ($browser.source_binding.head_tree // "") == $binding_after.head_tree and ($browser.source_binding.source_fingerprint // "") == $binding_after.source_fingerprint and ($browser.browser_smoke_ready // false) == true) as $browser_ready |
    false as $promotion_independent_verifiers_ready |
    ($source_ready and $package.local_package_ready == true and $browser_ready and $window_receipt.ready and $promotion_independent_verifiers_ready) as $local_ready |
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
        control_browser:{status:($browser.status // "not_run"),exit_code:$browser_exit_code,ready:$browser_ready,report:"control-browser-smoke.json"}
      },
      promotion_receipts:[$window_receipt,$matrix_receipt,$bridge_receipt,$device_receipt,$accessibility_receipt,$release_receipt],
      hard_boundaries:{promotion_independent_verifiers_ready:$promotion_independent_verifiers_ready,matrix_live_ready:false,hepta_live_bridge_ready:false,real_device_lab_ready:false,accessibility_ready:false,release_independent_verification_ready:$release_independent_verification_ready,signed:false,notarized:false,stapled:false,public_distribution_ready:false},
      external_side_effects_performed:false,
      blockers:([if $binding_stable then empty else "source_changed_during_gate" end,if $binding_after.repository_worktree_clean then empty else "repository_worktree_dirty" end,if $sync_exit_code == 0 then empty else "upstream_sync_child_failed" end,if $product_exit_code == 0 then empty else "native_product_child_failed" end,if $token_exit_code == 0 then empty else "token_sync_child_failed" end,if $feature_exit_code == 0 then empty else "feature_matrix_child_failed" end,if $sync_bound then empty else "upstream_sync_receipt_not_bound" end,if $product_bound then empty else "native_product_receipt_not_bound" end,if $token_bound then empty else "token_receipt_not_bound" end,if $feature_bound then empty else "feature_receipt_not_bound" end,if $package_bound then empty else "package_receipt_not_bound" end,if $sync.status == "ready" then empty else "upstream_sync_or_path_ledger_not_ready" end,if $product.status == "ready" then empty else "native_product_shell_not_ready" end,if $tokens.status == "ready" then empty else "token_sync_not_ready" end,if $feature.feature_matrix_ready == true then empty else "native_feature_matrix_not_ready" end,if $package.static_package_contract_ready == true then empty else "package_metadata_not_ready" end,if $package.local_package_ready == true then empty else "current_source_local_package_not_ready" end,if $browser_ready then empty else "control_browser_current_receipt_not_ready" end,"promotion_independent_verifiers_not_implemented",if $window_receipt.ready then empty else "native_window_current_receipt_not_ready" end,if $matrix_receipt.ready then empty else "matrix_live_not_ready" end,if $bridge_receipt.ready then empty else "hepta_live_bridge_not_ready" end,if $device_receipt.ready then empty else "real_device_lab_not_ready" end,if $accessibility_receipt.ready then empty else "accessibility_not_ready" end,"independent_release_verifier_not_implemented",if $release_receipt.ready then empty else "public_release_not_ready" end])
    }')"

printf '%s\n' "$report" >"$REPORT_PATH"
if [[ "$REPORT_PATH" != "$EVIDENCE_DIR/current-readiness.json" ]]; then printf '%s\n' "$report" >"$EVIDENCE_DIR/current-readiness.json"; fi
printf '%s\n' "$report"
[[ "$REQUIRE" == "none" ]] && exit 0
jq -e '.status == "ready"' <<<"$report" >/dev/null
