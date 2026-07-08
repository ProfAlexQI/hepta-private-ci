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
  echo "hepta-context-plane-operator-approval-packet-canonical-export-digest-gate: $*" >&2
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
  "Context Plane operator approval packet canonical export digest" \
  "context-plane-operator-approval-packet-canonical-export-digest=pass" \
  "approval report 95 lines" \
  "negative export report 4 lines" \
  "combined report 99 lines" \
  "deterministic and idempotent" \
  "SHA-256" \
  "hepta-context-plane-operator-approval-packet-negative-export-report.sh" \
  "hepta-context-plane-operator-approval-packet-canonical-export-digest-report.sh" \
  "hepta-context-plane-operator-approval-packet-canonical-export-digest-gate.sh" \
  "must not activate adaptive allocation" \
  "must not activate source-aware compression" \
  "must not write graph facts" \
  "must not write production memory" \
  "must not alter prompt assembly" \
  "must not enable runtime or operator activation"; do
  assert_file_contains "$contracts" "$term" "operator approval packet canonical digest contract"
done

assert_file_contains "$debug_gate" \
  "hepta-context-plane-operator-approval-packet-canonical-export-digest-gate.sh" \
  "operator approval packet canonical digest debug gate"
assert_file_contains "$preflight_script" \
  "context plane operator approval packet canonical export digest gate" \
  "operator approval packet canonical digest preflight stage"
assert_file_contains "$front_door_gate" \
  "context_plane_operator_approval_packet_canonical_digest_gate_script" \
  "operator approval packet canonical digest front-door static check"

assert_line_before \
  "$debug_gate" \
  "hepta-context-plane-operator-approval-packet-negative-export-gate.sh" \
  "hepta-context-plane-operator-approval-packet-canonical-export-digest-gate.sh" \
  "operator approval packet canonical digest debug order"
assert_line_before \
  "$debug_gate" \
  "hepta-context-plane-operator-approval-packet-canonical-export-digest-gate.sh" \
  "hepta-context-source-aware-compression-front-door-report.sh" \
  "operator approval packet canonical digest debug front-door order"
assert_line_before \
  "$preflight_script" \
  "context plane operator approval packet negative export guard" \
  "context plane operator approval packet canonical export digest gate" \
  "operator approval packet canonical digest preflight order"
assert_line_before \
  "$preflight_script" \
  "context plane operator approval packet canonical export digest gate" \
  "source-aware compression front-door machine-readable report" \
  "operator approval packet canonical digest front-door preflight order"

expected_negative_status="$(cat <<'STATUS'
context-plane-operator-approval-packet-negative-export=pass
context-plane-operator-approval-packet-negative-export.activation-command=absent
context-plane-operator-approval-packet-negative-export.payload-light=pass
context-plane-operator-approval-packet-negative-export.runtime-activation=disabled
STATUS
)"
expected_digest_status="$(cat <<'STATUS'
context-plane-operator-approval-packet-canonical-export-digest=pass
context-plane-operator-approval-packet-canonical-export-digest.schema=1
context-plane-operator-approval-packet-canonical-export-digest.approval-report-lines=95
context-plane-operator-approval-packet-canonical-export-digest.approval-report-sha256=3079a5e368ab61d13f53607895a8d2f9c50ba962333962dded7aa9979c250304
context-plane-operator-approval-packet-canonical-export-digest.negative-export-report-lines=4
context-plane-operator-approval-packet-canonical-export-digest.negative-export-report-sha256=06a70c53825a9a9d55573a2e108e2beb7a51f78ee4faf834918a656943e8aec2
context-plane-operator-approval-packet-canonical-export-digest.combined-report-lines=99
context-plane-operator-approval-packet-canonical-export-digest.combined-report-sha256=6f4adc16f244d7db1fef0c7ceea4c0f200fb916e4fb0f02a6ecd229c560639eb
context-plane-operator-approval-packet-canonical-export-digest.runtime-activation=disabled
context-plane-operator-approval-packet-canonical-export-digest.operator-activation=disabled
STATUS
)"

approval_status="$(bash "$approval_report")"
negative_status="$(bash "$negative_export_report")"
digest_status="$(bash "$digest_report")"
digest_status_second="$(bash "$digest_report")"
combined_status="$(printf '%s\n%s' "$approval_status" "$negative_status")"

if [ "$negative_status" != "$expected_negative_status" ]; then
  fail "operator approval packet negative export report output changed"
fi
if [ "$digest_status" != "$expected_digest_status" ]; then
  fail "operator approval packet canonical digest report output changed"
fi
if [ "$digest_status" != "$digest_status_second" ]; then
  fail "operator approval packet canonical digest report is not idempotent"
fi

if [ "$(line_count "$approval_status")" != "95" ]; then
  fail "approval packet canonical line count changed"
fi
if [ "$(line_count "$negative_status")" != "4" ]; then
  fail "negative export canonical line count changed"
fi
if [ "$(line_count "$combined_status")" != "99" ]; then
  fail "combined canonical line count changed"
fi

if [ "$(printf '%s\n' "$approval_status" | sha256_digest)" != "3079a5e368ab61d13f53607895a8d2f9c50ba962333962dded7aa9979c250304" ]; then
  fail "approval packet canonical digest changed"
fi
if [ "$(printf '%s\n' "$negative_status" | sha256_digest)" != "06a70c53825a9a9d55573a2e108e2beb7a51f78ee4faf834918a656943e8aec2" ]; then
  fail "negative export canonical digest changed"
fi
if [ "$(printf '%s\n' "$combined_status" | sha256_digest)" != "6f4adc16f244d7db1fef0c7ceea4c0f200fb916e4fb0f02a6ecd229c560639eb" ]; then
  fail "combined canonical digest changed"
fi

if printf '%s\n%s\n%s\n' "$approval_status" "$negative_status" "$digest_status" | grep -E 'activation_command|tool_args|raw_payload|prompt_text|transcript_text|memory_text|answer_text|source_id|session_id|memory_id|trace_id|query_text|ranked_payload|entity_hash|supersedes|idempotency|fixture_hash|operator@example\.com|activation-command=(run|enabled|present)|runtime-activation=enabled|production-write=enabled|graph-write=enabled|operator-activation=enabled' >/dev/null; then
  fail "operator approval packet canonical digest export leaked payload or enabled activation"
fi

echo "context-plane-operator-approval-packet-canonical-export-digest=pass"
echo "context-plane-operator-approval-packet-canonical-export-digest.approval-report-lines=95"
echo "context-plane-operator-approval-packet-canonical-export-digest.negative-export-report-lines=4"
echo "context-plane-operator-approval-packet-canonical-export-digest.combined-report-lines=99"
echo "context-plane-operator-approval-packet-canonical-export-digest.runtime-activation=disabled"
echo "Hepta context plane operator approval packet canonical export digest gate passed"
