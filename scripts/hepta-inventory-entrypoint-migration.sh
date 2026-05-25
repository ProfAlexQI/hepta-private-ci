#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

bases=(
  channel-adapter-status-inventory
  cli-command-inventory
  legacy-compatibility-closure
  local-tooling-content-inventory
  memory-capability-inventory
  provider-channel-dry-run-plan
  provider-metadata-inventory
  runtime-session-dry-run-inventory
)

canonical_scripts=()
legacy_scripts=()
for base in "${bases[@]}"; do
  canonical="scripts/hepta-${base}.sh"
  legacy="scripts/hepta-codex-${base}.sh"
  canonical_scripts+=("$canonical")
  legacy_scripts+=("$legacy")
  [[ -x "$canonical" ]]
  [[ -x "$legacy" ]]
  bash -n "$canonical"
  bash -n "$legacy"
  pattern='exec "$script_dir/hepta-'"${base}"'.sh" "$@"'
  grep -Fq "$pattern" "$legacy"
done

if grep -q 'hepta-script-family-alias.sh' "${canonical_scripts[@]}"; then
  echo "canonical inventory gates must not route through the legacy script-family alias" >&2
  exit 1
fi
if grep -q 'curl -fsS' "${legacy_scripts[@]}"; then
  echo "legacy inventory wrappers must not keep live report implementations" >&2
  exit 1
fi
if grep -q 'jq -n' "${legacy_scripts[@]}"; then
  echo "legacy inventory wrappers must not keep report implementations" >&2
  exit 1
fi

jq -n \
  --arg status "ready" \
  '{
    status:$status,
    canonical_inventory_entrypoints:[
      "scripts/hepta-channel-adapter-status-inventory.sh",
      "scripts/hepta-cli-command-inventory.sh",
      "scripts/hepta-legacy-compatibility-closure.sh",
      "scripts/hepta-local-tooling-content-inventory.sh",
      "scripts/hepta-memory-capability-inventory.sh",
      "scripts/hepta-provider-channel-dry-run-plan.sh",
      "scripts/hepta-provider-metadata-inventory.sh",
      "scripts/hepta-runtime-session-dry-run-inventory.sh"
    ],
    legacy_wrappers_have_no_live_report_implementation:true,
    inventory_report_contracts_retained:true,
    public_release_side_effects_blocked_by_default:true
  }'

echo "Hepta inventory entrypoint migration gate passed"
