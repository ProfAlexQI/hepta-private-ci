#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
manifest="$repo_root/codex-rs/Cargo.toml"
contracts="$repo_root/codex-rs/CONTEXT_DEBUG_CONTRACTS.md"
debug_gate="$repo_root/scripts/hepta-context-debug-gate.sh"
preflight_script="$repo_root/scripts/hepta-context-preflight.sh"
release_manifest="$repo_root/codex-rs/CONTEXT_LANE_RELEASE_MANIFEST.tsv"
report_script="$repo_root/scripts/hepta-context-memory-shadow-regression-dashboard-report.sh"
gate_script="$repo_root/scripts/hepta-context-memory-shadow-regression-dashboard-gate.sh"
front_door_gate="$repo_root/scripts/hepta-context-source-aware-compression-front-door-gate.sh"
ranked_gate="$repo_root/scripts/hepta-context-memory-ranked-recall-shadow-eval-gate.sh"
temporal_gate="$repo_root/scripts/hepta-context-memory-temporal-graph-shadow-eval-gate.sh"
recall_quality_gate="$repo_root/scripts/hepta-context-memory-recall-quality-gate.sh"
provider_gate="$repo_root/scripts/hepta-context-memory-provider-boundary-gate.sh"
hepta_core_memory="$repo_root/codex-rs/hepta-core/src/memory.rs"
hepta_core_eval_harness="$repo_root/codex-rs/hepta-core/src/memory/eval_harness.rs"
hepta_core_shadow_dashboard="$repo_root/codex-rs/hepta-core/src/memory/eval_harness/shadow_regression_dashboard.rs"
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
report_output="$(mktemp -t hepta-context-memory-shadow-regression-dashboard-report.XXXXXX)"

cleanup() {
  rm -f "$report_output"
}
trap cleanup EXIT

