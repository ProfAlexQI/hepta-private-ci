#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
tmp="$(mktemp -d "${TMPDIR:-/tmp}/hepta-active-binary-consistency-test.XXXXXX")"
cleanup() {
  if [[ -n "${different_pid:-}" ]]; then
    kill "$different_pid" 2>/dev/null || true
    wait "$different_pid" 2>/dev/null || true
  fi
  if [[ -n "${fixture_pid:-}" ]]; then
    kill "$fixture_pid" 2>/dev/null || true
    wait "$fixture_pid" 2>/dev/null || true
  fi
  chmod -R u+w "$tmp" 2>/dev/null || true
  rm -rf "$tmp"
}
trap cleanup EXIT
fixture_root="$tmp/source"
mkdir -p \
  "$fixture_root/scripts/lib" \
  "$fixture_root/docs/decisions" \
  "$fixture_root/codex-rs" \
  "$fixture_root/apps/hepta-native"
cp \
  "$ROOT/scripts/hepta-active-binary-consistency-gate" \
  "$ROOT/scripts/hepta-immutable-release-tree" \
  "$ROOT/scripts/hepta-watchdog.sh" \
  "$ROOT/scripts/hepta-live-soak.sh" \
  "$ROOT/scripts/hepta-generation-pointer" \
  "$ROOT/scripts/hepta-off-device-archive" \
  "$ROOT/scripts/hepta-log-rotate" \
  "$ROOT/scripts/hepta-final-evidence-index" \
  "$ROOT/scripts/hepta-stage-identity-chain" \
  "$ROOT/scripts/hepta-release-evidence-finalize" \
  "$ROOT/scripts/hepta-preflight-evidence-pack" \
  "$ROOT/scripts/hepta-install-live-watchdog" \
  "$ROOT/scripts/hepta-install-live-gateway" \
  "$fixture_root/scripts/"
cp \
  "$ROOT/scripts/lib/hepta-release-provenance.sh" \
  "$ROOT/scripts/lib/hepta-watchdog-release-evidence-v1.sh" \
  "$ROOT/scripts/lib/hepta-watchdog-product-boundary-v1.sh" \
  "$ROOT/scripts/lib/hepta-immutable-watchdog-closure-v1.sh" \
  "$fixture_root/scripts/lib/"
cp \
  "$ROOT/scripts/hepta-dependency-security-v1.json" \
  "$ROOT/scripts/hepta-dependency-exception-policy-v1.json" \
  "$fixture_root/scripts/"
cp \
  "$ROOT/docs/decisions/hepta-product-boundary-v1.json" \
  "$fixture_root/docs/decisions/"
cp "$ROOT/codex-rs/"{Cargo.lock,rust-toolchain.toml} "$fixture_root/codex-rs/"
cp "$ROOT/apps/hepta-native/"{Cargo.lock,rust-toolchain.toml} \
  "$fixture_root/apps/hepta-native/"
git -C "$fixture_root" init -q
git -C "$fixture_root" add .
git -C "$fixture_root" \
  -c user.name="Hepta Active Binary Self-Test" \
  -c user.email="hepta-active-binary-self-test@invalid" \
  commit -qm "fixture"
