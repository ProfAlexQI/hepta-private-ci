#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

HEPTA_CONTEXT_PREFLIGHT_SKIP_RUNTIME=1 \
  bash "$repo_root/scripts/hepta-context-preflight.sh"

echo "hepta-context-non-runtime-preflight=pass"
echo "hepta-context-non-runtime-preflight.runtime-stages=skipped"
echo "hepta-context-non-runtime-preflight.runtime-activation=disabled"
