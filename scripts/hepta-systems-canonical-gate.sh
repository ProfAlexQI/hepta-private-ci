#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
TARGET="$ROOT/scripts/hepta-systems-current-canonical-wrapper-gate.sh"

if [[ ! -x "$TARGET" ]]; then
  printf 'hepta-systems-canonical-gate: FAIL: missing executable current canonical wrapper gate: %s\n' "$TARGET" >&2
  exit 1
fi

exec "$TARGET" "$@"
