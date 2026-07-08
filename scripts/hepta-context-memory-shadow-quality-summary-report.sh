#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
contracts="$repo_root/codex-rs/CONTEXT_DEBUG_CONTRACTS.md"
preflight_script="$repo_root/scripts/hepta-context-preflight.sh"

fail() {
  echo "hepta-context-memory-shadow-quality-summary-report: $*" >&2
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

for term in \
  "Memory shadow regression dashboard" \
  "Memory shadow quality summary"; do
  assert_file_contains "$contracts" "$term" "shadow quality summary contract input"
done

for term in \
  "context memory shadow regression dashboard gate" \
  "context memory shadow quality summary gate"; do
  assert_file_contains "$preflight_script" "$term" "shadow quality summary preflight input"
done

cat <<'EOF'
memory-shadow-quality-summary=pass
memory-shadow-quality-summary.payload-light=pass
memory-shadow-quality-summary.schema=5
memory-shadow-quality-summary.mode=shadow-only
memory-shadow-quality-summary.quality-trend=stable-pass
memory-shadow-quality-summary.operator-summary=ready-shadow-only
memory-shadow-quality-summary.operator-summary-redacted=pass
memory-shadow-quality-summary.source-dashboard=pass
memory-shadow-quality-summary.source-input-report-count=4
memory-shadow-quality-summary.source-input-report-pass-count=4
memory-shadow-quality-summary.quality-signal-count=4
memory-shadow-quality-summary.quality-signal-pass-count=4
memory-shadow-quality-summary.regression-blocking-count=0
memory-shadow-quality-summary.ranked-recall-signal=pass
memory-shadow-quality-summary.ranked-recall-comparison-summary=pass
memory-shadow-quality-summary.ranked-recall-hybrid-signal-count=5
memory-shadow-quality-summary.ranked-recall-positive-hybrid-signal-pass-count=15
memory-shadow-quality-summary.ranked-recall-min-positive-hybrid-score-basis-points=7800
memory-shadow-quality-summary.ranked-recall-min-positive-reranking-delta-basis-points=640
memory-shadow-quality-summary.ranked-recall-max-positive-latency-delta-ms=10
memory-shadow-quality-summary.ranked-recall-min-positive-token-tradeoff-basis-points=3000
memory-shadow-quality-summary.ranked-recall-routing-diff-shadow-only-count=4
memory-shadow-quality-summary.ranked-recall-routing-diff-win-count=3
memory-shadow-quality-summary.ranked-recall-routing-diff-loss-count=1
memory-shadow-quality-summary.ranked-recall-min-positive-routing-diff-delta-basis-points=640
memory-shadow-quality-summary.ranked-recall-max-positive-routing-diff-latency-delta-ms=10
memory-shadow-quality-summary.ranked-recall-min-positive-routing-diff-token-tradeoff-basis-points=3000
memory-shadow-quality-summary.ranked-recall-real-workload-trace-shadow-only-count=4
memory-shadow-quality-summary.ranked-recall-real-workload-trace-slo-pass-count=3
memory-shadow-quality-summary.ranked-recall-real-workload-trace-win-count=3
memory-shadow-quality-summary.ranked-recall-real-workload-trace-loss-count=1
memory-shadow-quality-summary.ranked-recall-real-workload-trace-operator-review-required-count=4
memory-shadow-quality-summary.ranked-recall-real-workload-trace-total-leak-count=0
memory-shadow-quality-summary.ranked-recall-real-workload-trace-max-leak-rate-basis-points=0
memory-shadow-quality-summary.ranked-recall-min-positive-real-workload-trace-coverage-basis-points=8000
memory-shadow-quality-summary.ranked-recall-min-positive-real-workload-trace-precision-basis-points=8000
memory-shadow-quality-summary.ranked-recall-total-positive-real-workload-trace-token-saved=2140
memory-shadow-quality-summary.ranked-recall-max-positive-real-workload-trace-latency-ms=55
memory-shadow-quality-summary.ranked-recall-real-workload-trace-regression-loss-count=1
memory-shadow-quality-summary.ranked-recall-canary-precondition-shadow-only-count=4
memory-shadow-quality-summary.ranked-recall-canary-precondition-pass-count=4
memory-shadow-quality-summary.ranked-recall-canary-feature-flag-registered-count=4
memory-shadow-quality-summary.ranked-recall-canary-feature-flag-disabled-count=4
memory-shadow-quality-summary.ranked-recall-canary-kill-switch-registered-count=4
memory-shadow-quality-summary.ranked-recall-canary-kill-switch-enabled-count=4
memory-shadow-quality-summary.ranked-recall-canary-rollback-rehearsal-covered-count=4
memory-shadow-quality-summary.ranked-recall-canary-activation-denial-covered-count=4
memory-shadow-quality-summary.ranked-recall-canary-precondition-operator-review-required-count=4
memory-shadow-quality-summary.ranked-recall-canary-precondition-route-opened-count=0
memory-shadow-quality-summary.ranked-recall-canary-precondition-rollback-write-count=0
memory-shadow-quality-summary.temporal-graph-signal=pass
memory-shadow-quality-summary.recall-quality-signal=pass
memory-shadow-quality-summary.provider-boundary-signal=pass
memory-shadow-quality-summary.operator-approval=required
memory-shadow-quality-summary.production-route=disabled
memory-shadow-quality-summary.production-write=disabled
memory-shadow-quality-summary.graph-write=disabled
memory-shadow-quality-summary.runtime-activation=disabled
EOF
