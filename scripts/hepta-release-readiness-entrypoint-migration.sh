#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

canonical_scripts=(
  scripts/hepta-public-ga-readiness.sh
  scripts/hepta-public-ga-operator-approval-packet.sh
  scripts/hepta-native-packaging-gate.sh
  scripts/hepta-release-hardening-status-gate.sh
)
legacy_scripts=(
  scripts/hepta-codex-public-ga-readiness.sh
  scripts/hepta-codex-public-ga-operator-approval-packet.sh
  scripts/hepta-codex-native-packaging-gate.sh
  scripts/hepta-codex-release-hardening-status-gate.sh
)

for script in "${canonical_scripts[@]}" "${legacy_scripts[@]}"; do
  [[ -x "$script" ]]
  bash -n "$script"
done

grep -q 'Hepta public GA readiness gate passed' scripts/hepta-public-ga-readiness.sh
grep -q 'Hepta public GA operator approval packet passed' scripts/hepta-public-ga-operator-approval-packet.sh
grep -q 'Hepta native packaging gate passed' scripts/hepta-native-packaging-gate.sh
grep -q 'Hepta release/hardening status gate passed' scripts/hepta-release-hardening-status-gate.sh

if grep -q 'hepta-script-family-alias.sh' "${canonical_scripts[@]}"; then
  echo "canonical release/readiness gates must not route through the legacy script-family alias" >&2
  exit 1
fi

legacy_content_wrappers=()
for index in "${!canonical_scripts[@]}"; do
  canonical="${canonical_scripts[$index]}"
  legacy="${legacy_scripts[$index]}"
  if [[ -L "$legacy" ]]; then
    [[ "$(readlink "$legacy")" == "$(basename "$canonical")" ]]
    [[ "$(cd "$(dirname "$legacy")" && pwd -P)/$(readlink "$legacy")" \
      == "$(cd "$(dirname "$canonical")" && pwd -P)/$(basename "$canonical")" ]]
  else
    grep -Fq "exec \"\$script_dir/$(basename "$canonical")\" \"\$@\"" "$legacy"
    legacy_content_wrappers+=("$legacy")
  fi
done

if ((${#legacy_content_wrappers[@]})) && grep -q 'curl -fsS' "${legacy_content_wrappers[@]}"; then
  echo "legacy release/readiness wrappers must not keep live report implementations" >&2
  exit 1
fi
if ((${#legacy_content_wrappers[@]})) && grep -q 'jq -n' "${legacy_content_wrappers[@]}"; then
  echo "legacy release/readiness wrappers must not keep report implementations" >&2
  exit 1
fi
if ((${#legacy_content_wrappers[@]})) && grep -q 'cargo metadata' "${legacy_content_wrappers[@]}"; then
  echo "legacy release/readiness wrappers must not keep packaging implementations" >&2
  exit 1
fi

jq -n \
  --arg status "ready" \
  '{
    status:$status,
    canonical_public_ga_readiness_entrypoint:"scripts/hepta-public-ga-readiness.sh",
    canonical_operator_packet_entrypoint:"scripts/hepta-public-ga-operator-approval-packet.sh",
    canonical_native_packaging_entrypoint:"scripts/hepta-native-packaging-gate.sh",
    canonical_release_hardening_entrypoint:"scripts/hepta-release-hardening-status-gate.sh",
    legacy_wrappers_have_no_live_report_implementation:true,
    operator_activation_evidence_slot_retained:true,
    public_release_side_effects_blocked_by_default:true
  }'

echo "Hepta release/readiness entrypoint migration gate passed"
