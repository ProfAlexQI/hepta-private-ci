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

HEPTA_UI_RELEASE_RECEIPT="$TEST_DIR/forged-release.json" \
HEPTA_UI_MATRIX_LIVE_RECEIPT="$TEST_DIR/stale-matrix.json" \
  scripts/hepta-ui-current-readiness.sh --evidence-dir "$TEST_DIR" --output "$TEST_DIR/report.json" --require none >/dev/null

jq -e '
  .status == "report_complete"
  and .gates.native_feature_matrix.status == "not_run"
  and .gates.native_feature_matrix.ready == false
  and .gates.control_browser.status == "not_run"
  and ([.promotion_receipts[] | select(.name == "matrix_live") | .ready] == [false])
' "$TEST_DIR/report.json" >/dev/null
jq -e '
  .readiness.full_product == false
  and .readiness.public_ga == false
  and .hard_boundaries.promotion_independent_verifiers_ready == false
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

echo "hepta-ui current readiness fail-closed self-test: PASS"
