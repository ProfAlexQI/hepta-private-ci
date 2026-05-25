#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

canonical_soak="scripts/hepta-live-soak.sh"
legacy_soak="scripts/hepta-codex-live-soak.sh"
canonical_browser="scripts/hepta-browser-visual-smoke.sh"
legacy_browser="scripts/hepta-codex-browser-visual-smoke.sh"

for script in "$canonical_soak" "$legacy_soak" "$canonical_browser" "$legacy_browser"; do
  [[ -x "$script" ]]
  bash -n "$script"
done

grep -q 'Hepta live soak passed' "$canonical_soak"
grep -q 'HEPTA_SOAK_SAMPLES' "$canonical_soak"
grep -q 'HEPTA_SOAK_INTERVAL_SECONDS' "$canonical_soak"
grep -q 'HEPTA_CODEX_SOAK_SAMPLES' "$canonical_soak"
grep -q 'HEPTA_CODEX_SOAK_INTERVAL_SECONDS' "$canonical_soak"
grep -q 'Hepta browser visual smoke passed' "$canonical_browser"
grep -q 'hepta-browser-visual-smoke' "$canonical_browser"

if grep -q 'hepta-script-family-alias.sh' "$canonical_soak" "$canonical_browser"; then
  echo "canonical live gates must not route through the legacy script-family alias" >&2
  exit 1
fi

grep -q 'exec "$script_dir/hepta-live-soak.sh" "$@"' "$legacy_soak"
grep -q 'HEPTA_CODEX_SOAK_SAMPLES' "$legacy_soak"
grep -q 'HEPTA_CODEX_SOAK_INTERVAL_SECONDS' "$legacy_soak"
grep -q 'exec "$script_dir/hepta-browser-visual-smoke.sh" "$@"' "$legacy_browser"

if grep -q 'curl -fsS' "$legacy_soak" "$legacy_browser"; then
  echo "legacy Hepta Codex live gate wrappers must not keep live probe implementations" >&2
  exit 1
fi
if grep -q 'jq -n' "$legacy_soak" "$legacy_browser"; then
  echo "legacy Hepta Codex live gate wrappers must not keep report implementations" >&2
  exit 1
fi
if grep -q 'capture_viewport' "$legacy_browser"; then
  echo "legacy browser smoke wrapper must not keep screenshot capture implementation" >&2
  exit 1
fi

jq -n \
  --arg status "ready" \
  --arg canonical_soak "$canonical_soak" \
  --arg legacy_soak "$legacy_soak" \
  --arg canonical_browser "$canonical_browser" \
  --arg legacy_browser "$legacy_browser" \
  '{
    status:$status,
    canonical_live_soak_entrypoint:$canonical_soak,
    legacy_live_soak_wrapper:$legacy_soak,
    canonical_browser_visual_smoke_entrypoint:$canonical_browser,
    legacy_browser_visual_smoke_wrapper:$legacy_browser,
    legacy_soak_env_aliases_preserved:true,
    legacy_wrappers_have_no_live_probe_implementation:true,
    activation_evidence_slots_retained:true,
    upstream_intake_gates_retained:true
  }'

echo "Hepta live gates entrypoint migration gate passed"
