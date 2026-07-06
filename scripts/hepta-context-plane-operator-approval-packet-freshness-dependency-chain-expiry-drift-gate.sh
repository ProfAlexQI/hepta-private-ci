#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
contracts="$repo_root/codex-rs/CONTEXT_DEBUG_CONTRACTS.md"
debug_gate="$repo_root/scripts/hepta-context-debug-gate.sh"
preflight_script="$repo_root/scripts/hepta-context-preflight.sh"
front_door_gate="$repo_root/scripts/hepta-context-source-aware-compression-front-door-gate.sh"
canonical_digest_gate="$repo_root/scripts/hepta-context-plane-operator-approval-packet-freshness-dependency-chain-canonical-digest-gate.sh"
canonical_digest_report="$repo_root/scripts/hepta-context-plane-operator-approval-packet-freshness-dependency-chain-canonical-digest-report.sh"
expiry_drift_report="$repo_root/scripts/hepta-context-plane-operator-approval-packet-freshness-dependency-chain-expiry-drift-report.sh"

fail() {
  echo "hepta-context-plane-operator-approval-packet-freshness-dependency-chain-expiry-drift-gate: $*" >&2
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
  "Context Plane operator approval packet freshness dependency-chain expiry/readiness-window drift guard" \
  "context-plane-operator-approval-packet-freshness-dependency-chain-expiry-drift=pass" \
  "source canonical digest report 15 lines" \
  "source canonical digest report SHA-256" \
  "readiness-chain-generation=276" \
  "source-readiness-chain-generation=275" \
  "source-dependency-chain-generation=274" \
  "source-freshness-sequence=273" \
  "readiness-window-start-sequence=273" \
  "readiness-window-current-sequence=276" \
  "readiness-window-expires-after-sequence=277" \
  "readiness-window-max-drift-sequences=0" \
  "expired-window=reject" \
  "window-start-drift=reject" \
  "window-current-drift=reject" \
  "window-expiry-drift=reject" \
  "source-digest-replay=reject" \
  "expiry/readiness-window drift guard" \
  "hepta-context-plane-operator-approval-packet-freshness-dependency-chain-expiry-drift-report.sh" \
  "hepta-context-plane-operator-approval-packet-freshness-dependency-chain-expiry-drift-gate.sh" \
  "must not activate adaptive allocation" \
  "must not activate source-aware compression" \
  "must not write graph facts" \
  "must not write production memory" \
  "must not alter prompt assembly" \
  "must not enable runtime or operator activation"; do
  assert_file_contains "$contracts" "$term" "operator approval packet freshness dependency-chain expiry drift contract"
done

assert_file_contains "$debug_gate" \
  "hepta-context-plane-operator-approval-packet-freshness-dependency-chain-expiry-drift-gate.sh" \
  "operator approval packet freshness dependency-chain expiry drift debug gate"
assert_file_contains "$preflight_script" \
  "context plane operator approval packet freshness dependency-chain expiry drift gate" \
  "operator approval packet freshness dependency-chain expiry drift preflight stage"
assert_file_contains "$front_door_gate" \
  "context_plane_operator_approval_packet_freshness_dependency_chain_expiry_drift_gate_script" \
  "operator approval packet freshness dependency-chain expiry drift front-door static check"

assert_line_before \
  "$debug_gate" \
  "hepta-context-plane-operator-approval-packet-freshness-dependency-chain-canonical-digest-gate.sh" \
  "hepta-context-plane-operator-approval-packet-freshness-dependency-chain-expiry-drift-gate.sh" \
  "operator approval packet freshness dependency-chain expiry drift debug order"
assert_line_before \
  "$debug_gate" \
  "hepta-context-plane-operator-approval-packet-freshness-dependency-chain-expiry-drift-gate.sh" \
  "hepta-context-source-aware-compression-front-door-report.sh" \
  "operator approval packet freshness dependency-chain expiry drift debug front-door order"
assert_line_before \
  "$preflight_script" \
  "context plane operator approval packet freshness dependency-chain canonical digest gate" \
  "context plane operator approval packet freshness dependency-chain expiry drift gate" \
  "operator approval packet freshness dependency-chain expiry drift preflight order"
assert_line_before \
  "$preflight_script" \
  "context plane operator approval packet freshness dependency-chain expiry drift gate" \
  "source-aware compression front-door machine-readable report" \
  "operator approval packet freshness dependency-chain expiry drift front-door preflight order"

expected_source_status="$(cat <<'STATUS'
context-plane-operator-approval-packet-freshness-dependency-chain-canonical-digest=pass
context-plane-operator-approval-packet-freshness-dependency-chain-canonical-digest.schema=1
context-plane-operator-approval-packet-freshness-dependency-chain-canonical-digest.dependency-chain-report-lines=20
context-plane-operator-approval-packet-freshness-dependency-chain-canonical-digest.dependency-chain-report-sha256=8d0fe75f73732ab36b6b42a604f3f0b6957d94b0359f858d3486fe948479f29b
context-plane-operator-approval-packet-freshness-dependency-chain-canonical-digest.readiness-chain-generation=275
context-plane-operator-approval-packet-freshness-dependency-chain-canonical-digest.source-readiness-chain-generation=274
context-plane-operator-approval-packet-freshness-dependency-chain-canonical-digest.source-freshness-sequence=273
context-plane-operator-approval-packet-freshness-dependency-chain-canonical-digest.reordered-dependency-rows=reject
context-plane-operator-approval-packet-freshness-dependency-chain-canonical-digest.mismatched-upstream-digest=reject
context-plane-operator-approval-packet-freshness-dependency-chain-canonical-digest.mixed-generation-replay=reject
context-plane-operator-approval-packet-freshness-dependency-chain-canonical-digest.mixed-sequence-replay=reject
context-plane-operator-approval-packet-freshness-dependency-chain-canonical-digest.payload-field-injection=reject
context-plane-operator-approval-packet-freshness-dependency-chain-canonical-digest.write-activation-field-injection=reject
context-plane-operator-approval-packet-freshness-dependency-chain-canonical-digest.runtime-activation=disabled
context-plane-operator-approval-packet-freshness-dependency-chain-canonical-digest.operator-activation=disabled
STATUS
)"

