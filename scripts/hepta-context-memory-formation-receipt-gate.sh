#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
manifest="$repo_root/codex-rs/Cargo.toml"
protocol_turn_context="$repo_root/codex-rs/protocol/src/protocol/turn_context"
protocol_common="$protocol_turn_context/common.rs"
protocol_memory="$protocol_turn_context/memory.rs"
protocol_manifest="$protocol_turn_context/manifest.rs"
protocol_stable_hash="$protocol_turn_context/stable_hash.rs"
context_manifest_options="$repo_root/codex-rs/core/src/context_manager/manifest/options.rs"
context_manifest_tests="$repo_root/codex-rs/core/src/context_manager/manifest/tests.rs"
hepta_core_memory="$repo_root/codex-rs/hepta-core/src/memory.rs"
hepta_core_memory_formation="$repo_root/codex-rs/hepta-core/src/memory/formation.rs"
hepta_core_memory_recall_inspection="$repo_root/codex-rs/hepta-core/src/memory/recall/inspection.rs"
hepta_core_memory_tests="$repo_root/codex-rs/hepta-core/tests/memory_recall_contracts/formation.rs"
hepta_memory="$repo_root/codex-rs/hepta-memory/src/lib.rs"
hepta_memory_tests="$repo_root/codex-rs/hepta-memory/src/tests/recall_memory/formation.rs"
hepta_memory_recall_helpers="$repo_root/codex-rs/hepta-memory/src/recall_helpers/snapshot.rs"
response_debug="$repo_root/codex-rs/response-debug-context/src/lib.rs"
response_debug_memory="$repo_root/codex-rs/response-debug-context/src/rollout_context/memory.rs"
response_debug_tests="$repo_root/codex-rs/response-debug-context/src/tests.rs"
contracts="$repo_root/codex-rs/CONTEXT_DEBUG_CONTRACTS.md"
lane="${HEPTA_CARGO_LANE:-${HEPTA_LANE:-hepta-context}}"
target_root="${HEPTA_CARGO_TARGET_ROOT:-$HOME/.openclaw/tmp/cargo-targets}"
target_leaf="$lane"
if [[ "$target_leaf" != hepta-* ]]; then
  target_leaf="hepta-$target_leaf"
fi
export CARGO_TARGET_DIR="${HEPTA_CARGO_TARGET_DIR:-$target_root/$target_leaf}"
export CARGO_INCREMENTAL="${CARGO_INCREMENTAL:-0}"