release_tool="$fixture_root/scripts/hepta-immutable-release-tree"
consistency_gate="$fixture_root/scripts/hepta-active-binary-consistency-gate"
artifact="$tmp/hepta-fixture"
cat >"$tmp/fixture.c" <<'EOF'
#include <unistd.h>
int main(void) {
  sleep(30);
  return 0;
}
EOF
cc "$tmp/fixture.c" -o "$artifact"
source_commit="$(git -C "$fixture_root" rev-parse HEAD)"
source "$fixture_root/scripts/lib/hepta-release-provenance.sh"
provenance="$(
  hepta_release_fixture_complete_provenance_json "$fixture_root" "$source_commit" "$artifact"
)"
preflight="$tmp/preflight.log"
hepta_release_write_fixture_preflight_log "$preflight" "$source_commit" "$provenance"
manifest="$($release_tool materialize --artifact "$artifact" --source-commit "$source_commit" --preflight-log "$preflight" --release-root "$tmp/releases")"
installed_bin="$(dirname "$manifest")/bin/hepta"
"$installed_bin" 30 &
fixture_pid=$!
fixture_now=2000000000
report="$(
  HEPTA_CURRENT_REALITY_NOW_EPOCH="$fixture_now" \
  HEPTA_CURRENT_REALITY_OBSERVED_AT_EPOCH="$fixture_now" \
  HEPTA_CURRENT_REALITY_MAX_AGE_SECONDS=60 \
  "$consistency_gate" \
    --pid "$fixture_pid" \
    --installed-bin "$installed_bin" \
    --manifest "$manifest" \
    --expected-source-commit "$source_commit"
)"
jq -e '
  .status == "ready"
  and .status_scope == "deployment_consistency_gate"
  and .ready == true
  and .contract_valid == true
  and .locally_executable == true
  and .integration_verified == true
  and .active_binary_consistent == true
  and .observation_fresh == true
  and .current_runtime_ready == true
  and .deployment_consistent == true
  and .controlled_live == false
  and .live_enabled == false
  and .production_ready == false
  and .manifest_source_preflight_bound == true
  and .source_commit_matches == true
  and .installed_manifest_sha_match == true
  and .active_installed_sha_match == true
  and .active_installed_manifest_sha_match == true
  and .repo_head == .expected_source_commit
  and .current_reality.schema_version == "hepta_current_reality_observation_v1"
  and .current_reality.source == "active_process_readback"
  and .current_reality.clock_source == "fixture"
  and .current_reality.generated_at == 2000000000
  and .current_reality.observed_at == 2000000000
  and .current_reality.age_seconds == 0
  and .current_reality.max_age_seconds == 60
  and .current_reality.max_future_skew_seconds == 0
  and .current_reality.timestamp_present == true
  and .current_reality.timestamp_valid == true
  and .current_reality.fresh == true
  and .current_reality.stale == false
  and .current_reality.source_verified == true
  and .current_reality.source_head_matches == true
  and .current_reality.manifest_source_matches == true
  and .current_reality.manifest_readback_verified == true
  and .current_reality.readback_verifiable == true
  and .current_reality.current == true
  and .truth_semantics.highest_verified_level == "deployment_consistent"
  and .truth_semantics.ready_is_gate_scoped == true
  and .truth_semantics.contract_readiness_is_not_observation_freshness == true
  and .truth_semantics.observation_freshness_is_not_current_runtime_readiness == true
  and .truth_semantics.current_runtime_readiness_is_not_deployment_consistency == true
  and .truth_semantics.active_process_is_not_controlled_live_evidence == true
  and .truth_semantics.deployment_consistency_is_not_production_readiness == true
  and (.failure_reasons | length) == 0
' >/dev/null <<<"$report"
assert_current_reality_not_ready() {
  local label="$1"
  local expected_reason="$2"
  local expected_fresh="$3"
  shift 3
  local negative_report=""
  if negative_report="$(
    env \
      HEPTA_CURRENT_REALITY_NOW_EPOCH="$fixture_now" \
      HEPTA_CURRENT_REALITY_MAX_AGE_SECONDS=60 \
      "$@" \
      "$consistency_gate" \
        --pid "$fixture_pid" \
        --installed-bin "$installed_bin" \
        --manifest "$manifest" \
        --expected-source-commit "$source_commit" \
        2>/dev/null
  )"; then
    echo "active binary gate accepted $label current-reality evidence" >&2
    exit 1
  fi
  jq -e \
    --arg expected_reason "$expected_reason" \
    --argjson expected_fresh "$expected_fresh" \
    '
      .ready == false
      and .observation_fresh == $expected_fresh
      and .current_runtime_ready == false
      and .deployment_consistent == false
      and .controlled_live == false
      and .production_ready == false
      and .current_reality.stale == ($expected_fresh | not)
      and .current_reality.current == false
      and (.failure_reasons | index($expected_reason)) != null
    ' >/dev/null <<<"$negative_report" || {
      echo "active binary gate did not fail closed for $label" >&2
      exit 1
    }
}
assert_current_reality_not_ready \
  "missing timestamp" \
  current_reality_observed_at_missing \
  false \
  HEPTA_CURRENT_REALITY_OBSERVED_AT_EPOCH=
assert_current_reality_not_ready \
  "future timestamp" \
  current_reality_observation_from_future \
  false \
  HEPTA_CURRENT_REALITY_OBSERVED_AT_EPOCH=2000000001
assert_current_reality_not_ready \
  "expired timestamp" \
  current_reality_observation_stale \
  false \
  HEPTA_CURRENT_REALITY_OBSERVED_AT_EPOCH=1999999939
assert_current_reality_not_ready \
  "source mismatch" \
  current_reality_source_unverified \
  true \
  HEPTA_CURRENT_REALITY_OBSERVED_AT_EPOCH=2000000000 \
  HEPTA_CURRENT_REALITY_SOURCE=historical_receipt
assert_current_reality_not_ready \
  "manifest mismatch" \
  current_reality_manifest_readback_unverified \
  true \
  HEPTA_CURRENT_REALITY_OBSERVED_AT_EPOCH=2000000000 \
  HEPTA_CURRENT_REALITY_MANIFEST_SHA256=0000000000000000000000000000000000000000000000000000000000000000
