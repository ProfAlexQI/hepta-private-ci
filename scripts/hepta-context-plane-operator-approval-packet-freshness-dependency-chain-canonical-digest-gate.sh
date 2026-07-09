#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
contracts="$repo_root/codex-rs/CONTEXT_DEBUG_CONTRACTS.md"
debug_gate="$repo_root/scripts/hepta-context-debug-gate.sh"
preflight_script="$repo_root/scripts/hepta-context-preflight.sh"
front_door_gate="$repo_root/scripts/hepta-context-source-aware-compression-front-door-gate.sh"
chain_report="$repo_root/scripts/hepta-context-plane-operator-approval-packet-freshness-dependency-chain-report.sh"
chain_gate="$repo_root/scripts/hepta-context-plane-operator-approval-packet-freshness-dependency-chain-gate.sh"
digest_report="$repo_root/scripts/hepta-context-plane-operator-approval-packet-freshness-dependency-chain-canonical-digest-report.sh"

fail() {
  echo "hepta-context-plane-operator-approval-packet-freshness-dependency-chain-canonical-digest-gate: $*" >&2
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
  "Context Plane operator approval packet freshness dependency-chain canonical digest mixed-source tamper matrix" \
  "context-plane-operator-approval-packet-freshness-dependency-chain-canonical-digest=pass" \
  "dependency-chain report 20 lines" \
  "dependency-chain report SHA-256" \
  "readiness-chain-generation=275" \
  "source-readiness-chain-generation=274" \
  "source-freshness-sequence=273" \
  "reordered dependency rows" \
  "mismatched upstream digests" \
  "mixed generation/sequence replay windows" \
  "injected activation/write/payload fields" \
  "dependency-chain canonical digest guard" \
  "hepta-context-plane-operator-approval-packet-freshness-dependency-chain-canonical-digest-report.sh" \
  "hepta-context-plane-operator-approval-packet-freshness-dependency-chain-canonical-digest-gate.sh" \
  "must not activate adaptive allocation" \
  "must not activate source-aware compression" \
  "must not write graph facts" \
  "must not write production memory" \
  "must not alter prompt assembly" \
  "must not enable runtime or operator activation"; do
  assert_file_contains "$contracts" "$term" "operator approval packet freshness dependency-chain canonical digest contract"
done

assert_file_contains "$debug_gate" \
  "hepta-context-plane-operator-approval-packet-freshness-dependency-chain-canonical-digest-gate.sh" \
  "operator approval packet freshness dependency-chain canonical digest debug gate"
assert_file_contains "$preflight_script" \
  "context plane operator approval packet freshness dependency-chain canonical digest gate" \
  "operator approval packet freshness dependency-chain canonical digest preflight stage"
assert_file_contains "$front_door_gate" \
  "context_plane_operator_approval_packet_freshness_dependency_chain_canonical_digest_gate_script" \
  "operator approval packet freshness dependency-chain canonical digest front-door static check"

assert_line_before \
  "$debug_gate" \
  "hepta-context-plane-operator-approval-packet-freshness-dependency-chain-gate.sh" \
  "hepta-context-plane-operator-approval-packet-freshness-dependency-chain-canonical-digest-gate.sh" \
  "operator approval packet freshness dependency-chain canonical digest debug order"
assert_line_before \
  "$debug_gate" \
  "hepta-context-plane-operator-approval-packet-freshness-dependency-chain-canonical-digest-gate.sh" \
  "hepta-context-source-aware-compression-front-door-report.sh" \
  "operator approval packet freshness dependency-chain canonical digest debug front-door order"
assert_line_before \
  "$preflight_script" \
  "context plane operator approval packet freshness dependency-chain gate" \
  "context plane operator approval packet freshness dependency-chain canonical digest gate" \
  "operator approval packet freshness dependency-chain canonical digest preflight order"
assert_line_before \
  "$preflight_script" \
  "context plane operator approval packet freshness dependency-chain canonical digest gate" \
  "source-aware compression front-door machine-readable report" \
  "operator approval packet freshness dependency-chain canonical digest front-door preflight order"

expected_chain_status="$(cat <<'STATUS'
context-plane-operator-approval-packet-freshness-dependency-chain=pass
context-plane-operator-approval-packet-freshness-dependency-chain.schema=1
context-plane-operator-approval-packet-freshness-dependency-chain.approval-report-lines=129
context-plane-operator-approval-packet-freshness-dependency-chain.approval-report-sha256=8026268c21e13763e84e03175fe96f24ff23d83cea6ccd3164032a235d692d8a
context-plane-operator-approval-packet-freshness-dependency-chain.negative-export-report-lines=4
context-plane-operator-approval-packet-freshness-dependency-chain.negative-export-report-sha256=06a70c53825a9a9d55573a2e108e2beb7a51f78ee4faf834918a656943e8aec2
context-plane-operator-approval-packet-freshness-dependency-chain.canonical-digest-report-lines=10
context-plane-operator-approval-packet-freshness-dependency-chain.canonical-digest-report-sha256=db1b88d2f6c31a96f9f550116e08a13845c855900413feb94f230cdb78025058
context-plane-operator-approval-packet-freshness-dependency-chain.tamper-matrix-report-lines=16
context-plane-operator-approval-packet-freshness-dependency-chain.tamper-matrix-report-sha256=ef83b4e432ea96e5f73f8950f11e215d8149d5c4aa3e4290a5318800b5586e28
context-plane-operator-approval-packet-freshness-dependency-chain.freshness-report-lines=14
context-plane-operator-approval-packet-freshness-dependency-chain.freshness-report-sha256=e1c8c206446e6bcf3d0febb957e5feb6f27f252f609b590d72b6272eea73a2bf
context-plane-operator-approval-packet-freshness-dependency-chain.readiness-chain-generation=274
context-plane-operator-approval-packet-freshness-dependency-chain.freshness-source-sequence=273
context-plane-operator-approval-packet-freshness-dependency-chain.stale-source=reject
context-plane-operator-approval-packet-freshness-dependency-chain.mixed-generation=reject
context-plane-operator-approval-packet-freshness-dependency-chain.source-digest-mismatch=reject
context-plane-operator-approval-packet-freshness-dependency-chain.tamper-matrix-replay=reject
context-plane-operator-approval-packet-freshness-dependency-chain.runtime-activation=disabled
context-plane-operator-approval-packet-freshness-dependency-chain.operator-activation=disabled
STATUS
)"

