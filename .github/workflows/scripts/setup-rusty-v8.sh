#!/usr/bin/env bash
set -euo pipefail

target="${1:?target triple is required}"
destination="${2:?destination directory is required}"
env_file="${3:?environment output file is required}"
repo_root="${4:-$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)}"
checksums_override="${5:-}"

if command -v cygpath >/dev/null 2>&1; then
  repo_root="$(cygpath --unix "$repo_root")"
  if [[ -n "$checksums_override" ]]; then
    checksums_override="$(cygpath --unix "$checksums_override")"
  fi
  destination="$(cygpath --unix "$destination")"
  env_file="$(cygpath --unix "$env_file")"
fi

version="$(sed -n 's/^v8 = "=\([^"]*\)"$/\1/p' "${repo_root}/codex-rs/Cargo.toml")"
if [[ -z "$version" ]]; then
  echo "Unable to resolve the workspace v8 version" >&2
  exit 1
fi

profile="ptrcomp_sandbox_release"
release_tag="rusty-v8-v${version}"
base_url="https://github.com/openai/codex/releases/download/${release_tag}"

if [[ "$target" == *-pc-windows-msvc ]]; then
  archive_name="rusty_v8_${profile}_${target}.lib.gz"
else
  archive_name="librusty_v8_${profile}_${target}.a.gz"
fi
binding_name="src_binding_${profile}_${target}.rs"
checksums_name="rusty_v8_${profile}_${target}.sha256"
checksums_path="${checksums_override:-${repo_root}/third_party/v8/rusty_v8_${version//./_}.sha256}"

if [[ ! -f "$checksums_path" ]]; then
  echo "Missing checked V8 checksum manifest: ${checksums_path}" >&2
  exit 1
fi

checksum_for() {
  local artifact="$1"
  awk -v artifact="$artifact" '$2 == artifact { print $1 }' "$checksums_path"
}

file_checksum() {
  local artifact_path="$1"
  if command -v sha256sum >/dev/null 2>&1; then
    sha256sum "$artifact_path" | awk '{ print $1 }'
  else
    shasum -a 256 "$artifact_path" | awk '{ print $1 }'
  fi
}

ensure_artifact() {
  local artifact="$1"
  local expected
  expected="$(checksum_for "$artifact")"
  if [[ -z "$expected" ]]; then
    echo "No checked checksum for ${artifact} in ${checksums_path}" >&2
    exit 1
  fi

  local artifact_path="${destination}/${artifact}"
  if [[ -f "$artifact_path" ]] && [[ "$(file_checksum "$artifact_path")" == "$expected" ]]; then
    return
  fi

  local temporary_path="${artifact_path}.tmp.$$"
  rm -f "$temporary_path"
  curl --fail --location --silent --show-error \
    --retry 5 --retry-all-errors \
    "${base_url}/${artifact}" \
    --output "$temporary_path"
  local actual
  actual="$(file_checksum "$temporary_path")"
  if [[ "$actual" != "$expected" ]]; then
    rm -f "$temporary_path"
    echo "Checksum mismatch for ${artifact}: expected ${expected}, got ${actual}" >&2
    exit 1
  fi
  mv "$temporary_path" "$artifact_path"
}

mkdir -p "$destination"
ensure_artifact "$archive_name"
ensure_artifact "$binding_name"

archive_path="${destination}/${archive_name}"
binding_path="${destination}/${binding_name}"
if command -v cygpath >/dev/null 2>&1; then
  archive_path="$(cygpath --windows "$archive_path")"
  binding_path="$(cygpath --windows "$binding_path")"
fi

{
  printf 'RUSTY_V8_ARCHIVE=%s\n' "$archive_path"
  printf 'RUSTY_V8_SRC_BINDING_PATH=%s\n' "$binding_path"
} >>"$env_file"
