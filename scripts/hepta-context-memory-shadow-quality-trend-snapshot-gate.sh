#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
manifest="$repo_root/codex-rs/Cargo.toml"
contracts="$repo_root/codex-rs/CONTEXT_DEBUG_CONTRACTS.md"
debug_gate="$repo_root/scripts/hepta-context-debug-gate.sh"
preflight_script="$repo_root/scripts/hepta-context-preflight.sh"
release_manifest="$repo_root/codex-rs/CONTEXT_LANE_RELEASE_MANIFEST.tsv"
report_script="$repo_root/scripts/hepta-context-memory-shadow-quality-trend-snapshot-report.sh"
gate_script="$repo_root/scripts/hepta-context-memory-shadow-quality-trend-snapshot-gate.sh"
front_door_gate="$repo_root/scripts/hepta-context-source-aware-compression-front-door-gate.sh"
summary_gate="$repo_root/scripts/hepta-context-memory-shadow-quality-summary-gate.sh"
hepta_core_memory="$repo_root/codex-rs/hepta-core/src/memory.rs"
hepta_core_eval_harness="$repo_root/codex-rs/hepta-core/src/memory/eval_harness.rs"
hepta_core_trend_snapshot="$repo_root/codex-rs/hepta-core/src/memory/eval_harness/shadow_quality_trend_snapshot.rs"
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
report_output="$(mktemp -t hepta-context-memory-shadow-quality-trend-snapshot-report.XXXXXX)"

cleanup() {
  rm -f "$report_output"
}
trap cleanup EXIT

