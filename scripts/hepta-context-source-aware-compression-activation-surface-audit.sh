#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

fail() {
  echo "hepta-context-source-aware-compression-activation-surface-audit: $*" >&2
  exit 1
}

join_lines() {
  if [ "$#" -eq 0 ]; then
    return 0
  fi
  printf '%s\n' "$@" | sort -u
}

relative_paths() {
  sed "s#^$repo_root/##" | sort -u
}

assert_fixed_string_paths_match() {
  local needle="$1"
  local label="$2"
  local expected_count="$3"
  shift 3
  local expected=()
  local roots=()
  local expected_text
  local actual_text

  while [ "$#" -gt 0 ]; do
    if [ "$1" = "--" ]; then
      shift
      break
    fi
    expected+=("$1")
    shift
  done
  roots=("$@")

  if [ "${#expected[@]}" -ne "$expected_count" ]; then
    fail "$label internal expected path count mismatch"
  fi

  expected_text="$(join_lines "${expected[@]}")"
  actual_text="$(
    { rg -l --fixed-strings "$needle" "${roots[@]}" || true; } \
      | relative_paths
  )"

  if [ "$actual_text" != "$expected_text" ]; then
    fail "$label path allowlist mismatch; expected $(printf '%s' "$expected_text" | tr '\n' ','), got $(printf '%s' "$actual_text" | tr '\n' ',')"
  fi
}

assert_regex_paths_match() {
  local pattern="$1"
  local label="$2"
  local expected_count="$3"
  shift 3
  local expected=()
  local roots=()
  local expected_text
  local actual_text

  while [ "$#" -gt 0 ]; do
    if [ "$1" = "--" ]; then
      shift
      break
    fi
    expected+=("$1")
    shift
  done
  roots=("$@")

  if [ "${#expected[@]}" -ne "$expected_count" ]; then
    fail "$label internal expected path count mismatch"
  fi

  expected_text="$(join_lines "${expected[@]}")"
  actual_text="$(
    { rg -l "$pattern" "${roots[@]}" || true; } \
      | relative_paths
  )"

  if [ "$actual_text" != "$expected_text" ]; then
    fail "$label path allowlist mismatch; expected $(printf '%s' "$expected_text" | tr '\n' ','), got $(printf '%s' "$actual_text" | tr '\n' ',')"
  fi
}

assert_fixed_string_absent() {
  local needle="$1"
  local label="$2"
  shift 2
  local matches

  matches="$(
    { rg -n --fixed-strings "$needle" "$@" || true; } \
      | relative_paths
  )"

  if [ -n "$matches" ]; then
    fail "$label must not contain $needle; found $(printf '%s' "$matches" | tr '\n' ',')"
  fi
}

assert_regex_absent() {
  local pattern="$1"
  local label="$2"
  shift 2
  local matches

  matches="$(
    { rg -n "$pattern" "$@" || true; } \
      | relative_paths
  )"

  if [ -n "$matches" ]; then
    fail "$label must not match $pattern; found $(printf '%s' "$matches" | tr '\n' ',')"
  fi
}

assert_file_contains() {
  local file_path="$1"
  local needle="$2"
  local label="$3"

  if ! grep -F "$needle" "$file_path" >/dev/null; then
    fail "$label must contain: $needle"
  fi
}

code_roots=(
  "$repo_root/codex-rs/app-server-protocol/src"
  "$repo_root/codex-rs/app-server/src"
  "$repo_root/codex-rs/app-server/tests"
  "$repo_root/codex-rs/hepta-native-gateway/src/native_gateway.rs"
  "$repo_root/codex-rs/core/src"
  "$repo_root/codex-rs/exec/src"
  "$repo_root/codex-rs/hepta-runtime/src"
  "$repo_root/codex-rs/response-debug-context/src"
  "$repo_root/codex-rs/tui/src"
)

