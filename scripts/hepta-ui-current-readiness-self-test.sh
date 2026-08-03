#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
cd "$ROOT_DIR"
TEST_DIR="$(mktemp -d "${TMPDIR:-/tmp}/hepta-ui-current-readiness-self-test.XXXXXX")"
trap 'rm -rf "$TEST_DIR"' EXIT

# Exercise the exact pure predicates imported by the orchestrator.  The fully
# positive row proves that explicit false values can promote source and local;
# missing/null/true rows prove the same fields remain fail-closed.
jq -n '{
  binding_stable:true,
  binding_after:{repository_worktree_clean:true,head:"head",head_tree:"tree",source_fingerprint:"fingerprint"},
  sync_exit_code:0,product_exit_code:0,token_exit_code:0,feature_exit_code:0,package_exit_code:0,mobile_exit_code:0,
  sync_bound:true,product_bound:true,token_bound:true,feature_bound:true,package_bound:true,mobile_bound:true,
  sync:{status:"ready",path_ledger_ready:true},
  product:{status:"ready"},
  tokens:{status:"ready"},
  feature:{feature_matrix_ready:true},
  package:{status:"ready",static_package_contract_ready:true,local_package_ready:true,artifact:{path:"/evidence/native-current-package/Hepta.app"}},
  mobile:{status:"source_contract_ready",mobile_source_contract_ready:true,hard_boundaries:{android_emulator_visual_verified:false,android_emulator_rotation_verified:false,android_emulator_ime_verified:false}},
  browser:{schema_version:1,kind:"hepta-control-ui-browser-smoke-current-wrapper",producer:"scripts/hepta-ui-current-readiness.sh",original_receipt_valid:true,browser_child_exit_code:0,source_binding:{head:"head",head_tree:"tree",source_fingerprint:"fingerprint"},browser_smoke_ready:true},
  window_verifier_executed:true,window_exit_code:0,run_nonce:"11111111-1111-1111-1111-111111111111",
  window_receipt:{
    ready:true,source_stable_during_run:true,independent_verifier_ready:true,
    scope:"unauthenticated_local_macos_product_shell",run_nonce:"11111111-1111-1111-1111-111111111111",
    package:{current_source_local_package_ready:true,visual_capture_binary_is_exact_packaged_executable:true,visual_capture_binary_is_separate_developer_diagnostics_build:false,report_path:"/evidence/native-current-package.json",report_sha256:"package-report-sha",app_path:"/evidence/native-current-package/Hepta.app",binary_path:"/evidence/native-current-package/Hepta.app/Contents/MacOS/hepta-native",binary_sha256:"binary-sha",bundle_fingerprint_sha256:"bundle-sha"},
    automation:{no_remote:true,host_kind:"local",host_source:"forced_local_services",application_process:{identity_safe_termination_confirmed:true}},
    host_window:{title:"Hepta",exact_title_match_count:1,bounds_within_tolerance:true,minimum_capture_size_ready:true},
    isolation:{home_isolated:true,real_product_data_path_denied:true,real_product_cache_path_denied:true,keychain_services_denied:true,network_denied_by_sandbox:true,force_login_argument:true}
  },
  package_report_path:"/evidence/native-current-package.json",package_report_sha256:"package-report-sha",
  expected_package_app_path:"/evidence/native-current-package/Hepta.app",
  package_binary_path:"/evidence/native-current-package/Hepta.app/Contents/MacOS/hepta-native",
  package_binary_actual_sha256:"binary-sha",package_bundle_actual_sha256:"bundle-sha",package_artifact_hash_valid:true
}' >"$TEST_DIR/positive-truth-input.json"

