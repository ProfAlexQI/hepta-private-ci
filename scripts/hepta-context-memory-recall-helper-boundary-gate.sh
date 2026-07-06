#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
manifest="$repo_root/codex-rs/Cargo.toml"
hepta_memory="$repo_root/codex-rs/hepta-memory/src/lib.rs"
hepta_memory_recall_helpers="$repo_root/codex-rs/hepta-memory/src/recall_helpers.rs"
hepta_memory_recall_query_helpers="$repo_root/codex-rs/hepta-memory/src/recall_helpers/query.rs"
hepta_memory_recall_snapshot_helpers="$repo_root/codex-rs/hepta-memory/src/recall_helpers/snapshot.rs"
hepta_memory_recall_store_helpers="$repo_root/codex-rs/hepta-memory/src/recall_helpers/store.rs"
contracts="$repo_root/codex-rs/CONTEXT_DEBUG_CONTRACTS.md"
debug_gate="$repo_root/scripts/hepta-context-debug-gate.sh"
preflight_script="$repo_root/scripts/hepta-context-preflight.sh"
front_door_gate="$repo_root/scripts/hepta-context-source-aware-compression-front-door-gate.sh"
release_manifest="$repo_root/codex-rs/CONTEXT_LANE_RELEASE_MANIFEST.tsv"
lane="${HEPTA_CARGO_LANE:-${HEPTA_LANE:-hepta-context}}"
target_root="${HEPTA_CARGO_TARGET_ROOT:-$HOME/.openclaw/tmp/cargo-targets}"
target_leaf="$lane"
if [[ "$target_leaf" != hepta-* ]]; then
  target_leaf="hepta-$target_leaf"
fi
export CARGO_TARGET_DIR="${HEPTA_CARGO_TARGET_DIR:-$target_root/$target_leaf}"
export CARGO_INCREMENTAL="${CARGO_INCREMENTAL:-0}"

