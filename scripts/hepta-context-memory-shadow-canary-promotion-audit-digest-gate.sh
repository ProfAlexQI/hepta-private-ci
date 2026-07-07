#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
contracts="$repo_root/codex-rs/CONTEXT_DEBUG_CONTRACTS.md"
debug_gate="$repo_root/scripts/hepta-context-debug-gate.sh"
preflight_script="$repo_root/scripts/hepta-context-preflight.sh"
front_door_gate="$repo_root/scripts/hepta-context-source-aware-compression-front-door-gate.sh"
readiness_gate="$repo_root/scripts/hepta-context-memory-shadow-canary-promotion-readiness-gate.sh"
negative_rehearsal_gate="$repo_root/scripts/hepta-context-memory-shadow-canary-promotion-negative-rehearsal-gate.sh"
audit_digest_report="$repo_root/scripts/hepta-context-memory-shadow-canary-promotion-audit-digest-report.sh"

fail() {
  echo "hepta-context-memory-shadow-canary-promotion-audit-digest-gate: $*" >&2
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
  "Memory shadow canary promotion audit digest" \
  "memory-shadow-canary-promotion-audit-digest=pass" \
  "memory-shadow-canary-promotion-audit-digest.readiness-report-lines=32" \
  "memory-shadow-canary-promotion-audit-digest.negative-rehearsal-report-lines=14" \
  "memory-shadow-canary-promotion-audit-digest.combined-report-lines=46" \
  "readiness-report-sha256" \
  "negative-rehearsal-report-sha256" \
  "combined-report-sha256" \
  "hepta-context-memory-shadow-canary-promotion-audit-digest-report.sh" \
  "hepta-context-memory-shadow-canary-promotion-audit-digest-gate.sh"; do
  assert_file_contains "$contracts" "$term" "memory shadow canary promotion audit digest contract"
done

assert_file_contains "$debug_gate" \
  "hepta-context-memory-shadow-canary-promotion-audit-digest-gate.sh" \
  "memory shadow canary promotion audit digest debug gate"
assert_file_contains "$preflight_script" \
  "context memory shadow canary promotion audit digest gate" \
  "memory shadow canary promotion audit digest preflight stage"
assert_file_contains "$front_door_gate" \
  "memory_shadow_canary_promotion_audit_digest_gate_script" \
  "memory shadow canary promotion audit digest front-door static check"

assert_line_before \
  "$debug_gate" \
  "hepta-context-memory-shadow-canary-promotion-negative-rehearsal-gate.sh" \
  "hepta-context-memory-shadow-canary-promotion-audit-digest-gate.sh" \
  "memory shadow canary promotion audit digest debug order"
assert_line_before \
  "$debug_gate" \
  "hepta-context-memory-shadow-canary-promotion-audit-digest-gate.sh" \
  "hepta-context-plane-status-report-gate.sh" \
  "memory shadow canary promotion audit digest context plane debug order"
assert_line_before \
  "$preflight_script" \
  "context memory shadow canary promotion negative rehearsal gate" \
  "context memory shadow canary promotion audit digest gate" \
  "memory shadow canary promotion audit digest preflight order"
assert_line_before \
  "$preflight_script" \
  "context memory shadow canary promotion audit digest gate" \
  "context plane status/export report gate" \
  "memory shadow canary promotion audit digest context plane preflight order"

expected_audit_digest_status="$(cat <<'STATUS'
memory-shadow-canary-promotion-audit-digest=pass
memory-shadow-canary-promotion-audit-digest.schema=1
memory-shadow-canary-promotion-audit-digest.payload-light=pass
memory-shadow-canary-promotion-audit-digest.readiness-report-lines=32
memory-shadow-canary-promotion-audit-digest.readiness-report-sha256=3409150807636fe61968e15f6c10555333397080c3afac6af58a5fbab41c5565
memory-shadow-canary-promotion-audit-digest.negative-rehearsal-report-lines=14
memory-shadow-canary-promotion-audit-digest.negative-rehearsal-report-sha256=8aabd87561665b8e0f8054c6dd511966153761b06d7d25c2bc6ff27b611cbd05
memory-shadow-canary-promotion-audit-digest.combined-report-lines=46
memory-shadow-canary-promotion-audit-digest.combined-report-sha256=a889e450a9e9533c3b245e8498eb88309ea7359d4fe5453af194eab79c37122e
memory-shadow-canary-promotion-audit-digest.runtime-activation=disabled
memory-shadow-canary-promotion-audit-digest.operator-activation=disabled
STATUS
)"

