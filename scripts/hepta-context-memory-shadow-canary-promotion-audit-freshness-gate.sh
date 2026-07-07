#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
contracts="$repo_root/codex-rs/CONTEXT_DEBUG_CONTRACTS.md"
debug_gate="$repo_root/scripts/hepta-context-debug-gate.sh"
preflight_script="$repo_root/scripts/hepta-context-preflight.sh"
front_door_gate="$repo_root/scripts/hepta-context-source-aware-compression-front-door-gate.sh"
audit_digest_gate="$repo_root/scripts/hepta-context-memory-shadow-canary-promotion-audit-digest-gate.sh"
audit_freshness_report="$repo_root/scripts/hepta-context-memory-shadow-canary-promotion-audit-freshness-report.sh"

fail() {
  echo "hepta-context-memory-shadow-canary-promotion-audit-freshness-gate: $*" >&2
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

line_number_of() {
  local file_path="$1"
  local needle="$2"
  local line

  line="$(grep -n -F "$needle" "$file_path" | head -n 1 | cut -d: -f1 || true)"
  if [ -z "$line" ]; then
    fail "$file_path is missing required text: $needle"
  fi
  printf '%s\n' "$line"
}

assert_line_before() {
  local file_path="$1"
  local before_needle="$2"
  local after_needle="$3"
  local label="$4"
  local before_line
  local after_line

  before_line="$(line_number_of "$file_path" "$before_needle")"
  after_line="$(line_number_of "$file_path" "$after_needle")"
  if [ "$before_line" -ge "$after_line" ]; then
    fail "$label expected '$before_needle' before '$after_needle'"
  fi
}

line_count() {
  printf '%s\n' "$1" | wc -l | tr -d ' '
}

for term in \
  "Memory shadow canary promotion audit freshness" \
  "memory-shadow-canary-promotion-audit-freshness=pass" \
  "memory-shadow-canary-promotion-audit-freshness.source-audit-digest-report-lines=11" \
  "source-audit-digest-report-sha256" \
  "audit-readiness-sequence=309" \
  "current-readiness-sequence=309" \
  "expires-after-sequence=310" \
  "stale-sequence=reject" \
  "expired-sequence=reject" \
  "future-sequence=reject" \
  "digest-replay=reject" \
  "mixed-source-digest=reject" \
  "hepta-context-memory-shadow-canary-promotion-audit-freshness-report.sh" \
  "hepta-context-memory-shadow-canary-promotion-audit-freshness-gate.sh"; do
  assert_file_contains "$contracts" "$term" "memory shadow canary promotion audit freshness contract"
done

assert_file_contains "$debug_gate" \
  "hepta-context-memory-shadow-canary-promotion-audit-freshness-gate.sh" \
  "memory shadow canary promotion audit freshness debug gate"
assert_file_contains "$preflight_script" \
  "context memory shadow canary promotion audit freshness gate" \
  "memory shadow canary promotion audit freshness preflight stage"
assert_file_contains "$front_door_gate" \
  "memory_shadow_canary_promotion_audit_freshness_gate_script" \
  "memory shadow canary promotion audit freshness front-door static check"

assert_line_before \
  "$debug_gate" \
  "hepta-context-memory-shadow-canary-promotion-audit-digest-gate.sh" \
  "hepta-context-memory-shadow-canary-promotion-audit-freshness-gate.sh" \
  "memory shadow canary promotion audit freshness debug order"
assert_line_before \
  "$debug_gate" \
  "hepta-context-memory-shadow-canary-promotion-audit-freshness-gate.sh" \
  "hepta-context-plane-status-report-gate.sh" \
  "memory shadow canary promotion audit freshness context plane debug order"
assert_line_before \
  "$preflight_script" \
  "context memory shadow canary promotion audit digest gate" \
  "context memory shadow canary promotion audit freshness gate" \
  "memory shadow canary promotion audit freshness preflight order"
assert_line_before \
  "$preflight_script" \
  "context memory shadow canary promotion audit freshness gate" \
  "context plane status/export report gate" \
  "memory shadow canary promotion audit freshness context plane preflight order"

expected_freshness_status="$(cat <<'STATUS'
memory-shadow-canary-promotion-audit-freshness=pass
memory-shadow-canary-promotion-audit-freshness.schema=1
memory-shadow-canary-promotion-audit-freshness.payload-light=pass
memory-shadow-canary-promotion-audit-freshness.source-audit-digest-report-lines=11
memory-shadow-canary-promotion-audit-freshness.source-audit-digest-report-sha256=5c87042c4e9d49b27bfbac26e07102ffc4032fc23293b6204b26714c5f17c307
memory-shadow-canary-promotion-audit-freshness.audit-readiness-sequence=309
memory-shadow-canary-promotion-audit-freshness.current-readiness-sequence=309
memory-shadow-canary-promotion-audit-freshness.expires-after-sequence=310
memory-shadow-canary-promotion-audit-freshness.max-replay-age-sequences=0
memory-shadow-canary-promotion-audit-freshness.stale-sequence=reject
memory-shadow-canary-promotion-audit-freshness.expired-sequence=reject
memory-shadow-canary-promotion-audit-freshness.future-sequence=reject
memory-shadow-canary-promotion-audit-freshness.digest-replay=reject
memory-shadow-canary-promotion-audit-freshness.mixed-source-digest=reject
memory-shadow-canary-promotion-audit-freshness.runtime-activation=disabled
memory-shadow-canary-promotion-audit-freshness.operator-activation=disabled
STATUS
)"

