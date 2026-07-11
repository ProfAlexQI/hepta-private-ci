#!/usr/bin/env bash

HEPTA_NATIVE_GATEWAY_SOURCE_SET_V1="hepta-native-gateway-source-set-v1"
HEPTA_RUNTIME_PUBLIC_API_SOURCE_SET_V1="hepta-runtime-public-api-source-set-v1"
HEPTA_RUNTIME_QUERY_SOURCE_SET_V1="hepta-runtime-query-source-set-v1"

hepta_source_set_paths() {
  local source_path="$1"

  case "$source_path" in
    "$HEPTA_NATIVE_GATEWAY_SOURCE_SET_V1")
      printf '%s\n' \
        "codex-rs/hepta-native-gateway/src/native_gateway.rs" \
        "codex-rs/hepta-native-gateway/src/native_gateway"
      ;;
    "$HEPTA_RUNTIME_PUBLIC_API_SOURCE_SET_V1")
      printf '%s\n' \
        "codex-rs/hepta-runtime/src/lib.rs" \
        "codex-rs/hepta-runtime/src/runtime_kernel"
      ;;
    "$HEPTA_RUNTIME_QUERY_SOURCE_SET_V1")
      printf '%s\n' \
        "codex-rs/hepta-runtime/src/query.rs" \
        "codex-rs/hepta-runtime/src/query"
      ;;
    *)
      printf '%s\n' "$source_path"
      ;;
  esac
}

hepta_source_path_validate() {
  local source_path="$1"
  local member

  while IFS= read -r member; do
    if [[ ! -f "$member" && ! -d "$member" ]]; then
      echo "missing canonical source-set member: $member" >&2
      return 1
    fi
  done < <(hepta_source_set_paths "$source_path")
}

hepta_source_path_files() {
  local source_path="$1"
  local member

  hepta_source_path_validate "$source_path" || return 1

  while IFS= read -r member; do
    if [[ -f "$member" ]]; then
      printf '%s\n' "$member"
    elif [[ -d "$member" ]]; then
      find "$member" -type f -name '*.rs' | LC_ALL=C sort
    else
      echo "missing canonical source-set member: $member" >&2
      return 1
    fi
  done < <(hepta_source_set_paths "$source_path")
}

hepta_source_path_contains() {
  local source_path="$1"
  local source_text="$2"
  local member

  hepta_source_path_validate "$source_path" || return 1

  while IFS= read -r member; do
    if rg -Fq -- "$source_text" "$member"; then
      return 0
    fi
  done < <(hepta_source_set_paths "$source_path")

  return 1
}

hepta_source_path_sha256() {
  local source_path="$1"

  hepta_source_path_validate "$source_path" || return 1

  {
    local file
    while IFS= read -r file; do
      printf '%s\0' "$file"
      shasum -a 256 "$file" | awk '{print $1}'
    done < <(hepta_source_path_files "$source_path")
  } | shasum -a 256 | awk '{print $1}'
}

# Compatibility names used by historical static gates. Sourcing this helper
# after their local definitions upgrades both functions to source-set aware
# behavior without changing their report schema.
sha256_file() {
  hepta_source_path_sha256 "$1"
}

require_source_text() {
  local source_path="$1"
  local source_text="$2"
  local label="$3"

  if ! hepta_source_path_contains "$source_path" "$source_text"; then
    echo "missing canonical source-set text: $label" >&2
    return 1
  fi
}
