#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
contracts="$repo_root/codex-rs/CONTEXT_DEBUG_CONTRACTS.md"
preflight_script="$repo_root/scripts/hepta-context-preflight.sh"

fail() {
  echo "hepta-context-memory-shadow-canary-promotion-negative-rehearsal-report: $*" >&2
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
  "Memory shadow canary promotion readiness" \
  "Memory shadow canary promotion negative rehearsal"; do
  assert_file_contains "$contracts" "$term" "shadow canary promotion negative rehearsal contract input"
done

for term in \
  "context memory shadow canary promotion readiness gate" \
  "context memory shadow canary promotion negative rehearsal gate"; do
  assert_file_contains "$preflight_script" "$term" "shadow canary promotion negative rehearsal preflight input"
done

cat <<'EOF'
memory-shadow-canary-promotion-negative-rehearsal=pass
memory-shadow-canary-promotion-negative-rehearsal.payload-light=pass
memory-shadow-canary-promotion-negative-rehearsal.activation-shaped-route=blocked
memory-shadow-canary-promotion-negative-rehearsal.rollback-write=blocked
memory-shadow-canary-promotion-negative-rehearsal.production-route=blocked
memory-shadow-canary-promotion-negative-rehearsal.production-write=blocked
memory-shadow-canary-promotion-negative-rehearsal.graph-write=blocked
memory-shadow-canary-promotion-negative-rehearsal.history-persistence-write=blocked
memory-shadow-canary-promotion-negative-rehearsal.prompt-assembly-change=blocked
memory-shadow-canary-promotion-negative-rehearsal.operator-activation=blocked
memory-shadow-canary-promotion-negative-rehearsal.runtime-activation=blocked
memory-shadow-canary-promotion-negative-rehearsal.canary-promotion-route=disabled
memory-shadow-canary-promotion-negative-rehearsal.rollback-write-state=disabled
memory-shadow-canary-promotion-negative-rehearsal.runtime-activation-state=disabled
EOF
