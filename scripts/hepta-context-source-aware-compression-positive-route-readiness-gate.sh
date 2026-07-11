#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

fail() {
  echo "hepta-context-source-aware-compression-positive-route-readiness-gate: $*" >&2
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
positive_route_gate_script="hepta-context-source-aware-compression-positive-route-readiness-gate.sh"
reserved_runtime_activation_entrypoint="apply_source_aware_compression_operator_approved_runtime_activation_marker"
reserved_runtime_activation_key="source_aware_compression_operator_approved_runtime_activation"

required_contract_terms=(
  "Source-aware compression positive-route readiness review"
  "reserved route + canary + helper-injected marker + approval evidence"
  "remains unimplemented"
  "Before production code may consume"
  "$reserved_runtime_activation_key"
  "$reserved_runtime_activation_entrypoint"
  "source_aware_compression_operator_approval_evidence"
  "update the source-aware compression readiness"
  "operator-approval evidence"
  "activation negative-matrix, activation-surface, and leak-bait gates"
  "scripts/hepta-context-source-aware-compression-readiness-gate.sh"
  "scripts/hepta-context-source-aware-compression-operator-approval-evidence-gate.sh"
  "scripts/hepta-context-source-aware-compression-activation-negative-matrix-gate.sh"
  "scripts/hepta-context-source-aware-compression-activation-surface-audit.sh"
  "scripts/hepta-context-source-aware-compression-leak-bait-gate.sh"
  "rollout/debug/export no-leak tests"
  "response-debug export gate"
  "app-server thread-history contract"
  "source_aware_compression_positive_route_response_debug_export_no_leak"
  "source_aware_compression_positive_route_app_server_thread_history_no_leak"
  "source_aware_compression_positive_route_rollout_readback_no_leak"
  "source_aware_compression_canary"
  "insert_source_aware_compression_policy_opt_in_marker"
  "reserved activation"
  "seam"
  "default runtime remains non-rewriting"
  "positive route remains unimplemented"
  "Source-aware compression positive-route implementation-change detector"
  "source_aware_compression_positive_route_response_debug_export_no_leak"
  "source_aware_compression_positive_route_app_server_thread_history_no_leak"
  "source_aware_compression_positive_route_rollout_readback_no_leak"
  "preflight must fail"
  "route remains contract-only"
)

for term in "${required_contract_terms[@]}"; do
  assert_file_contains \
    "$context_contracts" \
    "$term" \
    "source-aware compression positive-route readiness review contract"
done

required_control_gate_terms=(
  "$repo_root/scripts/hepta-context-source-aware-compression-readiness-gate.sh|Runtime activation readiness checklist"
  "$repo_root/scripts/hepta-context-source-aware-compression-operator-approval-evidence-gate.sh|source_aware_compression_operator_approval_evidence"
  "$repo_root/scripts/hepta-context-source-aware-compression-activation-negative-matrix-gate.sh|missing-route+canary+helper-marker+approval-evidence"
  "$repo_root/scripts/hepta-context-source-aware-compression-activation-surface-audit.sh|insert_source_aware_compression_policy_opt_in_marker"
  "$repo_root/scripts/hepta-context-source-aware-compression-leak-bait-gate.sh|SOURCE_AWARE_COMPRESSION_ROUTING_BAIT_TERMS"
)

for gate_and_term in "${required_control_gate_terms[@]}"; do
  gate_path="${gate_and_term%%|*}"
  term="${gate_and_term#*|}"
  assert_file_contains \
    "$gate_path" \
    "$term" \
    "source-aware compression positive-route required control gate"
done

contract_only_names=(
  "$reserved_runtime_activation_entrypoint"
  "$reserved_runtime_activation_key"
  "source_aware_compression_operator_approval_evidence"
  "SourceAwareCompressionOperatorApprovalEvidence"
)

for name in "${contract_only_names[@]}"; do
  assert_fixed_string_absent \
    "$name" \
    "source-aware compression positive-route production consumption" \
    "${production_roots[@]}"
done

assert_regex_absent \
  '^[^"]*source_aware_compression.*positive.*route' \
  "source-aware compression positive route production code" \
  "${production_roots[@]}"

assert_file_contains \
  "$debug_gate" \
  "$positive_route_gate_script" \
  "source-aware compression positive-route readiness debug gate"

assert_file_contains \
  "$preflight_script" \
  "source-aware compression positive route readiness review gate" \
  "source-aware compression positive-route readiness preflight stage"

assert_line_before \
  "$preflight_script" \
  "source-aware compression leak bait contract gate" \
  "source-aware compression positive route readiness review gate" \
  "source-aware compression positive-route readiness preflight stage order"

assert_line_before \
  "$preflight_script" \
  "source-aware compression positive route readiness review gate" \
  "hepta-memory recall mixed-tier drift fixtures" \
  "source-aware compression positive-route readiness preflight stage order"

echo "Hepta context source-aware compression positive-route readiness gate passed"
