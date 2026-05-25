#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "$0")" && pwd -P)"

if [[ -n "${HEPTA_CODEX_PREFLIGHT_NATIVE:-}" && -z "${HEPTA_PREFLIGHT_NATIVE:-}" ]]; then
  export HEPTA_PREFLIGHT_NATIVE="$HEPTA_CODEX_PREFLIGHT_NATIVE"
fi

if [[ -n "${HEPTA_CODEX_PREFLIGHT_RELEASE:-}" && -z "${HEPTA_PREFLIGHT_RELEASE:-}" ]]; then
  export HEPTA_PREFLIGHT_RELEASE="$HEPTA_CODEX_PREFLIGHT_RELEASE"
fi

if [[ -n "${HEPTA_CODEX_MANIFEST:-}" && -z "${HEPTA_MANIFEST:-}" ]]; then
  export HEPTA_MANIFEST="$HEPTA_CODEX_MANIFEST"
fi

exec "$script_dir/hepta-preflight.sh" "$@"
