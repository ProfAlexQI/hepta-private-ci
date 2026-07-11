#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
exec "$ROOT/scripts/hepta-gate-pair-runner" report "hepta-systems-dirty-worktree-release-boundary-owner-freeze-classification-operator-decision-checklist-packet-readback-without-git-mutation"
