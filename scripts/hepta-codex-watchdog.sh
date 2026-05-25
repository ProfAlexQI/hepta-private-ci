#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "$0")" && pwd -P)"

if [[ -n "${HEPTA_CODEX_RELEASE_BIN:-}" && -z "${HEPTA_RELEASE_BIN:-}" ]]; then
  export HEPTA_RELEASE_BIN="$HEPTA_CODEX_RELEASE_BIN"
fi

if [[ -n "${HEPTA_CODEX_INSTALLED_BIN:-}" && -z "${HEPTA_INSTALLED_BIN:-}" ]]; then
  export HEPTA_INSTALLED_BIN="$HEPTA_CODEX_INSTALLED_BIN"
fi

exec "$script_dir/hepta-watchdog.sh" "$@"
