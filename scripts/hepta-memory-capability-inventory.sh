#!/usr/bin/env bash
set -euo pipefail
exec "$(cd "$(dirname "$0")" && pwd -P)/hepta-script-family-alias.sh" "hepta-codex-memory-capability-inventory.sh" "$@"
