#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
contracts="$repo_root/codex-rs/CONTEXT_DEBUG_CONTRACTS.md"
debug_gate="$repo_root/scripts/hepta-context-debug-gate.sh"
preflight_script="$repo_root/scripts/hepta-context-preflight.sh"
front_door_gate="$repo_root/scripts/hepta-context-source-aware-compression-front-door-gate.sh"
chain_report="$repo_root/scripts/hepta-context-plane-operator-approval-packet-freshness-dependency-chain-report.sh"
approval_gate="$repo_root/scripts/hepta-context-plane-operator-approval-packet-gate.sh"
negative_gate="$repo_root/scripts/hepta-context-plane-operator-approval-packet-negative-export-gate.sh"
canonical_digest_gate="$repo_root/scripts/hepta-context-plane-operator-approval-packet-canonical-export-digest-gate.sh"
tamper_matrix_gate="$repo_root/scripts/hepta-context-plane-operator-approval-packet-digest-tamper-matrix-gate.sh"
freshness_gate="$repo_root/scripts/hepta-context-plane-operator-approval-packet-freshness-gate.sh"

fail() {
  echo "hepta-context-plane-operator-approval-packet-freshness-dependency-chain-gate: $*" >&2
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
  "Context Plane operator approval packet freshness dependency-chain stale-source negative matrix" \
  "context-plane-operator-approval-packet-freshness-dependency-chain=pass" \
  "approval report dependency" \
  "negative export report dependency" \
  "canonical digest report dependency" \
  "tamper matrix report dependency" \
  "freshness report dependency" \
  "readiness-chain-generation=274" \
  "freshness-source-sequence=273" \
  "stale-source=reject" \
  "mixed-generation=reject" \
  "source-digest-mismatch=reject" \
  "tamper-matrix-replay=reject" \
  "dependency-chain guard" \
  "hepta-context-plane-operator-approval-packet-freshness-dependency-chain-report.sh" \
  "hepta-context-plane-operator-approval-packet-freshness-dependency-chain-gate.sh" \
  "must not activate adaptive allocation" \
  "must not activate source-aware compression" \
  "must not write graph facts" \
  "must not write production memory" \
  "must not alter prompt assembly" \
  "must not enable runtime or operator activation"; do
  assert_file_contains "$contracts" "$term" "operator approval packet freshness dependency chain contract"
done

assert_file_contains "$debug_gate" \
  "hepta-context-plane-operator-approval-packet-freshness-dependency-chain-gate.sh" \
  "operator approval packet freshness dependency chain debug gate"
assert_file_contains "$preflight_script" \
  "context plane operator approval packet freshness dependency-chain gate" \
  "operator approval packet freshness dependency chain preflight stage"
assert_file_contains "$front_door_gate" \
  "context_plane_operator_approval_packet_freshness_dependency_chain_gate_script" \
  "operator approval packet freshness dependency chain front-door static check"

assert_line_before \
  "$debug_gate" \
  "hepta-context-plane-operator-approval-packet-freshness-gate.sh" \
  "hepta-context-plane-operator-approval-packet-freshness-dependency-chain-gate.sh" \
  "operator approval packet freshness dependency chain debug order"
assert_line_before \
  "$debug_gate" \
  "hepta-context-plane-operator-approval-packet-freshness-dependency-chain-gate.sh" \
  "hepta-context-source-aware-compression-front-door-report.sh" \
  "operator approval packet freshness dependency chain debug front-door order"
assert_line_before \
  "$preflight_script" \
  "context plane operator approval packet freshness replay-protection gate" \
  "context plane operator approval packet freshness dependency-chain gate" \
  "operator approval packet freshness dependency chain preflight order"
assert_line_before \
  "$preflight_script" \
  "context plane operator approval packet freshness dependency-chain gate" \
  "source-aware compression front-door machine-readable report" \
  "operator approval packet freshness dependency chain front-door preflight order"

expected_chain_status="$(cat <<'STATUS'
context-plane-operator-approval-packet-freshness-dependency-chain=pass
context-plane-operator-approval-packet-freshness-dependency-chain.schema=1
context-plane-operator-approval-packet-freshness-dependency-chain.approval-report-lines=179
context-plane-operator-approval-packet-freshness-dependency-chain.approval-report-sha256=b572621a97a919ba06e0f2349d2979b23aeabdd0c1cfbb2aed3d5a40cd7109b4
context-plane-operator-approval-packet-freshness-dependency-chain.negative-export-report-lines=4
context-plane-operator-approval-packet-freshness-dependency-chain.negative-export-report-sha256=06a70c53825a9a9d55573a2e108e2beb7a51f78ee4faf834918a656943e8aec2
context-plane-operator-approval-packet-freshness-dependency-chain.canonical-digest-report-lines=10
context-plane-operator-approval-packet-freshness-dependency-chain.canonical-digest-report-sha256=bc53998127d1b4cdacb4ad44f273aaa9a1b4f47f3d1b8044f8fcf4cdf739caac
context-plane-operator-approval-packet-freshness-dependency-chain.tamper-matrix-report-lines=16
context-plane-operator-approval-packet-freshness-dependency-chain.tamper-matrix-report-sha256=ef83b4e432ea96e5f73f8950f11e215d8149d5c4aa3e4290a5318800b5586e28
context-plane-operator-approval-packet-freshness-dependency-chain.freshness-report-lines=14
context-plane-operator-approval-packet-freshness-dependency-chain.freshness-report-sha256=7020d0f2982b9d7faf5845039550138a5fcd7dddaa63d0d5a078062073150386
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

chain_guard_accepts() {
  local chain_status="$1"

  [ "$chain_status" = "$expected_chain_status" ] || return 1
  [ "$(line_count "$chain_status")" = "20" ] || return 1

  if printf '%s\n' "$chain_status" | grep -E 'activation_command|tool_args|raw_payload|prompt_text|transcript_text|memory_text|answer_text|source_id|session_id|memory_id|trace_id|query_text|ranked_payload|entity_hash|supersedes|idempotency|fixture_hash|operator@example\.com|activation-command=(run|enabled|present)|runtime-activation=enabled|production-write=enabled|graph-write=enabled|operator-activation=enabled' >/dev/null; then
    return 1
  fi

  return 0
}

assert_rejected() {
  local label="$1"
  local chain_status="$2"

  if chain_guard_accepts "$chain_status"; then
    fail "$label dependency-chain fixture was accepted"
  fi
}

bash "$approval_gate" >/dev/null
bash "$negative_gate" >/dev/null
bash "$canonical_digest_gate" >/dev/null
bash "$tamper_matrix_gate" >/dev/null
bash "$freshness_gate" >/dev/null

chain_status="$(bash "$chain_report")"
chain_status_second="$(bash "$chain_report")"

if ! chain_guard_accepts "$chain_status"; then
  fail "canonical dependency-chain report must pass before stale-source fixtures run"
fi
if [ "$chain_status" != "$chain_status_second" ]; then
  fail "operator approval packet freshness dependency-chain report is not idempotent"
fi

stale_approval_source_tamper="$(printf '%s\n' "$chain_status" | sed 's/approval-report-sha256=b572621a97a919ba06e0f2349d2979b23aeabdd0c1cfbb2aed3d5a40cd7109b4/approval-report-sha256=c572621a97a919ba06e0f2349d2979b23aeabdd0c1cfbb2aed3d5a40cd7109b4/')"
stale_negative_source_tamper="$(printf '%s\n' "$chain_status" | sed 's/negative-export-report-sha256=06a70c53825a9a9d55573a2e108e2beb7a51f78ee4faf834918a656943e8aec2/negative-export-report-sha256=16a70c53825a9a9d55573a2e108e2beb7a51f78ee4faf834918a656943e8aec2/')"
stale_canonical_source_tamper="$(printf '%s\n' "$chain_status" | sed 's/canonical-digest-report-sha256=bc53998127d1b4cdacb4ad44f273aaa9a1b4f47f3d1b8044f8fcf4cdf739caac/canonical-digest-report-sha256=cc53998127d1b4cdacb4ad44f273aaa9a1b4f47f3d1b8044f8fcf4cdf739caac/')"
tamper_matrix_replay_tamper="$(printf '%s\n' "$chain_status" | sed 's/tamper-matrix-report-sha256=ef83b4e432ea96e5f73f8950f11e215d8149d5c4aa3e4290a5318800b5586e28/tamper-matrix-report-sha256=ff83b4e432ea96e5f73f8950f11e215d8149d5c4aa3e4290a5318800b5586e28/')"
freshness_source_tamper="$(printf '%s\n' "$chain_status" | sed 's/freshness-report-sha256=7020d0f2982b9d7faf5845039550138a5fcd7dddaa63d0d5a078062073150386/freshness-report-sha256=8020d0f2982b9d7faf5845039550138a5fcd7dddaa63d0d5a078062073150386/')"
mixed_generation_tamper="$(printf '%s\n' "$chain_status" | sed 's/readiness-chain-generation=274/readiness-chain-generation=273/')"
mixed_sequence_tamper="$(printf '%s\n' "$chain_status" | sed 's/freshness-source-sequence=273/freshness-source-sequence=272/')"
line_count_tamper="$(
  printf '%s\n' "$chain_status"
  printf '%s\n' "context-plane-operator-approval-packet-freshness-dependency-chain.recombined-source=unexpected"
)"
activation_command_tamper="$(
  printf '%s\n' "$chain_status"
  printf '%s\n' "context-plane-operator-approval-packet-freshness-dependency-chain.activation-command=run"
)"
write_activation_tamper="$(printf '%s\n' "$chain_status" | sed 's/runtime-activation=disabled/runtime-activation=enabled/')"

