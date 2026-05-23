#!/usr/bin/env bash
set -euo pipefail

target_name="${1:-}"
if [[ -z "$target_name" || "$target_name" != hepta-codex-*.sh ]]; then
  echo "usage: $(basename "$0") hepta-codex-<script>.sh [args...]" >&2
  exit 64
fi
shift

script_dir="$(cd "$(dirname "$0")" && pwd -P)"
target_path="$script_dir/$target_name"
if [[ ! -x "$target_path" ]]; then
  echo "missing executable transition script: $target_path" >&2
  exit 66
fi

exec "$target_path" "$@"
