#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
manifest="$repo_root/codex-rs/Cargo.toml"
contracts="$repo_root/codex-rs/CONTEXT_DEBUG_CONTRACTS.md"
debug_gate="$repo_root/scripts/hepta-context-debug-gate.sh"
preflight_script="$repo_root/scripts/hepta-context-preflight.sh"
report_script="$repo_root/scripts/hepta-context-memory-shadow-canary-promotion-readiness-report.sh"
gate_script="$repo_root/scripts/hepta-context-memory-shadow-canary-promotion-readiness-gate.sh"
trend_gate="$repo_root/scripts/hepta-context-memory-shadow-quality-trend-snapshot-gate.sh"
hepta_core_memory="$repo_root/codex-rs/hepta-core/src/memory.rs"
hepta_core_eval_harness="$repo_root/codex-rs/hepta-core/src/memory/eval_harness.rs"
hepta_core_promotion="$repo_root/codex-rs/hepta-core/src/memory/eval_harness/shadow_canary_promotion.rs"
hepta_core_memory_tests="$repo_root/codex-rs/hepta-core/src/memory/tests/eval_harness.rs"
hepta_memory="$repo_root/codex-rs/hepta-memory/src/lib.rs"
hepta_memory_context_plane_helpers="$repo_root/codex-rs/hepta-memory/src/context_plane_helpers.rs"
hepta_memory_tests="$repo_root/codex-rs/hepta-memory/src/tests/context_memory.rs"
lane="${HEPTA_CARGO_LANE:-${HEPTA_LANE:-hepta-context}}"
target_root="${HEPTA_CARGO_TARGET_ROOT:-$HOME/.openclaw/tmp/cargo-targets}"
target_leaf="$lane"
if [[ "$target_leaf" != hepta-* ]]; then
  target_leaf="hepta-$target_leaf"
fi
export CARGO_TARGET_DIR="${HEPTA_CARGO_TARGET_DIR:-$target_root/$target_leaf}"
export CARGO_INCREMENTAL="${CARGO_INCREMENTAL:-0}"
report_output="$(mktemp -t hepta-context-memory-shadow-canary-promotion-readiness-report.XXXXXX)"

cleanup() {
  rm -f "$report_output"
}
trap cleanup EXIT