fail() {
  echo "hepta-context-memory-shadow-quality-trend-snapshot-gate: $*" >&2
  if [ -s "$report_output" ]; then
    echo "memory shadow quality trend snapshot report output:" >&2
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
    fail "memory shadow quality trend snapshot report must contain line: $expected"
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

bash "$summary_gate" >/dev/null
bash "$report_script" >"$report_output"

assert_report_line "memory-shadow-quality-trend-snapshot=pass"
assert_report_line "memory-shadow-quality-trend-snapshot.payload-light=pass"
assert_report_line "memory-shadow-quality-trend-snapshot.schema=2"
assert_report_line "memory-shadow-quality-trend-snapshot.mode=shadow-only"
assert_report_line "memory-shadow-quality-trend-snapshot.source-summary=pass"
assert_report_line "memory-shadow-quality-trend-snapshot.current-quality-trend=stable-pass"
assert_report_line "memory-shadow-quality-trend-snapshot.current-operator-summary=ready-shadow-only"
assert_report_line "memory-shadow-quality-trend-snapshot.window-observation-count=3"
assert_report_line "memory-shadow-quality-trend-snapshot.required-pass-streak=3"
assert_report_line "memory-shadow-quality-trend-snapshot.observed-pass-streak=3"
assert_report_line "memory-shadow-quality-trend-snapshot.stable-observation-count=3"
assert_report_line "memory-shadow-quality-trend-snapshot.trend-window=stable-window"
assert_report_line "memory-shadow-quality-trend-snapshot.regression-window-blocking-count=0"
assert_report_line "memory-shadow-quality-trend-snapshot.quality-signal-window-pass-count=12"
assert_report_line "memory-shadow-quality-trend-snapshot.ranked-recall-window-pass-count=3"
assert_report_line "memory-shadow-quality-trend-snapshot.ranked-recall-comparison-window-pass-count=3"
assert_report_line "memory-shadow-quality-trend-snapshot.ranked-recall-min-positive-hybrid-score-basis-points=7800"
assert_report_line "memory-shadow-quality-trend-snapshot.ranked-recall-min-positive-reranking-delta-basis-points=640"
assert_report_line "memory-shadow-quality-trend-snapshot.ranked-recall-max-positive-latency-delta-ms=10"
assert_report_line "memory-shadow-quality-trend-snapshot.ranked-recall-min-positive-token-tradeoff-basis-points=3000"
assert_report_line "memory-shadow-quality-trend-snapshot.temporal-graph-window-pass-count=3"
assert_report_line "memory-shadow-quality-trend-snapshot.recall-quality-window-pass-count=3"
assert_report_line "memory-shadow-quality-trend-snapshot.provider-boundary-window-pass-count=3"
assert_report_line "memory-shadow-quality-trend-snapshot.operator-snapshot-redacted=pass"
assert_report_line "memory-shadow-quality-trend-snapshot.operator-approval=required"
assert_report_line "memory-shadow-quality-trend-snapshot.history-persistence-write=disabled"
assert_report_line "memory-shadow-quality-trend-snapshot.production-route=disabled"
assert_report_line "memory-shadow-quality-trend-snapshot.production-write=disabled"
assert_report_line "memory-shadow-quality-trend-snapshot.graph-write=disabled"
assert_report_line "memory-shadow-quality-trend-snapshot.runtime-activation=disabled"

for term in \
  "Memory shadow quality trend snapshot" \
  "ContextMemoryShadowQualityTrendSnapshotReport" \
  "ContextMemoryShadowQualityTrendSnapshotMode" \
  "ContextMemoryShadowQualityTrendWindowVerdict" \
  "context_memory_shadow_quality_trend_snapshot_report" \
  "window_observation_count" \
  "required_pass_streak" \
  "observed_pass_streak" \
  "regression_window_blocking_count" \
  "quality_signal_window_pass_count" \
  "ranked_recall_comparison_window_pass_count" \
  "ranked_recall_min_positive_hybrid_score_basis_points" \
  "ranked_recall_min_positive_reranking_delta_basis_points" \
  "ranked_recall_min_positive_token_tradeoff_basis_points" \
  "operator_snapshot_redacted" \
  "stable_window" \
  "history_persistence_write" \
  "hepta-context-memory-shadow-quality-trend-snapshot-report.sh" \
  "hepta-context-memory-shadow-quality-trend-snapshot-gate.sh"; do
  assert_file_contains "$contracts" "$term" "memory shadow quality trend snapshot contract"
done

assert_file_contains "$hepta_core_memory" \
  "CONTEXT_MEMORY_SHADOW_QUALITY_TREND_SNAPSHOT_SCHEMA_VERSION" \
  "memory shadow quality trend snapshot schema version"
assert_file_contains "$hepta_core_memory" \
  "pub use eval_harness::ContextMemoryShadowQualityTrendSnapshotReport" \
  "memory shadow quality trend snapshot public re-export"
assert_file_contains "$hepta_core_eval_harness" \
  "mod shadow_quality_trend_snapshot" \
  "memory shadow quality trend snapshot module boundary"
assert_file_contains "$hepta_core_eval_harness" \
  "pub use shadow_quality_trend_snapshot::ContextMemoryShadowQualityTrendSnapshotReport" \
  "memory shadow quality trend snapshot wrapper re-export"
assert_file_contains "$hepta_core_trend_snapshot" \
  "ContextMemoryShadowQualityTrendSnapshotReport" \
  "memory shadow quality trend snapshot rust report"
assert_file_contains "$hepta_core_trend_snapshot" \
  "pub fn from_summary" \
  "memory shadow quality trend snapshot rust adapter"
assert_file_contains "$hepta_core_trend_snapshot" \
  "pub fn has_shadow_quality_trend_snapshot_integrity" \
  "memory shadow quality trend snapshot integrity gate"
assert_file_contains "$hepta_core_memory_tests" \
  "context_memory_shadow_quality_trend_snapshot_rolls_up_summary_window_without_activation" \
  "memory shadow quality trend snapshot hepta-core positive test"
assert_file_contains "$hepta_core_memory_tests" \
  "context_memory_shadow_quality_trend_snapshot_blocks_summary_regression_drift" \
  "memory shadow quality trend snapshot hepta-core regression test"

assert_file_contains "$hepta_memory" \
  "shadow quality trend snapshot" \
  "memory shadow quality trend snapshot hepta-memory docs"
assert_file_contains "$hepta_memory_context_plane_helpers" \
  "context_memory_shadow_quality_trend_snapshot_report" \
  "memory shadow quality trend snapshot hepta-memory helper"
assert_file_contains "$hepta_memory_tests" \
  "store_snapshot_context_memory_shadow_quality_trend_snapshot_is_payload_light" \
  "memory shadow quality trend snapshot hepta-memory snapshot test"
assert_file_contains "$hepta_memory_tests" \
  "store_context_memory_shadow_quality_trend_snapshot_matches_snapshot_helper" \
  "memory shadow quality trend snapshot hepta-memory store test"

assert_file_contains "$debug_gate" "hepta-context-memory-shadow-quality-trend-snapshot-gate.sh" \
  "memory shadow quality trend snapshot debug gate"
assert_file_contains "$preflight_script" "context memory shadow quality trend snapshot gate" \
  "memory shadow quality trend snapshot preflight stage"
assert_file_contains "$release_manifest" "codex-rs/hepta-core/src/memory/eval_harness/shadow_quality_trend_snapshot.rs" \
  "memory shadow quality trend snapshot rust release manifest"
assert_file_contains "$release_manifest" "scripts/hepta-context-memory-shadow-quality-trend-snapshot-report.sh" \
  "memory shadow quality trend snapshot report release manifest"
assert_file_contains "$release_manifest" "scripts/hepta-context-memory-shadow-quality-trend-snapshot-gate.sh" \
  "memory shadow quality trend snapshot gate release manifest"
assert_file_contains "$front_door_gate" "memory_shadow_quality_trend_snapshot_gate_script" \
  "memory shadow quality trend snapshot front-door static check"

assert_line_before \
  "$debug_gate" \
  "hepta-context-memory-shadow-quality-summary-gate.sh" \
  "hepta-context-memory-shadow-quality-trend-snapshot-gate.sh" \
  "memory shadow quality trend snapshot debug order"
assert_line_before \
  "$debug_gate" \
  "hepta-context-memory-shadow-quality-trend-snapshot-gate.sh" \
  "hepta-context-plane-status-report-gate.sh" \
  "memory shadow quality trend snapshot context-plane debug order"
assert_line_before \
  "$preflight_script" \
  "context memory shadow quality summary gate" \
  "context memory shadow quality trend snapshot gate" \
  "memory shadow quality trend snapshot preflight order"
assert_line_before \
  "$preflight_script" \
  "context memory shadow quality trend snapshot gate" \
  "context plane status/export report gate" \
  "memory shadow quality trend snapshot context-plane preflight order"

leak_pattern='(prompt_text|transcript_text|memory_text|answer_text|session_id|memory_id|source_id|trace_id|query_text|query_payload|tool_args|tool_outputs|raw_payload|raw_ranked_payload|raw_graph_payload|ranked_payload=|operator_identity|runtime-activation=enabled|production-route=enabled|operator-activation=enabled|graph-write=enabled|production-write=enabled|history-persistence-write=enabled)'
if grep -E "$leak_pattern" "$report_output" >/dev/null; then
  fail "memory shadow quality trend snapshot report leaked payload or activation state"
fi

bash -n "$report_script"
bash -n "$gate_script"

cargo test --manifest-path "$manifest" -p hepta-core \
  trend_snapshot \
  --lib --message-format=short

cargo test --manifest-path "$manifest" -p hepta-memory \
  trend_snapshot \
  --lib --message-format=short

echo "memory-shadow-quality-trend-snapshot=pass"
echo "memory-shadow-quality-trend-snapshot.payload-light=pass"
echo "memory-shadow-quality-trend-snapshot.trend-window=stable-window"
echo "memory-shadow-quality-trend-snapshot.regression-window-blocking-count=0"
echo "memory-shadow-quality-trend-snapshot.runtime-activation=disabled"