legacy_manifest="$($release_tool materialize --artifact "$artifact" --source-commit unknown --release-root "$tmp/legacy-releases")"
legacy_report=""
if legacy_report="$($consistency_gate --pid "$fixture_pid" --installed-bin "$installed_bin" --manifest "$legacy_manifest" --expected-source-commit "$source_commit" 2>/dev/null)"; then
  echo "active binary gate accepted a legacy-unbound manifest" >&2
  exit 1
fi
jq -e '
  .contract_valid == true
  and .locally_executable == true
  and .integration_verified == false
  and .active_binary_consistent == true
  and .deployment_consistent == false
  and .controlled_live == false
  and .live_enabled == false
  and .production_ready == false
  and .active_installed_manifest_sha_match == true
  and .truth_semantics.highest_verified_level == "active_binary_consistent"
  and (.failure_reasons | index("active_manifest_not_source_preflight_bound")) != null
  and (.failure_reasons | index("active_source_commit_mismatch")) != null
' >/dev/null <<<"$legacy_report"
source_mismatch_report=""
if source_mismatch_report="$($consistency_gate --pid "$fixture_pid" --installed-bin "$installed_bin" --manifest "$manifest" --expected-source-commit 0000000000000000000000000000000000000000 2>/dev/null)"; then
  echo "active binary gate accepted an unexpected source commit" >&2
  exit 1
fi
jq -e '
  .production_ready == false
  and .current_runtime_ready == false
  and .deployment_consistent == false
  and .current_reality.source_head_matches == false
  and (.failure_reasons | index("active_source_commit_mismatch")) != null
  and (.failure_reasons | index("current_reality_source_head_mismatch")) != null
' >/dev/null <<<"$source_mismatch_report"
missing_expected_rc=0
"$consistency_gate" --pid "$fixture_pid" --installed-bin "$installed_bin" --manifest "$manifest" >/dev/null 2>&1 || missing_expected_rc=$?
[[ "$missing_expected_rc" == "2" ]]
different="$tmp/different"
printf '#!/usr/bin/env bash\nexit 0\n' >"$different"
chmod 0755 "$different"
mismatch_rc=0
"$consistency_gate" --pid "$fixture_pid" --installed-bin "$different" --manifest "$manifest" >/dev/null 2>&1 || mismatch_rc=$?
[[ "$mismatch_rc" == "2" ]]
installed_drift_report=""
if installed_drift_report="$($consistency_gate --pid "$fixture_pid" --installed-bin "$different" --manifest "$manifest" --expected-source-commit "$source_commit" 2>/dev/null)"; then
  echo "active binary gate accepted installed/manifest drift" >&2
  exit 1
fi
jq -e '
  .production_ready == false
  and .active_binary_consistent == false
  and .deployment_consistent == false
  and .installed_manifest_sha_match == false
  and (.failure_reasons | index("installed_manifest_sha_mismatch")) != null
' >/dev/null <<<"$installed_drift_report"
active_different="$tmp/active-different"
sed 's/sleep(30)/sleep(29)/' "$tmp/fixture.c" >"$tmp/active-different.c"
cc "$tmp/active-different.c" -o "$active_different"
"$active_different" &
different_pid=$!
active_drift_report=""
if active_drift_report="$($consistency_gate --pid "$different_pid" --installed-bin "$installed_bin" --manifest "$manifest" --expected-source-commit "$source_commit" 2>/dev/null)"; then
  echo "active binary gate accepted active/installed drift" >&2
  exit 1
fi
jq -e '
  .production_ready == false
  and .integration_verified == true
  and .active_binary_consistent == false
  and .deployment_consistent == false
  and .truth_semantics.highest_verified_level == "integration_verified"
  and .active_installed_sha_match == false
  and (.failure_reasons | index("active_installed_sha_mismatch")) != null
' >/dev/null <<<"$active_drift_report"
kill "$different_pid" 2>/dev/null || true
wait "$different_pid" 2>/dev/null || true
different_pid=""
invalid_release="$tmp/invalid-release"
mkdir -p "$invalid_release/bin"
cp "$installed_bin" "$invalid_release/bin/hepta"
chmod 0555 "$invalid_release/bin/hepta"
jq 'del(.preflight)' "$manifest" >"$invalid_release/manifest.json"
chmod 0444 "$invalid_release/manifest.json"
missing_preflight_report=""
if missing_preflight_report="$($consistency_gate --pid "$fixture_pid" --installed-bin "$installed_bin" --manifest "$invalid_release/manifest.json" --expected-source-commit "$source_commit" 2>/dev/null)"; then
  echo "active binary gate accepted a manifest without preflight binding" >&2
  exit 1
fi
jq -e '
  .contract_valid == false
  and .production_ready == false
  and (.failure_reasons | index("active_manifest_contract_invalid")) != null
  and (.failure_reasons | index("active_manifest_not_source_preflight_bound")) != null
' >/dev/null <<<"$missing_preflight_report"
echo "hepta active binary consistency self-test passed"
