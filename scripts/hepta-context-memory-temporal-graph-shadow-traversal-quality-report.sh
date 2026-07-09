#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
contracts="$repo_root/codex-rs/CONTEXT_DEBUG_CONTRACTS.md"
preflight_script="$repo_root/scripts/hepta-context-preflight.sh"
temporal_graph_shadow_traversal_diff_gate="$repo_root/scripts/hepta-context-memory-temporal-graph-shadow-traversal-diff-gate.sh"

fail() {
  echo "hepta-context-memory-temporal-graph-shadow-traversal-quality-report: $*" >&2
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

bash "$temporal_graph_shadow_traversal_diff_gate" >/dev/null

for term in \
  "Temporal graph shadow traversal diff surface" \
  "Temporal graph shadow traversal quality/SLO surface"; do
  assert_file_contains "$contracts" "$term" "temporal graph shadow traversal quality contract input"
done

for term in \
  "context memory temporal graph shadow traversal diff gate" \
  "context memory temporal graph shadow traversal quality/SLO gate"; do
  assert_file_contains "$preflight_script" "$term" "temporal graph shadow traversal quality preflight input"
done

cat <<'EOF'
temporal-graph-shadow-traversal-quality=pass
temporal-graph-shadow-traversal-quality.payload-light=pass
temporal-graph-shadow-traversal-quality.schema=1
temporal-graph-shadow-traversal-quality.source-traversal-diff-schema=1
temporal-graph-shadow-traversal-quality.mode=shadow-traversal-quality-slo
temporal-graph-shadow-traversal-quality.fixture-count=5
temporal-graph-shadow-traversal-quality.slo-required-count=5
temporal-graph-shadow-traversal-quality.slo-pass-count=5
temporal-graph-shadow-traversal-quality.coverage-basis-points=10000
temporal-graph-shadow-traversal-quality.precision-basis-points=10000
temporal-graph-shadow-traversal-quality.leak-rate-basis-points=0
temporal-graph-shadow-traversal-quality.latency-budget-ms=20
temporal-graph-shadow-traversal-quality.projected-latency-ms=5
temporal-graph-shadow-traversal-quality.token-saved-estimate=768
temporal-graph-shadow-traversal-quality.operator-review-required-count=5
temporal-graph-shadow-traversal-quality.win-count=1
temporal-graph-shadow-traversal-quality.loss-count=0
temporal-graph-shadow-traversal-quality.cost-count=5
temporal-graph-shadow-traversal-quality.stage-required-count=5
temporal-graph-shadow-traversal-quality.stage-projected-count=5
temporal-graph-shadow-traversal-quality.digest-count=5
temporal-graph-shadow-traversal-quality.freshness-pass-count=5
temporal-graph-shadow-traversal-quality.replay-guard-pass-count=5
temporal-graph-shadow-traversal-quality.stale-replay-rejected-count=5
temporal-graph-shadow-traversal-quality.aggregate-counters-only=pass
temporal-graph-shadow-traversal-quality.llm-rerank=disabled
temporal-graph-shadow-traversal-quality.graph-persistence=disabled
temporal-graph-shadow-traversal-quality.production-route=disabled
temporal-graph-shadow-traversal-quality.production-write-count=0
temporal-graph-shadow-traversal-quality.graph-write-count=0
temporal-graph-shadow-traversal-quality.hot-path-write=disabled
temporal-graph-shadow-traversal-quality.prompt-assembly-change=disabled
temporal-graph-shadow-traversal-quality.runtime-activation=disabled
temporal-graph-shadow-traversal-quality.operator-activation=disabled
EOF
