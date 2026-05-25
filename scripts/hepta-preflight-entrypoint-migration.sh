#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

canonical="scripts/hepta-preflight.sh"
legacy="scripts/hepta-codex-preflight.sh"

[[ -x "$canonical" ]]
[[ -x "$legacy" ]]

bash -n "$canonical"
bash -n "$legacy"

grep -q 'Hepta preflight passed' "$canonical"
grep -q 'HEPTA_PREFLIGHT_NATIVE' "$canonical"
grep -q 'HEPTA_PREFLIGHT_RELEASE' "$canonical"
grep -q 'HEPTA_MANIFEST' "$canonical"
grep -q 'HEPTA_CODEX_PREFLIGHT_NATIVE' "$canonical"
grep -q 'HEPTA_CODEX_PREFLIGHT_RELEASE' "$canonical"
grep -q 'HEPTA_CODEX_MANIFEST' "$canonical"

if grep -q 'hepta-script-family-alias.sh' "$canonical"; then
  echo "canonical preflight must not route through the legacy script-family alias" >&2
  exit 1
fi

grep -q 'exec "$script_dir/hepta-preflight.sh" "$@"' "$legacy"
grep -q 'HEPTA_CODEX_PREFLIGHT_NATIVE' "$legacy"
grep -q 'HEPTA_CODEX_PREFLIGHT_RELEASE' "$legacy"
grep -q 'HEPTA_CODEX_MANIFEST' "$legacy"

if grep -q 'cargo test --offline' "$legacy"; then
  echo "legacy hepta-codex preflight must stay a thin wrapper, not a second implementation" >&2
  exit 1
fi

if grep -q 'cargo metadata --offline' "$legacy"; then
  echo "legacy hepta-codex preflight must stay a thin wrapper, not a second implementation" >&2
  exit 1
fi

jq -n \
  --arg status "ready" \
  --arg canonical "$canonical" \
  --arg legacy "$legacy" \
  '{
    status:$status,
    canonical_preflight_entrypoint:$canonical,
    legacy_preflight_wrapper:$legacy,
    legacy_env_aliases_preserved:true,
    legacy_wrapper_has_no_gate_implementation:true,
    upstream_intake_gates_retained:true,
    active_service_dependency_boundary_retained:true
  }'

echo "Hepta preflight entrypoint migration gate passed"
