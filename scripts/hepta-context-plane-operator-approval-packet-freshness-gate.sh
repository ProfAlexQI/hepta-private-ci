#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
contracts="$repo_root/codex-rs/CONTEXT_DEBUG_CONTRACTS.md"
debug_gate="$repo_root/scripts/hepta-context-debug-gate.sh"
preflight_script="$repo_root/scripts/hepta-context-preflight.sh"
front_door_gate="$repo_root/scripts/hepta-context-source-aware-compression-front-door-gate.sh"
digest_gate="$repo_root/scripts/hepta-context-plane-operator-approval-packet-canonical-export-digest-gate.sh"
tamper_gate="$repo_root/scripts/hepta-context-plane-operator-approval-packet-digest-tamper-matrix-gate.sh"
freshness_report="$repo_root/scripts/hepta-context-plane-operator-approval-packet-freshness-report.sh"

fail() {
  echo "hepta-context-plane-operator-approval-packet-freshness-gate: $*" >&2
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
  "Context Plane operator approval packet freshness/staleness replay-protection dry-run" \
  "context-plane-operator-approval-packet-freshness=pass" \
  "approval-readiness-sequence=273" \
  "current-readiness-sequence=273" \
  "expires-after-sequence=274" \
  "stale-sequence=reject" \
  "expired-sequence=reject" \
  "future-sequence=reject" \
  "digest-replay=reject" \
  "canary false-green source digest replay" \
  "freshness/staleness/replay guard" \
  "hepta-context-plane-operator-approval-packet-freshness-report.sh" \
  "hepta-context-plane-operator-approval-packet-freshness-gate.sh" \
  "must not activate adaptive allocation" \
  "must not activate source-aware compression" \
  "must not write graph facts" \
  "must not write production memory" \
  "must not alter prompt assembly" \
  "must not enable runtime or operator activation"; do
  assert_file_contains "$contracts" "$term" "operator approval packet freshness contract"
done

assert_file_contains "$debug_gate" \
  "hepta-context-plane-operator-approval-packet-freshness-gate.sh" \
  "operator approval packet freshness debug gate"
assert_file_contains "$preflight_script" \
  "context plane operator approval packet freshness replay-protection gate" \
  "operator approval packet freshness preflight stage"
assert_file_contains "$front_door_gate" \
  "context_plane_operator_approval_packet_freshness_gate_script" \
  "operator approval packet freshness front-door static check"

assert_line_before \
  "$debug_gate" \
  "hepta-context-plane-operator-approval-packet-digest-tamper-matrix-gate.sh" \
  "hepta-context-plane-operator-approval-packet-freshness-gate.sh" \
  "operator approval packet freshness debug order"
assert_line_before \
  "$debug_gate" \
  "hepta-context-plane-operator-approval-packet-freshness-gate.sh" \
  "hepta-context-source-aware-compression-front-door-report.sh" \
  "operator approval packet freshness debug front-door order"
assert_line_before \
  "$preflight_script" \
  "context plane operator approval packet digest tamper matrix gate" \
  "context plane operator approval packet freshness replay-protection gate" \
  "operator approval packet freshness preflight order"
assert_line_before \
  "$preflight_script" \
  "context plane operator approval packet freshness replay-protection gate" \
  "source-aware compression front-door machine-readable report" \
  "operator approval packet freshness front-door preflight order"

expected_freshness_status="$(cat <<'STATUS'
context-plane-operator-approval-packet-freshness=pass
context-plane-operator-approval-packet-freshness.schema=1
context-plane-operator-approval-packet-freshness.source-canonical-digest-report-lines=10
context-plane-operator-approval-packet-freshness.source-canonical-digest-report-sha256=2312b53a92864cfe32b02a5524a4c36256d03b5fbe217d81e09cf536cb1e6030
context-plane-operator-approval-packet-freshness.approval-readiness-sequence=273
context-plane-operator-approval-packet-freshness.current-readiness-sequence=273
context-plane-operator-approval-packet-freshness.expires-after-sequence=274
context-plane-operator-approval-packet-freshness.max-replay-age-sequences=0
context-plane-operator-approval-packet-freshness.stale-sequence=reject
context-plane-operator-approval-packet-freshness.expired-sequence=reject
context-plane-operator-approval-packet-freshness.future-sequence=reject
context-plane-operator-approval-packet-freshness.digest-replay=reject
context-plane-operator-approval-packet-freshness.runtime-activation=disabled
context-plane-operator-approval-packet-freshness.operator-activation=disabled
STATUS
)"