activation_surface_roots=(
  "$repo_root/codex-rs/app-server-protocol/src"
  "$repo_root/codex-rs/app-server/src"
  "$repo_root/codex-rs/app-server/tests"
  "$repo_root/codex-rs/hepta-native-gateway/src/native_gateway.rs"
  "$repo_root/codex-rs/exec/src"
  "$repo_root/codex-rs/hepta-runtime/src"
  "$repo_root/codex-rs/response-debug-context/src"
  "$repo_root/codex-rs/tui/src"
)

helper_name="insert_source_aware_compression_policy_opt_in_marker"
reserved_runtime_activation_entrypoint="apply_source_aware_compression_operator_approved_runtime_activation_marker"
reserved_runtime_activation_key="source_aware_compression_operator_approved_runtime_activation"
context_contracts="$repo_root/codex-rs/CONTEXT_DEBUG_CONTRACTS.md"

assert_fixed_string_paths_match \
  "$helper_name" \
  "source-aware compression marker helper" \
  3 \
  "codex-rs/core/src/context_manager/manifest.rs" \
  "codex-rs/core/src/context_manager/manifest/tests.rs" \
  "codex-rs/core/src/session/tests/contract_part_04.rs" \
  -- \
  "${code_roots[@]}"

assert_fixed_string_absent \
  "$helper_name" \
  "source-aware compression activation surfaces" \
  "${activation_surface_roots[@]}"

assert_regex_paths_match \
  '^[^"]*extension_data\.insert\(TurnContextAssemblyPolicyOptIn::SourceAwareCompression\)' \
  "raw source-aware compression marker extension-data insert" \
  1 \
  "codex-rs/core/src/context_manager/manifest.rs" \
  -- \
  "${code_roots[@]}"

assert_regex_paths_match \
  '^[^"]*TurnContextAssemblyPolicyOptIn::SourceAwareCompression' \
  "raw source-aware compression marker code reference" \
  1 \
  "codex-rs/core/src/context_manager/manifest.rs" \
  -- \
  "${code_roots[@]}"

assert_fixed_string_paths_match \
  "turn_context_assembly_policy_from_extension_data" \
  "source-aware compression assembly policy resolver" \
  3 \
  "codex-rs/core/src/context_manager/manifest.rs" \
  "codex-rs/core/src/context_manager/manifest/tests.rs" \
  "codex-rs/core/src/session/mod.rs" \
  -- \
  "${code_roots[@]}"

assert_fixed_string_absent \
  "$reserved_runtime_activation_entrypoint" \
  "reserved source-aware compression runtime activation entrypoint is not implemented yet" \
  "${code_roots[@]}"

assert_fixed_string_absent \
  "$reserved_runtime_activation_key" \
  "reserved source-aware compression runtime activation key is not implemented yet" \
  "${code_roots[@]}"

assert_regex_absent \
  '^[^"]*source_aware_compression.*runtime.*activation' \
  "ad-hoc source-aware compression runtime activation code" \
  "${code_roots[@]}"

assert_file_contains \
  "$context_contracts" \
  "Runtime activation readiness checklist" \
  "reserved source-aware compression readiness contract"

assert_file_contains \
  "$context_contracts" \
  "operator approval evidence" \
  "reserved source-aware compression readiness contract"

assert_file_contains \
  "$context_contracts" \
  'the `source_aware_compression_canary`' \
  "reserved source-aware compression readiness contract"

assert_file_contains \
  "$context_contracts" \
  "feature enabled" \
  "reserved source-aware compression readiness contract"

assert_file_contains \
  "$context_contracts" \
  "an explicit helper-injected turn-scoped source-aware compression" \
  "reserved source-aware compression readiness contract"

assert_file_contains \
  "$context_contracts" \
  "marker" \
  "reserved source-aware compression readiness contract"

assert_file_contains \
  "$context_contracts" \
  "no rollout/debug/export marker or canary leakage" \
  "reserved source-aware compression readiness contract"

assert_file_contains \
  "$context_contracts" \
  "default runtime must remain on the non-omitting replay baseline" \
  "reserved source-aware compression readiness contract"

echo "Hepta context source-aware compression activation surface audit passed"
