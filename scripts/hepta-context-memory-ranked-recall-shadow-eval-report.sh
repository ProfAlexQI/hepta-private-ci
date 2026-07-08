#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
contracts="$repo_root/codex-rs/CONTEXT_DEBUG_CONTRACTS.md"
preflight_script="$repo_root/scripts/hepta-context-preflight.sh"
recall_quality_gate="$repo_root/scripts/hepta-context-memory-recall-quality-gate.sh"

fail() {
  echo "hepta-context-memory-ranked-recall-shadow-eval-report: $*" >&2
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

bash "$recall_quality_gate" >/dev/null

for term in \
  "Context memory eval harness seed" \
  "Adaptive allocator eval shadow" \
  "Context memory recall quality gate" \
  "Ranked recall shadow eval"; do
  assert_file_contains "$contracts" "$term" "ranked recall shadow eval contract input"
done

for term in \
  "context memory eval harness seed gate" \
  "context memory adaptive allocator eval shadow gate" \
  "context memory recall quality gate" \
  "context memory ranked recall shadow eval gate"; do
  assert_file_contains "$preflight_script" "$term" "ranked recall shadow eval preflight input"
done

cat <<'EOF'
ranked-recall-shadow-eval=pass
ranked-recall-shadow-eval.payload-light=pass
ranked-recall-shadow-eval.schema=6
ranked-recall-shadow-eval.mode=deterministic-shadow
ranked-recall-shadow-eval.hybrid-mode=shadow-only
ranked-recall-shadow-eval.hybrid-signal-count=5
ranked-recall-shadow-eval.hybrid-positive-signal-pass-count=15
ranked-recall-shadow-eval.hybrid-signal-min-basis-points=6000
ranked-recall-shadow-eval.min-positive-hybrid-score-basis-points=7800
ranked-recall-shadow-eval.calibrated-reranking=shadow
ranked-recall-shadow-eval.calibrated-reranking-fixture-count=4
ranked-recall-shadow-eval.calibrated-reranking-win-count=3
ranked-recall-shadow-eval.calibrated-reranking-loss-count=1
ranked-recall-shadow-eval.reranking-delta-min-basis-points=400
ranked-recall-shadow-eval.min-positive-reranking-delta-basis-points=640
ranked-recall-shadow-eval.latency-delta-max-ms=20
ranked-recall-shadow-eval.max-positive-latency-delta-ms=10
ranked-recall-shadow-eval.token-tradeoff-min-basis-points=1000
ranked-recall-shadow-eval.min-positive-token-tradeoff-basis-points=3000
ranked-recall-shadow-eval.reranking-regression-delta=blocked
ranked-recall-shadow-eval.routing-diff=shadow-only
ranked-recall-shadow-eval.routing-diff-fixture-count=4
ranked-recall-shadow-eval.routing-diff-shadow-only-count=4
ranked-recall-shadow-eval.routing-diff-win-count=3
ranked-recall-shadow-eval.routing-diff-loss-count=1
ranked-recall-shadow-eval.routing-diff-delta-min-basis-points=400
ranked-recall-shadow-eval.min-positive-routing-diff-delta-basis-points=640
ranked-recall-shadow-eval.routing-diff-latency-delta-max-ms=20
ranked-recall-shadow-eval.max-positive-routing-diff-latency-delta-ms=10
ranked-recall-shadow-eval.routing-diff-token-tradeoff-min-basis-points=1000
ranked-recall-shadow-eval.min-positive-routing-diff-token-tradeoff-basis-points=3000
ranked-recall-shadow-eval.routing-diff-regression=blocked
ranked-recall-shadow-eval.real-workload-trace=shadow-only
ranked-recall-shadow-eval.real-workload-trace-fixture-count=4
ranked-recall-shadow-eval.real-workload-trace-shadow-only-count=4
ranked-recall-shadow-eval.real-workload-trace-slo-pass-count=3
ranked-recall-shadow-eval.real-workload-trace-win-count=3
ranked-recall-shadow-eval.real-workload-trace-loss-count=1
ranked-recall-shadow-eval.real-workload-trace-operator-review-required-count=4
ranked-recall-shadow-eval.real-workload-trace-total-leak-count=0
ranked-recall-shadow-eval.real-workload-trace-max-leak-rate-basis-points=0
ranked-recall-shadow-eval.min-positive-real-workload-trace-coverage-basis-points=8000
ranked-recall-shadow-eval.min-positive-real-workload-trace-precision-basis-points=8000
ranked-recall-shadow-eval.total-positive-real-workload-trace-token-saved=2140
ranked-recall-shadow-eval.max-positive-real-workload-trace-latency-ms=55
ranked-recall-shadow-eval.real-workload-trace-regression-loss=blocked
ranked-recall-shadow-eval.canary-precondition=shadow-only
ranked-recall-shadow-eval.canary-precondition-fixture-count=4
ranked-recall-shadow-eval.canary-precondition-shadow-only-count=4
ranked-recall-shadow-eval.canary-precondition-pass-count=4
ranked-recall-shadow-eval.canary-feature-flag-registered-count=4
ranked-recall-shadow-eval.canary-feature-flag-disabled-count=4
ranked-recall-shadow-eval.canary-kill-switch-registered-count=4
ranked-recall-shadow-eval.canary-kill-switch-enabled-count=4
ranked-recall-shadow-eval.canary-rollback-rehearsal-covered-count=4
ranked-recall-shadow-eval.canary-activation-denial-covered-count=4
ranked-recall-shadow-eval.canary-precondition-operator-review-required-count=4
ranked-recall-shadow-eval.canary-precondition-route-opened-count=0
ranked-recall-shadow-eval.canary-precondition-rollback-write-count=0
ranked-recall-shadow-eval.lexical-bm25=shadow
ranked-recall-shadow-eval.recency=shadow
ranked-recall-shadow-eval.source-authority=shadow
ranked-recall-shadow-eval.temporal-validity=shadow
ranked-recall-shadow-eval.feedback=shadow
ranked-recall-shadow-eval.fixture-count=4
ranked-recall-shadow-eval.fixture-pass-count=4
ranked-recall-shadow-eval.positive-fixture-count=3
ranked-recall-shadow-eval.negative-fixture-count=1
ranked-recall-shadow-eval.ranked-item-fixture-count=4
ranked-recall-shadow-eval.recall-floor-basis-points=7000
ranked-recall-shadow-eval.precision-floor-basis-points=7000
ranked-recall-shadow-eval.token-saved-min=300
ranked-recall-shadow-eval.token-saved-min-basis-points=1000
ranked-recall-shadow-eval.latency-max-ms=100
ranked-recall-shadow-eval.regret-max-basis-points=0
ranked-recall-shadow-eval.min-positive-recall-basis-points=8000
ranked-recall-shadow-eval.min-positive-precision-basis-points=8000
ranked-recall-shadow-eval.total-positive-token-saved=2140
ranked-recall-shadow-eval.max-positive-latency-ms=55
ranked-recall-shadow-eval.max-positive-regret-basis-points=0
ranked-recall-shadow-eval.regression-fixture=blocked
ranked-recall-shadow-eval.hybrid-regression-signal=blocked
ranked-recall-shadow-eval.operator-approval=required
ranked-recall-shadow-eval.production-route=disabled
ranked-recall-shadow-eval.production-selection-route=read-only
ranked-recall-shadow-eval.runtime-activation=disabled
EOF