freshness_guard_accepts() {
  local freshness_status="$1"

  [ "$freshness_status" = "$expected_freshness_status" ] || return 1
  [ "$(line_count "$freshness_status")" = "16" ] || return 1

  if printf '%s\n' "$freshness_status" | grep -E 'activation_command|tool_args|raw_payload|prompt_text|transcript_text|memory_text|answer_text|source_id|session_id|memory_id|trace_id|query_text|ranked_payload|raw_graph_payload|operator_identity|operator@example\.com|activation-command=(run|enabled|present)|runtime-activation=enabled|operator-activation=enabled|production-write=enabled|graph-write=enabled|rollback-write=enabled|canary-promotion-route=enabled' >/dev/null; then
    return 1
  fi

  return 0
}

assert_rejected() {
  local label="$1"
  local freshness_status="$2"

  if freshness_guard_accepts "$freshness_status"; then
    fail "$label freshness fixture was accepted"
  fi
}

bash "$audit_digest_gate" >/dev/null

freshness_status="$(bash "$audit_freshness_report")"
freshness_status_second="$(bash "$audit_freshness_report")"

if ! freshness_guard_accepts "$freshness_status"; then
  fail "canonical canary promotion audit freshness report must pass before replay fixtures run"
fi
if [ "$freshness_status" != "$freshness_status_second" ]; then
  fail "memory shadow canary promotion audit freshness report is not idempotent"
fi

stale_sequence_tamper="$(printf '%s\n' "$freshness_status" | sed 's/audit-readiness-sequence=309/audit-readiness-sequence=308/')"
expired_sequence_tamper="$(printf '%s\n' "$freshness_status" | sed 's/expires-after-sequence=310/expires-after-sequence=309/')"
future_sequence_tamper="$(printf '%s\n' "$freshness_status" | sed 's/audit-readiness-sequence=309/audit-readiness-sequence=310/')"
source_digest_replay_tamper="$(printf '%s\n' "$freshness_status" | sed 's/source-audit-digest-report-sha256=5c87042c4e9d49b27bfbac26e07102ffc4032fc23293b6204b26714c5f17c307/source-audit-digest-report-sha256=6c87042c4e9d49b27bfbac26e07102ffc4032fc23293b6204b26714c5f17c307/')"
line_count_tamper="$(
  printf '%s\n' "$freshness_status"
  printf '%s\n' "memory-shadow-canary-promotion-audit-freshness.replayed-copy=unexpected"
)"
activation_tamper="$(printf '%s\n' "$freshness_status" | sed 's/runtime-activation=disabled/runtime-activation=enabled/')"

assert_rejected "stale-sequence" "$stale_sequence_tamper"
assert_rejected "expired-sequence" "$expired_sequence_tamper"
assert_rejected "future-sequence" "$future_sequence_tamper"
assert_rejected "source-digest-replay" "$source_digest_replay_tamper"
assert_rejected "line-count" "$line_count_tamper"
assert_rejected "activation flag" "$activation_tamper"

bash -n "$audit_freshness_report"

echo "memory-shadow-canary-promotion-audit-freshness=pass"
echo "memory-shadow-canary-promotion-audit-freshness.stale-sequence=reject"
echo "memory-shadow-canary-promotion-audit-freshness.expired-sequence=reject"
echo "memory-shadow-canary-promotion-audit-freshness.future-sequence=reject"
echo "memory-shadow-canary-promotion-audit-freshness.digest-replay=reject"
echo "memory-shadow-canary-promotion-audit-freshness.runtime-activation=disabled"
