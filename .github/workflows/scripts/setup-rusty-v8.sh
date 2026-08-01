#!/usr/bin/env bash
set -euo pipefail

target="${1:?target triple is required}"
destination="${2:?destination directory is required}"
env_file="${3:?environment output file is required}"

if command -v cygpath >/dev/null 2>&1; then
  destination="$(cygpath --unix "$destination")"
  env_file="$(cygpath --unix "$env_file")"
fi

version="$(sed -n 's/^v8 = "=\([^"]*\)"$/\1/p' codex-rs/Cargo.toml)"
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

mkdir -p "$destination"
for artifact in "$archive_name" "$binding_name" "$checksums_name"; do
  curl --fail --location --silent --show-error \
    --retry 5 --retry-all-errors \
    "${base_url}/${artifact}" \
    --output "${destination}/${artifact}"
done

if [[ "$(wc -l <"${destination}/${checksums_name}")" -ne 2 ]]; then
  echo "Expected exactly two checksums in ${checksums_name}" >&2
  exit 1
fi

if command -v sha256sum >/dev/null 2>&1; then
  (cd "$destination" && sha256sum --check "$checksums_name")
else
  (cd "$destination" && shasum -a 256 --check "$checksums_name")
fi

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
