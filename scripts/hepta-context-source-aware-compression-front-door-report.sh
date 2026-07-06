#!/usr/bin/env bash
set -uo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
front_door_gate="$repo_root/scripts/hepta-context-source-aware-compression-front-door-gate.sh"
front_door_output="$(mktemp -t hepta-context-source-aware-front-door.XXXXXX)"

cleanup() {
  rm -f "$front_door_output"
}
trap cleanup EXIT

status="pass"
bash "$front_door_gate" >"$front_door_output" 2>&1
exit_code=$?
if [ "$exit_code" -ne 0 ]; then
  status="fail"
fi

cat "$front_door_output"

runtime_dirty_classifier="none"
if grep -F "non-blocking sibling runtime generated preview dirty state detected:" "$front_door_output" >/dev/null; then
  runtime_dirty_classifier="non-blocking"
fi

if [ "$status" = "pass" ]; then
  echo "source-aware-contracts=pass"
  echo "source-aware-contracts.front-door=pass"
else
  echo "source-aware-contracts=fail"
  echo "source-aware-contracts.front-door=fail"
fi
echo "source-aware-contracts.runtime-dirty-classifier=$runtime_dirty_classifier"
echo "source-aware-contracts.runtime-activation=disabled"
echo "source-aware-contracts.gates=readiness,operator-approval-evidence,readiness-export,activation-negative-matrix,activation-surface,leak-bait,positive-route-readiness,positive-route-change-detector"

exit "$exit_code"