expected_expiry_status="$(cat <<'STATUS'
context-plane-operator-approval-packet-freshness-dependency-chain-expiry-drift=pass
context-plane-operator-approval-packet-freshness-dependency-chain-expiry-drift.schema=1
context-plane-operator-approval-packet-freshness-dependency-chain-expiry-drift.source-canonical-digest-report-lines=15
context-plane-operator-approval-packet-freshness-dependency-chain-expiry-drift.source-canonical-digest-report-sha256=a1ac32ab4e52b22d761930cf83ed820cb228993b5b060d667aca18ba69d62f76
context-plane-operator-approval-packet-freshness-dependency-chain-expiry-drift.readiness-chain-generation=276
context-plane-operator-approval-packet-freshness-dependency-chain-expiry-drift.source-readiness-chain-generation=275
context-plane-operator-approval-packet-freshness-dependency-chain-expiry-drift.source-dependency-chain-generation=274
context-plane-operator-approval-packet-freshness-dependency-chain-expiry-drift.source-freshness-sequence=273
context-plane-operator-approval-packet-freshness-dependency-chain-expiry-drift.readiness-window-start-sequence=273
context-plane-operator-approval-packet-freshness-dependency-chain-expiry-drift.readiness-window-current-sequence=276
context-plane-operator-approval-packet-freshness-dependency-chain-expiry-drift.readiness-window-expires-after-sequence=277
context-plane-operator-approval-packet-freshness-dependency-chain-expiry-drift.readiness-window-max-drift-sequences=0
context-plane-operator-approval-packet-freshness-dependency-chain-expiry-drift.expired-window=reject
context-plane-operator-approval-packet-freshness-dependency-chain-expiry-drift.window-start-drift=reject
context-plane-operator-approval-packet-freshness-dependency-chain-expiry-drift.window-current-drift=reject
context-plane-operator-approval-packet-freshness-dependency-chain-expiry-drift.window-expiry-drift=reject
context-plane-operator-approval-packet-freshness-dependency-chain-expiry-drift.source-digest-replay=reject
context-plane-operator-approval-packet-freshness-dependency-chain-expiry-drift.payload-field-injection=reject
context-plane-operator-approval-packet-freshness-dependency-chain-expiry-drift.write-activation-field-injection=reject
context-plane-operator-approval-packet-freshness-dependency-chain-expiry-drift.runtime-activation=disabled
context-plane-operator-approval-packet-freshness-dependency-chain-expiry-drift.operator-activation=disabled
STATUS
)"

expiry_guard_accepts() {
  local source_status="$1"
  local expiry_status="$2"

  [ "$source_status" = "$expected_source_status" ] || return 1
  [ "$expiry_status" = "$expected_expiry_status" ] || return 1
  [ "$(line_count "$source_status")" = "15" ] || return 1
  [ "$(line_count "$expiry_status")" = "21" ] || return 1
  [ "$(printf '%s\n' "$source_status" | sha256_digest)" = "a1ac32ab4e52b22d761930cf83ed820cb228993b5b060d667aca18ba69d62f76" ] || return 1

  if printf '%s\n%s\n' "$source_status" "$expiry_status" | grep -E 'activation_command|tool_args|raw_payload|prompt_text|transcript_text|memory_text|answer_text|source_id|session_id|memory_id|trace_id|query_text|ranked_payload|entity_hash|supersedes|idempotency|fixture_hash|operator@example\.com|activation-command=(run|enabled|present)|runtime-activation=enabled|production-write=enabled|graph-write=enabled|operator-activation=enabled' >/dev/null; then
    return 1
  fi

  return 0
}