expected_digest_status="$(cat <<'STATUS'
context-plane-operator-approval-packet-freshness-dependency-chain-canonical-digest=pass
context-plane-operator-approval-packet-freshness-dependency-chain-canonical-digest.schema=1
context-plane-operator-approval-packet-freshness-dependency-chain-canonical-digest.dependency-chain-report-lines=20
context-plane-operator-approval-packet-freshness-dependency-chain-canonical-digest.dependency-chain-report-sha256=962740260b44f6bda9110a5374736d2b8c9ea7baf4f8eb879413d191b06f75cc
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

canonical_guard_accepts() {
  local chain_status="$1"
  local digest_status="$2"

  [ "$chain_status" = "$expected_chain_status" ] || return 1
  [ "$digest_status" = "$expected_digest_status" ] || return 1
  [ "$(line_count "$chain_status")" = "20" ] || return 1
  [ "$(line_count "$digest_status")" = "15" ] || return 1
  [ "$(printf '%s\n' "$chain_status" | sha256_digest)" = "962740260b44f6bda9110a5374736d2b8c9ea7baf4f8eb879413d191b06f75cc" ] || return 1

  if printf '%s\n%s\n' "$chain_status" "$digest_status" | grep -E 'activation_command|tool_args|raw_payload|prompt_text|transcript_text|memory_text|answer_text|source_id|session_id|memory_id|trace_id|query_text|ranked_payload|entity_hash|supersedes|idempotency|fixture_hash|operator@example\.com|activation-command=(run|enabled|present)|runtime-activation=enabled|production-write=enabled|graph-write=enabled|operator-activation=enabled' >/dev/null; then
    return 1
  fi

  return 0
}

