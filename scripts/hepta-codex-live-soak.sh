#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "$0")" && pwd -P)"

if [[ -n "${HEPTA_CODEX_SOAK_SAMPLES:-}" && -z "${HEPTA_SOAK_SAMPLES:-}" ]]; then
  export HEPTA_SOAK_SAMPLES="$HEPTA_CODEX_SOAK_SAMPLES"
fi
if [[ -n "${HEPTA_CODEX_SOAK_INTERVAL_SECONDS:-}" && -z "${HEPTA_SOAK_INTERVAL_SECONDS:-}" ]]; then
  export HEPTA_SOAK_INTERVAL_SECONDS="$HEPTA_CODEX_SOAK_INTERVAL_SECONDS"
fi

exec "$script_dir/hepta-live-soak.sh" "$@"
