#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
contracts="$repo_root/codex-rs/CONTEXT_DEBUG_CONTRACTS.md"
debug_gate="$repo_root/scripts/hepta-context-debug-gate.sh"
preflight_script="$repo_root/scripts/hepta-context-preflight.sh"
front_door_gate="$repo_root/scripts/hepta-context-source-aware-compression-front-door-gate.sh"
approval_report="$repo_root/scripts/hepta-context-plane-operator-approval-packet-report.sh"
negative_export_report="$repo_root/scripts/hepta-context-plane-operator-approval-packet-negative-export-report.sh"
digest_report="$repo_root/scripts/hepta-context-plane-operator-approval-packet-canonical-export-digest-report.sh"

fail() {
  echo "hepta-context-plane-operator-approval-packet-digest-tamper-matrix-gate: $*" >&2
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

sha256_digest() {
  shasum -a 256 | awk '{print $1}'
}

for term in \
  "Context Plane operator approval packet digest tamper fixture negative matrix" \
  "context-plane-operator-approval-packet-digest-tamper-matrix=pass" \
  "line-order tamper" \
  "line-count tamper" \
  "digest-value tamper" \
  "activation-command injection" \
  "raw-payload injection" \
  "PII-shaped value injection" \
  "write/activation flag injection" \
  "canonical digest/no-payload guard" \
  "hepta-context-plane-operator-approval-packet-digest-tamper-matrix-gate.sh" \
  "must not activate adaptive allocation" \
  "must not activate source-aware compression" \
  "must not write graph facts" \
  "must not write production memory" \
  "must not alter prompt assembly" \
  "must not enable runtime or operator activation"; do
  assert_file_contains "$contracts" "$term" "operator approval packet digest tamper matrix contract"
done

assert_file_contains "$debug_gate" \
  "hepta-context-plane-operator-approval-packet-digest-tamper-matrix-gate.sh" \
  "operator approval packet digest tamper matrix debug gate"
assert_file_contains "$preflight_script" \
  "context plane operator approval packet digest tamper matrix gate" \
  "operator approval packet digest tamper matrix preflight stage"
assert_file_contains "$front_door_gate" \
  "context_plane_operator_approval_packet_digest_tamper_matrix_gate_script" \
  "operator approval packet digest tamper matrix front-door static check"

assert_line_before \
  "$debug_gate" \
  "hepta-context-plane-operator-approval-packet-canonical-export-digest-gate.sh" \
  "hepta-context-plane-operator-approval-packet-digest-tamper-matrix-gate.sh" \
  "operator approval packet digest tamper matrix debug order"
assert_line_before \
  "$debug_gate" \
  "hepta-context-plane-operator-approval-packet-digest-tamper-matrix-gate.sh" \
  "hepta-context-source-aware-compression-front-door-report.sh" \
  "operator approval packet digest tamper matrix debug front-door order"
assert_line_before \
  "$preflight_script" \
  "context plane operator approval packet canonical export digest gate" \
  "context plane operator approval packet digest tamper matrix gate" \
  "operator approval packet digest tamper matrix preflight order"
assert_line_before \
  "$preflight_script" \
  "context plane operator approval packet digest tamper matrix gate" \
  "source-aware compression front-door machine-readable report" \
  "operator approval packet digest tamper matrix front-door preflight order"

expected_digest_status="$(cat <<'STATUS'
context-plane-operator-approval-packet-canonical-export-digest=pass
context-plane-operator-approval-packet-canonical-export-digest.schema=1
context-plane-operator-approval-packet-canonical-export-digest.approval-report-lines=32
context-plane-operator-approval-packet-canonical-export-digest.approval-report-sha256=44a6e04d66624c023cb308e5bb09950fc0a779d81dc110ef690fa1598f537960
context-plane-operator-approval-packet-canonical-export-digest.negative-export-report-lines=4
context-plane-operator-approval-packet-canonical-export-digest.negative-export-report-sha256=06a70c53825a9a9d55573a2e108e2beb7a51f78ee4faf834918a656943e8aec2
context-plane-operator-approval-packet-canonical-export-digest.combined-report-lines=36
context-plane-operator-approval-packet-canonical-export-digest.combined-report-sha256=9f935f8f23c1c1c7605047ea8cbc31abdf23cff1b203f5d09e4ed44a4b90d1df
context-plane-operator-approval-packet-canonical-export-digest.runtime-activation=disabled
context-plane-operator-approval-packet-canonical-export-digest.operator-activation=disabled
STATUS
)"

