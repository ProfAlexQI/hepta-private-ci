#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
cd "$ROOT_DIR"
TEST_DIR="$(mktemp -d "${TMPDIR:-/tmp}/hepta-ui-current-readiness-self-test.XXXXXX")"
trap 'rm -rf "$TEST_DIR"' EXIT

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
  scripts/hepta-ui-current-readiness.sh --evidence-dir "$TEST_DIR" --output "$TEST_DIR/report.json" --require none >/dev/null

jq -e '
  .status == "report_complete"
  and .gates.native_feature_matrix.status == "not_run"
  and .gates.native_feature_matrix.ready == false
  and .gates.control_browser.status == "not_run"
  and .gates.native_mobile.generic_android_visual_rotation_ime_claims_hard_false == true
  and ([.promotion_receipts[] | select(.name == "matrix_live") | .ready] == [false])
  and ([.promotion_receipts[] | select(.name == "native_window") | .ready] == [false])
  and .gates.native_window.verifier_executed == false
  and .gates.native_window.exit_code == 125
  and .gates.native_window.independent_promotion_ready == false
' "$TEST_DIR/report.json" >/dev/null
jq -e '
  .readiness.local_demo == false
  and .readiness.full_product == false
  and .readiness.public_ga == false
  and .hard_boundaries.promotion_independent_verifiers_ready == false
  and .hard_boundaries.android_emulator_visual_verified == false
  and .hard_boundaries.android_emulator_rotation_verified == false
  and .hard_boundaries.android_emulator_ime_verified == false
  and .hard_boundaries.release_independent_verification_ready == false
  and .hard_boundaries.signed == false
  and .hard_boundaries.notarized == false
  and .hard_boundaries.stapled == false
  and .hard_boundaries.public_distribution_ready == false
' "$TEST_DIR/report.json" >/dev/null

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
grep -Fq -- 'and ($window_receipt.package.report_sha256 // "") == $package_report_sha256' scripts/hepta-ui-current-readiness.sh
grep -Fq -- '$window_verifier_executed == true' scripts/hepta-ui-current-readiness.sh
grep -Fq -- 'and ($window_receipt.run_nonce // "") == $run_nonce' scripts/hepta-ui-current-readiness.sh
grep -Fq -- 'and ($window_receipt.package.bundle_fingerprint_sha256 // "") == $package_bundle_actual_sha256' scripts/hepta-ui-current-readiness.sh
grep -Fq -- 'NATIVE_WINDOW_RECEIPT="$WINDOW_REPORT"' scripts/hepta-ui-current-readiness.sh
if rg -n 'HEPTA_UI_NATIVE_WINDOW_RECEIPT' scripts/hepta-ui-current-readiness.sh >/dev/null; then
  echo "current readiness still accepts an external native-window receipt" >&2
  exit 1
fi
if grep -Fq -- 'false as $promotion_independent_verifiers_ready' scripts/hepta-ui-current-readiness.sh; then
  echo "native-window promotion verifier remains hard-coded false" >&2
  exit 1
fi

echo "hepta-ui current readiness fail-closed self-test: PASS"
