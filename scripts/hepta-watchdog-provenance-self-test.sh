#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"
WATCHDOG="$ROOT/scripts/hepta-watchdog.sh"
RELEASE_TOOL="$ROOT/scripts/hepta-immutable-release-tree"
source "$ROOT/scripts/lib/hepta-json-report-capture.sh"
source "$ROOT/scripts/lib/hepta-release-provenance.sh"
tmp="$(mktemp -d /tmp/hepta-watchdog-provenance-self-test.XXXXXX)"
trap 'chmod -R u+w "$tmp" 2>/dev/null || true; rm -rf "$tmp"' EXIT
source_commit="$(git -C "$ROOT" rev-parse HEAD)"
make_artifact() {
  local destination="$1"
  local label="$2"
  printf '%s\n' '#!/usr/bin/env bash' "printf '%s\\n' '$label'" >"$destination"
  chmod 0755 "$destination"
}
materialize_bound_release() {
  local artifact="$1"
  local release_root="$2"
  local preflight_log="$3"
  local provenance
  provenance="$(
    hepta_release_fixture_complete_provenance_json "$ROOT" "$source_commit" "$artifact"
  )"
  hepta_release_write_fixture_preflight_log "$preflight_log" "$source_commit" "$provenance"
  "$RELEASE_TOOL" materialize \
    --artifact "$artifact" \
    --source-commit "$source_commit" \
    --preflight-log "$preflight_log" \
    --release-root "$release_root"
}
watchdog_output=""
watchdog_report=""
watchdog_rc=0
capture_watchdog() {
  local expected_rc="$1"
  shift
  watchdog_rc=0
  watchdog_output="$(env "$@" "$WATCHDOG" 2>&1)" || watchdog_rc=$?
  if [[ "$watchdog_rc" -ne "$expected_rc" ]]; then
    echo "watchdog fixture returned $watchdog_rc; expected $expected_rc" >&2
    printf '%s\n' "$watchdog_output" >&2
    exit 1
  fi
  watchdog_report="$(printf '%s\n' "$watchdog_output" | extract_first_json_object)"
  jq -e . >/dev/null <<<"$watchdog_report" || {
    echo "watchdog fixture did not emit a JSON report" >&2
    printf '%s\n' "$watchdog_output" >&2
    exit 1
  }
}
artifact_a="$tmp/hepta-a"
artifact_b="$tmp/hepta-b"
make_artifact "$artifact_a" "candidate-a"
make_artifact "$artifact_b" "candidate-b"
manifest_a="$(
  materialize_bound_release \
    "$artifact_a" \
    "$tmp/releases-a" \
    "$tmp/preflight-a.log"
)"
manifest_b="$(
  materialize_bound_release \
    "$artifact_b" \
    "$tmp/releases-b" \
    "$tmp/preflight-b.log"
)"
candidate_a="$(dirname "$manifest_a")/bin/hepta"
candidate_b="$(dirname "$manifest_b")/bin/hepta"
capture_watchdog 0 \
  HEPTA_WATCHDOG_MODE=candidate-artifact \
  HEPTA_RELEASE_BIN="$candidate_a" \
  HEPTA_EXPECTED_SOURCE_COMMIT="$source_commit"
jq -e --arg manifest "$manifest_a" '
  .status == "ok"
  and .watchdog_mode == "candidate-artifact"
  and .candidate_artifact.required == true
  and .candidate_artifact.evidence.ready == true
  and .candidate_artifact.evidence.manifest == $manifest
  and .candidate_artifact.evidence.manifest_source_toolchain_dependency_preflight_bound == true
  and .active_health.status == "not_checked"
  and (.failure_reasons | length) == 0
' >/dev/null <<<"$watchdog_report"
mkdir -p "$tmp/installed-a"
ln -s "$candidate_a" "$tmp/installed-a/hepta"
ln -s "$manifest_a" "$tmp/installed-a/hepta.manifest"
capture_watchdog 0 \
  HEPTA_WATCHDOG_MODE=deployed-receipt \
  HEPTA_INSTALLED_BIN="$tmp/installed-a/hepta" \
  HEPTA_INSTALLED_RECEIPT="$tmp/installed-a/hepta.manifest" \
  HEPTA_EXPECTED_SOURCE_COMMIT="$source_commit"
jq -e '
  .status == "ok"
  and .watchdog_mode == "deployed-receipt"
  and .deployed_receipt.required == true
  and .deployed_receipt.evidence.ready == true
  and .active_health.status == "not_checked"
