#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
manifest="$repo_root/codex-rs/Cargo.toml"
hepta_memory="$repo_root/codex-rs/hepta-memory/src/lib.rs"
hepta_memory_snapshot_helpers="$repo_root/codex-rs/hepta-memory/src/snapshot_helpers.rs"
hepta_memory_snapshot_impl_helpers="$repo_root/codex-rs/hepta-memory/src/snapshot_helpers/snapshot.rs"
hepta_memory_inspected_snapshot_helpers="$repo_root/codex-rs/hepta-memory/src/snapshot_helpers/inspected_snapshot.rs"
hepta_memory_snapshot_store_helpers="$repo_root/codex-rs/hepta-memory/src/snapshot_helpers/store.rs"
contracts="$repo_root/codex-rs/CONTEXT_DEBUG_CONTRACTS.md"
debug_gate="$repo_root/scripts/hepta-context-debug-gate.sh"
preflight_script="$repo_root/scripts/hepta-context-preflight.sh"
front_door_gate="$repo_root/scripts/lib/hepta-context-gates-v1/hepta-context-source-aware-compression-front-door.gate"
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
  echo "hepta-context-memory-snapshot-helper-boundary-gate: $*" >&2
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
  "Hepta-memory snapshot helper boundary" \
  "codex-rs/hepta-memory/src/snapshot_helpers.rs" \
  "codex-rs/hepta-memory/src/snapshot_helpers/inspected_snapshot.rs" \
  "codex-rs/hepta-memory/src/snapshot_helpers/snapshot.rs" \
  "codex-rs/hepta-memory/src/snapshot_helpers/store.rs" \
  "StoreSnapshot" \
  "InspectedStoreSnapshot" \
  "inspection_bundle" \
  "inspection_drift_report" \
  "inspection_health" \
  "restore_preview_against" \
  "restore_readiness_against" \
  "preview_restore" \
  "snapshot_inspection_bundle" \
  "hepta-context-memory-snapshot-helper-boundary-gate.sh"; do
  assert_file_contains "$contracts" "$term" "memory snapshot helper boundary contract"
done

assert_file_contains "$hepta_memory" \
  "mod snapshot_helpers;" \
  "hepta-memory snapshot helper module declaration"
assert_file_contains "$hepta_memory_snapshot_helpers" \
  "mod inspected_snapshot;" \
  "hepta-memory inspected snapshot helper submodule declaration"
assert_file_contains "$hepta_memory_snapshot_helpers" \
  "mod snapshot;" \
  "hepta-memory snapshot helper submodule declaration"
assert_file_contains "$hepta_memory_snapshot_helpers" \
  "mod store;" \
  "hepta-memory snapshot store helper submodule declaration"
assert_file_not_contains "$hepta_memory_snapshot_helpers" \
  "impl StoreSnapshot" \
  "hepta-memory snapshot helper wrapper"
assert_file_not_contains "$hepta_memory_snapshot_helpers" \
  "impl InMemoryStore" \
  "hepta-memory snapshot helper wrapper"
assert_file_contains "$hepta_memory_snapshot_impl_helpers" \
  "impl StoreSnapshot" \
  "hepta-memory snapshot helper snapshot impl"
assert_file_contains "$hepta_memory_inspected_snapshot_helpers" \
  "impl InspectedStoreSnapshot" \
  "hepta-memory inspected snapshot helper impl"
assert_file_contains "$hepta_memory_inspected_snapshot_helpers" \
  "impl<'de> Deserialize<'de> for InspectedStoreSnapshot" \
  "hepta-memory inspected snapshot serde impl"
assert_file_contains "$hepta_memory_snapshot_store_helpers" \
  "impl InMemoryStore" \
  "hepta-memory snapshot helper store impl"
assert_file_contains "$hepta_memory_snapshot_impl_helpers" \
  "pub fn inspection_bundle" \
  "hepta-memory snapshot inspection bundle helper"
assert_file_contains "$hepta_memory_snapshot_impl_helpers" \
  "pub fn restore_preview_against" \
  "hepta-memory snapshot restore preview helper"
assert_file_contains "$hepta_memory_snapshot_store_helpers" \
  "pub fn snapshot_inspection_bundle" \
  "hepta-memory store snapshot inspection helper"
assert_file_contains "$hepta_memory_snapshot_store_helpers" \
  "pub fn preview_restore" \
  "hepta-memory store restore preview helper"

assert_file_contains "$debug_gate" \
  "hepta-context-memory-snapshot-helper-boundary-gate.sh" \
  "memory snapshot helper boundary debug gate"
assert_file_contains "$preflight_script" \
  "context memory snapshot helper boundary gate" \
  "memory snapshot helper boundary preflight stage"
assert_file_contains "$front_door_gate" \
  "memory_snapshot_helper_boundary_gate_script" \
  "memory snapshot helper boundary front-door static check"
assert_file_contains "$release_manifest" \
  "codex-rs/hepta-memory/src/snapshot_helpers.rs" \
  "memory snapshot helper boundary release manifest rust entry"
assert_file_contains "$release_manifest" \
  "scripts/hepta-context-memory-snapshot-helper-boundary-gate.sh" \
  "memory snapshot helper boundary release manifest script entry"
for path in \
  "codex-rs/hepta-memory/src/snapshot_helpers/inspected_snapshot.rs" \
  "codex-rs/hepta-memory/src/snapshot_helpers/snapshot.rs" \
  "codex-rs/hepta-memory/src/snapshot_helpers/store.rs"; do
  assert_file_contains "$release_manifest" "$path" \
    "memory snapshot helper submodule release manifest entry"
done

assert_line_before \
  "$debug_gate" \
  "hepta-context-adaptive-budget-allocation-report-gate.sh" \
  "hepta-context-memory-snapshot-helper-boundary-gate.sh" \
  "memory snapshot helper boundary debug gate order"
assert_line_before \
  "$debug_gate" \
  "hepta-context-memory-snapshot-helper-boundary-gate.sh" \
  "hepta-context-memory-recall-helper-boundary-gate.sh" \
  "memory snapshot helper boundary debug gate order"
assert_line_before \
  "$preflight_script" \
  "context adaptive budget allocation dry-run report gate" \
  "context memory snapshot helper boundary gate" \
  "memory snapshot helper boundary preflight order"
assert_line_before \
  "$preflight_script" \
  "context memory snapshot helper boundary gate" \
  "context memory recall helper boundary gate" \
  "memory snapshot helper boundary preflight order"

cargo test --manifest-path "$manifest" -p hepta-memory \
  store_snapshot \
  --lib --message-format=short

cargo test --manifest-path "$manifest" -p hepta-memory \
  inspected_store_snapshot \
  --lib --message-format=short

cargo test --manifest-path "$manifest" -p hepta-memory \
  snapshot_inspection \
  --lib --message-format=short

cargo test --manifest-path "$manifest" -p hepta-memory \
  restore \
  --lib --message-format=short

echo "context-memory-snapshot-helper-boundary=pass"
echo "context-memory-snapshot-helper-boundary.submodules=3"
echo "context-memory-snapshot-helper-boundary.inspection=pass"
echo "context-memory-snapshot-helper-boundary.restore-preview=pass"
echo "context-memory-snapshot-helper-boundary.runtime-activation=disabled"
echo "Hepta context memory snapshot helper boundary gate passed"