fail() {
  echo "hepta-context-memory-shadow-canary-promotion-readiness-gate: $*" >&2
  if [ -s "$report_output" ]; then
    echo "memory shadow canary promotion readiness report output:" >&2
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
    fail "memory shadow canary promotion readiness report must contain line: $expected"
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

bash "$trend_gate" >/dev/null
bash "$report_script" >"$report_output"

assert_report_line "memory-shadow-canary-promotion-readiness=pass"
assert_report_line "memory-shadow-canary-promotion-readiness.payload-light=pass"
assert_report_line "memory-shadow-canary-promotion-readiness.schema=1"
assert_report_line "memory-shadow-canary-promotion-readiness.mode=shadow-only"
assert_report_line "memory-shadow-canary-promotion-readiness.source-trend-snapshot=pass"
assert_report_line "memory-shadow-canary-promotion-readiness.source-trend-window=stable-window"
assert_report_line "memory-shadow-canary-promotion-readiness.required-stable-window-count=1"
assert_report_line "memory-shadow-canary-promotion-readiness.observed-stable-window-count=1"
assert_report_line "memory-shadow-canary-promotion-readiness.required-pass-streak=3"
assert_report_line "memory-shadow-canary-promotion-readiness.observed-pass-streak=3"
assert_report_line "memory-shadow-canary-promotion-readiness.promotion-decision=ready-shadow-only"
assert_report_line "memory-shadow-canary-promotion-readiness.promotion-blocker-count=0"
assert_report_line "memory-shadow-canary-promotion-readiness.regression-window-blocking-count=0"
assert_report_line "memory-shadow-canary-promotion-readiness.rollback-rehearsal=covered"
assert_report_line "memory-shadow-canary-promotion-readiness.rollback-rehearsal-count=3"
assert_report_line "memory-shadow-canary-promotion-readiness.rollback-rehearsal-pass-count=3"
assert_report_line "memory-shadow-canary-promotion-readiness.rollback-rehearsal-blocking-count=0"
assert_report_line "memory-shadow-canary-promotion-readiness.kill-switch-rehearsal=covered"
assert_report_line "memory-shadow-canary-promotion-readiness.kill-switch-rehearsal-count=3"
assert_report_line "memory-shadow-canary-promotion-readiness.kill-switch-rehearsal-pass-count=3"
assert_report_line "memory-shadow-canary-promotion-readiness.soak-readback=covered"
assert_report_line "memory-shadow-canary-promotion-readiness.soak-readback-window-count=3"
assert_report_line "memory-shadow-canary-promotion-readiness.soak-readback-pass-count=3"
assert_report_line "memory-shadow-canary-promotion-readiness.operator-packet-redacted=pass"
assert_report_line "memory-shadow-canary-promotion-readiness.operator-approval=required"
assert_report_line "memory-shadow-canary-promotion-readiness.history-persistence-write=disabled"
assert_report_line "memory-shadow-canary-promotion-readiness.canary-promotion-route=disabled"
assert_report_line "memory-shadow-canary-promotion-readiness.rollback-write=disabled"
assert_report_line "memory-shadow-canary-promotion-readiness.production-route=disabled"
assert_report_line "memory-shadow-canary-promotion-readiness.production-write=disabled"
assert_report_line "memory-shadow-canary-promotion-readiness.graph-write=disabled"
assert_report_line "memory-shadow-canary-promotion-readiness.runtime-activation=disabled"

for term in \
  "Memory shadow canary promotion readiness" \
  "ContextMemoryShadowCanaryPromotionReadinessReport" \
  "ContextMemoryShadowCanaryPromotionMode" \
  "ContextMemoryShadowCanaryPromotionDecision" \
  "ContextMemoryShadowCanaryRehearsalVerdict" \
  "context_memory_shadow_canary_promotion_readiness_report" \
  "rollback_rehearsal_verdict" \
  "kill_switch_rehearsal_verdict" \
  "soak_readback_verdict" \
  "operator_packet_redacted" \
  "canary_promotion_route_opened" \
  "rollback_write" \
  "hepta-context-memory-shadow-canary-promotion-readiness-report.sh" \
  "hepta-context-memory-shadow-canary-promotion-readiness-gate.sh"; do
  assert_file_contains "$contracts" "$term" "memory shadow canary promotion readiness contract"
done

assert_file_contains "$hepta_core_memory" \
  "CONTEXT_MEMORY_SHADOW_CANARY_PROMOTION_READINESS_SCHEMA_VERSION" \
  "memory shadow canary promotion readiness schema version"
assert_file_contains "$hepta_core_memory" \
  "pub use eval_harness::ContextMemoryShadowCanaryPromotionReadinessReport" \
  "memory shadow canary promotion readiness public re-export"
assert_file_contains "$hepta_core_eval_harness" \
  "mod shadow_canary_promotion" \
  "memory shadow canary promotion readiness module boundary"
assert_file_contains "$hepta_core_eval_harness" \
  "pub use shadow_canary_promotion::ContextMemoryShadowCanaryPromotionReadinessReport" \
  "memory shadow canary promotion readiness wrapper re-export"
assert_file_contains "$hepta_core_promotion" \
  "pub fn from_trend_snapshot" \
  "memory shadow canary promotion readiness adapter"
assert_file_contains "$hepta_core_promotion" \
  "pub fn has_shadow_canary_promotion_readiness_integrity" \
  "memory shadow canary promotion readiness integrity gate"
assert_file_contains "$hepta_core_memory_tests" \
  "context_memory_shadow_canary_promotion_readiness_rehearses_without_activation" \
  "memory shadow canary promotion readiness hepta-core positive test"
assert_file_contains "$hepta_core_memory_tests" \
  "context_memory_shadow_canary_promotion_readiness_blocks_trend_regression_drift" \
  "memory shadow canary promotion readiness hepta-core regression test"

assert_file_contains "$hepta_memory" \
  "shadow canary promotion" \
  "memory shadow canary promotion readiness hepta-memory docs"
assert_file_contains "$hepta_memory_context_plane_helpers" \
  "context_memory_shadow_canary_promotion_readiness_report" \
  "memory shadow canary promotion readiness hepta-memory helper"
assert_file_contains "$hepta_memory_tests" \
  "store_snapshot_context_memory_shadow_canary_promotion_readiness_is_payload_light" \
  "memory shadow canary promotion readiness hepta-memory snapshot test"
assert_file_contains "$hepta_memory_tests" \
  "store_context_memory_shadow_canary_promotion_readiness_matches_snapshot_helper" \
  "memory shadow canary promotion readiness hepta-memory store test"

assert_file_contains "$debug_gate" "hepta-context-memory-shadow-canary-promotion-readiness-gate.sh" \
  "memory shadow canary promotion readiness debug gate"
assert_file_contains "$preflight_script" "context memory shadow canary promotion readiness gate" \
  "memory shadow canary promotion readiness preflight stage"

assert_line_before \
  "$debug_gate" \
  "hepta-context-memory-shadow-quality-trend-snapshot-gate.sh" \
  "hepta-context-memory-shadow-canary-promotion-readiness-gate.sh" \
  "memory shadow canary promotion readiness debug order"
assert_line_before \
  "$debug_gate" \
  "hepta-context-memory-shadow-canary-promotion-readiness-gate.sh" \
  "hepta-context-plane-status-report-gate.sh" \
  "memory shadow canary promotion readiness context-plane debug order"
assert_line_before \
  "$preflight_script" \
  "context memory shadow quality trend snapshot gate" \
  "context memory shadow canary promotion readiness gate" \
  "memory shadow canary promotion readiness preflight order"
assert_line_before \
  "$preflight_script" \
  "context memory shadow canary promotion readiness gate" \
  "context plane status/export report gate" \
  "memory shadow canary promotion readiness context-plane preflight order"

leak_pattern='(prompt_text|transcript_text|memory_text|answer_text|session_id|memory_id|source_id|trace_id|query_text|query_payload|tool_args|tool_outputs|raw_payload|raw_ranked_payload|raw_graph_payload|ranked_payload=|operator_identity|activation_command|runtime-activation=enabled|production-route=enabled|canary-promotion-route=enabled|rollback-write=enabled|graph-write=enabled|production-write=enabled|history-persistence-write=enabled)'
if grep -E "$leak_pattern" "$report_output" >/dev/null; then
  fail "memory shadow canary promotion readiness report leaked payload or activation state"
fi

bash -n "$report_script"
bash -n "$gate_script"

cargo test --manifest-path "$manifest" -p hepta-core \
  shadow_canary_promotion_readiness \
  --lib --message-format=short

cargo test --manifest-path "$manifest" -p hepta-memory \
  shadow_canary_promotion_readiness \
  --lib --message-format=short

echo "memory-shadow-canary-promotion-readiness=pass"
echo "memory-shadow-canary-promotion-readiness.payload-light=pass"
echo "memory-shadow-canary-promotion-readiness.promotion-decision=ready-shadow-only"
echo "memory-shadow-canary-promotion-readiness.rollback-rehearsal=covered"
echo "memory-shadow-canary-promotion-readiness.canary-promotion-route=disabled"
echo "memory-shadow-canary-promotion-readiness.rollback-write=disabled"
echo "memory-shadow-canary-promotion-readiness.runtime-activation=disabled"