fail() {
  echo "hepta-context-memory-recall-helper-boundary-gate: $*" >&2
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
  "Hepta-memory recall helper boundary" \
  "codex-rs/hepta-memory/src/recall_helpers.rs" \
  "codex-rs/hepta-memory/src/recall_helpers/query.rs" \
  "codex-rs/hepta-memory/src/recall_helpers/snapshot.rs" \
  "codex-rs/hepta-memory/src/recall_helpers/store.rs" \
  "search_report" \
  "transcript_search_report" \
  "recall_context_parts" \
  "recall_context_report" \
  "recall_context_inspection" \
  "recall_context_coverage" \
  "recall_context_source_availability" \
  "memory_records_matching_recall_query" \
  "[hepta-memory:tombstone]" \
  "[hepta-memory:conflict]" \
  "must filter them before matched/availability counts" \
  "hepta-context-memory-recall-helper-boundary-gate.sh"; do
  assert_file_contains "$contracts" "$term" "memory recall helper boundary contract"
done

assert_file_contains "$hepta_memory" \
  "mod recall_helpers;" \
  "hepta-memory recall helper module declaration"
assert_file_contains "$hepta_memory_recall_helpers" \
  "mod query;" \
  "hepta-memory recall helper query submodule declaration"
assert_file_contains "$hepta_memory_recall_helpers" \
  "mod snapshot;" \
  "hepta-memory recall helper snapshot submodule declaration"
assert_file_contains "$hepta_memory_recall_helpers" \
  "mod store;" \
  "hepta-memory recall helper store submodule declaration"
assert_file_contains "$hepta_memory_recall_helpers" \
  "pub(crate) use query::memory_records_matching_recall_query;" \
  "hepta-memory recall helper control filtering re-export"
assert_file_not_contains "$hepta_memory_recall_helpers" \
  "impl StoreSnapshot" \
  "hepta-memory recall helper wrapper"
assert_file_not_contains "$hepta_memory_recall_helpers" \
  "impl InMemoryStore" \
  "hepta-memory recall helper wrapper"
assert_file_contains "$hepta_memory_recall_snapshot_helpers" \
  "impl StoreSnapshot" \
  "hepta-memory recall helper snapshot impl"
assert_file_contains "$hepta_memory_recall_store_helpers" \
  "impl InMemoryStore" \
  "hepta-memory recall helper store impl"
assert_file_contains "$hepta_memory_recall_snapshot_helpers" \
  "fn recall_context_parts" \
  "hepta-memory recall context parts helper"
assert_file_contains "$hepta_memory_recall_snapshot_helpers" \
  "pub fn recall_context_report" \
  "hepta-memory recall report helper"
assert_file_contains "$hepta_memory_recall_snapshot_helpers" \
  "pub fn recall_context_inspection" \
  "hepta-memory recall inspection helper"
assert_file_contains "$hepta_memory_recall_snapshot_helpers" \
  "pub fn recall_context_coverage" \
  "hepta-memory recall coverage helper"
assert_file_contains "$hepta_memory_recall_snapshot_helpers" \
  "pub fn recall_context_source_availability" \
  "hepta-memory recall source availability helper"
assert_file_contains "$hepta_memory_recall_query_helpers" \
  "pub(crate) fn memory_records_matching_recall_query" \
  "hepta-memory recall control filtering helper"
assert_file_contains "$hepta_memory_recall_query_helpers" \
  "pub(super) fn transcript_query_hits" \
  "hepta-memory recall transcript query helper"
assert_file_contains "$hepta_memory_recall_query_helpers" \
  "MEMORY_RECALL_TOMBSTONE_MARKER" \
  "hepta-memory recall tombstone marker"
assert_file_contains "$hepta_memory_recall_query_helpers" \
  "MEMORY_RECALL_CONFLICT_MARKER" \
  "hepta-memory recall conflict marker"
assert_file_contains "$hepta_memory" \
  "recall_helpers::memory_records_matching_recall_query" \
  "hepta-memory async search control filtering handoff"

assert_file_contains "$debug_gate" \
  "hepta-context-memory-recall-helper-boundary-gate.sh" \
  "memory recall helper boundary debug gate"
assert_file_contains "$preflight_script" \
  "context memory recall helper boundary gate" \
  "memory recall helper boundary preflight stage"
assert_file_contains "$front_door_gate" \
  "memory_recall_helper_boundary_gate_script" \
  "memory recall helper boundary front-door static check"
assert_file_contains "$release_manifest" \
  "scripts/hepta-context-memory-recall-helper-boundary-gate.sh" \
  "memory recall helper boundary release manifest entry"
for path in \
  "codex-rs/hepta-memory/src/recall_helpers/query.rs" \
  "codex-rs/hepta-memory/src/recall_helpers/snapshot.rs" \
  "codex-rs/hepta-memory/src/recall_helpers/store.rs"; do
  assert_file_contains "$release_manifest" "$path" \
    "memory recall helper submodule release manifest entry"
done

assert_line_before \
  "$debug_gate" \
  "hepta-context-adaptive-budget-allocation-report-gate.sh" \
  "hepta-context-memory-recall-helper-boundary-gate.sh" \
  "memory recall helper boundary debug gate order"
assert_line_before \
  "$debug_gate" \
  "hepta-context-memory-recall-helper-boundary-gate.sh" \
  "hepta-context-memory-taxonomy-report-gate.sh" \
  "memory recall helper boundary debug gate order"
assert_line_before \
  "$preflight_script" \
  "context adaptive budget allocation dry-run report gate" \
  "context memory recall helper boundary gate" \
  "memory recall helper boundary preflight order"
assert_line_before \
  "$preflight_script" \
  "context memory recall helper boundary gate" \
  "context memory taxonomy report gate" \
  "memory recall helper boundary preflight order"

cargo test --manifest-path "$manifest" -p hepta-memory \
  recall_context \
  --lib --message-format=short

cargo test --manifest-path "$manifest" -p hepta-memory \
  tombstone \
  --lib --message-format=short

echo "context-memory-recall-helper-boundary=pass"
echo "context-memory-recall-helper-boundary.submodules=3"
echo "context-memory-recall-helper-boundary.control-filtering=pass"
echo "context-memory-recall-helper-boundary.payload-light=pass"
echo "context-memory-recall-helper-boundary.runtime-activation=disabled"
echo "Hepta context memory recall helper boundary gate passed"