' >/dev/null <<<"$watchdog_report"
capture_watchdog 1 \
  HEPTA_RELEASE_BIN="$tmp/missing-candidate" \
  HEPTA_CANDIDATE_MANIFEST="$tmp/missing-candidate.manifest" \
  HEPTA_INSTALLED_BIN="$tmp/installed-a/hepta" \
  HEPTA_INSTALLED_RECEIPT="$tmp/installed-a/hepta.manifest" \
  HEPTA_EXPECTED_SOURCE_COMMIT="$source_commit"
jq -e '
  .status == "failed"
  and .watchdog_mode == "deployment-consistency"
  and .active_health.status == "not_checked"
  and (.failure_reasons | index("candidate_binary_missing")) != null
  and (.failure_reasons | index("candidate_manifest_missing")) != null
' >/dev/null <<<"$watchdog_report"
legacy_manifest="$(
  "$RELEASE_TOOL" materialize \
    --artifact "$artifact_a" \
    --source-commit unknown \
    --release-root "$tmp/releases-legacy"
)"
legacy_artifact="$(dirname "$legacy_manifest")/bin/hepta"
capture_watchdog 1 \
  HEPTA_WATCHDOG_MODE=candidate-artifact \
  HEPTA_RELEASE_BIN="$legacy_artifact" \
  HEPTA_CANDIDATE_MANIFEST="$legacy_manifest"
jq -e '
  .status == "failed"
  and .candidate_artifact.evidence.manifest_contract_valid == true
  and .candidate_artifact.evidence.manifest_source_toolchain_dependency_preflight_bound == false
  and (.failure_reasons | index("candidate_manifest_not_source_toolchain_dependency_preflight_bound")) != null
' >/dev/null <<<"$watchdog_report"
invalid_release="$tmp/releases-invalid/$(basename "$(dirname "$manifest_a")")"
mkdir -p "$invalid_release/bin"
cp "$candidate_a" "$invalid_release/bin/hepta"
chmod 0555 "$invalid_release/bin/hepta" "$invalid_release/bin"
jq 'del(.build_provenance)' "$manifest_a" >"$invalid_release/manifest.json"
chmod 0444 "$invalid_release/manifest.json"
chmod 0555 "$invalid_release"
capture_watchdog 1 \
  HEPTA_WATCHDOG_MODE=candidate-artifact \
  HEPTA_RELEASE_BIN="$invalid_release/bin/hepta" \
  HEPTA_CANDIDATE_MANIFEST="$invalid_release/manifest.json"
jq -e '
  .status == "failed"
  and .candidate_artifact.evidence.manifest_contract_valid == false
  and .candidate_artifact.evidence.manifest_source_toolchain_dependency_preflight_bound == false
  and (.failure_reasons | index("candidate_manifest_contract_invalid")) != null
' >/dev/null <<<"$watchdog_report"
mkdir -p "$tmp/installed-b"
ln -s "$candidate_b" "$tmp/installed-b/hepta"
ln -s "$manifest_b" "$tmp/installed-b/hepta.manifest"
capture_watchdog 1 \
  HEPTA_RELEASE_BIN="$candidate_a" \
  HEPTA_CANDIDATE_MANIFEST="$manifest_a" \
  HEPTA_INSTALLED_BIN="$tmp/installed-b/hepta" \
  HEPTA_INSTALLED_RECEIPT="$tmp/installed-b/hepta.manifest" \
  HEPTA_EXPECTED_SOURCE_COMMIT="$source_commit"
jq -e '
  .status == "failed"
  and .deployment_consistency_required == true
  and .candidate_artifact.evidence.ready == true
  and .deployed_receipt.evidence.ready == true
  and .binary_sha_match == false
  and .active_health.status == "not_checked"
  and (.failure_reasons | index("candidate_installed_sha_mismatch")) != null
' >/dev/null <<<"$watchdog_report"
jq -n \
  --arg status ready \
  --arg source_commit "$source_commit" \
  '{
    status:$status,
    source_commit:$source_commit,
    default_mode_fail_closed:true,
    missing_candidate_denied:true,
    legacy_unbound_candidate_denied:true,
    missing_build_provenance_denied:true,
    candidate_installed_sha_mismatch_nonzero:true,
    candidate_only_mode_explicit:true,
    deployed_receipt_only_mode_explicit:true,
    live_endpoint_read:false,
    service_restarted:false,
    installed_binary_mutated:false
  }'
echo "Hepta watchdog provenance self-test passed"
