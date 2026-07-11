#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

fail() {
  echo "hepta-context-source-aware-compression-activation-negative-matrix-gate: $*" >&2
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
  "$repo_root/codex-rs/hepta-native-gateway/src"
  "$repo_root/codex-rs/core/src"
  "$repo_root/codex-rs/exec/src"
  "$repo_root/codex-rs/hepta-runtime/src"
  "$repo_root/codex-rs/response-debug-context/src"
  "$repo_root/codex-rs/tui/src"
)

context_contracts="$repo_root/codex-rs/CONTEXT_DEBUG_CONTRACTS.md"
debug_gate="$repo_root/scripts/hepta-context-debug-gate.sh"
preflight_script="$repo_root/scripts/hepta-context-preflight.sh"
negative_matrix_gate_script="hepta-context-source-aware-compression-activation-negative-matrix-gate.sh"

required_contract_terms=(
  "Source-aware compression activation negative matrix"
  "baseline/non-rewriting behavior"
  "no summary/defragment/prune prompt marker"
  "no activation evidence persistence"
  "no rollout/debug/export routing leakage"
  "reserved runtime activation route"
  '`source_aware_compression_canary` feature enabled state'
  "helper-injected"
  "turn-scoped source-aware compression marker"
  '`source_aware_compression_operator_approval_evidence` record'
  "Only the complete"
  "positive set of reserved route + canary + helper-injected marker + approval"
  "positive route remains unimplemented"
)

for term in "${required_contract_terms[@]}"; do
  assert_file_contains \
    "$context_contracts" \
    "$term" \
    "source-aware compression activation negative matrix contract"
done

negative_case_labels=(
  "missing-route"
  "missing-canary"
  "missing-helper-marker"
  "missing-approval-evidence"
  "missing-route+canary"
  "missing-route+helper-marker"
  "missing-route+approval-evidence"
  "missing-canary+helper-marker"
  "missing-canary+approval-evidence"
  "missing-helper-marker+approval-evidence"
  "missing-route+canary+helper-marker"
  "missing-route+canary+approval-evidence"
  "missing-route+helper-marker+approval-evidence"
  "missing-canary+helper-marker+approval-evidence"
  "missing-route+canary+helper-marker+approval-evidence"
)

for label in "${negative_case_labels[@]}"; do
  assert_file_contains \
    "$context_contracts" \
    "$label" \
    "source-aware compression activation negative matrix case"
done

contract_only_names=(
  "apply_source_aware_compression_operator_approved_runtime_activation_marker"
  "source_aware_compression_operator_approved_runtime_activation"
  "source_aware_compression_operator_approval_evidence"
  "SourceAwareCompressionOperatorApprovalEvidence"
  "source_aware_compression_operator_approval_id"
  "source_aware_compression_operator_identity_hash"
  "source_aware_compression_activation_request_id"
  "source_aware_compression_operator_approval_scope_hash"
  "source_aware_compression_operator_approval_nonce"
  "source_aware_compression_operator_approval_expires_at"
)

for name in "${contract_only_names[@]}"; do
  assert_fixed_string_absent \
    "$name" \
    "source-aware compression activation negative matrix production surface" \
    "${production_roots[@]}"
done

assert_regex_absent \
  '^[^"]*source_aware_compression.*runtime.*activation' \
  "source-aware compression runtime activation production code" \
  "${production_roots[@]}"

assert_regex_absent \
  '^[^"]*source_aware_compression.*operator.*approval.*evidence' \
  "source-aware compression operator approval evidence production code" \
  "${production_roots[@]}"

assert_file_contains \
  "$debug_gate" \
  "$negative_matrix_gate_script" \
  "source-aware compression activation negative matrix debug gate"

assert_file_contains \
  "$preflight_script" \
  "source-aware compression activation negative matrix contract gate" \
  "source-aware compression activation negative matrix preflight stage"

assert_line_before \
  "$preflight_script" \
  "source-aware compression operator approval evidence contract gate" \
  "source-aware compression activation negative matrix contract gate" \
  "source-aware compression activation negative matrix preflight stage order"

assert_line_before \
  "$preflight_script" \
  "source-aware compression activation negative matrix contract gate" \
  "hepta-runtime recall selector budget fixtures" \
  "source-aware compression activation negative matrix preflight stage order"

echo "Hepta context source-aware compression activation negative matrix gate passed"
