#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
manifest="$repo_root/codex-rs/Cargo.toml"
contracts="$repo_root/codex-rs/CONTEXT_DEBUG_CONTRACTS.md"
debug_gate="$repo_root/scripts/hepta-context-debug-gate.sh"
preflight_script="$repo_root/scripts/hepta-context-preflight.sh"
release_manifest="$repo_root/codex-rs/CONTEXT_LANE_RELEASE_MANIFEST.tsv"
report_script="$repo_root/scripts/hepta-context-selected-recall-summary-canary-eval-report.sh"
gate_script="$repo_root/scripts/hepta-context-selected-recall-summary-canary-eval-gate.sh"
hepta_core_memory="$repo_root/codex-rs/hepta-core/src/memory.rs"
hepta_core_eval_harness="$repo_root/codex-rs/hepta-core/src/memory/eval_harness.rs"
hepta_core_selected_recall_canary="$repo_root/codex-rs/hepta-core/src/memory/eval_harness/selected_recall_canary.rs"
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
report_output="$(mktemp -t hepta-context-selected-recall-summary-canary-eval-report.XXXXXX)"

cleanup() {
  rm -f "$report_output"
}
trap cleanup EXIT

fail() {
  echo "hepta-context-selected-recall-summary-canary-eval-gate: $*" >&2
  if [ -s "$report_output" ]; then
    echo "selected recall summary canary eval report output:" >&2
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
    fail "selected recall summary canary eval report must contain line: $expected"
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

bash "$report_script" >"$report_output"

assert_report_line "selected-recall-summary-canary-eval=pass"
assert_report_line "selected-recall-summary-canary-eval.schema=1"
assert_report_line "selected-recall-summary-canary-eval.mode=golden-replay-shadow"
assert_report_line "selected-recall-summary-canary-eval.fixture-count=4"
assert_report_line "selected-recall-summary-canary-eval.fixture-pass-count=4"
assert_report_line "selected-recall-summary-canary-eval.fixture-blocked-count=0"
assert_report_line "selected-recall-summary-canary-eval.positive-fixture-count=3"
assert_report_line "selected-recall-summary-canary-eval.negative-fixture-count=1"
assert_report_line "selected-recall-summary-canary-eval.shadow-vs-live-pair-count=3"
assert_report_line "selected-recall-summary-canary-eval.rollback-readback-fixture-count=1"
assert_report_line "selected-recall-summary-canary-eval.prompt-input-proof=covered"
assert_report_line "selected-recall-summary-canary-eval.response-debug-proof=covered"
assert_report_line "selected-recall-summary-canary-eval.token-saved-min-basis-points=1000"
assert_report_line "selected-recall-summary-canary-eval.latency-delta-max-ms=250"
assert_report_line "selected-recall-summary-canary-eval.quality-delta-min-basis-points=0"
assert_report_line "selected-recall-summary-canary-eval.regression-fixture=blocked"
assert_report_line "selected-recall-summary-canary-eval.operator-approval=required"
assert_report_line "selected-recall-summary-canary-eval.production-route=disabled"
assert_report_line "selected-recall-summary-canary-eval.runtime-activation=disabled"

for term in \
  "Selected-Recall Summary Canary Eval Replay Gate" \
  "hepta-context-selected-recall-summary-canary-eval-report.sh" \
  "hepta-context-selected-recall-summary-canary-eval-gate.sh" \
  "golden-replay-shadow" \
  "token-saved-min-basis-points" \
  "latency-delta-max-ms" \
  "quality-delta-min-basis-points" \
  "Rust-backed fixture" \
  "ContextMemorySelectedRecallSummaryCanaryEvalReport" \
  "context_memory_selected_recall_summary_canary_eval_report" \
  "regression fixture" \
  "rollback-readback fixture"; do
  assert_file_contains "$contracts" "$term" "selected-recall summary canary eval contract"
done

assert_file_contains "$hepta_core_memory" \
  "CONTEXT_MEMORY_SELECTED_RECALL_SUMMARY_CANARY_EVAL_SCHEMA_VERSION" \
  "selected-recall summary canary eval schema version"
assert_file_contains "$hepta_core_memory" \
  "pub use eval_harness::ContextMemorySelectedRecallSummaryCanaryEvalReport" \
  "selected-recall summary canary eval public re-export"
assert_file_contains "$hepta_core_eval_harness" \
  "mod selected_recall_canary" \
  "selected-recall summary canary eval module boundary"
assert_file_contains "$hepta_core_eval_harness" \
  "pub use selected_recall_canary::ContextMemorySelectedRecallSummaryCanaryEvalReport" \
  "selected-recall summary canary eval wrapper re-export"
assert_file_contains "$hepta_core_selected_recall_canary" \
  "ContextMemorySelectedRecallSummaryCanaryEvalReport" \
  "selected-recall summary canary eval rust report"
assert_file_contains "$hepta_core_selected_recall_canary" \
  "ContextMemorySelectedRecallSummaryCanaryEvalFixtureResult" \
  "selected-recall summary canary eval rust fixture"
assert_file_contains "$hepta_core_selected_recall_canary" \
  "pub fn has_canary_eval_integrity" \
  "selected-recall summary canary eval integrity gate"
assert_file_contains "$hepta_core_selected_recall_canary" \
  "SELECTED_RECALL_SUMMARY_CANARY_TOKEN_SAVED_MIN_BASIS_POINTS" \
  "selected-recall summary canary eval token threshold"
assert_file_contains "$hepta_core_selected_recall_canary" \
  "SELECTED_RECALL_SUMMARY_CANARY_LATENCY_DELTA_MAX_MS" \
  "selected-recall summary canary eval latency threshold"
assert_file_contains "$hepta_core_selected_recall_canary" \
  "SELECTED_RECALL_SUMMARY_CANARY_QUALITY_DELTA_MIN_BASIS_POINTS" \
  "selected-recall summary canary eval quality threshold"
assert_file_contains "$hepta_core_memory_tests" \
  "context_memory_selected_recall_summary_canary_eval_replays_without_activation" \
  "selected-recall summary canary eval hepta-core positive test"
assert_file_contains "$hepta_core_memory_tests" \
  "context_memory_selected_recall_summary_canary_eval_blocks_regression_drift" \
  "selected-recall summary canary eval hepta-core regression test"
assert_file_contains "$hepta_memory" \
  "selected-recall summary canary eval replay" \
  "selected-recall summary canary eval hepta-memory docs"
assert_file_contains "$hepta_memory_context_plane_helpers" \
  "context_memory_selected_recall_summary_canary_eval_report" \
  "selected-recall summary canary eval hepta-memory helper"
assert_file_contains "$hepta_memory_tests" \
  "store_snapshot_context_memory_selected_recall_summary_canary_eval_is_payload_light" \
  "selected-recall summary canary eval hepta-memory snapshot test"
assert_file_contains "$hepta_memory_tests" \
  "store_context_memory_selected_recall_summary_canary_eval_matches_snapshot_helper" \
  "selected-recall summary canary eval hepta-memory store test"

assert_file_contains "$debug_gate" "hepta-context-selected-recall-summary-canary-eval-gate.sh" \
  "selected-recall summary canary eval debug gate"
assert_file_contains "$preflight_script" "selected recall summary canary eval replay gate" \
  "selected-recall summary canary eval preflight stage"
assert_file_contains "$release_manifest" "codex-rs/hepta-core/src/memory/eval_harness/selected_recall_canary.rs" \
  "selected-recall summary canary eval rust release manifest"
assert_file_contains "$release_manifest" "scripts/hepta-context-selected-recall-summary-canary-eval-report.sh" \
  "selected-recall summary canary eval report release manifest"
assert_file_contains "$release_manifest" "scripts/hepta-context-selected-recall-summary-canary-eval-gate.sh" \
  "selected-recall summary canary eval gate release manifest"

assert_line_before \
  "$debug_gate" \
  "hepta-context-selected-recall-summary-canary-gate.sh" \
  "hepta-context-selected-recall-summary-canary-eval-gate.sh" \
  "selected-recall summary canary eval debug order"
assert_line_before \
  "$preflight_script" \
  "selected recall summary canary readiness gate" \
  "selected recall summary canary eval replay gate" \
  "selected-recall summary canary eval preflight order"

leak_pattern='(prompt_text|transcript_text|memory_text|answer_text|session_id|memory_id|trace_id|query_text|query_payload|tool_args|tool_outputs|raw_payload|raw_ranked_payload|rank_explanation|score_reason|snippet_hash=|source_id|text_hash|rollback_hash|runtime-activation=enabled|production-route=enabled|operator-activation=enabled|graph-write=enabled|production-write=enabled)'
if grep -E "$leak_pattern" "$report_output" >/dev/null; then
  fail "selected recall summary canary eval report leaked payload or activation state"
fi

bash -n "$report_script"
bash -n "$gate_script"

cargo test --manifest-path "$manifest" -p hepta-core \
  selected_recall_summary_canary_eval \
  --lib --message-format=short

cargo test --manifest-path "$manifest" -p hepta-memory \
  selected_recall_summary_canary_eval \
  --lib --message-format=short

echo "selected-recall-summary-canary-eval=pass"
echo "selected-recall-summary-canary-eval.payload-light=pass"
echo "selected-recall-summary-canary-eval.fixtures=4"
echo "selected-recall-summary-canary-eval.regression-fixture=blocked"
echo "selected-recall-summary-canary-eval.runtime-activation=disabled"
