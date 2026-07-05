#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
manifest="$repo_root/codex-rs/Cargo.toml"
hepta_core_memory="$repo_root/codex-rs/hepta-core/src/memory.rs"
hepta_core_memory_tests="$repo_root/codex-rs/hepta-core/src/memory/tests/eval_harness.rs"
hepta_core_eval_harness="$repo_root/codex-rs/hepta-core/src/memory/eval_harness.rs"
hepta_core_adaptive_shadow="$repo_root/codex-rs/hepta-core/src/memory/eval_harness/adaptive_shadow.rs"
hepta_core_adaptive_shadow_comparison="$repo_root/codex-rs/hepta-core/src/memory/eval_harness/adaptive_shadow/comparison.rs"
hepta_core_adaptive_shadow_report="$repo_root/codex-rs/hepta-core/src/memory/eval_harness/adaptive_shadow/report.rs"
hepta_core_adaptive_shadow_result="$repo_root/codex-rs/hepta-core/src/memory/eval_harness/adaptive_shadow/result.rs"
hepta_memory="$repo_root/codex-rs/hepta-memory/src/lib.rs"
hepta_memory_tests="$repo_root/codex-rs/hepta-memory/src/tests/context_memory.rs"
hepta_memory_context_plane_helpers="$repo_root/codex-rs/hepta-memory/src/context_plane_helpers.rs"
contracts="$repo_root/codex-rs/CONTEXT_DEBUG_CONTRACTS.md"
debug_gate="$repo_root/scripts/hepta-context-debug-gate.sh"
preflight_script="$repo_root/scripts/hepta-context-preflight.sh"
front_door_gate="$repo_root/scripts/hepta-context-source-aware-compression-front-door-gate.sh"
lane="${HEPTA_CARGO_LANE:-${HEPTA_LANE:-hepta-context}}"
target_root="${HEPTA_CARGO_TARGET_ROOT:-$HOME/.openclaw/tmp/cargo-targets}"
target_leaf="$lane"
if [[ "$target_leaf" != hepta-* ]]; then
  target_leaf="hepta-$target_leaf"
fi
export CARGO_TARGET_DIR="${HEPTA_CARGO_TARGET_DIR:-$target_root/$target_leaf}"
export CARGO_INCREMENTAL="${CARGO_INCREMENTAL:-0}"

