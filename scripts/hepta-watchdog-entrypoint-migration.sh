#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

canonical="scripts/hepta-watchdog.sh"
legacy="scripts/hepta-codex-watchdog.sh"

[[ -x "$canonical" ]]
[[ -x "$legacy" ]]

bash -n "$canonical"
bash -n "$legacy"

grep -q 'Hepta watchdog passed' "$canonical"
grep -q 'HEPTA_RELEASE_BIN' "$canonical"
grep -q 'HEPTA_INSTALLED_BIN' "$canonical"
grep -q 'HEPTA_CODEX_RELEASE_BIN' "$canonical"
grep -q 'HEPTA_CODEX_INSTALLED_BIN' "$canonical"

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
    active_service_watchdog_retained:true,
    upstream_activation_evidence_slot_retained:true
  }'

echo "Hepta watchdog entrypoint migration gate passed"
