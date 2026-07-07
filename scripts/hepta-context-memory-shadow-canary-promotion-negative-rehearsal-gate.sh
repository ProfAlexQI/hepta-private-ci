#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
manifest="$repo_root/codex-rs/Cargo.toml"
contracts="$repo_root/codex-rs/CONTEXT_DEBUG_CONTRACTS.md"
debug_gate="$repo_root/scripts/hepta-context-debug-gate.sh"
preflight_script="$repo_root/scripts/hepta-context-preflight.sh"
readiness_gate="$repo_root/scripts/hepta-context-memory-shadow-canary-promotion-readiness-gate.sh"
report_script="$repo_root/scripts/hepta-context-memory-shadow-canary-promotion-negative-rehearsal-report.sh"
gate_script="$repo_root/scripts/hepta-context-memory-shadow-canary-promotion-negative-rehearsal-gate.sh"
hepta_core_memory_tests="$repo_root/codex-rs/hepta-core/src/memory/tests/eval_harness.rs"
lane="${HEPTA_CARGO_LANE:-${HEPTA_LANE:-hepta-context}}"
target_root="${HEPTA_CARGO_TARGET_ROOT:-$HOME/.openclaw/tmp/cargo-targets}"
target_leaf="$lane"
if [[ "$target_leaf" != hepta-* ]]; then
  target_leaf="hepta-$target_leaf"
fi
export CARGO_TARGET_DIR="${HEPTA_CARGO_TARGET_DIR:-$target_root/$target_leaf}"
export CARGO_INCREMENTAL="${CARGO_INCREMENTAL:-0}"
report_output="$(mktemp -t hepta-context-memory-shadow-canary-promotion-negative-rehearsal-report.XXXXXX)"

cleanup() {
  rm -f "$report_output"
}
trap cleanup EXIT

fail() {
  echo "hepta-context-memory-shadow-canary-promotion-negative-rehearsal-gate: $*" >&2
  if [ -s "$report_output" ]; then
    echo "memory shadow canary promotion negative rehearsal report output:" >&2
    cat "$report_output" >&2
  fi
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

assert_report_line() {
  local expected="$1"
  if ! grep -F -x "$expected" "$report_output" >/dev/null; then
    fail "memory shadow canary promotion negative rehearsal report must contain line: $expected"
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

bash "$readiness_gate" >/dev/null
bash "$report_script" >"$report_output"

assert_report_line "memory-shadow-canary-promotion-negative-rehearsal=pass"
assert_report_line "memory-shadow-canary-promotion-negative-rehearsal.payload-light=pass"
assert_report_line "memory-shadow-canary-promotion-negative-rehearsal.activation-shaped-route=blocked"
assert_report_line "memory-shadow-canary-promotion-negative-rehearsal.rollback-write=blocked"
assert_report_line "memory-shadow-canary-promotion-negative-rehearsal.production-route=blocked"
assert_report_line "memory-shadow-canary-promotion-negative-rehearsal.production-write=blocked"
assert_report_line "memory-shadow-canary-promotion-negative-rehearsal.graph-write=blocked"
assert_report_line "memory-shadow-canary-promotion-negative-rehearsal.history-persistence-write=blocked"
assert_report_line "memory-shadow-canary-promotion-negative-rehearsal.prompt-assembly-change=blocked"
assert_report_line "memory-shadow-canary-promotion-negative-rehearsal.operator-activation=blocked"
assert_report_line "memory-shadow-canary-promotion-negative-rehearsal.runtime-activation=blocked"
assert_report_line "memory-shadow-canary-promotion-negative-rehearsal.canary-promotion-route=disabled"
assert_report_line "memory-shadow-canary-promotion-negative-rehearsal.rollback-write-state=disabled"
assert_report_line "memory-shadow-canary-promotion-negative-rehearsal.runtime-activation-state=disabled"

for term in \
  "Memory shadow canary promotion negative rehearsal" \
  "context_memory_shadow_canary_promotion_negative_rehearsal_blocks_activation_shaped_side_effects" \
  "hepta-context-memory-shadow-canary-promotion-negative-rehearsal-report.sh" \
  "hepta-context-memory-shadow-canary-promotion-negative-rehearsal-gate.sh"; do
  assert_file_contains "$contracts" "$term" "memory shadow canary promotion negative rehearsal contract"
done

assert_file_contains "$hepta_core_memory_tests" \
  "context_memory_shadow_canary_promotion_negative_rehearsal_blocks_activation_shaped_side_effects" \
  "memory shadow canary promotion negative rehearsal hepta-core test"
assert_file_contains "$hepta_core_memory_tests" \
  "canary_promotion_route_opened = true" \
  "memory shadow canary promotion negative route-open mutation"
assert_file_contains "$hepta_core_memory_tests" \
  "rollback_write = true" \
  "memory shadow canary promotion negative rollback-write mutation"

assert_file_contains "$debug_gate" \
  "hepta-context-memory-shadow-canary-promotion-negative-rehearsal-gate.sh" \
  "memory shadow canary promotion negative rehearsal debug gate"
assert_file_contains "$preflight_script" \
  "context memory shadow canary promotion negative rehearsal gate" \
  "memory shadow canary promotion negative rehearsal preflight stage"

assert_line_before \
  "$debug_gate" \
  "hepta-context-memory-shadow-canary-promotion-readiness-gate.sh" \
  "hepta-context-memory-shadow-canary-promotion-negative-rehearsal-gate.sh" \
  "memory shadow canary promotion negative rehearsal debug order"
assert_line_before \
  "$debug_gate" \
  "hepta-context-memory-shadow-canary-promotion-negative-rehearsal-gate.sh" \
  "hepta-context-plane-status-report-gate.sh" \
  "memory shadow canary promotion negative rehearsal context-plane debug order"
assert_line_before \
  "$preflight_script" \
  "context memory shadow canary promotion readiness gate" \
  "context memory shadow canary promotion negative rehearsal gate" \
  "memory shadow canary promotion negative rehearsal preflight order"
assert_line_before \
  "$preflight_script" \
  "context memory shadow canary promotion negative rehearsal gate" \
  "context plane status/export report gate" \
  "memory shadow canary promotion negative rehearsal context-plane preflight order"

leak_pattern='(prompt_text|transcript_text|memory_text|answer_text|session_id|memory_id|source_id|trace_id|query_text|query_payload|tool_args|tool_outputs|raw_payload|raw_ranked_payload|raw_graph_payload|ranked_payload=|operator_identity|activation_command|runtime-activation=enabled|production-route=enabled|canary-promotion-route=enabled|rollback-write=enabled|graph-write=enabled|production-write=enabled|history-persistence-write=enabled)'
if grep -E "$leak_pattern" "$report_output" >/dev/null; then
  fail "memory shadow canary promotion negative rehearsal report leaked payload or activation state"
fi

bash -n "$report_script"
bash -n "$gate_script"

cargo test --manifest-path "$manifest" -p hepta-core \
  shadow_canary_promotion_negative_rehearsal \
  --lib --message-format=short

echo "memory-shadow-canary-promotion-negative-rehearsal=pass"
echo "memory-shadow-canary-promotion-negative-rehearsal.payload-light=pass"
echo "memory-shadow-canary-promotion-negative-rehearsal.activation-shaped-route=blocked"
echo "memory-shadow-canary-promotion-negative-rehearsal.rollback-write=blocked"
echo "memory-shadow-canary-promotion-negative-rehearsal.runtime-activation=disabled"