jq -L scripts/lib -n --slurpfile positive "$TEST_DIR/positive-truth-input.json" '
  include "hepta-ui-current-readiness-v1";
  $positive[0] as $base |
  [
    {name:"explicit_false_positive",input:$base,expected:{source:true,browser:true,promotion:true,local:true}},
    {name:"mobile_visual_missing",input:($base | del(.mobile.hard_boundaries.android_emulator_visual_verified)),expected:{source:false,browser:true,promotion:true,local:false}},
    {name:"mobile_rotation_null",input:($base | .mobile.hard_boundaries.android_emulator_rotation_verified = null),expected:{source:false,browser:true,promotion:true,local:false}},
    {name:"mobile_ime_true",input:($base | .mobile.hard_boundaries.android_emulator_ime_verified = true),expected:{source:false,browser:true,promotion:true,local:false}},
    {name:"diagnostics_flag_missing",input:($base | del(.window_receipt.package.visual_capture_binary_is_separate_developer_diagnostics_build)),expected:{source:true,browser:true,promotion:false,local:false}},
    {name:"diagnostics_flag_null",input:($base | .window_receipt.package.visual_capture_binary_is_separate_developer_diagnostics_build = null),expected:{source:true,browser:true,promotion:false,local:false}},
    {name:"diagnostics_flag_true",input:($base | .window_receipt.package.visual_capture_binary_is_separate_developer_diagnostics_build = true),expected:{source:true,browser:true,promotion:false,local:false}},
    {name:"isolated_home_missing",input:($base | del(.window_receipt.isolation.home_isolated)),expected:{source:true,browser:true,promotion:false,local:false}},
    {name:"window_process_termination_missing",input:($base | del(.window_receipt.automation.application_process.identity_safe_termination_confirmed)),expected:{source:true,browser:true,promotion:false,local:false}},
    {name:"window_bounds_tolerance_missing",input:($base | del(.window_receipt.host_window.bounds_within_tolerance)),expected:{source:true,browser:true,promotion:false,local:false}},
    {name:"local_package_false",input:($base | .package.local_package_ready = false),expected:{source:true,browser:true,promotion:true,local:false}},
    {name:"static_package_contract_survives_nonbuild_exit",input:($base | .package_exit_code = 1),expected:{source:true,browser:true,promotion:true,local:false}},
    {name:"static_package_contract_survives_nonbuild_status",input:($base | .package.status = "not_ready"),expected:{source:true,browser:true,promotion:true,local:false}}
  ]
  | map(. + {actual:(.input | hepta_ui_readiness_truth)} | del(.input))
' >"$TEST_DIR/readiness-truth-table.json"
jq -e 'length == 13 and all(.[]; .actual == .expected)' "$TEST_DIR/readiness-truth-table.json" >/dev/null

# A reused evidence directory must not preserve a ready feature/browser receipt.
jq -n '{schema_version:1,kind:"hepta-native-feature-matrix-gate",status:"ready",feature_matrix_ready:true}' >"$TEST_DIR/native-feature-matrix.json"
jq -n '{schema_version:1,kind:"hepta-control-ui-browser-smoke-current-wrapper",status:"ready",browser_smoke_ready:true}' >"$TEST_DIR/control-browser-smoke.json"

artifact="$TEST_DIR/forged-attestation.txt"
printf '%s\n' forged >"$artifact"
artifact_sha="$(shasum -a 256 "$artifact" | awk '{print $1}')"
binding="$(scripts/hepta-ui-source-fingerprint)"
jq -n --argjson binding "$binding" --arg path "$artifact" --arg sha "$artifact_sha" '{
  schema_version:1,kind:"hepta-ui-release-receipt-v1",producer:"scripts/hepta-ui-release-verifier-v1",
  status:"ready",source_binding:$binding,artifact:{path:$path,sha256:$sha},public_distribution_ready:true,
  signed:true,notarized:true,stapled:true
}' >"$TEST_DIR/forged-release.json"
jq -n --argjson binding "$binding" --arg path "$artifact" --arg sha "$artifact_sha" '
  ($binding | .head = ("0" * 40)) as $stale |
  {schema_version:1,kind:"hepta-ui-matrix-live-receipt-v1",producer:"scripts/hepta-ui-matrix-live-verifier-v1",
   status:"ready",source_binding:$stale,artifact:{path:$path,sha256:$sha},matrix_live_ready:true}
