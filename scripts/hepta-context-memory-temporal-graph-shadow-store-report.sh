#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
contracts="$repo_root/codex-rs/CONTEXT_DEBUG_CONTRACTS.md"
preflight_script="$repo_root/scripts/hepta-context-preflight.sh"
temporal_graph_shadow_eval_gate="$repo_root/scripts/hepta-context-memory-temporal-graph-shadow-eval-gate.sh"

fail() {
  echo "hepta-context-memory-temporal-graph-shadow-store-report: $*" >&2
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

bash "$temporal_graph_shadow_eval_gate" >/dev/null

for term in \
  "Memory temporal fact graph dry-run" \
  "Temporal graph shadow eval" \
  "Temporal graph shadow store skeleton"; do
  assert_file_contains "$contracts" "$term" "temporal graph shadow store contract input"
done

for term in \
  "context memory temporal fact graph dry-run gate" \
  "context memory temporal graph shadow eval gate" \
  "context memory temporal graph shadow store skeleton gate"; do
  assert_file_contains "$preflight_script" "$term" "temporal graph shadow store preflight input"
done

cat <<'EOF'
temporal-graph-shadow-store=pass
temporal-graph-shadow-store.payload-light=pass
temporal-graph-shadow-store.schema=1
temporal-graph-shadow-store.source-graph-schema=1
temporal-graph-shadow-store.mode=approval-gated-shadow-store-skeleton
temporal-graph-shadow-store.node-count=5
temporal-graph-shadow-store.edge-count=10
temporal-graph-shadow-store.provenance-edge-count=5
temporal-graph-shadow-store.validity-window-edge-count=5
temporal-graph-shadow-store.supersedes-edge-count=0
temporal-graph-shadow-store.open-node-count=5
temporal-graph-shadow-store.invalidated-node-count=0
temporal-graph-shadow-store.stage-required-count=6
temporal-graph-shadow-store.stage-projected-count=6
temporal-graph-shadow-store.store-digest=present
temporal-graph-shadow-store.freshness-check=pass
temporal-graph-shadow-store.replay-guard=pass
temporal-graph-shadow-store.stale-replay-rejected=pass
temporal-graph-shadow-store.operator-approval=required
temporal-graph-shadow-store.operator-approval-recorded-count=0
temporal-graph-shadow-store.recorded-receipt-count=0
temporal-graph-shadow-store.persisted-receipt-count=0
temporal-graph-shadow-store.production-route=disabled
temporal-graph-shadow-store.production-write-count=0
temporal-graph-shadow-store.graph-write-count=0
temporal-graph-shadow-store.hot-path-write=disabled
temporal-graph-shadow-store.prompt-assembly-change=disabled
temporal-graph-shadow-store.runtime-activation=disabled
temporal-graph-shadow-store.operator-activation=disabled
EOF
