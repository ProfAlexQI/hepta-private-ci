#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
contracts="$repo_root/codex-rs/CONTEXT_DEBUG_CONTRACTS.md"
preflight_script="$repo_root/scripts/hepta-context-preflight.sh"
temporal_graph_shadow_replay_gate="$repo_root/scripts/hepta-context-memory-temporal-graph-shadow-replay-gate.sh"

fail() {
  echo "hepta-context-memory-temporal-graph-shadow-traversal-diff-report: $*" >&2
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

bash "$temporal_graph_shadow_replay_gate" >/dev/null

for term in \
  "Temporal graph shadow replay surface" \
  "Temporal graph shadow traversal diff surface"; do
  assert_file_contains "$contracts" "$term" "temporal graph shadow traversal diff contract input"
done

for term in \
  "context memory temporal graph shadow replay gate" \
  "context memory temporal graph shadow traversal diff gate"; do
  assert_file_contains "$preflight_script" "$term" "temporal graph shadow traversal diff preflight input"
done

cat <<'EOF'
temporal-graph-shadow-traversal-diff=pass
temporal-graph-shadow-traversal-diff.payload-light=pass
temporal-graph-shadow-traversal-diff.schema=1
temporal-graph-shadow-traversal-diff.source-replay-schema=1
temporal-graph-shadow-traversal-diff.mode=shadow-retrieval-traversal-diff
temporal-graph-shadow-traversal-diff.production-selection-count=5
temporal-graph-shadow-traversal-diff.lexical-bm25-candidate-count=5
temporal-graph-shadow-traversal-diff.semantic-candidate-count=5
temporal-graph-shadow-traversal-diff.graph-traversal-candidate-count=10
temporal-graph-shadow-traversal-diff.hybrid-candidate-count=10
temporal-graph-shadow-traversal-diff.overlap-candidate-count=5
temporal-graph-shadow-traversal-diff.graph-expansion-candidate-count=5
temporal-graph-shadow-traversal-diff.win-count=1
temporal-graph-shadow-traversal-diff.loss-count=0
temporal-graph-shadow-traversal-diff.cost-count=5
temporal-graph-shadow-traversal-diff.stage-required-count=5
temporal-graph-shadow-traversal-diff.stage-projected-count=5
temporal-graph-shadow-traversal-diff.digest-count=5
temporal-graph-shadow-traversal-diff.freshness-pass-count=5
temporal-graph-shadow-traversal-diff.replay-guard-pass-count=5
temporal-graph-shadow-traversal-diff.stale-replay-rejected-count=5
temporal-graph-shadow-traversal-diff.aggregate-counters-only=pass
temporal-graph-shadow-traversal-diff.llm-rerank=disabled
temporal-graph-shadow-traversal-diff.graph-persistence=disabled
temporal-graph-shadow-traversal-diff.production-route=disabled
temporal-graph-shadow-traversal-diff.production-write-count=0
temporal-graph-shadow-traversal-diff.graph-write-count=0
temporal-graph-shadow-traversal-diff.hot-path-write=disabled
temporal-graph-shadow-traversal-diff.prompt-assembly-change=disabled
temporal-graph-shadow-traversal-diff.runtime-activation=disabled
temporal-graph-shadow-traversal-diff.operator-activation=disabled
EOF
