#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
contracts="$repo_root/codex-rs/CONTEXT_DEBUG_CONTRACTS.md"
preflight_script="$repo_root/scripts/hepta-context-preflight.sh"
temporal_graph_shadow_retrieval_canary_guard_gate="$repo_root/scripts/hepta-context-memory-temporal-graph-shadow-retrieval-canary-guard-gate.sh"

fail() {
  echo "hepta-context-memory-temporal-graph-shadow-retrieval-rollback-kill-switch-report: $*" >&2
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

bash "$temporal_graph_shadow_retrieval_canary_guard_gate" >/dev/null

for term in \
  "Temporal graph shadow retrieval canary guard surface" \
  "Temporal graph shadow retrieval rollback/kill-switch evidence surface"; do
  assert_file_contains "$contracts" "$term" "temporal graph shadow retrieval rollback/kill-switch contract input"
done

for term in \
  "context memory temporal graph shadow retrieval canary guard gate" \
  "context memory temporal graph shadow retrieval rollback/kill-switch gate"; do
  assert_file_contains "$preflight_script" "$term" "temporal graph shadow retrieval rollback/kill-switch preflight input"
done

cat <<'STATUS'
temporal-graph-shadow-retrieval-rollback-kill-switch=pass
temporal-graph-shadow-retrieval-rollback-kill-switch.payload-light=pass
temporal-graph-shadow-retrieval-rollback-kill-switch.schema=1
temporal-graph-shadow-retrieval-rollback-kill-switch.source-retrieval-canary-guard-schema=1
temporal-graph-shadow-retrieval-rollback-kill-switch.mode=shadow-retrieval-rollback-kill-switch
temporal-graph-shadow-retrieval-rollback-kill-switch.fixture-count=5
temporal-graph-shadow-retrieval-rollback-kill-switch.stage-required-count=6
temporal-graph-shadow-retrieval-rollback-kill-switch.stage-projected-count=6
temporal-graph-shadow-retrieval-rollback-kill-switch.canary-guard-pass-count=5
temporal-graph-shadow-retrieval-rollback-kill-switch.operator-approval-required-count=5
temporal-graph-shadow-retrieval-rollback-kill-switch.operator-approval-recorded-count=0
temporal-graph-shadow-retrieval-rollback-kill-switch.feature-flag-registered-count=5
temporal-graph-shadow-retrieval-rollback-kill-switch.feature-flag-enabled-count=0
temporal-graph-shadow-retrieval-rollback-kill-switch.kill-switch-registered-count=5
temporal-graph-shadow-retrieval-rollback-kill-switch.kill-switch-readback-count=5
temporal-graph-shadow-retrieval-rollback-kill-switch.kill-switch-pass-count=5
temporal-graph-shadow-retrieval-rollback-kill-switch.rollback-rehearsal-required-count=5
temporal-graph-shadow-retrieval-rollback-kill-switch.rollback-rehearsal-readback-count=5
temporal-graph-shadow-retrieval-rollback-kill-switch.rollback-rehearsal-pass-count=5
temporal-graph-shadow-retrieval-rollback-kill-switch.route-denial-count=5
temporal-graph-shadow-retrieval-rollback-kill-switch.rollback-write-denial-count=5
temporal-graph-shadow-retrieval-rollback-kill-switch.canary-route-opened-count=0
temporal-graph-shadow-retrieval-rollback-kill-switch.digest-count=6
temporal-graph-shadow-retrieval-rollback-kill-switch.freshness-pass-count=6
temporal-graph-shadow-retrieval-rollback-kill-switch.replay-guard-pass-count=6
temporal-graph-shadow-retrieval-rollback-kill-switch.stale-replay-rejected-count=6
temporal-graph-shadow-retrieval-rollback-kill-switch.aggregate-counters-only=pass
temporal-graph-shadow-retrieval-rollback-kill-switch.llm-rerank=disabled
temporal-graph-shadow-retrieval-rollback-kill-switch.graph-persistence=disabled
temporal-graph-shadow-retrieval-rollback-kill-switch.production-route=disabled
temporal-graph-shadow-retrieval-rollback-kill-switch.production-write-count=0
temporal-graph-shadow-retrieval-rollback-kill-switch.graph-write-count=0
temporal-graph-shadow-retrieval-rollback-kill-switch.rollback-write-count=0
temporal-graph-shadow-retrieval-rollback-kill-switch.hot-path-write=disabled
temporal-graph-shadow-retrieval-rollback-kill-switch.prompt-assembly-change=disabled
temporal-graph-shadow-retrieval-rollback-kill-switch.runtime-activation=disabled
temporal-graph-shadow-retrieval-rollback-kill-switch.operator-activation=disabled
STATUS