' >"$TEST_DIR/stale-matrix.json"
jq -n --argjson binding "$binding" --arg path "$artifact" --arg sha "$artifact_sha" '{
  schema_version:1,kind:"hepta-ui-native-window-receipt-v1",producer:"scripts/hepta-ui-native-window-verifier-v1",
  status:"ready",source_binding:$binding,source_stable_during_run:true,
  run_nonce:"11111111-1111-1111-1111-111111111111",
  scope:"unauthenticated_local_macos_product_shell",independent_promotion_verifier_ready:true,
  artifact:{path:$path,sha256:$sha},native_window_ready:true,
  package:{report_path:"/forged/package.json",report_sha256:("a" * 64),app_path:"/forged/Hepta.app",binary_path:"/forged/Hepta.app/Contents/MacOS/hepta-native",binary_sha256:("b" * 64),bundle_fingerprint_sha256:("c" * 64),current_source_local_package_ready:true,visual_capture_binary_is_exact_packaged_executable:true,visual_capture_binary_is_separate_developer_diagnostics_build:false},
  automation:{no_remote:true,host_kind:"local",host_source:"forced_local_services"},
  isolation:{real_product_data_path_denied:true,real_product_cache_path_denied:true,keychain_services_denied:true,network_denied_by_sandbox:true,force_login_argument:true}
}' >"$TEST_DIR/forged-window.json"

HEPTA_UI_RELEASE_RECEIPT="$TEST_DIR/forged-release.json" \
HEPTA_UI_MATRIX_LIVE_RECEIPT="$TEST_DIR/stale-matrix.json" \
HEPTA_UI_NATIVE_WINDOW_RECEIPT="$TEST_DIR/forged-window.json" \
  scripts/hepta-ui-current-readiness.sh --evidence-dir "$TEST_DIR" --output "$TEST_DIR/current-readiness.json" --require none >/dev/null

jq -e '
  .status == "report_complete"
  and .gates.native_feature_matrix.status == "not_run"
  and .gates.native_feature_matrix.ready == false
  and .gates.control_browser.status == "not_run"
  and .gates.native_mobile.ios_unauthenticated_login_surface.software_keyboard_verified == false
  and .gates.native_mobile.ios_unauthenticated_login_surface.safe_area_verified == false
  and .gates.native_mobile.generic_android_visual_rotation_ime_claims_hard_false == true
  and ([.promotion_receipts[] | select(.name == "matrix_live") | .ready] == [false])
  and ([.promotion_receipts[] | select(.name == "native_window") | .ready] == [false])
  and .gates.native_window.verifier_executed == false
  and .gates.native_window.exit_code == 125
  and .gates.native_window.independent_promotion_ready == false
' "$TEST_DIR/current-readiness.json" >/dev/null

# Reusable output directories must not turn child/report writes into symlink
# follows, input replacement, or target-directory overlap.
printf '%s\n' preserve-me >"$TEST_DIR/symlink-target.txt"
mkdir -p "$TEST_DIR/symlink-evidence" "$TEST_DIR/symlink-cargo-target"
ln -s "$TEST_DIR/symlink-target.txt" "$TEST_DIR/symlink-evidence/native-feature-matrix.json"
if scripts/hepta-ui-current-readiness.sh \
  --evidence-dir "$TEST_DIR/symlink-evidence" \
  --target-dir "$TEST_DIR/symlink-cargo-target" \
  --output "$TEST_DIR/symlink-output.json" \
  --require none >/dev/null 2>&1; then
  echo "current readiness accepted a symlinked child receipt" >&2
  exit 1
fi
[[ "$(cat "$TEST_DIR/symlink-target.txt")" == preserve-me ]]