assert_rejected "stale approval source" "$stale_approval_source_tamper"
assert_rejected "stale negative export source" "$stale_negative_source_tamper"
assert_rejected "stale canonical digest source" "$stale_canonical_source_tamper"
assert_rejected "tamper matrix replay source" "$tamper_matrix_replay_tamper"
assert_rejected "freshness source digest" "$freshness_source_tamper"
assert_rejected "mixed generation" "$mixed_generation_tamper"
assert_rejected "mixed freshness sequence" "$mixed_sequence_tamper"
assert_rejected "line-count" "$line_count_tamper"
assert_rejected "activation-command" "$activation_command_tamper"
assert_rejected "write/activation flag" "$write_activation_tamper"

echo "context-plane-operator-approval-packet-freshness-dependency-chain=pass"
echo "context-plane-operator-approval-packet-freshness-dependency-chain.stale-approval-source=reject"
echo "context-plane-operator-approval-packet-freshness-dependency-chain.stale-negative-source=reject"
echo "context-plane-operator-approval-packet-freshness-dependency-chain.stale-canonical-source=reject"
echo "context-plane-operator-approval-packet-freshness-dependency-chain.tamper-matrix-replay=reject"
echo "context-plane-operator-approval-packet-freshness-dependency-chain.freshness-source-digest=reject"
echo "context-plane-operator-approval-packet-freshness-dependency-chain.mixed-generation=reject"
echo "context-plane-operator-approval-packet-freshness-dependency-chain.mixed-sequence=reject"
echo "context-plane-operator-approval-packet-freshness-dependency-chain.activation-command=reject"
echo "context-plane-operator-approval-packet-freshness-dependency-chain.write-activation-flag=reject"
echo "context-plane-operator-approval-packet-freshness-dependency-chain.runtime-activation=disabled"
echo "Hepta context plane operator approval packet freshness dependency-chain gate passed"
