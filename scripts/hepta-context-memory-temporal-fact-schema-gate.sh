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
hepta_core_memory_recall_inspection="$repo_root/codex-rs/hepta-core/src/memory/recall/inspection.rs"
hepta_core_memory_temporal="$repo_root/codex-rs/hepta-core/src/memory/temporal.rs"
hepta_core_memory_temporal_fact="$repo_root/codex-rs/hepta-core/src/memory/temporal/fact.rs"
hepta_core_memory_tests="$repo_root/codex-rs/hepta-core/tests/memory_recall_contracts/temporal.rs"
hepta_memory="$repo_root/codex-rs/hepta-memory/src/lib.rs"
hepta_memory_tests="$repo_root/codex-rs/hepta-memory/src/tests/recall_memory/temporal.rs"
hepta_memory_recall_helpers="$repo_root/codex-rs/hepta-memory/src/recall_helpers/snapshot.rs"
response_debug="$repo_root/codex-rs/response-debug-context/src/lib.rs"
response_debug_memory="$repo_root/codex-rs/response-debug-context/src/rollout_context/memory.rs"
response_debug_tests="$repo_root/codex-rs/response-debug-context/src/tests.rs"
response_debug_export_gate="$repo_root/scripts/hepta-context-response-debug-export-gate.sh"
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
  echo "hepta-context-memory-temporal-fact-schema-gate: $*" >&2
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
  "Memory temporal fact schema dry-run" \
  "memory_temporal_facts" \
  "attribute" \
  "preference" \
  "task_state" \
  "decision" \
  "summary" \
  "entity_hash" \
  "valid_from_sequence" \
  "invalid_at_sequence" \
  "supersedes_fact_hash" \
  "dry_run_only=true" \
  "production_write=false" \
  "must not write graph facts" \
  "must not write production memory" \
  "must not promote dry-run facts into durable memory" \
  "must not alter prompt assembly" \
  "manifest_memory_temporal_facts_invalid" \
  "hepta-context-memory-temporal-fact-schema-gate.sh" \
  "runtime-activation=disabled"; do
  assert_file_contains "$contracts" "$term" "memory temporal fact contract"
done

assert_file_contains "$protocol_common" \
  "TURN_CONTEXT_MEMORY_TEMPORAL_FACT_SCHEMA_VERSION" \
  "memory temporal fact protocol schema version"
assert_file_contains "$protocol_memory" \
  "TurnContextMemoryTemporalFactType" \
  "memory temporal fact protocol type"
assert_file_contains "$protocol_memory" \
  "TurnContextMemoryTemporalFact" \
  "memory temporal fact protocol struct"
assert_file_contains "$protocol_manifest" \
  "memory_temporal_facts_have_integrity" \
  "memory temporal fact protocol integrity"
assert_file_contains "$protocol_stable_hash" \
  "update_memory_temporal_facts" \
  "memory temporal fact protocol ledger hash"
assert_unique_protocol_owner \
  '^pub const TURN_CONTEXT_MEMORY_TEMPORAL_FACT_SCHEMA_VERSION:' \
  "$protocol_common" \
  "memory temporal fact protocol schema version"
assert_unique_protocol_owner \
  '^pub enum TurnContextMemoryTemporalFactType ' \
  "$protocol_memory" \
  "memory temporal fact protocol type"
assert_unique_protocol_owner \
  '^pub struct TurnContextMemoryTemporalFact ' \
  "$protocol_memory" \
  "memory temporal fact protocol struct"
assert_unique_protocol_owner \
  '^    pub fn memory_temporal_facts_have_integrity\(' \
  "$protocol_manifest" \
  "memory temporal fact protocol integrity"
assert_unique_protocol_owner \
  '^    pub\(super\) fn update_memory_temporal_facts\(' \
  "$protocol_stable_hash" \
  "memory temporal fact protocol ledger hash"

assert_file_contains "$context_manifest_options" \
  "Vec<TurnContextMemoryTemporalFact>" \
  "memory temporal fact core manifest options"
assert_file_contains "$context_manifest_options" \
  "memory_temporal_facts: extension_data" \
  "memory temporal fact extension-data read"
assert_file_contains "$context_manifest_tests" \
  "turn_context_manifest_resolves_memory_temporal_facts_without_payload_text" \
  "memory temporal fact core no-payload test"

assert_file_contains "$hepta_core_memory_temporal" \
  "mod fact;" \
  "memory temporal fact leaf module"
assert_file_contains "$hepta_core_memory_temporal" \
  "pub use fact::ContextMemoryTemporalFactReport" \
  "memory temporal fact wrapper re-export"
assert_file_contains "$hepta_core_memory_temporal_fact" \
  "ContextMemoryTemporalFactReport" \
  "memory temporal fact hepta-core report"
assert_file_contains "$hepta_core_memory_recall_inspection" \
  "memory_temporal_fact_report" \
  "memory temporal fact hepta-core inspection helper"
assert_file_contains "$hepta_core_memory_tests" \
  "context_recall_memory_temporal_facts_are_payload_light_and_non_writing" \
  "memory temporal fact hepta-core no-payload test"

assert_file_contains "$hepta_memory" \
  "mod recall_helpers;" \
  "memory temporal fact hepta-memory recall helper module"
assert_file_contains "$hepta_memory_recall_helpers" \
  "recall_context_memory_temporal_fact_report" \
  "memory temporal fact hepta-memory helper"
assert_file_contains "$hepta_memory_tests" \
  "store_snapshot_recall_context_memory_temporal_facts_are_payload_light" \
  "memory temporal fact hepta-memory snapshot test"
assert_file_contains "$hepta_memory_tests" \
  "store_recall_context_memory_temporal_facts_match_snapshot_helper" \
  "memory temporal fact hepta-memory store test"

assert_file_contains "$response_debug_memory" \
  "latest_manifest_memory_temporal_fact_count" \
  "memory temporal fact response-debug summary"
assert_file_contains "$response_debug_memory" \
  "manifest_memory_temporal_facts_invalid" \
  "memory temporal fact response-debug audit"
assert_file_contains "$response_debug_tests" \
  "rollout_context_debug_summary_surfaces_memory_temporal_facts_without_payloads" \
  "memory temporal fact response-debug no-payload test"
assert_file_contains "$response_debug_export_gate" \
  "memory-temporal-fact-good.jsonl" \
  "memory temporal fact response-debug CLI fixture"

cargo test --manifest-path "$manifest" -p codex-protocol \
  memory_temporal \
  --lib --message-format=short

cargo test --manifest-path "$manifest" -p hepta-core \
  memory_temporal \
  --test memory_recall_contracts --message-format=short

cargo test --manifest-path "$manifest" -p hepta-memory \
  memory_temporal \
  --lib --message-format=short

cargo test --manifest-path "$manifest" -p codex-core \
  memory_temporal \
  --lib --message-format=short

cargo test --manifest-path "$manifest" -p codex-response-debug-context \
  memory_temporal \
  --lib --message-format=short

echo "context-memory-temporal-fact-schema=pass"
echo "context-memory-temporal-fact-schema.payload-light=pass"
echo "context-memory-temporal-fact-schema.production-write=disabled"
echo "context-memory-temporal-fact-schema.runtime-activation=disabled"
echo "Hepta context memory temporal fact schema gate passed"
