#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

fail() {
  echo "hepta-context-source-aware-compression-positive-route-change-detector: $*" >&2
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
  "$repo_root/codex-rs/cli/src/native_gateway.rs"
  "$repo_root/codex-rs/core/src"
  "$repo_root/codex-rs/exec/src"
  "$repo_root/codex-rs/hepta-runtime/src"
  "$repo_root/codex-rs/response-debug-context/src"
  "$repo_root/codex-rs/tui/src"
)

context_contracts="$repo_root/codex-rs/CONTEXT_DEBUG_CONTRACTS.md"
positive_route_gate="$repo_root/scripts/hepta-context-source-aware-compression-positive-route-readiness-gate.sh"
response_debug_export_gate="$repo_root/scripts/hepta-context-response-debug-export-gate.sh"
app_server_turn_start="$repo_root/codex-rs/app-server/tests/suite/v2/turn_start.rs"
core_session_tests="$repo_root/codex-rs/core/src/session/tests.rs"
debug_gate="$repo_root/scripts/hepta-context-debug-gate.sh"
preflight_script="$repo_root/scripts/hepta-context-preflight.sh"
change_detector_script="hepta-context-source-aware-compression-positive-route-change-detector.sh"

reserved_runtime_activation_entrypoint="apply_source_aware_compression_operator_approved_runtime_activation_marker"
reserved_runtime_activation_key="source_aware_compression_operator_approved_runtime_activation"
operator_approval_evidence_name="source_aware_compression_operator_approval_evidence"
operator_approval_evidence_type="SourceAwareCompressionOperatorApprovalEvidence"

response_debug_no_leak_fixture="source_aware_compression_positive_route_response_debug_export_no_leak"
app_server_history_no_leak_fixture="source_aware_compression_positive_route_app_server_thread_history_no_leak"
rollout_readback_no_leak_fixture="source_aware_compression_positive_route_rollout_readback_no_leak"

required_contract_terms=(
  "Source-aware compression positive-route implementation-change detector"
  "$reserved_runtime_activation_key"
  "$reserved_runtime_activation_entrypoint"
  "$operator_approval_evidence_name"
  "$response_debug_no_leak_fixture"
  "$app_server_history_no_leak_fixture"
  "$rollout_readback_no_leak_fixture"
  "preflight must fail"
  "route remains contract-only"
)

for term in "${required_contract_terms[@]}"; do
  assert_file_contains \
    "$context_contracts" \
    "$term" \
    "source-aware compression positive-route implementation-change detector contract"
  assert_file_contains \
    "$positive_route_gate" \
    "$term" \
    "source-aware compression positive-route readiness gate implementation-change detector terms"
done

assert_file_contains \
  "$debug_gate" \
  "$change_detector_script" \
  "source-aware compression positive-route change detector debug gate"

assert_file_contains \
  "$preflight_script" \
  "source-aware compression positive route implementation-change detector" \
  "source-aware compression positive-route change detector preflight stage"

assert_line_before \
  "$preflight_script" \
  "source-aware compression positive route readiness review gate" \
  "source-aware compression positive route implementation-change detector" \
  "source-aware compression positive-route change detector preflight stage order"

assert_line_before \
  "$preflight_script" \
  "source-aware compression positive route implementation-change detector" \
  "hepta-memory recall mixed-tier drift fixtures" \
  "source-aware compression positive-route change detector preflight stage order"

implementation_refs="$(
  {
    rg -n --fixed-strings "$reserved_runtime_activation_entrypoint" "${production_roots[@]}" || true
    rg -n --fixed-strings "$reserved_runtime_activation_key" "${production_roots[@]}" || true
    rg -n --fixed-strings "$operator_approval_evidence_name" "${production_roots[@]}" || true
    rg -n --fixed-strings "$operator_approval_evidence_type" "${production_roots[@]}" || true
  } | relative_paths
)"

if [ -n "$implementation_refs" ]; then
  assert_file_contains \
    "$response_debug_export_gate" \
    "$response_debug_no_leak_fixture" \
    "source-aware compression positive-route response-debug no-leak fixture"
  assert_file_contains \
    "$app_server_turn_start" \
    "$app_server_history_no_leak_fixture" \
    "source-aware compression positive-route app-server thread-history no-leak fixture"
  assert_file_contains \
    "$core_session_tests" \
    "$rollout_readback_no_leak_fixture" \
    "source-aware compression positive-route rollout/readback no-leak fixture"

  fail "reserved source-aware compression positive-route production references require a reviewed implementation update; found $(printf '%s' "$implementation_refs" | tr '\n' ',')"
fi

echo "Hepta context source-aware compression positive-route change detector passed"
