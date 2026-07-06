#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
manifest="$repo_root/codex-rs/Cargo.toml"
protocol="$repo_root/codex-rs/protocol/src/protocol.rs"
context_manifest_options="$repo_root/codex-rs/core/src/context_manager/manifest/options.rs"
context_manifest_tests="$repo_root/codex-rs/core/src/context_manager/manifest/tests.rs"
hepta_core_memory="$repo_root/codex-rs/hepta-core/src/memory.rs"
hepta_core_memory_recall_inspection="$repo_root/codex-rs/hepta-core/src/memory/recall/inspection.rs"
hepta_core_memory_taxonomy="$repo_root/codex-rs/hepta-core/src/memory/taxonomy.rs"
hepta_core_memory_tests="$repo_root/codex-rs/hepta-core/src/memory/tests/recall_memory/taxonomy.rs"
hepta_memory="$repo_root/codex-rs/hepta-memory/src/lib.rs"
hepta_memory_tests="$repo_root/codex-rs/hepta-memory/src/tests/recall_memory/taxonomy.rs"
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
  echo "hepta-context-memory-taxonomy-report-gate: $*" >&2
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

for term in \
  "Memory taxonomy report" \
  "memory_taxonomy" \
  "semantic" \
  "episodic" \
  "control" \
  "transcript" \
  "class" \
  "source_count" \
  "returned_count" \
  "available_count" \
  "omitted_count" \
  "provenance_span_count" \
  "must not write production memory" \
  "runtime-activation=disabled" \
  "hepta-context-memory-taxonomy-report-gate.sh"; do
  assert_file_contains "$contracts" "$term" "memory taxonomy contract"
done

assert_file_contains "$protocol" \
  "TURN_CONTEXT_MEMORY_TAXONOMY_SCHEMA_VERSION" \
  "memory taxonomy protocol schema version"
assert_file_contains "$protocol" \
  "TurnContextMemoryTaxonomyClass" \
  "memory taxonomy protocol class"
assert_file_contains "$protocol" \
  "TurnContextMemoryTaxonomyBucket" \
  "memory taxonomy protocol bucket"
assert_file_contains "$protocol" \
  "memory_taxonomy_has_integrity" \
  "memory taxonomy protocol integrity"
assert_file_contains "$protocol" \
  "update_memory_taxonomy" \
  "memory taxonomy protocol ledger hash"

assert_file_contains "$context_manifest_options" \
  "Vec<TurnContextMemoryTaxonomyBucket>" \
  "memory taxonomy core manifest options"
assert_file_contains "$context_manifest_options" \
  "memory_taxonomy: extension_data" \
  "memory taxonomy extension-data read"
assert_file_contains "$context_manifest_tests" \
  "turn_context_manifest_resolves_memory_taxonomy_without_payload_text" \
  "memory taxonomy core carry-forward no-payload test"

assert_file_contains "$hepta_core_memory_taxonomy" \
  "ContextMemoryTaxonomyReport" \
  "memory taxonomy hepta-core report"
assert_file_contains "$hepta_core_memory_taxonomy" \
  "from_recall_inspection" \
  "memory taxonomy hepta-core recall mapping"
assert_file_contains "$hepta_core_memory_recall_inspection" \
  "memory_taxonomy_report" \
  "memory taxonomy hepta-core inspection helper"
assert_file_contains "$hepta_core_memory_tests" \
  "context_recall_memory_taxonomy_report_maps_recall_counts_without_payloads" \
  "memory taxonomy hepta-core no-payload test"

assert_file_contains "$hepta_memory" \
  "mod recall_helpers;" \
  "memory taxonomy hepta-memory recall helper module"
assert_file_contains "$hepta_memory_recall_helpers" \
  "memory_control_omitted_count" \
  "memory taxonomy control omission source"
assert_file_contains "$hepta_memory_recall_helpers" \
  "recall_context_memory_taxonomy_report" \
  "memory taxonomy hepta-memory helper"
assert_file_contains "$hepta_memory_tests" \
  "store_snapshot_recall_context_memory_taxonomy_maps_sources_without_payloads" \
  "memory taxonomy hepta-memory snapshot test"

assert_file_contains "$response_debug_memory" \
  "latest_manifest_memory_taxonomy_count" \
  "memory taxonomy response-debug summary"
assert_file_contains "$response_debug_memory" \
  "manifest_memory_taxonomy_invalid" \
  "memory taxonomy response-debug audit"
assert_file_contains "$response_debug_tests" \
  "rollout_context_debug_summary_surfaces_memory_taxonomy_without_payloads" \
  "memory taxonomy response-debug no-payload test"

cargo test --manifest-path "$manifest" -p codex-protocol \
  turn_context_manifest_memory_taxonomy_is_payload_light_and_hashed \
  --lib --message-format=short

cargo test --manifest-path "$manifest" -p hepta-core \
  context_recall_memory_taxonomy_report_maps_recall_counts_without_payloads \
  --lib --message-format=short

cargo test --manifest-path "$manifest" -p hepta-memory \
  recall_context_memory_taxonomy \
  --lib --message-format=short

cargo test --manifest-path "$manifest" -p codex-core \
  memory_taxonomy \
  --lib --message-format=short

cargo test --manifest-path "$manifest" -p codex-response-debug-context \
  memory_taxonomy \
  --lib --message-format=short

echo "context-memory-taxonomy=pass"
echo "context-memory-taxonomy.payload-light=pass"
echo "context-memory-taxonomy.runtime-activation=disabled"
echo "Hepta context memory taxonomy report gate passed"