canonical_guard_accepts() {
  local approval_status="$1"
  local negative_status="$2"
  local digest_status="$3"
  local combined_status

  combined_status="$(printf '%s\n%s' "$approval_status" "$negative_status")"

  [ "$digest_status" = "$expected_digest_status" ] || return 1
  [ "$(line_count "$approval_status")" = "32" ] || return 1
  [ "$(line_count "$negative_status")" = "4" ] || return 1
  [ "$(line_count "$combined_status")" = "36" ] || return 1
  [ "$(printf '%s\n' "$approval_status" | sha256_digest)" = "44a6e04d66624c023cb308e5bb09950fc0a779d81dc110ef690fa1598f537960" ] || return 1
  [ "$(printf '%s\n' "$negative_status" | sha256_digest)" = "06a70c53825a9a9d55573a2e108e2beb7a51f78ee4faf834918a656943e8aec2" ] || return 1
  [ "$(printf '%s\n' "$combined_status" | sha256_digest)" = "9f935f8f23c1c1c7605047ea8cbc31abdf23cff1b203f5d09e4ed44a4b90d1df" ] || return 1

  if printf '%s\n%s\n%s\n' "$approval_status" "$negative_status" "$digest_status" | grep -E 'activation_command|tool_args|raw_payload|prompt_text|transcript_text|memory_text|answer_text|source_id|session_id|memory_id|trace_id|query_text|ranked_payload|entity_hash|supersedes|idempotency|fixture_hash|operator@example\.com|activation-command=(run|enabled|present)|runtime-activation=enabled|production-write=enabled|graph-write=enabled|operator-activation=enabled' >/dev/null; then
    return 1
  fi

  return 0
}

assert_rejected() {
  local label="$1"
  local approval_status="$2"
  local negative_status="$3"
  local digest_status="$4"

  if canonical_guard_accepts "$approval_status" "$negative_status" "$digest_status"; then
    fail "$label tamper was accepted by canonical digest/no-payload guard"
  fi
}

approval_status="$(bash "$approval_report")"
negative_status="$(bash "$negative_export_report")"
digest_status="$(bash "$digest_report")"

if ! canonical_guard_accepts "$approval_status" "$negative_status" "$digest_status"; then
  fail "canonical baseline export must pass before tamper fixtures run"
fi

line_order_tamper="$(
  printf '%s\n' "$approval_status" | awk 'NR == 1 { first = $0; next } NR == 2 { print; print first; next } { print }'
)"
line_count_tamper="$(
  printf '%s\n' "$approval_status"
  printf '%s\n' "context-plane-operator-approval-packet.extra-line=unexpected"
)"
digest_value_tamper="$(printf '%s\n' "$digest_status" | sed 's/44a6e04d66624c023cb308e5bb09950fc0a779d81dc110ef690fa1598f537960/7a20d497af07bc06a02339a94622ceec4c917e91b218ebf4df89aa899628d994/')"
activation_command_tamper="$(
  printf '%s\n' "$approval_status"
  printf '%s\n' "context-plane-operator-approval-packet.activation-command=run"
)"
raw_payload_tamper="$(
  printf '%s\n' "$approval_status"
  printf '%s\n' "context-plane-operator-approval-packet.raw_payload=leak"
)"
pii_tamper="$(
  printf '%s\n' "$approval_status"
  printf '%s\n' "context-plane-operator-approval-packet.operator-email=operator@example.com"
)"
write_activation_tamper="$(printf '%s\n' "$approval_status" | sed 's/context-plane-operator-approval-packet.production-write=disabled/context-plane-operator-approval-packet.production-write=enabled/')"

assert_rejected "line-order" "$line_order_tamper" "$negative_status" "$digest_status"
assert_rejected "line-count" "$line_count_tamper" "$negative_status" "$digest_status"
assert_rejected "digest-value" "$approval_status" "$negative_status" "$digest_value_tamper"
assert_rejected "activation-command" "$activation_command_tamper" "$negative_status" "$digest_status"
assert_rejected "raw-payload" "$raw_payload_tamper" "$negative_status" "$digest_status"
assert_rejected "PII-shaped value" "$pii_tamper" "$negative_status" "$digest_status"
assert_rejected "write/activation flag" "$write_activation_tamper" "$negative_status" "$digest_status"

echo "context-plane-operator-approval-packet-digest-tamper-matrix=pass"
echo "context-plane-operator-approval-packet-digest-tamper-matrix.line-order=reject"
echo "context-plane-operator-approval-packet-digest-tamper-matrix.line-count=reject"
echo "context-plane-operator-approval-packet-digest-tamper-matrix.digest-value=reject"
echo "context-plane-operator-approval-packet-digest-tamper-matrix.activation-command=reject"
echo "context-plane-operator-approval-packet-digest-tamper-matrix.raw-payload=reject"
echo "context-plane-operator-approval-packet-digest-tamper-matrix.pii-shaped=reject"
echo "context-plane-operator-approval-packet-digest-tamper-matrix.write-activation-flag=reject"
echo "context-plane-operator-approval-packet-digest-tamper-matrix.runtime-activation=disabled"
echo "Hepta context plane operator approval packet digest tamper matrix gate passed"
