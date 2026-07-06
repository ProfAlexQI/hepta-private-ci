#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
manifest="$repo_root/codex-rs/Cargo.toml"
hepta_core_memory="$repo_root/codex-rs/hepta-core/src/memory.rs"
hepta_core_memory_tests="$repo_root/codex-rs/hepta-core/src/memory/tests/eval_harness.rs"
hepta_core_eval_harness="$repo_root/codex-rs/hepta-core/src/memory/eval_harness.rs"
hepta_core_eval_seed="$repo_root/codex-rs/hepta-core/src/memory/eval_harness/eval_seed.rs"
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
  echo "hepta-context-memory-eval-harness-seed-gate: $*" >&2
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
  "Context memory eval harness seed" \
  "recall_coverage" \
  "missing_critical_fact" \
  "precision" \
  "latency" \
  "token_cost" \
  "token_saved" \
  "safety_leak" \
  "answer_quality_regression" \
  "synthetic_long_session" \
  "redacted_trace" \
  "must not contain prompt text" \
  "must not contain transcript text" \
  "must not contain memory text" \
  "must not contain answer text" \
  "zero safety leaks" \
  "zero answer-quality regressions" \
  "no production memory writes" \
  "no graph writes" \
  "no runtime activation" \
  "no operator activation allowance" \
  "must not activate adaptive allocation" \
  "must not activate source-aware compression" \
  "must not write graph facts" \
  "must not write production memory" \
  "must not alter prompt assembly" \
  "hepta-context-memory-eval-harness-seed-gate.sh" \
  "runtime-activation=disabled"; do
  assert_file_contains "$contracts" "$term" "context memory eval harness contract"
done

assert_file_contains "$hepta_core_memory" \
  "CONTEXT_MEMORY_EVAL_HARNESS_SCHEMA_VERSION" \
  "context memory eval harness schema version"
assert_file_contains "$hepta_core_memory" \
  "mod eval_harness" \
  "context memory eval harness module boundary"
assert_file_contains "$hepta_core_memory" \
  "pub use eval_harness::ContextMemoryEvalHarnessReport" \
  "context memory eval harness public re-export"
assert_file_contains "$hepta_core_eval_harness" \
  "mod eval_seed" \
  "context memory eval harness seed internal module boundary"
assert_file_contains "$hepta_core_eval_harness" \
  "pub use eval_seed::ContextMemoryEvalHarnessReport" \
  "context memory eval harness seed wrapper re-export"
assert_file_contains "$hepta_core_eval_seed" \
  "ContextMemoryEvalMetric" \
  "context memory eval metric enum"
assert_file_contains "$hepta_core_eval_seed" \
  "ContextMemoryEvalFixtureKind" \
  "context memory eval fixture enum"
assert_file_contains "$hepta_core_eval_seed" \
  "ContextMemoryEvalHarnessReport" \
  "context memory eval harness report"
assert_file_contains "$hepta_core_eval_seed" \
  "impl ContextMemoryEvalHarnessReport" \
  "context memory eval harness implementation"
assert_file_contains "$hepta_core_eval_seed" \
  "pub fn seeded() -> Self" \
  "context memory eval harness seed constructor"
assert_file_contains "$hepta_core_memory_tests" \
  "context_memory_eval_harness_seed_is_payload_light_and_non_activating" \
  "context memory eval harness hepta-core test"

assert_file_contains "$hepta_memory" \
  "mod context_plane_helpers" \
  "context memory eval harness hepta-memory helper module"
assert_file_contains "$hepta_memory_context_plane_helpers" \
  "context_memory_eval_harness_seed_report" \
  "context memory eval harness hepta-memory helper"
assert_file_contains "$hepta_memory_tests" \
  "store_snapshot_context_memory_eval_harness_seed_is_payload_light" \
  "context memory eval harness hepta-memory snapshot test"
assert_file_contains "$hepta_memory_tests" \
  "store_context_memory_eval_harness_seed_matches_snapshot_helper" \
  "context memory eval harness hepta-memory store test"

assert_file_contains "$debug_gate" \
  "hepta-context-memory-eval-harness-seed-gate.sh" \
  "context memory eval harness debug gate"
assert_file_contains "$preflight_script" \
  "context memory eval harness seed gate" \
  "context memory eval harness preflight stage"
assert_file_contains "$front_door_gate" \
  "memory_eval_harness_seed_gate_script" \
  "context memory eval harness front-door static check"

assert_line_before \
  "$debug_gate" \
  "hepta-context-memory-temporal-fact-schema-gate.sh" \
  "hepta-context-memory-eval-harness-seed-gate.sh" \
  "context memory eval harness debug gate order"
assert_line_before \
  "$debug_gate" \
  "hepta-context-memory-eval-harness-seed-gate.sh" \
  "hepta-context-source-aware-compression-front-door-report.sh" \
  "context memory eval harness debug gate front-door order"
assert_line_before \
  "$preflight_script" \
  "context memory temporal fact schema dry-run gate" \
  "context memory eval harness seed gate" \
  "context memory eval harness preflight order"
assert_line_before \
  "$preflight_script" \
  "context memory eval harness seed gate" \
  "source-aware compression front-door machine-readable report" \
  "context memory eval harness front-door preflight order"

cargo test --manifest-path "$manifest" -p hepta-core \
  memory_eval \
  --lib --message-format=short

cargo test --manifest-path "$manifest" -p hepta-memory \
  memory_eval \
  --lib --message-format=short

echo "context-memory-eval-harness-seed=pass"
echo "context-memory-eval-harness-seed.production-write=disabled"
echo "context-memory-eval-harness-seed.graph-write=disabled"
echo "context-memory-eval-harness-seed.runtime-activation=disabled"
echo "Hepta context memory eval harness seed gate passed"
