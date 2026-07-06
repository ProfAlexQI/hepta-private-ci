#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

fail() {
  echo "hepta-context-source-aware-compression-leak-bait-gate: $*" >&2
  exit 1
}

assert_file_contains() {
  local file_path="$1"
  local needle="$2"
  local label="$3"

  if ! grep -F "$needle" "$file_path" >/dev/null; then
    fail "$label must contain: $needle"
  fi
}

response_debug_export_gate="$repo_root/scripts/hepta-context-response-debug-export-gate.sh"
app_server_turn_start="$repo_root/codex-rs/app-server/tests/suite/v2/turn_start.rs"
debug_gate="$repo_root/scripts/hepta-context-debug-gate.sh"
preflight_script="$repo_root/scripts/hepta-context-preflight.sh"
leak_bait_gate_script="hepta-context-source-aware-compression-leak-bait-gate.sh"

required_bait_terms=(
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
  "source_aware_compression_operator_approval_evidence"
  "SourceAwareCompressionOperatorApprovalEvidence"
  "source_aware_compression_operator_approval_id"
  "source_aware_compression_operator_identity_hash"
  "source_aware_compression_activation_request_id"
  "source_aware_compression_operator_approval_scope_hash"
  "source_aware_compression_operator_approval_nonce"
  "source_aware_compression_operator_approval_expires_at"
)

for term in "${required_bait_terms[@]}"; do
  assert_file_contains \
    "$response_debug_export_gate" \
    "$term" \
    "response-debug source-aware compression leak bait"
  assert_file_contains \
    "$app_server_turn_start" \
    "$term" \
    "app-server thread-history source-aware compression leak bait"
done

assert_file_contains \
  "$app_server_turn_start" \
  "test_context_recall_selected_snippets_with_source_aware_compression_routing_bait" \
  "app-server source-aware compression leak-bait selected-snippet fixture"

assert_file_contains \
  "$app_server_turn_start" \
  "SOURCE_AWARE_COMPRESSION_ROUTING_BAIT_TERMS" \
  "app-server source-aware compression leak-bait no-history assertion"

assert_file_contains \
  "$debug_gate" \
  "$leak_bait_gate_script" \
  "source-aware compression leak bait debug gate"

assert_file_contains \
  "$preflight_script" \
  "source-aware compression leak bait contract gate" \
  "source-aware compression leak bait preflight stage"

echo "Hepta context source-aware compression leak bait gate passed"