mkdir -p "$TEST_DIR/collision-evidence" "$TEST_DIR/collision-cargo-target"
if scripts/hepta-ui-current-readiness.sh \
  --evidence-dir "$TEST_DIR/collision-evidence" \
  --target-dir "$TEST_DIR/collision-cargo-target" \
  --output "$TEST_DIR/collision-evidence/native-current-package.json" \
  --require none >/dev/null 2>&1; then
  echo "current readiness accepted --output equal to a child receipt" >&2
  exit 1
fi

mkdir -p "$TEST_DIR/overlap-evidence"
if scripts/hepta-ui-current-readiness.sh \
  --evidence-dir "$TEST_DIR/overlap-evidence" \
  --target-dir "$TEST_DIR/overlap-evidence/cargo-target" \
  --output "$TEST_DIR/overlap-output.json" \
  --require none >/dev/null 2>&1; then
  echo "current readiness accepted overlapping evidence and target directories" >&2
  exit 1
fi

# A caller-selected report must never replace a producer artifact after that
# artifact has already been hashed and promoted into the readiness truth.
mkdir -p "$TEST_DIR/producer-tree-evidence/native-window" "$TEST_DIR/producer-tree-target"
producer_artifact="$TEST_DIR/producer-tree-evidence/native-window/native-window-evidence.tar.gz"
printf '%s\n' preserve-producer-artifact >"$producer_artifact"
if scripts/hepta-ui-current-readiness.sh \
  --evidence-dir "$TEST_DIR/producer-tree-evidence" \
  --target-dir "$TEST_DIR/producer-tree-target" \
  --output "$producer_artifact" \
  --require none >/dev/null 2>&1; then
  echo "current readiness accepted --output inside a producer-owned evidence subtree" >&2
  exit 1
fi
[[ "$(cat "$producer_artifact")" == preserve-producer-artifact ]]

printf '%s\n' preserve-output-target >"$TEST_DIR/output-symlink-target.txt"
ln -s "$TEST_DIR/output-symlink-target.txt" "$TEST_DIR/output-symlink.json"
if scripts/hepta-ui-current-readiness.sh \
  --evidence-dir "$TEST_DIR/output-symlink-evidence" \
  --target-dir "$TEST_DIR/output-symlink-cargo-target" \
  --output "$TEST_DIR/output-symlink.json" \
  --require none >/dev/null 2>&1; then
  echo "current readiness accepted a symlinked final report" >&2
  exit 1
fi
[[ "$(cat "$TEST_DIR/output-symlink-target.txt")" == preserve-output-target ]]
jq -e '
  .readiness.local_demo == false
  and .readiness.full_product == false
  and .readiness.public_ga == false
  and .hard_boundaries.promotion_independent_verifiers_ready == false
  and .hard_boundaries.ios_simulator_unauthenticated_login_surface_software_keyboard_verified == false
  and .hard_boundaries.ios_simulator_unauthenticated_login_surface_safe_area_verified == false
  and .hard_boundaries.android_emulator_visual_verified == false
  and .hard_boundaries.android_emulator_rotation_verified == false
  and .hard_boundaries.android_emulator_ime_verified == false
  and .hard_boundaries.release_independent_verification_ready == false
  and .hard_boundaries.signed == false
  and .hard_boundaries.notarized == false
  and .hard_boundaries.stapled == false
  and .hard_boundaries.public_distribution_ready == false
' "$TEST_DIR/current-readiness.json" >/dev/null

# The exact binding predicate rejects concurrent HEAD/tree/fingerprint changes.
for key in head head_tree source_fingerprint; do
  jq --arg key "$key" '.[$key] = ("f" * (if $key == "source_fingerprint" then 64 else 40 end))' <<<"$binding" >"$TEST_DIR/mutated-binding.json"
  if jq -e --argjson current "$binding" '
      .head == $current.head and .head_tree == $current.head_tree and .source_fingerprint == $current.source_fingerprint
    ' "$TEST_DIR/mutated-binding.json" >/dev/null; then
    echo "binding mismatch was accepted for $key" >&2
    exit 1
  fi