audit_digest_guard_accepts() {
  local audit_status="$1"

  [ "$audit_status" = "$expected_audit_digest_status" ] || return 1
  [ "$(line_count "$audit_status")" = "11" ] || return 1

  if printf '%s\n' "$audit_status" | grep -E 'activation_command|tool_args|raw_payload|prompt_text|transcript_text|memory_text|answer_text|source_id|session_id|memory_id|trace_id|query_text|ranked_payload|raw_graph_payload|operator_identity|operator@example\.com|activation-command=(run|enabled|present)|runtime-activation=enabled|operator-activation=enabled|production-write=enabled|graph-write=enabled|rollback-write=enabled|canary-promotion-route=enabled' >/dev/null; then
    return 1
  fi

  return 0
}

assert_rejected() {
  local label="$1"
  local audit_status="$2"

  if audit_digest_guard_accepts "$audit_status"; then
    fail "$label audit digest fixture was accepted"
  fi
}

bash "$readiness_gate" >/dev/null
bash "$negative_rehearsal_gate" >/dev/null

audit_status="$(bash "$audit_digest_report")"
audit_status_second="$(bash "$audit_digest_report")"

if ! audit_digest_guard_accepts "$audit_status"; then
  fail "canonical canary promotion audit digest report must pass before tamper fixtures run"
fi
if [ "$audit_status" != "$audit_status_second" ]; then
  fail "memory shadow canary promotion audit digest report is not idempotent"
fi

readiness_digest_tamper="$(printf '%s\n' "$audit_status" | sed 's/readiness-report-sha256=3409150807636fe61968e15f6c10555333397080c3afac6af58a5fbab41c5565/readiness-report-sha256=4409150807636fe61968e15f6c10555333397080c3afac6af58a5fbab41c5565/')"
negative_digest_tamper="$(printf '%s\n' "$audit_status" | sed 's/negative-rehearsal-report-sha256=8aabd87561665b8e0f8054c6dd511966153761b06d7d25c2bc6ff27b611cbd05/negative-rehearsal-report-sha256=9aabd87561665b8e0f8054c6dd511966153761b06d7d25c2bc6ff27b611cbd05/')"
combined_digest_tamper="$(printf '%s\n' "$audit_status" | sed 's/combined-report-sha256=a889e450a9e9533c3b245e8498eb88309ea7359d4fe5453af194eab79c37122e/combined-report-sha256=b889e450a9e9533c3b245e8498eb88309ea7359d4fe5453af194eab79c37122e/')"
line_count_tamper="$(
  printf '%s\n' "$audit_status"
  printf '%s\n' "memory-shadow-canary-promotion-audit-digest.replayed-copy=unexpected"
)"
activation_tamper="$(printf '%s\n' "$audit_status" | sed 's/runtime-activation=disabled/runtime-activation=enabled/')"

assert_rejected "readiness digest" "$readiness_digest_tamper"
assert_rejected "negative rehearsal digest" "$negative_digest_tamper"
assert_rejected "combined digest" "$combined_digest_tamper"
assert_rejected "line-count" "$line_count_tamper"
assert_rejected "activation flag" "$activation_tamper"

bash -n "$audit_digest_report"

echo "memory-shadow-canary-promotion-audit-digest=pass"
echo "memory-shadow-canary-promotion-audit-digest.readiness-report-lines=32"
echo "memory-shadow-canary-promotion-audit-digest.negative-rehearsal-report-lines=14"
echo "memory-shadow-canary-promotion-audit-digest.combined-report-lines=46"
echo "memory-shadow-canary-promotion-audit-digest.runtime-activation=disabled"
