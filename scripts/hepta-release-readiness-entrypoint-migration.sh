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

grep -q 'exec "$script_dir/hepta-public-ga-readiness.sh" "$@"' scripts/hepta-codex-public-ga-readiness.sh
grep -q 'exec "$script_dir/hepta-public-ga-operator-approval-packet.sh" "$@"' scripts/hepta-codex-public-ga-operator-approval-packet.sh
grep -q 'exec "$script_dir/hepta-native-packaging-gate.sh" "$@"' scripts/hepta-codex-native-packaging-gate.sh
grep -q 'exec "$script_dir/hepta-release-hardening-status-gate.sh" "$@"' scripts/hepta-codex-release-hardening-status-gate.sh

if grep -q 'curl -fsS' "${legacy_scripts[@]}"; then
  echo "legacy release/readiness wrappers must not keep live report implementations" >&2
  exit 1
fi
if grep -q 'jq -n' "${legacy_scripts[@]}"; then
  echo "legacy release/readiness wrappers must not keep report implementations" >&2
  exit 1
fi
if grep -q 'cargo metadata' "${legacy_scripts[@]}"; then
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
