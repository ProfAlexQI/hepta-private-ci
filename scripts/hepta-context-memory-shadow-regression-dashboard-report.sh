#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
contracts="$repo_root/codex-rs/CONTEXT_DEBUG_CONTRACTS.md"
preflight_script="$repo_root/scripts/hepta-context-preflight.sh"

fail() {
  echo "hepta-context-memory-shadow-regression-dashboard-report: $*" >&2
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
  "Temporal graph shadow eval" \
  "Context memory recall quality gate" \
  "Ranked recall shadow eval" \
  "MemoryProvider boundary" \
  "Memory shadow regression dashboard"; do
  assert_file_contains "$contracts" "$term" "shadow regression dashboard contract input"
done

for term in \
  "context memory temporal graph shadow eval gate" \
  "context memory recall quality gate" \
  "context memory ranked recall shadow eval gate" \
  "context memory provider boundary gate" \
  "context memory shadow regression dashboard gate"; do
  assert_file_contains "$preflight_script" "$term" "shadow regression dashboard preflight input"
done

cat <<'EOF'
memory-shadow-regression-dashboard=pass
memory-shadow-regression-dashboard.payload-light=pass
memory-shadow-regression-dashboard.schema=1
memory-shadow-regression-dashboard.mode=shadow-only
memory-shadow-regression-dashboard.input-report-count=4
memory-shadow-regression-dashboard.input-report-pass-count=4
memory-shadow-regression-dashboard.regression-blocking-count=0
memory-shadow-regression-dashboard.ranked-recall-fixture-count=4
memory-shadow-regression-dashboard.ranked-recall-regression-fixture=blocked
memory-shadow-regression-dashboard.ranked-recall-min-positive-recall-basis-points=8000
memory-shadow-regression-dashboard.ranked-recall-min-positive-precision-basis-points=8000
memory-shadow-regression-dashboard.ranked-recall-total-positive-token-saved=2140
memory-shadow-regression-dashboard.ranked-recall-max-positive-latency-ms=55
memory-shadow-regression-dashboard.temporal-graph-fixture-count=4
memory-shadow-regression-dashboard.temporal-graph-regression-fixture=blocked
memory-shadow-regression-dashboard.temporal-graph-min-positive-node-coverage-basis-points=10000
memory-shadow-regression-dashboard.temporal-graph-min-positive-edge-coverage-basis-points=10000
memory-shadow-regression-dashboard.temporal-graph-min-positive-validity-window-coverage-basis-points=10000
memory-shadow-regression-dashboard.temporal-graph-min-positive-supersedes-coverage-basis-points=10000
memory-shadow-regression-dashboard.temporal-graph-max-positive-latency-ms=47
memory-shadow-regression-dashboard.recall-quality-fixture-count=2
memory-shadow-regression-dashboard.recall-quality-blocking-reason-count=0
memory-shadow-regression-dashboard.provider-boundary=pass
memory-shadow-regression-dashboard.provider-payload-light=pass
memory-shadow-regression-dashboard.operator-approval=required
memory-shadow-regression-dashboard.production-route=disabled
memory-shadow-regression-dashboard.production-write=disabled
memory-shadow-regression-dashboard.graph-write=disabled
memory-shadow-regression-dashboard.runtime-activation=disabled
EOF