done

# Browser evidence can exceed macOS ARG_MAX. It must be passed to jq as a
# file, never expanded into an `--argjson` command-line argument.
grep -Fq -- '--slurpfile browser_file "$BROWSER_REPORT"' scripts/hepta-ui-current-readiness.sh
if grep -Fq -- '--argjson browser "$(cat "$BROWSER_REPORT")"' scripts/hepta-ui-current-readiness.sh; then
  echo "current readiness expands the browser receipt onto the command line" >&2
  exit 1
fi

# A current window capability alone is insufficient: promotion must be produced
# in this exact run and bind back to the exact current-run package artifact.
grep -Fq -- 'and ($c.window_receipt.package.report_sha256 // "") == $c.package_report_sha256' scripts/lib/hepta-ui-current-readiness-v1.jq
grep -Fq -- '$c.window_verifier_executed == true' scripts/lib/hepta-ui-current-readiness-v1.jq
grep -Fq -- 'and ($c.window_receipt.run_nonce // "") == $c.run_nonce' scripts/lib/hepta-ui-current-readiness-v1.jq
grep -Fq -- 'and ($c.window_receipt.package.bundle_fingerprint_sha256 // "") == $c.package_bundle_actual_sha256' scripts/lib/hepta-ui-current-readiness-v1.jq
grep -Fq -- 'NATIVE_WINDOW_RECEIPT="$WINDOW_REPORT"' scripts/hepta-ui-current-readiness.sh
grep -Fq -- 'include "hepta-ui-current-readiness-v1";' scripts/hepta-ui-current-readiness.sh
grep -Fq -- 'write_report_atomically()' scripts/hepta-ui-current-readiness.sh
grep -Fq -- 'current_run_producer_not_completed' scripts/hepta-ui-current-readiness.sh
grep -Fq -- 'source_binding_at_publish:$final_binding[0]' scripts/hepta-ui-current-readiness.sh
grep -Fq -- 'source_changed_or_became_dirty_before_atomic_publish' scripts/hepta-ui-current-readiness.sh
grep -Fq -- 'package_args+=(--build --bootstrap-tools --stage-dir "$EVIDENCE_DIR/native-current-package")' scripts/hepta-ui-current-readiness.sh
grep -Fq -- 'ios_unauthenticated_login_surface:{software_keyboard_verified:' scripts/hepta-ui-current-readiness.sh
grep -Fq -- 'ios_simulator_unauthenticated_login_surface_software_keyboard_verified:' scripts/hepta-ui-current-readiness.sh
grep -Fq -- 'ios_simulator_unauthenticated_login_surface_safe_area_verified:' scripts/hepta-ui-current-readiness.sh
if grep -Fq -- ': >"$current_run_report"' scripts/hepta-ui-current-readiness.sh; then
  echo "current readiness still truncates reusable child receipt paths in place" >&2
  exit 1
fi
if rg -n 'android_emulator_(visual|rotation|ime)_verified // true|visual_capture_binary_is_separate_developer_diagnostics_build // true' scripts/hepta-ui-current-readiness.sh scripts/lib/hepta-ui-current-readiness-v1.jq >/dev/null; then
  echo "current readiness contains false-coalescing that makes explicit false unreachable" >&2
  exit 1
fi
if rg -n 'HEPTA_UI_NATIVE_WINDOW_RECEIPT' scripts/hepta-ui-current-readiness.sh >/dev/null; then
  echo "current readiness still accepts an external native-window receipt" >&2
  exit 1
fi
if grep -Fq -- 'false as $promotion_independent_verifiers_ready' scripts/hepta-ui-current-readiness.sh; then
  echo "native-window promotion verifier remains hard-coded false" >&2
  exit 1
fi

echo "hepta-ui current readiness fail-closed self-test: PASS"