fail() {
  echo "hepta-context-memory-formation-receipt-gate: $*" >&2
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

assert_unique_protocol_owner() {
  local pattern="$1"
  local expected_owner="$2"
  local label="$3"
  local owners

  owners="$(rg -l "$pattern" "$protocol_turn_context" -g '*.rs' || true)"
  if [[ "$(printf '%s\n' "$owners" | sed '/^$/d' | wc -l | tr -d ' ')" != "1" \
    || "$owners" != "$expected_owner" ]]; then
    fail "$label must have exactly one typed owner: $expected_owner"
  fi
}

for term in \
  "Background memory formation receipt report" \
  "memory_formation_receipts" \
  "fact" \
  "task" \
  "preference" \
  "decision" \
  "summary" \
  "confidence_basis_points" \
  "idempotency_key_hash" \
  "queued_for_background=true" \
  "production_write=false" \
  "must not write production memory" \
  "must not create durable memory candidates" \
  "manifest_memory_formation_receipts_invalid" \
  "runtime-activation=disabled" \
  "hepta-context-memory-formation-receipt-gate.sh"; do
  assert_file_contains "$contracts" "$term" "memory formation receipt contract"
done

assert_file_contains "$protocol_common" \
  "TURN_CONTEXT_MEMORY_FORMATION_RECEIPT_SCHEMA_VERSION" \
  "memory formation receipt protocol schema version"
assert_file_contains "$protocol_memory" \
  "TurnContextMemoryFormationCandidateType" \
  "memory formation receipt protocol candidate type"
assert_file_contains "$protocol_memory" \
  "TurnContextMemoryFormationReceipt" \
  "memory formation receipt protocol receipt"
assert_file_contains "$protocol_manifest" \
  "memory_formation_receipts_have_integrity" \
  "memory formation receipt protocol integrity"
assert_file_contains "$protocol_stable_hash" \
  "update_memory_formation_receipts" \
  "memory formation receipt protocol ledger hash"
assert_unique_protocol_owner \
  '^pub const TURN_CONTEXT_MEMORY_FORMATION_RECEIPT_SCHEMA_VERSION:' \
  "$protocol_common" \
  "memory formation receipt protocol schema version"
assert_unique_protocol_owner \
  '^pub enum TurnContextMemoryFormationCandidateType ' \
  "$protocol_memory" \
  "memory formation receipt protocol candidate type"
assert_unique_protocol_owner \
  '^pub struct TurnContextMemoryFormationReceipt ' \
  "$protocol_memory" \
  "memory formation receipt protocol receipt"
assert_unique_protocol_owner \
  '^    pub fn memory_formation_receipts_have_integrity\(' \
  "$protocol_manifest" \
  "memory formation receipt protocol integrity"
assert_unique_protocol_owner \
  '^    pub\(super\) fn update_memory_formation_receipts\(' \
  "$protocol_stable_hash" \
  "memory formation receipt protocol ledger hash"

assert_file_contains "$context_manifest_options" \
  "Vec<TurnContextMemoryFormationReceipt>" \
  "memory formation receipt core manifest options"
assert_file_contains "$context_manifest_options" \
  "memory_formation_receipts: extension_data" \
  "memory formation receipt extension-data read"
assert_file_contains "$context_manifest_tests" \
  "turn_context_manifest_resolves_memory_formation_receipts_without_payload_text" \
  "memory formation receipt core no-payload test"

assert_file_contains "$hepta_core_memory_formation" \
  "ContextMemoryFormationReceiptReport" \
  "memory formation receipt hepta-core report"
assert_file_contains "$hepta_core_memory_formation" \
  "from_recall_inspection" \
  "memory formation receipt hepta-core recall mapping"
assert_file_contains "$hepta_core_memory_recall_inspection" \
  "memory_formation_receipt_report" \
  "memory formation receipt hepta-core recall helper"
assert_file_contains "$hepta_core_memory_tests" \
  "context_recall_memory_formation_receipts_are_payload_light_and_non_writing" \
  "memory formation receipt hepta-core no-payload test"

assert_file_contains "$hepta_memory" \
  "mod recall_helpers;" \
  "memory formation receipt hepta-memory recall helper module"
assert_file_contains "$hepta_memory_recall_helpers" \
  "recall_context_memory_formation_receipt_report" \
  "memory formation receipt hepta-memory helper"
assert_file_contains "$hepta_memory_tests" \
  "store_snapshot_recall_context_memory_formation_receipts_are_payload_light" \
  "memory formation receipt hepta-memory snapshot test"
assert_file_contains "$hepta_memory_tests" \
  "store_recall_context_memory_formation_receipts_match_snapshot_helper" \
  "memory formation receipt hepta-memory store test"

assert_file_contains "$response_debug_memory" \
  "latest_manifest_memory_formation_receipt_count" \
  "memory formation receipt response-debug summary"
assert_file_contains "$response_debug_memory" \
  "manifest_memory_formation_receipts_invalid" \
  "memory formation receipt response-debug audit"
assert_file_contains "$response_debug_tests" \
  "rollout_context_debug_summary_surfaces_memory_formation_receipts_without_payloads" \
  "memory formation receipt response-debug no-payload test"

cargo test --manifest-path "$manifest" -p codex-protocol \
  turn_context_manifest_memory_formation_receipts_are_payload_light_and_hashed \
  --lib --message-format=short

cargo test --manifest-path "$manifest" -p hepta-core \
  context_recall_memory_formation_receipts_are_payload_light_and_non_writing \
  --test memory_recall_contracts --message-format=short

cargo test --manifest-path "$manifest" -p hepta-memory \
  memory_formation_receipts \
  --lib --message-format=short

cargo test --manifest-path "$manifest" -p codex-core \
  memory_formation_receipts \
  --lib --message-format=short

cargo test --manifest-path "$manifest" -p codex-response-debug-context \
  memory_formation_receipts \
  --lib --message-format=short

echo "context-memory-formation-receipts=pass"
echo "context-memory-formation-receipts.payload-light=pass"
echo "context-memory-formation-receipts.production-write=disabled"
echo "context-memory-formation-receipts.runtime-activation=disabled"
echo "Hepta context memory formation receipt gate passed"
