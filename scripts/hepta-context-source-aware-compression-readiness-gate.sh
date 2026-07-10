#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

fail() {
  echo "hepta-context-source-aware-compression-readiness-gate: $*" >&2
  exit 1
}

relative_paths() {
  sed "s#^$repo_root/##" | sort -u
}

assert_file_contains() {
  local file_path="$1"
  local needle="$2"
  local label="$3"

  if ! grep -F "$needle" "$file_path" >/dev/null; then
    fail "$label must contain: $needle"
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

line_number_of() {
  local file_path="$1"
  local needle="$2"
  local line

  line="$(grep -n -F "$needle" "$file_path" | head -n 1 | cut -d: -f1 || true)"
  if [ -z "$line" ]; then
    fail "$file_path is missing required text: $needle"
  fi
  printf '%s\n' "$line"
}

assert_line_before() {
  local file_path="$1"
  local before_needle="$2"
  local after_needle="$3"
  local label="$4"
  local before_line
  local after_line

  before_line="$(line_number_of "$file_path" "$before_needle")"
  after_line="$(line_number_of "$file_path" "$after_needle")"
  if [ "$before_line" -ge "$after_line" ]; then
    fail "$label expected '$before_needle' before '$after_needle'"
  fi
}

production_roots=(
  "$repo_root/codex-rs/app-server-protocol/src"
  "$repo_root/codex-rs/app-server/src"
  "$repo_root/codex-rs/app-server/tests"
  "$repo_root/codex-rs/hepta-native-gateway/src/native_gateway.rs"
  "$repo_root/codex-rs/exec/src"
  "$repo_root/codex-rs/hepta-runtime/src"
  "$repo_root/codex-rs/response-debug-context/src"
  "$repo_root/codex-rs/tui/src"
)

context_contracts="$repo_root/codex-rs/CONTEXT_DEBUG_CONTRACTS.md"
preflight_script="$repo_root/scripts/hepta-context-preflight.sh"
reserved_runtime_activation_entrypoint="apply_source_aware_compression_operator_approved_runtime_activation_marker"
reserved_runtime_activation_key="source_aware_compression_operator_approved_runtime_activation"

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
  "explicit helper-injected turn-scoped source-aware compression" \
  "reserved source-aware compression readiness contract"

assert_file_contains \
  "$context_contracts" \
  "no rollout/debug/export marker or canary leakage" \
  "reserved source-aware compression readiness contract"

assert_file_contains \
  "$context_contracts" \
  "default runtime must remain on the non-omitting replay baseline" \
  "reserved source-aware compression readiness contract"

assert_file_contains \
  "$context_contracts" \
  "source-aware compression readiness gate" \
  "reserved source-aware compression readiness contract"

assert_file_contains \
  "$context_contracts" \
  "before runtime cargo stages" \
  "reserved source-aware compression readiness contract"

assert_fixed_string_absent \
  "$reserved_runtime_activation_entrypoint" \
  "reserved source-aware compression runtime activation entrypoint production consumption" \
  "${production_roots[@]}"

assert_fixed_string_absent \
  "$reserved_runtime_activation_key" \
  "reserved source-aware compression runtime activation key production consumption" \
  "${production_roots[@]}"

assert_regex_absent \
  '^[^"]*source_aware_compression.*runtime.*activation' \
  "ad-hoc source-aware compression runtime activation production code" \
  "${production_roots[@]}"

assert_file_contains \
  "$preflight_script" \
  "source-aware compression readiness checklist gate" \
  "source-aware compression readiness preflight stage"

assert_line_before \
  "$preflight_script" \
  "source-aware compression readiness checklist gate" \
  "hepta-runtime recall selector budget fixtures" \
  "source-aware compression readiness preflight stage order"

echo "Hepta context source-aware compression readiness gate passed"
