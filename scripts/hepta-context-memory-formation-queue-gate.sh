#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
manifest="$repo_root/codex-rs/Cargo.toml"
hepta_core_memory="$repo_root/codex-rs/hepta-core/src/memory.rs"
hepta_core_memory_formation="$repo_root/codex-rs/hepta-core/src/memory/formation.rs"
hepta_core_memory_recall_inspection="$repo_root/codex-rs/hepta-core/src/memory/recall/inspection.rs"
hepta_core_memory_tests="$repo_root/codex-rs/hepta-core/src/memory/tests/recall_memory/formation.rs"
hepta_memory="$repo_root/codex-rs/hepta-memory/src/lib.rs"
hepta_memory_tests="$repo_root/codex-rs/hepta-memory/src/tests/recall_memory/formation.rs"
hepta_memory_recall_helpers="$repo_root/codex-rs/hepta-memory/src/recall_helpers/snapshot.rs"
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
  echo "hepta-context-memory-formation-queue-gate: $*" >&2
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

for term in \
  "Memory formation queue dry-run report" \
  "memory_formation_queue" \
  "source_receipt_hash" \
  "revocation_key_hash" \
  "operator_review_required" \
  "retention_ttl_turns" \
  "dry_run_only=true" \
  "idempotency_enforced=true" \
  "can_revoke_before_commit=true" \
  "production_write=false" \
  "graph_write=false" \
  "hot_path_write=false" \
  "must not write production memory" \
  "must not write graph facts" \
  "must not alter prompt assembly" \
  "hepta-context-memory-formation-queue-gate.sh" \
  "runtime-activation=disabled"; do
  assert_file_contains "$contracts" "$term" "memory formation queue contract"
done

assert_file_contains "$hepta_core_memory" \
  "CONTEXT_MEMORY_FORMATION_QUEUE_SCHEMA_VERSION" \
  "memory formation queue schema version"
assert_file_contains "$hepta_core_memory_formation" \
  "ContextMemoryFormationQueueOperatorPolicy" \
  "memory formation queue operator policy"
assert_file_contains "$hepta_core_memory_formation" \
  "ContextMemoryFormationQueueItem" \
  "memory formation queue item"
assert_file_contains "$hepta_core_memory_formation" \
  "ContextMemoryFormationQueueReport" \
  "memory formation queue report"
assert_file_contains "$hepta_core_memory_recall_inspection" \
  "memory_formation_queue_report" \
  "memory formation queue recall helper"
assert_file_contains "$hepta_core_memory_tests" \
  "context_recall_memory_formation_queue_is_payload_light_reversible_and_non_writing" \
  "memory formation queue hepta-core test"

assert_file_contains "$hepta_memory" \
  "mod recall_helpers;" \
  "memory formation queue hepta-memory recall helper module"
assert_file_contains "$hepta_memory_recall_helpers" \
  "recall_context_memory_formation_queue_report" \
  "memory formation queue hepta-memory helper"
assert_file_contains "$hepta_memory_tests" \
  "store_snapshot_recall_context_memory_formation_queue_is_payload_light" \
  "memory formation queue hepta-memory snapshot test"
assert_file_contains "$hepta_memory_tests" \
  "store_recall_context_memory_formation_queue_matches_snapshot_helper" \
  "memory formation queue hepta-memory store test"

assert_file_contains "$debug_gate" \
  "hepta-context-memory-formation-queue-gate.sh" \
  "memory formation queue debug gate"
assert_file_contains "$preflight_script" \
  "context memory formation queue dry-run gate" \
  "memory formation queue preflight stage"
assert_file_contains "$front_door_gate" \
  "memory_formation_queue_gate_script" \
  "memory formation queue front-door static check"
assert_file_contains "$release_manifest" \
  "scripts/hepta-context-memory-formation-queue-gate.sh" \
  "memory formation queue release manifest entry"

assert_line_before \
  "$debug_gate" \
  "hepta-context-memory-formation-receipt-gate.sh" \
  "hepta-context-memory-formation-queue-gate.sh" \
  "memory formation queue debug gate order"
assert_line_before \
  "$debug_gate" \
  "hepta-context-memory-formation-queue-gate.sh" \
  "hepta-context-memory-formation-candidate-no-leak-export-gate.sh" \
  "memory formation queue debug gate order"
assert_line_before \
  "$preflight_script" \
  "context memory formation receipt gate" \
  "context memory formation queue dry-run gate" \
  "memory formation queue preflight order"
assert_line_before \
  "$preflight_script" \
  "context memory formation queue dry-run gate" \
  "context memory formation candidate no-leak export gate" \
  "memory formation queue preflight order"

cargo test --manifest-path "$manifest" -p hepta-core \
  memory_formation_queue \
  --lib --message-format=short

cargo test --manifest-path "$manifest" -p hepta-memory \
  memory_formation_queue \
  --lib --message-format=short

echo "context-memory-formation-queue=pass"
echo "context-memory-formation-queue.payload-light=pass"
echo "context-memory-formation-queue.operator-review=required"
echo "context-memory-formation-queue.idempotency=enabled"
echo "context-memory-formation-queue.revocation-before-commit=enabled"
echo "context-memory-formation-queue.production-write=disabled"
echo "context-memory-formation-queue.graph-write=disabled"
echo "context-memory-formation-queue.hot-path-write=disabled"
echo "context-memory-formation-queue.runtime-activation=disabled"
echo "Hepta context memory formation queue gate passed"