assert_rejected() {
  local label="$1"
  local source_status="$2"
  local expiry_status="$3"

  if expiry_guard_accepts "$source_status" "$expiry_status"; then
    fail "$label expiry drift fixture was accepted"
  fi
}

bash "$canonical_digest_gate" >/dev/null

source_status="$(bash "$canonical_digest_report")"
expiry_status="$(bash "$expiry_drift_report")"
expiry_status_second="$(bash "$expiry_drift_report")"

if ! expiry_guard_accepts "$source_status" "$expiry_status"; then
  fail "canonical expiry drift report must pass before drift fixtures run"
fi
if [ "$expiry_status" != "$expiry_status_second" ]; then
  fail "operator approval packet freshness dependency-chain expiry drift report is not idempotent"
fi

source_digest_replay_tamper="$(printf '%s\n' "$expiry_status" | sed 's/a1ac32ab4e52b22d761930cf83ed820cb228993b5b060d667aca18ba69d62f76/b1ac32ab4e52b22d761930cf83ed820cb228993b5b060d667aca18ba69d62f76/')"
source_generation_tamper="$(printf '%s\n' "$expiry_status" | sed 's/source-readiness-chain-generation=275/source-readiness-chain-generation=274/')"
dependency_generation_tamper="$(printf '%s\n' "$expiry_status" | sed 's/source-dependency-chain-generation=274/source-dependency-chain-generation=273/')"
freshness_sequence_tamper="$(printf '%s\n' "$expiry_status" | sed 's/source-freshness-sequence=273/source-freshness-sequence=272/')"
window_start_tamper="$(printf '%s\n' "$expiry_status" | sed 's/readiness-window-start-sequence=273/readiness-window-start-sequence=274/')"
window_current_tamper="$(printf '%s\n' "$expiry_status" | sed 's/readiness-window-current-sequence=276/readiness-window-current-sequence=275/')"
window_expiry_tamper="$(printf '%s\n' "$expiry_status" | sed 's/readiness-window-expires-after-sequence=277/readiness-window-expires-after-sequence=276/')"
expired_window_tamper="$(printf '%s\n' "$expiry_status" | sed 's/expired-window=reject/expired-window=accept/')"
payload_field_tamper="$(
  printf '%s\n' "$expiry_status"
  printf '%s\n' "context-plane-operator-approval-packet-freshness-dependency-chain-expiry-drift.raw_payload=leak"
)"
activation_field_tamper="$(
  printf '%s\n' "$expiry_status"
  printf '%s\n' "context-plane-operator-approval-packet-freshness-dependency-chain-expiry-drift.activation-command=run"
)"
write_activation_tamper="$(printf '%s\n' "$expiry_status" | sed 's/runtime-activation=disabled/runtime-activation=enabled/')"

assert_rejected "source digest replay" "$source_status" "$source_digest_replay_tamper"
assert_rejected "source readiness generation" "$source_status" "$source_generation_tamper"
assert_rejected "source dependency generation" "$source_status" "$dependency_generation_tamper"
assert_rejected "source freshness sequence" "$source_status" "$freshness_sequence_tamper"
assert_rejected "readiness window start" "$source_status" "$window_start_tamper"
assert_rejected "readiness window current" "$source_status" "$window_current_tamper"
assert_rejected "readiness window expiry" "$source_status" "$window_expiry_tamper"
assert_rejected "expired window decision" "$source_status" "$expired_window_tamper"
assert_rejected "payload field injection" "$source_status" "$payload_field_tamper"
assert_rejected "activation field injection" "$source_status" "$activation_field_tamper"
assert_rejected "write/activation flag" "$source_status" "$write_activation_tamper"

echo "context-plane-operator-approval-packet-freshness-dependency-chain-expiry-drift=pass"
echo "context-plane-operator-approval-packet-freshness-dependency-chain-expiry-drift.source-digest-replay=reject"
echo "context-plane-operator-approval-packet-freshness-dependency-chain-expiry-drift.source-generation-drift=reject"
echo "context-plane-operator-approval-packet-freshness-dependency-chain-expiry-drift.source-sequence-drift=reject"
echo "context-plane-operator-approval-packet-freshness-dependency-chain-expiry-drift.window-start-drift=reject"
echo "context-plane-operator-approval-packet-freshness-dependency-chain-expiry-drift.window-current-drift=reject"
echo "context-plane-operator-approval-packet-freshness-dependency-chain-expiry-drift.window-expiry-drift=reject"
echo "context-plane-operator-approval-packet-freshness-dependency-chain-expiry-drift.payload-field-injection=reject"
echo "context-plane-operator-approval-packet-freshness-dependency-chain-expiry-drift.write-activation-field-injection=reject"
echo "context-plane-operator-approval-packet-freshness-dependency-chain-expiry-drift.runtime-activation=disabled"
echo "Hepta context plane operator approval packet freshness dependency-chain expiry drift gate passed"
