#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
manifest="$repo_root/codex-rs/Cargo.toml"
contracts="$repo_root/codex-rs/CONTEXT_DEBUG_CONTRACTS.md"
debug_gate="$repo_root/scripts/hepta-context-debug-gate.sh"
preflight_script="$repo_root/scripts/hepta-context-preflight.sh"
release_manifest="$repo_root/codex-rs/CONTEXT_LANE_RELEASE_MANIFEST.tsv"
report_script="$repo_root/scripts/hepta-context-memory-ranked-recall-shadow-eval-report.sh"
gate_script="$repo_root/scripts/hepta-context-memory-ranked-recall-shadow-eval-gate.sh"
front_door_gate="$repo_root/scripts/hepta-context-source-aware-compression-front-door-gate.sh"
hepta_core_memory="$repo_root/codex-rs/hepta-core/src/memory.rs"
hepta_core_eval_harness="$repo_root/codex-rs/hepta-core/src/memory/eval_harness.rs"
hepta_core_ranked_recall_shadow="$repo_root/codex-rs/hepta-core/src/memory/eval_harness/ranked_recall_shadow.rs"
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
report_output="$(mktemp -t hepta-context-memory-ranked-recall-shadow-eval-report.XXXXXX)"

cleanup() {
  rm -f "$report_output"
}
trap cleanup EXIT