assert_rejected() {
  local label="$1"
  local chain_status="$2"
  local digest_status="$3"

  if canonical_guard_accepts "$chain_status" "$digest_status"; then
    fail "$label dependency-chain canonical digest fixture was accepted"
  fi
}

bash "$chain_gate" >/dev/null

chain_status="$(bash "$chain_report")"
digest_status="$(bash "$digest_report")"
digest_status_second="$(bash "$digest_report")"

if ! canonical_guard_accepts "$chain_status" "$digest_status"; then
  fail "canonical dependency-chain digest report must pass before mixed-source fixtures run"
fi
if [ "$digest_status" != "$digest_status_second" ]; then
  fail "operator approval packet freshness dependency-chain canonical digest report is not idempotent"
fi

reordered_dependency_rows_tamper="$(
  printf '%s\n' "$chain_status" | awk 'NR == 3 { third = $0; next } NR == 4 { print; print third; next } { print }'
)"
mismatched_upstream_digest_tamper="$(printf '%s\n' "$chain_status" | sed 's/approval-report-sha256=8026268c21e13763e84e03175fe96f24ff23d83cea6ccd3164032a235d692d8a/approval-report-sha256=9026268c21e13763e84e03175fe96f24ff23d83cea6ccd3164032a235d692d8a/')"
mixed_generation_replay_tamper="$(printf '%s\n' "$chain_status" | sed 's/readiness-chain-generation=274/readiness-chain-generation=273/')"
mixed_sequence_replay_tamper="$(printf '%s\n' "$chain_status" | sed 's/freshness-source-sequence=273/freshness-source-sequence=272/')"
digest_value_tamper="$(printf '%s\n' "$digest_status" | sed 's/962740260b44f6bda9110a5374736d2b8c9ea7baf4f8eb879413d191b06f75cc/a62740260b44f6bda9110a5374736d2b8c9ea7baf4f8eb879413d191b06f75cc/')"
payload_field_tamper="$(
  printf '%s\n' "$chain_status"
  printf '%s\n' "context-plane-operator-approval-packet-freshness-dependency-chain.raw_payload=leak"
)"
activation_field_tamper="$(
  printf '%s\n' "$chain_status"
  printf '%s\n' "context-plane-operator-approval-packet-freshness-dependency-chain.activation-command=run"
)"
write_activation_tamper="$(printf '%s\n' "$digest_status" | sed 's/runtime-activation=disabled/runtime-activation=enabled/')"

assert_rejected "reordered dependency rows" "$reordered_dependency_rows_tamper" "$digest_status"
assert_rejected "mismatched upstream digest" "$mismatched_upstream_digest_tamper" "$digest_status"
assert_rejected "mixed generation replay" "$mixed_generation_replay_tamper" "$digest_status"
assert_rejected "mixed sequence replay" "$mixed_sequence_replay_tamper" "$digest_status"
assert_rejected "digest value" "$chain_status" "$digest_value_tamper"
assert_rejected "payload field injection" "$payload_field_tamper" "$digest_status"
assert_rejected "activation field injection" "$activation_field_tamper" "$digest_status"
assert_rejected "write/activation flag" "$chain_status" "$write_activation_tamper"

echo "context-plane-operator-approval-packet-freshness-dependency-chain-canonical-digest=pass"
echo "context-plane-operator-approval-packet-freshness-dependency-chain-canonical-digest.reordered-dependency-rows=reject"
echo "context-plane-operator-approval-packet-freshness-dependency-chain-canonical-digest.mismatched-upstream-digest=reject"
echo "context-plane-operator-approval-packet-freshness-dependency-chain-canonical-digest.mixed-generation-replay=reject"
echo "context-plane-operator-approval-packet-freshness-dependency-chain-canonical-digest.mixed-sequence-replay=reject"
echo "context-plane-operator-approval-packet-freshness-dependency-chain-canonical-digest.payload-field-injection=reject"
echo "context-plane-operator-approval-packet-freshness-dependency-chain-canonical-digest.write-activation-field-injection=reject"
echo "context-plane-operator-approval-packet-freshness-dependency-chain-canonical-digest.runtime-activation=disabled"
echo "Hepta context plane operator approval packet freshness dependency-chain canonical digest gate passed"
