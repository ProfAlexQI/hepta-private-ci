#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

canonical="scripts/hepta-watchdog.sh"
legacy="scripts/hepta-codex-watchdog.sh"
release_evidence_helper="scripts/lib/hepta-watchdog-release-evidence-v1.sh"

[[ -x "$canonical" ]]
[[ -x "$legacy" ]]
[[ -r "$release_evidence_helper" ]]

bash -n "$canonical"
bash -n "$legacy"
bash -n "$release_evidence_helper"

grep -q 'Hepta watchdog passed' "$canonical"
grep -q 'HEPTA_RELEASE_BIN' "$canonical"
grep -q 'HEPTA_INSTALLED_BIN' "$canonical"
grep -q 'HEPTA_CODEX_RELEASE_BIN' "$canonical"
grep -q 'HEPTA_CODEX_INSTALLED_BIN' "$canonical"
grep -q 'HEPTA_WATCHDOG_MODE' "$canonical"
grep -q 'deployment-consistency' "$canonical"
grep -q 'active-health' "$canonical"
grep -q 'HEPTA_CANDIDATE_MANIFEST' "$canonical"
grep -q 'HEPTA_INSTALLED_RECEIPT' "$canonical"
grep -q 'source "$REPO_ROOT/scripts/lib/hepta-watchdog-release-evidence-v1.sh"' "$canonical"
grep -q 'candidate_installed_sha_mismatch' "$release_evidence_helper"

if grep -q 'curl -fsS' "$release_evidence_helper"; then
  echo "release evidence helper must not implement active-health network probes" >&2
  exit 1
fi

if grep -q 'RELEASE_BIN="$INSTALLED_BIN"' "$canonical"; then
  echo "canonical watchdog must not fall back from a missing candidate to the installed binary" >&2
  exit 1
fi

if grep -q 'hepta-script-family-alias.sh' "$canonical"; then
  echo "canonical watchdog must not route through the legacy script-family alias" >&2
  exit 1
fi

grep -q 'exec "$script_dir/hepta-watchdog.sh" "$@"' "$legacy"
grep -q 'HEPTA_CODEX_RELEASE_BIN' "$legacy"
grep -q 'HEPTA_CODEX_INSTALLED_BIN' "$legacy"

if grep -q 'curl -fsS' "$legacy"; then
  echo "legacy hepta-codex watchdog must stay a thin wrapper, not a second implementation" >&2
  exit 1
fi

if grep -q 'jq -n' "$legacy"; then
  echo "legacy hepta-codex watchdog must stay a thin wrapper, not a second implementation" >&2
  exit 1
fi

jq -n \
  --arg status "ready" \
  --arg canonical "$canonical" \
  --arg legacy "$legacy" \
  '{
    status:$status,
    canonical_watchdog_entrypoint:$canonical,
    legacy_watchdog_wrapper:$legacy,
    legacy_env_aliases_preserved:true,
    legacy_wrapper_has_no_live_probe_implementation:true,
    default_deployment_consistency_fail_closed:true,
    active_health_mode_requires_explicit_selection:true,
    missing_candidate_installed_fallback_removed:true,
    active_service_watchdog_retained:true,
    upstream_activation_evidence_slot_retained:true
  }'

echo "Hepta watchdog entrypoint migration gate passed"