freshness_guard_accepts() {
  local freshness_status="$1"

  [ "$freshness_status" = "$expected_freshness_status" ] || return 1
  [ "$(line_count "$freshness_status")" = "14" ] || return 1

  if printf '%s\n' "$freshness_status" | grep -E 'activation_command|tool_args|raw_payload|prompt_text|transcript_text|memory_text|answer_text|source_id|session_id|memory_id|trace_id|query_text|ranked_payload|entity_hash|supersedes|idempotency|fixture_hash|operator@example\.com|activation-command=(run|enabled|present)|runtime-activation=enabled|production-write=enabled|graph-write=enabled|operator-activation=enabled' >/dev/null; then
    return 1
  fi

  return 0
}

assert_rejected() {
  local label="$1"
  local freshness_status="$2"

  if freshness_guard_accepts "$freshness_status"; then
    fail "$label freshness replay fixture was accepted"
  fi
}

bash "$digest_gate" >/dev/null
bash "$tamper_gate" >/dev/null

freshness_status="$(bash "$freshness_report")"
freshness_status_second="$(bash "$freshness_report")"

if ! freshness_guard_accepts "$freshness_status"; then
  fail "canonical freshness report must pass before freshness replay fixtures run"
fi
if [ "$freshness_status" != "$freshness_status_second" ]; then
  fail "operator approval packet freshness report is not idempotent"
fi

stale_sequence_tamper="$(printf '%s\n' "$freshness_status" | sed 's/approval-readiness-sequence=273/approval-readiness-sequence=272/')"
expired_sequence_tamper="$(printf '%s\n' "$freshness_status" | sed 's/expires-after-sequence=274/expires-after-sequence=273/')"
future_sequence_tamper="$(printf '%s\n' "$freshness_status" | sed 's/approval-readiness-sequence=273/approval-readiness-sequence=274/')"
source_digest_replay_tamper="$(printf '%s\n' "$freshness_status" | sed 's/2312b53a92864cfe32b02a5524a4c36256d03b5fbe217d81e09cf536cb1e6030/3312b53a92864cfe32b02a5524a4c36256d03b5fbe217d81e09cf536cb1e6030/')"
canary_false_green_source_tamper="$(printf '%s\n' "$freshness_status" | sed 's/2312b53a92864cfe32b02a5524a4c36256d03b5fbe217d81e09cf536cb1e6030/4312b53a92864cfe32b02a5524a4c36256d03b5fbe217d81e09cf536cb1e6030/')"
line_count_tamper="$(
  printf '%s\n' "$freshness_status"
  printf '%s\n' "context-plane-operator-approval-packet-freshness.replayed-copy=unexpected"
)"
activation_command_tamper="$(
  printf '%s\n' "$freshness_status"
  printf '%s\n' "context-plane-operator-approval-packet-freshness.activation-command=run"
)"
write_activation_tamper="$(printf '%s\n' "$freshness_status" | sed 's/runtime-activation=disabled/runtime-activation=enabled/')"

assert_rejected "stale-sequence" "$stale_sequence_tamper"
assert_rejected "expired-sequence" "$expired_sequence_tamper"
assert_rejected "future-sequence" "$future_sequence_tamper"
assert_rejected "source-digest-replay" "$source_digest_replay_tamper"
assert_rejected "canary false-green source digest" "$canary_false_green_source_tamper"
assert_rejected "line-count" "$line_count_tamper"
assert_rejected "activation-command" "$activation_command_tamper"
assert_rejected "write/activation flag" "$write_activation_tamper"

echo "context-plane-operator-approval-packet-freshness=pass"
echo "context-plane-operator-approval-packet-freshness.stale-sequence=reject"
echo "context-plane-operator-approval-packet-freshness.expired-sequence=reject"
echo "context-plane-operator-approval-packet-freshness.future-sequence=reject"
echo "context-plane-operator-approval-packet-freshness.digest-replay=reject"
echo "context-plane-operator-approval-packet-freshness.canary-false-green-source=reject"
echo "context-plane-operator-approval-packet-freshness.line-count=reject"
echo "context-plane-operator-approval-packet-freshness.activation-command=reject"
echo "context-plane-operator-approval-packet-freshness.write-activation-flag=reject"
echo "context-plane-operator-approval-packet-freshness.runtime-activation=disabled"
echo "Hepta context plane operator approval packet freshness gate passed"
