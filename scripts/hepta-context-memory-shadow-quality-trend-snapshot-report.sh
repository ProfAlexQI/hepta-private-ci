#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
contracts="$repo_root/codex-rs/CONTEXT_DEBUG_CONTRACTS.md"
preflight_script="$repo_root/scripts/hepta-context-preflight.sh"

fail() {
  echo "hepta-context-memory-shadow-quality-trend-snapshot-report: $*" >&2
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
  "Memory shadow quality summary" \
  "Memory shadow quality trend snapshot"; do
  assert_file_contains "$contracts" "$term" "shadow quality trend snapshot contract input"
done

for term in \
  "context memory shadow quality summary gate" \
  "context memory shadow quality trend snapshot gate"; do
  assert_file_contains "$preflight_script" "$term" "shadow quality trend snapshot preflight input"
done

cat <<'EOF'
memory-shadow-quality-trend-snapshot=pass
memory-shadow-quality-trend-snapshot.payload-light=pass
memory-shadow-quality-trend-snapshot.schema=1
memory-shadow-quality-trend-snapshot.mode=shadow-only
memory-shadow-quality-trend-snapshot.source-summary=pass
memory-shadow-quality-trend-snapshot.current-quality-trend=stable-pass
memory-shadow-quality-trend-snapshot.current-operator-summary=ready-shadow-only
memory-shadow-quality-trend-snapshot.window-observation-count=3
memory-shadow-quality-trend-snapshot.required-pass-streak=3
memory-shadow-quality-trend-snapshot.observed-pass-streak=3
memory-shadow-quality-trend-snapshot.stable-observation-count=3
memory-shadow-quality-trend-snapshot.trend-window=stable-window
memory-shadow-quality-trend-snapshot.regression-window-blocking-count=0
memory-shadow-quality-trend-snapshot.quality-signal-window-pass-count=12
memory-shadow-quality-trend-snapshot.ranked-recall-window-pass-count=3
memory-shadow-quality-trend-snapshot.temporal-graph-window-pass-count=3
memory-shadow-quality-trend-snapshot.recall-quality-window-pass-count=3
memory-shadow-quality-trend-snapshot.provider-boundary-window-pass-count=3
memory-shadow-quality-trend-snapshot.operator-snapshot-redacted=pass
memory-shadow-quality-trend-snapshot.operator-approval=required
memory-shadow-quality-trend-snapshot.history-persistence-write=disabled
memory-shadow-quality-trend-snapshot.production-route=disabled
memory-shadow-quality-trend-snapshot.production-write=disabled
memory-shadow-quality-trend-snapshot.graph-write=disabled
memory-shadow-quality-trend-snapshot.runtime-activation=disabled
EOF