fail() {
  echo "hepta-context-memory-shadow-regression-dashboard-gate: $*" >&2
  if [ -s "$report_output" ]; then
    echo "memory shadow regression dashboard report output:" >&2
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
    fail "memory shadow regression dashboard report must contain line: $expected"
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

bash "$ranked_gate" >/dev/null
bash "$temporal_gate" >/dev/null
bash "$recall_quality_gate" >/dev/null
bash "$provider_gate" >/dev/null
bash "$report_script" >"$report_output"

assert_report_line "memory-shadow-regression-dashboard=pass"
assert_report_line "memory-shadow-regression-dashboard.payload-light=pass"
assert_report_line "memory-shadow-regression-dashboard.schema=2"
assert_report_line "memory-shadow-regression-dashboard.mode=shadow-only"
assert_report_line "memory-shadow-regression-dashboard.input-report-count=4"
assert_report_line "memory-shadow-regression-dashboard.input-report-pass-count=4"
assert_report_line "memory-shadow-regression-dashboard.regression-blocking-count=0"
assert_report_line "memory-shadow-regression-dashboard.ranked-recall-fixture-count=4"
assert_report_line "memory-shadow-regression-dashboard.ranked-recall-regression-fixture=blocked"
assert_report_line "memory-shadow-regression-dashboard.ranked-recall-min-positive-recall-basis-points=8000"
assert_report_line "memory-shadow-regression-dashboard.ranked-recall-min-positive-precision-basis-points=8000"
assert_report_line "memory-shadow-regression-dashboard.ranked-recall-total-positive-token-saved=2140"
assert_report_line "memory-shadow-regression-dashboard.ranked-recall-max-positive-latency-ms=55"
assert_report_line "memory-shadow-regression-dashboard.ranked-recall-comparison-summary=pass"
assert_report_line "memory-shadow-regression-dashboard.ranked-recall-hybrid-signal-count=5"
assert_report_line "memory-shadow-regression-dashboard.ranked-recall-positive-hybrid-signal-pass-count=15"
assert_report_line "memory-shadow-regression-dashboard.ranked-recall-hybrid-regression-blocked-count=1"
assert_report_line "memory-shadow-regression-dashboard.ranked-recall-min-positive-hybrid-score-basis-points=7800"
assert_report_line "memory-shadow-regression-dashboard.ranked-recall-calibrated-reranking-win-count=3"
assert_report_line "memory-shadow-regression-dashboard.ranked-recall-calibrated-reranking-loss-count=1"
assert_report_line "memory-shadow-regression-dashboard.ranked-recall-min-positive-reranking-delta-basis-points=640"
assert_report_line "memory-shadow-regression-dashboard.ranked-recall-max-positive-latency-delta-ms=10"
assert_report_line "memory-shadow-regression-dashboard.ranked-recall-min-positive-token-tradeoff-basis-points=3000"
assert_report_line "memory-shadow-regression-dashboard.ranked-recall-reranking-regression-blocked-count=1"
assert_report_line "memory-shadow-regression-dashboard.temporal-graph-fixture-count=4"
assert_report_line "memory-shadow-regression-dashboard.temporal-graph-regression-fixture=blocked"
assert_report_line "memory-shadow-regression-dashboard.temporal-graph-min-positive-node-coverage-basis-points=10000"
assert_report_line "memory-shadow-regression-dashboard.temporal-graph-min-positive-edge-coverage-basis-points=10000"
assert_report_line "memory-shadow-regression-dashboard.temporal-graph-min-positive-validity-window-coverage-basis-points=10000"
assert_report_line "memory-shadow-regression-dashboard.temporal-graph-min-positive-supersedes-coverage-basis-points=10000"
assert_report_line "memory-shadow-regression-dashboard.temporal-graph-max-positive-latency-ms=47"
assert_report_line "memory-shadow-regression-dashboard.recall-quality-fixture-count=2"
assert_report_line "memory-shadow-regression-dashboard.recall-quality-blocking-reason-count=0"
assert_report_line "memory-shadow-regression-dashboard.provider-boundary=pass"
assert_report_line "memory-shadow-regression-dashboard.provider-payload-light=pass"
assert_report_line "memory-shadow-regression-dashboard.operator-approval=required"
assert_report_line "memory-shadow-regression-dashboard.production-route=disabled"
assert_report_line "memory-shadow-regression-dashboard.production-write=disabled"
assert_report_line "memory-shadow-regression-dashboard.graph-write=disabled"
assert_report_line "memory-shadow-regression-dashboard.runtime-activation=disabled"

for term in \
  "Memory shadow regression dashboard" \
  "ContextMemoryShadowRegressionDashboardReport" \
  "context_memory_shadow_regression_dashboard_report" \
  "ranked recall shadow" \
  "temporal graph shadow" \
  "recall quality" \
  "provider boundary" \
  "input_report_count" \
  "input_report_pass_count" \
  "regression_blocking_count" \
  "ranked_recall_comparison_summary_pass" \
  "ranked_recall_min_positive_hybrid_score_basis_points" \
  "ranked_recall_min_positive_reranking_delta_basis_points" \
  "ranked_recall_min_positive_token_tradeoff_basis_points" \
  "provider_payload_light" \
  "hepta-context-memory-shadow-regression-dashboard-report.sh" \
  "hepta-context-memory-shadow-regression-dashboard-gate.sh"; do
  assert_file_contains "$contracts" "$term" "memory shadow regression dashboard contract"
done

assert_file_contains "$hepta_core_memory" \
  "CONTEXT_MEMORY_SHADOW_REGRESSION_DASHBOARD_SCHEMA_VERSION" \
  "memory shadow regression dashboard schema version"
assert_file_contains "$hepta_core_memory" \
  "pub use eval_harness::ContextMemoryShadowRegressionDashboardReport" \
  "memory shadow regression dashboard public re-export"
assert_file_contains "$hepta_core_eval_harness" \
  "mod shadow_regression_dashboard" \
  "memory shadow regression dashboard module boundary"
assert_file_contains "$hepta_core_eval_harness" \
  "pub use shadow_regression_dashboard::ContextMemoryShadowRegressionDashboardReport" \
  "memory shadow regression dashboard wrapper re-export"
assert_file_contains "$hepta_core_shadow_dashboard" \
  "ContextMemoryShadowRegressionDashboardReport" \
  "memory shadow regression dashboard rust report"
assert_file_contains "$hepta_core_shadow_dashboard" \
  "pub fn from_reports" \
  "memory shadow regression dashboard rust adapter"
assert_file_contains "$hepta_core_shadow_dashboard" \
  "pub fn has_shadow_regression_dashboard_integrity" \
  "memory shadow regression dashboard integrity gate"
assert_file_contains "$hepta_core_memory_tests" \
  "context_memory_shadow_regression_dashboard_rolls_up_shadow_reports_without_activation" \
  "memory shadow regression dashboard hepta-core positive test"
assert_file_contains "$hepta_core_memory_tests" \
  "context_memory_shadow_regression_dashboard_blocks_input_regression_drift" \
  "memory shadow regression dashboard hepta-core regression test"
assert_file_contains "$hepta_core_memory_tests" \
  "context_memory_shadow_regression_dashboard_blocks_ranked_recall_comparison_false_green" \
  "memory shadow regression dashboard ranked recall comparison false-green test"

assert_file_contains "$hepta_memory" \
  "memory shadow regression dashboard" \
  "memory shadow regression dashboard hepta-memory docs"
assert_file_contains "$hepta_memory_context_plane_helpers" \
  "context_memory_shadow_regression_dashboard_report" \
  "memory shadow regression dashboard hepta-memory helper"
assert_file_contains "$hepta_memory_tests" \
  "store_snapshot_context_memory_shadow_regression_dashboard_is_payload_light" \
  "memory shadow regression dashboard hepta-memory snapshot test"
assert_file_contains "$hepta_memory_tests" \
  "store_context_memory_shadow_regression_dashboard_matches_snapshot_helper" \
  "memory shadow regression dashboard hepta-memory store test"

assert_file_contains "$debug_gate" "hepta-context-memory-shadow-regression-dashboard-gate.sh" \
  "memory shadow regression dashboard debug gate"
assert_file_contains "$preflight_script" "context memory shadow regression dashboard gate" \
  "memory shadow regression dashboard preflight stage"
assert_file_contains "$release_manifest" "codex-rs/hepta-core/src/memory/eval_harness/shadow_regression_dashboard.rs" \
  "memory shadow regression dashboard rust release manifest"
assert_file_contains "$release_manifest" "scripts/hepta-context-memory-shadow-regression-dashboard-report.sh" \
  "memory shadow regression dashboard report release manifest"
assert_file_contains "$release_manifest" "scripts/hepta-context-memory-shadow-regression-dashboard-gate.sh" \
  "memory shadow regression dashboard gate release manifest"
assert_file_contains "$front_door_gate" "memory_shadow_regression_dashboard_gate_script" \
  "memory shadow regression dashboard front-door static check"

assert_line_before \
  "$debug_gate" \
  "hepta-context-memory-provider-boundary-gate.sh" \
  "hepta-context-memory-shadow-regression-dashboard-gate.sh" \
  "memory shadow regression dashboard debug order"
assert_line_before \
  "$debug_gate" \
  "hepta-context-memory-shadow-regression-dashboard-gate.sh" \
  "hepta-context-plane-status-report-gate.sh" \
  "memory shadow regression dashboard context-plane debug order"
assert_line_before \
  "$preflight_script" \
  "context memory provider boundary gate" \
  "context memory shadow regression dashboard gate" \
  "memory shadow regression dashboard preflight order"
assert_line_before \
  "$preflight_script" \
  "context memory shadow regression dashboard gate" \
  "context plane status/export report gate" \
  "memory shadow regression dashboard context-plane preflight order"

leak_pattern='(prompt_text|transcript_text|memory_text|answer_text|session_id|memory_id|source_id|trace_id|query_text|query_payload|tool_args|tool_outputs|raw_payload|raw_ranked_payload|raw_graph_payload|ranked_payload=|operator_identity|runtime-activation=enabled|production-route=enabled|operator-activation=enabled|graph-write=enabled|production-write=enabled)'
if grep -E "$leak_pattern" "$report_output" >/dev/null; then
  fail "memory shadow regression dashboard report leaked payload or activation state"
fi

bash -n "$report_script"
bash -n "$gate_script"

cargo test --manifest-path "$manifest" -p hepta-core \
  shadow_regression_dashboard \
  --lib --message-format=short

cargo test --manifest-path "$manifest" -p hepta-memory \
  shadow_regression_dashboard \
  --lib --message-format=short

echo "memory-shadow-regression-dashboard=pass"
echo "memory-shadow-regression-dashboard.payload-light=pass"
echo "memory-shadow-regression-dashboard.input-report-pass-count=4"
echo "memory-shadow-regression-dashboard.regression-blocking-count=0"
echo "memory-shadow-regression-dashboard.runtime-activation=disabled"
