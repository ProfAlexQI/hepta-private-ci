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

artifact="$tmp/hepta-fixture"
cat >"$tmp/fixture.c" <<'EOF'
#include <unistd.h>

int main(void) {
  sleep(30);
  return 0;
}
EOF
cc "$tmp/fixture.c" -o "$artifact"
source_commit="$(git -C "$ROOT" rev-parse HEAD)"
provenance="$(source "$ROOT/scripts/lib/hepta-release-provenance.sh"; hepta_release_build_provenance_json "$ROOT" "$source_commit" "$artifact")"
provenance="$(jq -c '. + {
  preflight_profiles:{backend:true,native:true,release:true},
  watchdog_gate_mode:"fixture",
  deployment_consistency:{checked_during_candidate_preflight:false,required_before_activation:true}
}' <<<"$provenance")"
preflight="$tmp/preflight.log"
printf '%s\n' \
  "[hepta-preflight-resume] head=$source_commit attempt=1 start_line=1 marker=<start> log=$preflight" \
  "[hepta-preflight-provenance] $provenance" \
  'Hepta preflight passed' >"$preflight"

manifest="$($ROOT/scripts/hepta-immutable-release-tree materialize --artifact "$artifact" --source-commit "$source_commit" --preflight-log "$preflight" --release-root "$tmp/releases")"
installed_bin="$(dirname "$manifest")/bin/hepta"
"$installed_bin" 30 &
fixture_pid=$!

report="$($ROOT/scripts/hepta-active-binary-consistency-gate --pid "$fixture_pid" --installed-bin "$installed_bin" --manifest "$manifest" --expected-source-commit "$source_commit")"
jq -e '
  .status == "ready"
  and .contract_valid == true
  and .locally_executable == true
  and .integration_verified == true
  and .live_enabled == true
  and .production_ready == true
  and .manifest_source_preflight_bound == true
  and .source_commit_matches == true
  and .installed_manifest_sha_match == true
  and .active_installed_sha_match == true
  and .active_installed_manifest_sha_match == true
  and (.failure_reasons | length) == 0
' >/dev/null <<<"$report"

legacy_manifest="$($ROOT/scripts/hepta-immutable-release-tree materialize --artifact "$artifact" --source-commit unknown --release-root "$tmp/legacy-releases")"
legacy_report=""
if legacy_report="$($ROOT/scripts/hepta-active-binary-consistency-gate --pid "$fixture_pid" --installed-bin "$installed_bin" --manifest "$legacy_manifest" --expected-source-commit "$source_commit" 2>/dev/null)"; then
  echo "active binary gate accepted a legacy-unbound manifest" >&2
  exit 1
fi
jq -e '
  .contract_valid == true
  and .locally_executable == true
  and .integration_verified == false
  and .live_enabled == true
  and .production_ready == false
  and .active_installed_manifest_sha_match == true
  and (.failure_reasons | index("active_manifest_not_source_preflight_bound")) != null
  and (.failure_reasons | index("active_source_commit_mismatch")) != null
' >/dev/null <<<"$legacy_report"

source_mismatch_report=""
if source_mismatch_report="$($ROOT/scripts/hepta-active-binary-consistency-gate --pid "$fixture_pid" --installed-bin "$installed_bin" --manifest "$manifest" --expected-source-commit 0000000000000000000000000000000000000000 2>/dev/null)"; then
  echo "active binary gate accepted an unexpected source commit" >&2
  exit 1
fi
jq -e '.production_ready == false and (.failure_reasons | index("active_source_commit_mismatch")) != null' >/dev/null <<<"$source_mismatch_report"

missing_expected_rc=0
"$ROOT/scripts/hepta-active-binary-consistency-gate" --pid "$fixture_pid" --installed-bin "$installed_bin" --manifest "$manifest" >/dev/null 2>&1 || missing_expected_rc=$?
[[ "$missing_expected_rc" == "2" ]]

different="$tmp/different"
printf '#!/usr/bin/env bash\nexit 0\n' >"$different"
chmod 0755 "$different"
mismatch_rc=0
"$ROOT/scripts/hepta-active-binary-consistency-gate" --pid "$fixture_pid" --installed-bin "$different" --manifest "$manifest" >/dev/null 2>&1 || mismatch_rc=$?
[[ "$mismatch_rc" == "2" ]]

installed_drift_report=""
if installed_drift_report="$($ROOT/scripts/hepta-active-binary-consistency-gate --pid "$fixture_pid" --installed-bin "$different" --manifest "$manifest" --expected-source-commit "$source_commit" 2>/dev/null)"; then
  echo "active binary gate accepted installed/manifest drift" >&2
  exit 1
fi
jq -e '
  .production_ready == false
  and .installed_manifest_sha_match == false
  and (.failure_reasons | index("installed_manifest_sha_mismatch")) != null
' >/dev/null <<<"$installed_drift_report"

active_different="$tmp/active-different"
sed 's/sleep(30)/sleep(29)/' "$tmp/fixture.c" >"$tmp/active-different.c"
cc "$tmp/active-different.c" -o "$active_different"
"$active_different" &
different_pid=$!
active_drift_report=""
if active_drift_report="$($ROOT/scripts/hepta-active-binary-consistency-gate --pid "$different_pid" --installed-bin "$installed_bin" --manifest "$manifest" --expected-source-commit "$source_commit" 2>/dev/null)"; then
  echo "active binary gate accepted active/installed drift" >&2
  exit 1
fi
jq -e '
  .production_ready == false
  and .integration_verified == true
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
if missing_preflight_report="$($ROOT/scripts/hepta-active-binary-consistency-gate --pid "$fixture_pid" --installed-bin "$installed_bin" --manifest "$invalid_release/manifest.json" --expected-source-commit "$source_commit" 2>/dev/null)"; then
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
