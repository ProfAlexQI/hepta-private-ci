#!/usr/bin/env bash
set -euo pipefail

ROOT="${HEPTA_REPO_ROOT:-$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd -P)}"
LIB_SOURCE="$ROOT/codex-rs/hepta-runtime/src/lib.rs"

fail() {
  printf 'hepta-runtime-crate-root-source-set-v1: FAIL: %s\n' "$1" >&2
  exit 2
}

[[ -f "$LIB_SOURCE" ]] || fail "missing hepta-runtime lib source: $LIB_SOURCE"

source_files=("$LIB_SOURCE")
while IFS= read -r include_path; do
  [[ "$include_path" =~ ^runtime_kernel/[a-z0-9_]+\.rs$ ]] \
    || fail "unsupported crate-root include path: $include_path"
  include_source="$ROOT/codex-rs/hepta-runtime/src/$include_path"
  [[ -f "$include_source" ]] || fail "missing crate-root include source: $include_source"
  source_files+=("$include_source")
done < <(
  sed -nE 's/^include!\("([^"]+)"\);$/\1/p' "$LIB_SOURCE"
)

case "${1:-}" in
  list)
    [[ $# == 1 ]] || fail 'list does not accept additional arguments'
    printf '%s\n' "${source_files[@]}"
    ;;
  grep|rg)
    engine="$1"
    shift
    [[ $# -ge 2 ]] || fail "$engine requires search arguments and the hepta-runtime lib path"
    last_argument="${!#}"
    [[ "$last_argument" == "$LIB_SOURCE" ]] \
      || fail "$engine target must be the hepta-runtime lib path: $last_argument"
    forwarded_argument_count=$(( $# - 1 ))
    unset -f grep rg 2>/dev/null || true
    command "$engine" "${@:1:$forwarded_argument_count}" "${source_files[@]}"
    ;;
  *)
    fail 'usage: scripts/lib/hepta-runtime-crate-root-source-set-v1.sh <list|grep|rg> [search arguments] <hepta-runtime-lib>'
    ;;
esac
