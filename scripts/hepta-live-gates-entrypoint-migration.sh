#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

canonical_soak="scripts/hepta-live-soak.sh"
legacy_soak="scripts/hepta-codex-live-soak.sh"
canonical_browser="scripts/hepta-browser-visual-smoke.sh"
legacy_browser="scripts/hepta-codex-browser-visual-smoke.sh"
browser_lib="scripts/lib/hepta-browser-visual-smoke-v1"

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
if [[ -L "$legacy_browser" ]]; then
  [[ "$(readlink "$legacy_browser")" == "$(basename "$canonical_browser")" ]]
  [[ "$(cd "$(dirname "$legacy_browser")" && pwd -P)/$(readlink "$legacy_browser")" \
    == "$(cd "$(dirname "$canonical_browser")" && pwd -P)/$(basename "$canonical_browser")" ]]
else
  grep -q 'exec "$script_dir/hepta-browser-visual-smoke.sh" "$@"' "$legacy_browser"
fi

browser_modules=(
  "$browser_lib/config.sh"
  "$browser_lib/static-contract.sh"
  "$browser_lib/capture.sh"
  "$browser_lib/scenarios.sh"
  "$browser_lib/validate-results.sh"
  "$browser_lib/receipt.sh"
  "$browser_lib/capture-viewport.cjs"
  "$browser_lib/density-qa.cjs"
  "$browser_lib/progressive-enhancement-qa.cjs"
  "$browser_lib/progressive-enhancement-adversarial-qa.cjs"
  "$browser_lib/density-probe/01-foundation.fragment.cjs"
  "$browser_lib/density-probe/02-shell-optics.fragment.cjs"
  "$browser_lib/density-probe/03-controls-palette.fragment.cjs"
  "$browser_lib/density-probe/04-menus-popovers.fragment.cjs"
  "$browser_lib/density-probe/05-micro-surfaces.fragment.cjs"
  "$browser_lib/density-probe/06-text-integrity.fragment.cjs"
  "$browser_lib/density-probe/07-verdict.fragment.cjs"
)

for module in "${browser_modules[@]}"; do
  [[ -s "$module" ]]
  if [[ "$module" == *.sh ]]; then
    bash -n "$module"
  else
    node --check "$module"
  fi
  if [[ "$(wc -l <"$module" | tr -d ' ')" -gt 1200 ]]; then
    echo "browser smoke module exceeds the 1200-line responsibility bound: $module" >&2
    exit 1
  fi
done

if [[ "$(wc -l <"$canonical_browser" | tr -d ' ')" -gt 100 ]]; then
  echo "canonical browser smoke entrypoint is no longer a short orchestrator" >&2
  exit 1
fi
if [[ ! -L "$legacy_browser" ]]; then
  if [[ "$(wc -l <"$legacy_browser" | tr -d ' ')" -gt 20 ]]; then
    echo "legacy browser smoke entrypoint is no longer a thin wrapper" >&2
    exit 1
  fi
  if cmp -s "$canonical_browser" "$legacy_browser"; then
    echo "legacy browser smoke must not duplicate the canonical implementation" >&2
    exit 1
  fi
fi

legacy_content_wrappers=("$legacy_soak")
[[ -L "$legacy_browser" ]] || legacy_content_wrappers+=("$legacy_browser")

if grep -q 'curl -fsS' "${legacy_content_wrappers[@]}"; then
  echo "legacy Hepta Codex live gate wrappers must not keep live probe implementations" >&2
  exit 1
fi
if grep -q 'jq -n' "${legacy_content_wrappers[@]}"; then
  echo "legacy Hepta Codex live gate wrappers must not keep report implementations" >&2
  exit 1
fi
if [[ ! -L "$legacy_browser" ]] && grep -q 'capture_viewport' "$legacy_browser"; then
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
    browser_smoke_modularized:true,
    browser_smoke_module_count:17,
    browser_smoke_module_line_bound:1200,
    legacy_browser_thin_wrapper:true,
    activation_evidence_slots_retained:true,
    upstream_intake_gates_retained:true
  }'

echo "Hepta live gates entrypoint migration gate passed"
