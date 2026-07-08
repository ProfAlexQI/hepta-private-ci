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
memory-shadow-quality-summary.schema=2
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
memory-shadow-quality-summary.temporal-graph-signal=pass
memory-shadow-quality-summary.recall-quality-signal=pass
memory-shadow-quality-summary.provider-boundary-signal=pass
memory-shadow-quality-summary.operator-approval=required
memory-shadow-quality-summary.production-route=disabled
memory-shadow-quality-summary.production-write=disabled
memory-shadow-quality-summary.graph-write=disabled
memory-shadow-quality-summary.runtime-activation=disabled
EOF
