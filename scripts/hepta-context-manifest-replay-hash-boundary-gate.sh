#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
contracts="$repo_root/codex-rs/CONTEXT_DEBUG_CONTRACTS.md"
protocol="$repo_root/codex-rs/protocol/src/protocol.rs"
manifest="$repo_root/codex-rs/core/src/context_manager/manifest.rs"
updates="$repo_root/codex-rs/core/src/context_manager/updates.rs"
debug_gate="$repo_root/scripts/hepta-context-debug-gate.sh"
preflight_script="$repo_root/scripts/hepta-context-preflight.sh"
canonical_digest_report="$repo_root/scripts/hepta-context-plane-operator-approval-packet-canonical-export-digest-report.sh"
dependency_canonical_digest_report="$repo_root/scripts/hepta-context-plane-operator-approval-packet-freshness-dependency-chain-canonical-digest-report.sh"

fail() {
  echo "hepta-context-manifest-replay-hash-boundary-gate: $*" >&2
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

assert_file_not_contains() {
  local file_path="$1"
  local needle="$2"
  local label="$3"

  if grep -F "$needle" "$file_path" >/dev/null; then
    fail "$label must not contain: $needle"
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

for term in \
  "Stable manifest replay hash" \
  "stable_turn_context_manifest_replay_hash" \
  "stable_turn_context_manifest_text_hash" \
  "not canonical trust digests" \
  "canonical SHA-256 digest reports" \
  "hepta-context-manifest-replay-hash-boundary-gate.sh"; do
  assert_file_contains "$contracts" "$term" "manifest replay hash boundary contract"
done

for term in \
  "StableManifestReplayHash" \
  "stable_turn_context_manifest_replay_hash" \
  "stable_turn_context_manifest_text_hash" \
  "is_stable_manifest_replay_hash" \
  "turn_context_manifest_hashes_are_replay_hashes_not_trust_digests" \
  "not a cryptographic trust digest"; do
  assert_file_contains "$protocol" "$term" "protocol manifest replay hash boundary"
done

assert_file_not_contains "$protocol" "StableManifestHash" \
  "protocol legacy manifest hash type name"
assert_file_not_contains "$protocol" "is_stable_manifest_hash" \
  "protocol legacy manifest hash predicate name"
assert_file_not_contains "$manifest" "stable_turn_context_manifest_text_hash" \
  "core manifest should use replay hash naming"
assert_file_not_contains "$updates" "stable_turn_context_manifest_text_hash" \
  "core context updates should use replay hash naming"
assert_file_contains "$manifest" "stable_turn_context_manifest_replay_hash" \
  "core manifest replay hash naming"
assert_file_contains "$updates" "stable_turn_context_manifest_replay_hash" \
  "core context updates replay hash naming"

for file_path in "$canonical_digest_report" "$dependency_canonical_digest_report"; do
  assert_file_contains "$file_path" "sha256_digest()" \
    "canonical trust digest report"
  assert_file_not_contains "$file_path" "stable_turn_context_manifest" \
    "canonical trust digest report must not use manifest replay hash"
done

assert_file_contains "$debug_gate" "hepta-context-manifest-replay-hash-boundary-gate.sh" \
  "manifest replay hash boundary debug gate"
assert_file_contains "$preflight_script" "context manifest replay hash boundary gate" \
  "manifest replay hash boundary preflight stage"
assert_line_before \
  "$debug_gate" \
  "hepta-context-manifest-replay-hash-boundary-gate.sh" \
  "hepta-context-source-registry-catalog-gate.sh" \
  "manifest replay hash boundary debug order"
assert_line_before \
  "$preflight_script" \
  "context manifest replay hash boundary gate" \
  "context source registry catalog gate" \
  "manifest replay hash boundary preflight order"

echo "context-manifest-replay-hash-boundary=pass"
echo "context-manifest-replay-hash-boundary.replay-hash-hex-len=16"
echo "context-manifest-replay-hash-boundary.trust-digest=sha256"
echo "context-manifest-replay-hash-boundary.runtime-activation=disabled"
echo "Hepta context manifest replay hash boundary gate passed"