fail() {
  echo "hepta-context-memory-adaptive-allocator-eval-shadow-gate: $*" >&2
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

for term in \
  "Adaptive allocator eval shadow" \
  "current_heuristic" \
  "proposed_adaptive" \
  "recall_coverage" \
  "missing_critical_fact" \
  "precision" \
  "latency" \
  "token_cost" \
  "token_saved" \
  "safety_leak" \
  "answer_quality_regression" \
  "comparison_verdict" \
  "shadow_threshold_pass" \
  "missing_critical_fact_regression_count" \
  "token_saved_regression_count" \
  "synthetic_long_session" \
  "redacted_trace" \
  "must not contain prompt text" \
  "must not contain transcript text" \
  "must not contain memory text" \
  "must not contain answer text" \
  "no recall regression" \
  "no precision regression" \
  "no latency regression" \
  "no token-cost regression" \
  "no token-saved regression" \
  "zero safety leaks" \
  "zero answer-quality regressions" \
  "no production memory writes" \
  "no graph writes" \
  "no runtime activation" \
  "no adaptive allocator runtime activation" \
  "no source-aware runtime activation" \
  "no prompt assembly changes" \
  "no operator activation allowance" \
  "hepta-context-memory-adaptive-allocator-eval-shadow-gate.sh" \
  "runtime-activation=disabled"; do
  assert_file_contains "$contracts" "$term" "adaptive allocator eval shadow contract"
done

assert_file_contains "$hepta_core_memory" \
  "CONTEXT_MEMORY_ADAPTIVE_ALLOCATOR_EVAL_SHADOW_SCHEMA_VERSION" \
  "adaptive allocator eval shadow schema version"
assert_file_contains "$hepta_core_memory" \
  "mod eval_harness" \
  "adaptive allocator eval shadow module boundary"
assert_file_contains "$hepta_core_memory" \
  "pub use eval_harness::ContextMemoryAdaptiveAllocatorEvalShadowReport" \
  "adaptive allocator eval shadow public re-export"
assert_file_contains "$hepta_core_eval_harness" \
  "mod adaptive_shadow" \
  "adaptive allocator eval shadow internal module boundary"
assert_file_contains "$hepta_core_eval_harness" \
  "pub use adaptive_shadow::ContextMemoryAdaptiveAllocatorEvalShadowReport" \
  "adaptive allocator eval shadow wrapper re-export"
assert_file_contains "$hepta_core_adaptive_shadow" \
  "mod comparison" \
  "adaptive allocator eval shadow comparison module boundary"
assert_file_contains "$hepta_core_adaptive_shadow" \
  "mod report" \
  "adaptive allocator eval shadow report module boundary"
assert_file_contains "$hepta_core_adaptive_shadow" \
  "mod result" \
  "adaptive allocator eval shadow result module boundary"
assert_file_contains "$hepta_core_adaptive_shadow_result" \
  "ContextMemoryAdaptiveAllocatorEvalArm" \
  "adaptive allocator eval shadow arm enum"
assert_file_contains "$hepta_core_adaptive_shadow_report" \
  "ContextMemoryAdaptiveAllocatorEvalShadowReport" \
  "adaptive allocator eval shadow report"
assert_file_contains "$hepta_core_adaptive_shadow_comparison" \
  "ContextMemoryAdaptiveAllocatorEvalShadowComparisonVerdict" \
  "adaptive allocator eval shadow comparison verdict"
assert_file_contains "$hepta_core_adaptive_shadow_result" \
  "prompt_assembly_change" \
  "adaptive allocator eval shadow prompt assembly side effect flag"
assert_file_contains "$hepta_core_adaptive_shadow_report" \
  "pub fn from_seed(seed: &ContextMemoryEvalHarnessReport) -> Self" \
  "adaptive allocator eval shadow seed adapter"
assert_file_contains "$hepta_core_adaptive_shadow_report" \
  "passes_shadow_thresholds" \
  "adaptive allocator eval shadow threshold gate"
assert_file_contains "$hepta_core_adaptive_shadow_comparison" \
  "has_shadow_threshold_integrity" \
  "adaptive allocator eval shadow comparison verdict threshold gate"
assert_file_contains "$hepta_core_adaptive_shadow_comparison" \
  "ContextMemoryAdaptiveAllocatorEvalShadowVerdict::ShadowThresholdPass" \
  "adaptive allocator eval shadow comparison pass verdict"
assert_file_contains "$hepta_core_memory_tests" \
  "context_memory_adaptive_allocator_eval_shadow_compares_without_activation" \
  "adaptive allocator eval shadow hepta-core test"

assert_file_contains "$hepta_memory" \
  "mod context_plane_helpers" \
  "adaptive allocator eval shadow hepta-memory helper module"
assert_file_contains "$hepta_memory_context_plane_helpers" \
  "context_memory_adaptive_allocator_eval_shadow_report" \
  "adaptive allocator eval shadow hepta-memory helper"
assert_file_contains "$hepta_memory_tests" \
  "store_snapshot_context_memory_adaptive_allocator_eval_shadow_is_payload_light" \
  "adaptive allocator eval shadow hepta-memory snapshot test"
assert_file_contains "$hepta_memory_tests" \
  "store_context_memory_adaptive_allocator_eval_shadow_matches_snapshot_helper" \
  "adaptive allocator eval shadow hepta-memory store test"

assert_file_contains "$debug_gate" \
  "hepta-context-memory-adaptive-allocator-eval-shadow-gate.sh" \
  "adaptive allocator eval shadow debug gate"
assert_file_contains "$preflight_script" \
  "context memory adaptive allocator eval shadow gate" \
  "adaptive allocator eval shadow preflight stage"
assert_file_contains "$front_door_gate" \
  "memory_adaptive_allocator_eval_shadow_gate_script" \
  "adaptive allocator eval shadow front-door static check"

assert_line_before \
  "$debug_gate" \
  "hepta-context-memory-eval-harness-seed-gate.sh" \
  "hepta-context-memory-adaptive-allocator-eval-shadow-gate.sh" \
  "adaptive allocator eval shadow debug gate order"
assert_line_before \
  "$debug_gate" \
  "hepta-context-memory-adaptive-allocator-eval-shadow-gate.sh" \
  "hepta-context-source-aware-compression-front-door-report.sh" \
  "adaptive allocator eval shadow debug gate front-door order"
assert_line_before \
  "$preflight_script" \
  "context memory eval harness seed gate" \
  "context memory adaptive allocator eval shadow gate" \
  "adaptive allocator eval shadow preflight order"
assert_line_before \
  "$preflight_script" \
  "context memory adaptive allocator eval shadow gate" \
  "source-aware compression front-door machine-readable report" \
  "adaptive allocator eval shadow front-door preflight order"

cargo test --manifest-path "$manifest" -p hepta-core \
  adaptive_allocator_eval_shadow \
  --lib --message-format=short

cargo test --manifest-path "$manifest" -p hepta-memory \
  adaptive_allocator_eval_shadow \
  --lib --message-format=short

echo "context-memory-adaptive-allocator-eval-shadow=pass"
echo "context-memory-adaptive-allocator-eval-shadow.current-heuristic=shadow"
echo "context-memory-adaptive-allocator-eval-shadow.proposed-adaptive=shadow"
echo "context-memory-adaptive-allocator-eval-shadow.runtime-activation=disabled"
echo "Hepta context memory adaptive allocator eval shadow gate passed"