fail() {
  echo "hepta-context-memory-ranked-recall-shadow-eval-gate: $*" >&2
  if [ -s "$report_output" ]; then
    echo "ranked recall shadow eval report output:" >&2
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
    fail "ranked recall shadow eval report must contain line: $expected"
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

assert_report_line "ranked-recall-shadow-eval=pass"
assert_report_line "ranked-recall-shadow-eval.payload-light=pass"
assert_report_line "ranked-recall-shadow-eval.schema=6"
assert_report_line "ranked-recall-shadow-eval.mode=deterministic-shadow"
assert_report_line "ranked-recall-shadow-eval.hybrid-mode=shadow-only"
assert_report_line "ranked-recall-shadow-eval.hybrid-signal-count=5"
assert_report_line "ranked-recall-shadow-eval.hybrid-positive-signal-pass-count=15"
assert_report_line "ranked-recall-shadow-eval.hybrid-signal-min-basis-points=6000"
assert_report_line "ranked-recall-shadow-eval.min-positive-hybrid-score-basis-points=7800"
assert_report_line "ranked-recall-shadow-eval.calibrated-reranking=shadow"
assert_report_line "ranked-recall-shadow-eval.calibrated-reranking-fixture-count=4"
assert_report_line "ranked-recall-shadow-eval.calibrated-reranking-win-count=3"
assert_report_line "ranked-recall-shadow-eval.calibrated-reranking-loss-count=1"
assert_report_line "ranked-recall-shadow-eval.reranking-delta-min-basis-points=400"
assert_report_line "ranked-recall-shadow-eval.min-positive-reranking-delta-basis-points=640"
assert_report_line "ranked-recall-shadow-eval.latency-delta-max-ms=20"
assert_report_line "ranked-recall-shadow-eval.max-positive-latency-delta-ms=10"
assert_report_line "ranked-recall-shadow-eval.token-tradeoff-min-basis-points=1000"
assert_report_line "ranked-recall-shadow-eval.min-positive-token-tradeoff-basis-points=3000"
assert_report_line "ranked-recall-shadow-eval.reranking-regression-delta=blocked"
assert_report_line "ranked-recall-shadow-eval.routing-diff=shadow-only"
assert_report_line "ranked-recall-shadow-eval.routing-diff-fixture-count=4"
assert_report_line "ranked-recall-shadow-eval.routing-diff-shadow-only-count=4"
assert_report_line "ranked-recall-shadow-eval.routing-diff-win-count=3"
assert_report_line "ranked-recall-shadow-eval.routing-diff-loss-count=1"
assert_report_line "ranked-recall-shadow-eval.routing-diff-delta-min-basis-points=400"
assert_report_line "ranked-recall-shadow-eval.min-positive-routing-diff-delta-basis-points=640"
assert_report_line "ranked-recall-shadow-eval.routing-diff-latency-delta-max-ms=20"
assert_report_line "ranked-recall-shadow-eval.max-positive-routing-diff-latency-delta-ms=10"
assert_report_line "ranked-recall-shadow-eval.routing-diff-token-tradeoff-min-basis-points=1000"
assert_report_line "ranked-recall-shadow-eval.min-positive-routing-diff-token-tradeoff-basis-points=3000"
assert_report_line "ranked-recall-shadow-eval.routing-diff-regression=blocked"
assert_report_line "ranked-recall-shadow-eval.real-workload-trace=shadow-only"
assert_report_line "ranked-recall-shadow-eval.real-workload-trace-fixture-count=4"
assert_report_line "ranked-recall-shadow-eval.real-workload-trace-shadow-only-count=4"
assert_report_line "ranked-recall-shadow-eval.real-workload-trace-slo-pass-count=3"
assert_report_line "ranked-recall-shadow-eval.real-workload-trace-win-count=3"
assert_report_line "ranked-recall-shadow-eval.real-workload-trace-loss-count=1"
assert_report_line "ranked-recall-shadow-eval.real-workload-trace-operator-review-required-count=4"
assert_report_line "ranked-recall-shadow-eval.real-workload-trace-total-leak-count=0"
assert_report_line "ranked-recall-shadow-eval.real-workload-trace-max-leak-rate-basis-points=0"
assert_report_line "ranked-recall-shadow-eval.min-positive-real-workload-trace-coverage-basis-points=8000"
assert_report_line "ranked-recall-shadow-eval.min-positive-real-workload-trace-precision-basis-points=8000"
assert_report_line "ranked-recall-shadow-eval.total-positive-real-workload-trace-token-saved=2140"
assert_report_line "ranked-recall-shadow-eval.max-positive-real-workload-trace-latency-ms=55"
assert_report_line "ranked-recall-shadow-eval.real-workload-trace-regression-loss=blocked"
assert_report_line "ranked-recall-shadow-eval.canary-precondition=shadow-only"
assert_report_line "ranked-recall-shadow-eval.canary-precondition-fixture-count=4"
assert_report_line "ranked-recall-shadow-eval.canary-precondition-shadow-only-count=4"
assert_report_line "ranked-recall-shadow-eval.canary-precondition-pass-count=4"
assert_report_line "ranked-recall-shadow-eval.canary-feature-flag-registered-count=4"
assert_report_line "ranked-recall-shadow-eval.canary-feature-flag-disabled-count=4"
assert_report_line "ranked-recall-shadow-eval.canary-kill-switch-registered-count=4"
assert_report_line "ranked-recall-shadow-eval.canary-kill-switch-enabled-count=4"
assert_report_line "ranked-recall-shadow-eval.canary-rollback-rehearsal-covered-count=4"
assert_report_line "ranked-recall-shadow-eval.canary-activation-denial-covered-count=4"
assert_report_line "ranked-recall-shadow-eval.canary-precondition-operator-review-required-count=4"
assert_report_line "ranked-recall-shadow-eval.canary-precondition-route-opened-count=0"
assert_report_line "ranked-recall-shadow-eval.canary-precondition-rollback-write-count=0"
assert_report_line "ranked-recall-shadow-eval.lexical-bm25=shadow"
assert_report_line "ranked-recall-shadow-eval.recency=shadow"
assert_report_line "ranked-recall-shadow-eval.source-authority=shadow"
assert_report_line "ranked-recall-shadow-eval.temporal-validity=shadow"
assert_report_line "ranked-recall-shadow-eval.feedback=shadow"
assert_report_line "ranked-recall-shadow-eval.fixture-count=4"
assert_report_line "ranked-recall-shadow-eval.fixture-pass-count=4"
assert_report_line "ranked-recall-shadow-eval.positive-fixture-count=3"
assert_report_line "ranked-recall-shadow-eval.negative-fixture-count=1"
assert_report_line "ranked-recall-shadow-eval.ranked-item-fixture-count=4"
assert_report_line "ranked-recall-shadow-eval.recall-floor-basis-points=7000"
assert_report_line "ranked-recall-shadow-eval.precision-floor-basis-points=7000"
assert_report_line "ranked-recall-shadow-eval.token-saved-min=300"
assert_report_line "ranked-recall-shadow-eval.token-saved-min-basis-points=1000"
assert_report_line "ranked-recall-shadow-eval.latency-max-ms=100"
assert_report_line "ranked-recall-shadow-eval.regret-max-basis-points=0"
assert_report_line "ranked-recall-shadow-eval.min-positive-recall-basis-points=8000"
assert_report_line "ranked-recall-shadow-eval.min-positive-precision-basis-points=8000"
assert_report_line "ranked-recall-shadow-eval.total-positive-token-saved=2140"
assert_report_line "ranked-recall-shadow-eval.max-positive-latency-ms=55"
assert_report_line "ranked-recall-shadow-eval.max-positive-regret-basis-points=0"
assert_report_line "ranked-recall-shadow-eval.regression-fixture=blocked"
assert_report_line "ranked-recall-shadow-eval.hybrid-regression-signal=blocked"
assert_report_line "ranked-recall-shadow-eval.operator-approval=required"
assert_report_line "ranked-recall-shadow-eval.production-route=disabled"
assert_report_line "ranked-recall-shadow-eval.production-selection-route=read-only"
assert_report_line "ranked-recall-shadow-eval.runtime-activation=disabled"

for term in \
  "Ranked recall shadow eval" \
  "hepta-context-memory-ranked-recall-shadow-eval-report.sh" \
  "hepta-context-memory-ranked-recall-shadow-eval-gate.sh" \
  "deterministic-shadow" \
  "hybrid shadow-only" \
  "calibrated reranking shadow" \
  "ContextMemoryRankedRecallShadowEvalReport" \
  "ContextMemoryRankedRecallShadowHybridSignal" \
  "context_memory_ranked_recall_shadow_eval_report" \
  "query_match" \
  "recency_tie_break" \
  "budget_pressure" \
  "regression_guard" \
  "lexical_bm25" \
  "source_authority" \
  "temporal_validity" \
  "feedback" \
  "ranked item counts" \
  "hybrid-signal-min-basis-points" \
  "hybrid-positive-signal-pass-count" \
  "hybrid-regression-signal" \
  "calibrated-reranking-win-count" \
  "reranking-delta-min-basis-points" \
  "token-tradeoff-min-basis-points" \
  "reranking-regression-delta" \
  "routing diff shadow-only" \
  "production selection score" \
  "hybrid calibrated selection score" \
  "routing-diff-delta-min-basis-points" \
  "routing-diff-regression" \
  "real workload trace shadow-only" \
  "real_workload_trace_slo_pass" \
  "real_workload_trace_operator_review_required" \
  "real_workload_trace_total_leak_count" \
  "min-positive-real-workload-trace-coverage-basis-points" \
  "canary precondition shadow-only" \
  "canary_feature_flag_default_disabled" \
  "canary_kill_switch_default_enabled" \
  "canary_precondition_route_opened" \
  "recall-floor-basis-points" \
  "precision-floor-basis-points" \
  "token-saved-min-basis-points" \
  "latency-max-ms" \
  "regret-max-basis-points" \
  "regression fixture"; do
  assert_file_contains "$contracts" "$term" "ranked recall shadow eval contract"
done

assert_file_contains "$hepta_core_memory" \
  "CONTEXT_MEMORY_RANKED_RECALL_SHADOW_EVAL_SCHEMA_VERSION" \
  "ranked recall shadow eval schema version"
assert_file_contains "$hepta_core_memory" \
  "pub use eval_harness::ContextMemoryRankedRecallShadowEvalReport" \
  "ranked recall shadow eval public re-export"
assert_file_contains "$hepta_core_eval_harness" \
  "mod ranked_recall_shadow" \
  "ranked recall shadow eval module boundary"
assert_file_contains "$hepta_core_eval_harness" \
  "pub use ranked_recall_shadow::ContextMemoryRankedRecallShadowEvalReport" \
  "ranked recall shadow eval wrapper re-export"
assert_file_contains "$hepta_core_ranked_recall_shadow" \
  "ContextMemoryRankedRecallShadowEvalReport" \
  "ranked recall shadow eval rust report"
assert_file_contains "$hepta_core_ranked_recall_shadow" \
  "ContextMemoryRankedRecallShadowEvalFixtureResult" \
  "ranked recall shadow eval rust fixture"
assert_file_contains "$hepta_core_ranked_recall_shadow" \
  "ContextMemoryRankedRecallShadowHybridSignal" \
  "ranked recall shadow eval hybrid signal enum"
assert_file_contains "$hepta_core_ranked_recall_shadow" \
  "pub fn has_ranked_recall_shadow_integrity" \
  "ranked recall shadow eval integrity gate"
assert_file_contains "$hepta_core_ranked_recall_shadow" \
  "RANKED_RECALL_SHADOW_RECALL_FLOOR_BASIS_POINTS" \
  "ranked recall shadow eval recall threshold"
assert_file_contains "$hepta_core_ranked_recall_shadow" \
  "RANKED_RECALL_SHADOW_REGRET_MAX_BASIS_POINTS" \
  "ranked recall shadow eval regret threshold"
assert_file_contains "$hepta_core_ranked_recall_shadow" \
  "RANKED_RECALL_SHADOW_HYBRID_SIGNAL_MIN_BASIS_POINTS" \
  "ranked recall shadow eval hybrid threshold"
assert_file_contains "$hepta_core_ranked_recall_shadow" \
  "RANKED_RECALL_SHADOW_RERANKING_DELTA_MIN_BASIS_POINTS" \
  "ranked recall shadow eval reranking threshold"
assert_file_contains "$hepta_core_memory_tests" \
  "context_memory_ranked_recall_shadow_eval_tracks_metrics_without_activation" \
  "ranked recall shadow eval hepta-core positive test"
assert_file_contains "$hepta_core_memory_tests" \
  "context_memory_ranked_recall_shadow_eval_blocks_regression_drift" \
  "ranked recall shadow eval hepta-core regression test"
assert_file_contains "$hepta_core_memory_tests" \
  "context_memory_ranked_recall_shadow_eval_blocks_hybrid_signal_drift" \
  "ranked recall shadow eval hepta-core hybrid regression test"
assert_file_contains "$hepta_core_memory_tests" \
  "context_memory_ranked_recall_shadow_eval_blocks_calibrated_reranking_drift" \
  "ranked recall shadow eval hepta-core reranking regression test"
assert_file_contains "$hepta_core_memory_tests" \
  "context_memory_ranked_recall_shadow_eval_blocks_real_workload_slo_drift" \
  "ranked recall shadow eval hepta-core real workload SLO regression test"
assert_file_contains "$hepta_core_memory_tests" \
  "context_memory_ranked_recall_shadow_eval_blocks_canary_precondition_drift" \
  "ranked recall shadow eval hepta-core canary precondition regression test"

assert_file_contains "$hepta_memory" \
  "ranked-recall shadow eval" \
  "ranked recall shadow eval hepta-memory docs"
assert_file_contains "$hepta_memory_context_plane_helpers" \
  "context_memory_ranked_recall_shadow_eval_report" \
  "ranked recall shadow eval hepta-memory helper"
assert_file_contains "$hepta_memory_tests" \
  "store_snapshot_context_memory_ranked_recall_shadow_eval_is_payload_light" \
  "ranked recall shadow eval hepta-memory snapshot test"
assert_file_contains "$hepta_memory_tests" \
  "store_context_memory_ranked_recall_shadow_eval_matches_snapshot_helper" \
  "ranked recall shadow eval hepta-memory store test"

assert_file_contains "$debug_gate" "hepta-context-memory-ranked-recall-shadow-eval-gate.sh" \
  "ranked recall shadow eval debug gate"
assert_file_contains "$preflight_script" "context memory ranked recall shadow eval gate" \
  "ranked recall shadow eval preflight stage"
assert_file_contains "$release_manifest" "codex-rs/hepta-core/src/memory/eval_harness/ranked_recall_shadow.rs" \
  "ranked recall shadow eval rust release manifest"
assert_file_contains "$release_manifest" "scripts/hepta-context-memory-ranked-recall-shadow-eval-report.sh" \
  "ranked recall shadow eval report release manifest"
assert_file_contains "$release_manifest" "scripts/hepta-context-memory-ranked-recall-shadow-eval-gate.sh" \
  "ranked recall shadow eval gate release manifest"
assert_file_contains "$front_door_gate" "memory_ranked_recall_shadow_eval_gate_script" \
  "ranked recall shadow eval front-door static check"

assert_line_before \
  "$debug_gate" \
  "hepta-context-memory-recall-quality-gate.sh" \
  "hepta-context-memory-ranked-recall-shadow-eval-gate.sh" \
  "ranked recall shadow eval debug order"
assert_line_before \
  "$debug_gate" \
  "hepta-context-memory-ranked-recall-shadow-eval-gate.sh" \
  "hepta-context-plane-status-report-gate.sh" \
  "ranked recall shadow eval context-plane debug order"
assert_line_before \
  "$preflight_script" \
  "context memory recall quality gate" \
  "context memory ranked recall shadow eval gate" \
  "ranked recall shadow eval preflight order"
assert_line_before \
  "$preflight_script" \
  "context memory ranked recall shadow eval gate" \
  "context plane status/export report gate" \
  "ranked recall shadow eval context-plane preflight order"

leak_pattern='(prompt_text|transcript_text|memory_text|answer_text|session_id|memory_id|source_id|trace_id|query_text|query_payload|tool_args|tool_outputs|raw_payload|raw_ranked_payload|rank_explanation|score_reason|snippet_hash=|rollback_hash|runtime-activation=enabled|production-route=enabled|operator-activation=enabled|graph-write=enabled|production-write=enabled)'
if grep -E "$leak_pattern" "$report_output" >/dev/null; then
  fail "ranked recall shadow eval report leaked payload or activation state"
fi

bash -n "$report_script"
bash -n "$gate_script"

cargo test --manifest-path "$manifest" -p hepta-core \
  ranked_recall_shadow \
  --lib --message-format=short

cargo test --manifest-path "$manifest" -p hepta-memory \
  ranked_recall_shadow \
  --lib --message-format=short

echo "ranked-recall-shadow-eval=pass"
echo "ranked-recall-shadow-eval.payload-light=pass"
echo "ranked-recall-shadow-eval.fixtures=4"
echo "ranked-recall-shadow-eval.hybrid-signals=5"
echo "ranked-recall-shadow-eval.calibrated-reranking=shadow"
echo "ranked-recall-shadow-eval.regression-fixture=blocked"
echo "ranked-recall-shadow-eval.runtime-activation=disabled"
