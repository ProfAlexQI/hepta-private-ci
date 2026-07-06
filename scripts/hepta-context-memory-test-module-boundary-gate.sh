#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
manifest="$repo_root/codex-rs/Cargo.toml"
hepta_memory="$repo_root/codex-rs/hepta-memory/src/lib.rs"
hepta_memory_tests="$repo_root/codex-rs/hepta-memory/src/tests/mod.rs"
hepta_memory_tests_dir="$repo_root/codex-rs/hepta-memory/src/tests"
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
  echo "hepta-context-memory-test-module-boundary-gate: $*" >&2
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
  "Hepta-memory test module boundary" \
  "codex-rs/hepta-memory/src/tests/context_memory.rs" \
  "codex-rs/hepta-memory/src/tests/context_plane.rs" \
  "codex-rs/hepta-memory/src/tests/context_plane/activation_matrix.rs" \
  "codex-rs/hepta-memory/src/tests/context_plane/operator_packet.rs" \
  "codex-rs/hepta-memory/src/tests/context_plane/status.rs" \
  "codex-rs/hepta-memory/src/tests/mod.rs" \
  "codex-rs/hepta-memory/src/tests/recall_context_core.rs" \
  "codex-rs/hepta-memory/src/tests/recall_context_helpers.rs" \
  "codex-rs/hepta-memory/src/tests/recall_context_helpers/availability.rs" \
  "codex-rs/hepta-memory/src/tests/recall_context_helpers/bundle.rs" \
  "codex-rs/hepta-memory/src/tests/recall_context_helpers/coverage.rs" \
  "codex-rs/hepta-memory/src/tests/recall_context_helpers/limit_pressure.rs" \
  "codex-rs/hepta-memory/src/tests/recall_context_helpers/omission.rs" \
  "codex-rs/hepta-memory/src/tests/recall_context_helpers/provenance.rs" \
  "codex-rs/hepta-memory/src/tests/recall_context_quality.rs" \
  "codex-rs/hepta-memory/src/tests/recall_context_quality/availability.rs" \
  "codex-rs/hepta-memory/src/tests/recall_context_quality/coverage.rs" \
  "codex-rs/hepta-memory/src/tests/recall_context_quality/inspection.rs" \
  "codex-rs/hepta-memory/src/tests/recall_context_quality/limit_pressure.rs" \
  "codex-rs/hepta-memory/src/tests/recall_context_quality/omission.rs" \
  "codex-rs/hepta-memory/src/tests/recall_context_quality/provenance.rs" \
  "codex-rs/hepta-memory/src/tests/recall_memory.rs" \
  "codex-rs/hepta-memory/src/tests/recall_memory/formation.rs" \
  "codex-rs/hepta-memory/src/tests/recall_memory/taxonomy.rs" \
  "codex-rs/hepta-memory/src/tests/recall_memory/temporal.rs" \
  "codex-rs/hepta-memory/src/tests/restore_preview.rs" \
  "codex-rs/hepta-memory/src/tests/search.rs" \
  "codex-rs/hepta-memory/src/tests/snapshot_core.rs" \
  "codex-rs/hepta-memory/src/tests/snapshot_inspection.rs" \
  "codex-rs/hepta-memory/src/tests/snapshot_inspection/audit.rs" \
  "codex-rs/hepta-memory/src/tests/snapshot_inspection/drift.rs" \
  "codex-rs/hepta-memory/src/tests/snapshot_inspection/health.rs" \
  "codex-rs/hepta-memory/src/tests/snapshot_inspection/inspected.rs" \
  "codex-rs/hepta-memory/src/tests/snapshot_integrity.rs" \
  "codex-rs/hepta-memory/src/tests/snapshot_inventory.rs" \
  "codex-rs/hepta-memory/src/tests/snapshot_inventory/manifest.rs" \
  "codex-rs/hepta-memory/src/tests/snapshot_inventory/session_inventory.rs" \
  "codex-rs/hepta-memory/src/tests/snapshot_inventory/stats.rs" \
  "codex-rs/hepta-memory/src/tests/snapshot_restore.rs" \
  "codex-rs/hepta-memory/src/tests/snapshot_restore/impact.rs" \
  "codex-rs/hepta-memory/src/tests/snapshot_restore/inspected.rs" \
  "codex-rs/hepta-memory/src/tests/snapshot_restore/preview.rs" \
  "codex-rs/hepta-memory/src/tests/snapshot_restore/readiness.rs" \
  "codex-rs/hepta-memory/src/tests/snapshot_restore/roundtrip.rs" \
  "codex-rs/hepta-memory/src/tests/store.rs" \
  'inline `#[cfg(test)]`' \
  "store_snapshot" \
  "recall_context" \
  "context_plane" \
  "hepta-context-memory-test-module-boundary-gate.sh"; do
  assert_file_contains "$contracts" "$term" "memory test module boundary contract"
done

assert_file_contains "$hepta_memory" \
  "mod tests;" \
  "hepta-memory external test module declaration"
assert_file_not_contains "$hepta_memory" \
  "mod tests {" \
  "hepta-memory inline test module body"
assert_file_contains "$hepta_memory_tests" \
  "use super::*;" \
  "hepta-memory test module crate import"
assert_file_contains "$hepta_memory_tests" \
  "fn session_record" \
  "hepta-memory shared test helper"
for module_name in \
  "context_memory" \
  "context_plane" \
  "recall_context_core" \
  "recall_context_helpers" \
  "recall_context_quality" \
  "recall_memory" \
  "restore_preview" \
  "search" \
  "snapshot_core" \
  "snapshot_inspection" \
  "snapshot_integrity" \
  "snapshot_inventory" \
  "snapshot_restore" \
  "store"; do
  assert_file_contains "$hepta_memory_tests" \
    "mod $module_name;" \
    "hepta-memory test module declaration"
  assert_file_contains "$hepta_memory_tests_dir/$module_name.rs" \
    "use super::*;" \
    "hepta-memory test submodule import"
  assert_file_contains "$release_manifest" \
    "codex-rs/hepta-memory/src/tests/$module_name.rs" \
    "memory test submodule release manifest rust entry"
done
for nested_module in \
  "context_plane/activation_matrix" \
  "context_plane/operator_packet" \
  "context_plane/status" \
  "recall_context_helpers/availability" \
  "recall_context_helpers/bundle" \
  "recall_context_helpers/coverage" \
  "recall_context_helpers/limit_pressure" \
  "recall_context_helpers/omission" \
  "recall_context_helpers/provenance" \
  "recall_context_quality/availability" \
  "recall_context_quality/coverage" \
  "recall_context_quality/inspection" \
  "recall_context_quality/limit_pressure" \
  "recall_context_quality/omission" \
  "recall_context_quality/provenance" \
  "recall_memory/formation" \
  "recall_memory/taxonomy" \
  "recall_memory/temporal" \
  "snapshot_inspection/audit" \
  "snapshot_inspection/drift" \
  "snapshot_inspection/health" \
  "snapshot_inspection/inspected" \
  "snapshot_inventory/manifest" \
  "snapshot_inventory/session_inventory" \
  "snapshot_inventory/stats" \
  "snapshot_restore/impact" \
  "snapshot_restore/inspected" \
  "snapshot_restore/preview" \
  "snapshot_restore/readiness" \
  "snapshot_restore/roundtrip"; do
  nested_file="$hepta_memory_tests_dir/$nested_module.rs"
  assert_file_contains "$nested_file" \
    "use super::*;" \
    "hepta-memory nested test submodule import"
  assert_file_contains "$release_manifest" \
    "codex-rs/hepta-memory/src/tests/$nested_module.rs" \
    "memory nested test submodule release manifest rust entry"
done
assert_file_contains "$hepta_memory_tests_dir/context_plane.rs" \
  "mod status;" \
  "hepta-memory context plane nested module declaration"
assert_file_contains "$hepta_memory_tests_dir/context_plane.rs" \
  "mod activation_matrix;" \
  "hepta-memory context plane nested module declaration"
assert_file_contains "$hepta_memory_tests_dir/context_plane.rs" \
  "mod operator_packet;" \
  "hepta-memory context plane nested module declaration"
assert_file_contains "$hepta_memory_tests_dir/recall_context_helpers.rs" \
  "mod bundle;" \
  "hepta-memory recall context helper nested module declaration"
assert_file_contains "$hepta_memory_tests_dir/recall_context_helpers.rs" \
  "mod availability;" \
  "hepta-memory recall context helper nested module declaration"
assert_file_contains "$hepta_memory_tests_dir/recall_context_helpers.rs" \
  "mod provenance;" \
  "hepta-memory recall context helper nested module declaration"
assert_file_contains "$hepta_memory_tests_dir/recall_context_helpers.rs" \
  "mod coverage;" \
  "hepta-memory recall context helper nested module declaration"
assert_file_contains "$hepta_memory_tests_dir/recall_context_helpers.rs" \
  "mod omission;" \
  "hepta-memory recall context helper nested module declaration"
assert_file_contains "$hepta_memory_tests_dir/recall_context_helpers.rs" \
  "mod limit_pressure;" \
  "hepta-memory recall context helper nested module declaration"
assert_file_contains "$hepta_memory_tests_dir/recall_context_quality.rs" \
  "mod inspection;" \
  "hepta-memory recall context quality nested module declaration"
assert_file_contains "$hepta_memory_tests_dir/recall_context_quality.rs" \
  "mod provenance;" \
  "hepta-memory recall context quality nested module declaration"
assert_file_contains "$hepta_memory_tests_dir/recall_context_quality.rs" \
  "mod coverage;" \
  "hepta-memory recall context quality nested module declaration"
assert_file_contains "$hepta_memory_tests_dir/recall_context_quality.rs" \
  "mod omission;" \
  "hepta-memory recall context quality nested module declaration"
assert_file_contains "$hepta_memory_tests_dir/recall_context_quality.rs" \
  "mod limit_pressure;" \
  "hepta-memory recall context quality nested module declaration"
assert_file_contains "$hepta_memory_tests_dir/recall_context_quality.rs" \
  "mod availability;" \
  "hepta-memory recall context quality nested module declaration"
assert_file_contains "$hepta_memory_tests_dir/recall_memory.rs" \
  "mod formation;" \
  "hepta-memory recall memory nested module declaration"
assert_file_contains "$hepta_memory_tests_dir/recall_memory.rs" \
  "mod taxonomy;" \
  "hepta-memory recall memory nested module declaration"
assert_file_contains "$hepta_memory_tests_dir/recall_memory.rs" \
  "mod temporal;" \
  "hepta-memory recall memory nested module declaration"
assert_file_contains "$hepta_memory_tests_dir/snapshot_inspection.rs" \
  "mod audit;" \
  "hepta-memory snapshot inspection nested module declaration"
assert_file_contains "$hepta_memory_tests_dir/snapshot_inspection.rs" \
  "mod drift;" \
  "hepta-memory snapshot inspection nested module declaration"
assert_file_contains "$hepta_memory_tests_dir/snapshot_inspection.rs" \
  "mod health;" \
  "hepta-memory snapshot inspection nested module declaration"
assert_file_contains "$hepta_memory_tests_dir/snapshot_inspection.rs" \
  "mod inspected;" \
  "hepta-memory snapshot inspection nested module declaration"
assert_file_contains "$hepta_memory_tests_dir/snapshot_inventory.rs" \
  "mod manifest;" \
  "hepta-memory snapshot inventory nested module declaration"
assert_file_contains "$hepta_memory_tests_dir/snapshot_inventory.rs" \
  "mod session_inventory;" \
  "hepta-memory snapshot inventory nested module declaration"
assert_file_contains "$hepta_memory_tests_dir/snapshot_inventory.rs" \
  "mod stats;" \
  "hepta-memory snapshot inventory nested module declaration"
assert_file_contains "$hepta_memory_tests_dir/snapshot_restore.rs" \
  "mod impact;" \
  "hepta-memory snapshot restore nested module declaration"
assert_file_contains "$hepta_memory_tests_dir/snapshot_restore.rs" \
  "mod inspected;" \
  "hepta-memory snapshot restore nested module declaration"
assert_file_contains "$hepta_memory_tests_dir/snapshot_restore.rs" \
  "mod preview;" \
  "hepta-memory snapshot restore nested module declaration"
assert_file_contains "$hepta_memory_tests_dir/snapshot_restore.rs" \
  "mod readiness;" \
  "hepta-memory snapshot restore nested module declaration"
assert_file_contains "$hepta_memory_tests_dir/snapshot_restore.rs" \
  "mod roundtrip;" \
  "hepta-memory snapshot restore nested module declaration"
assert_file_not_contains "$hepta_memory_tests" \
  "fn store_snapshot_roundtrips_through_json" \
  "hepta-memory test module wrapper snapshot body"
assert_file_not_contains "$hepta_memory_tests" \
  "fn store_snapshot_recall_context_uses_recent_window_query_hits_and_scope_split" \
  "hepta-memory test module wrapper recall body"
assert_file_not_contains "$hepta_memory_tests" \
  "fn store_snapshot_context_plane_status_report_is_payload_light" \
  "hepta-memory test module wrapper context plane body"
assert_file_contains "$hepta_memory_tests_dir/snapshot_core.rs" \
  "fn store_snapshot_roundtrips_through_json" \
  "hepta-memory snapshot tests"
assert_file_contains "$hepta_memory_tests_dir/recall_context_core.rs" \
  "fn store_snapshot_recall_context_uses_recent_window_query_hits_and_scope_split" \
  "hepta-memory recall tests"
assert_file_not_contains "$hepta_memory_tests_dir/recall_context_helpers.rs" \
  "fn store_recall_context_availability_matches_snapshot_helper" \
  "hepta-memory recall context helper wrapper body"
assert_file_contains "$hepta_memory_tests_dir/recall_context_helpers/bundle.rs" \
  "async fn store_recall_context_matches_snapshot_helper" \
  "hepta-memory recall context helper bundle tests"
assert_file_contains "$hepta_memory_tests_dir/recall_context_helpers/availability.rs" \
  "async fn store_recall_context_availability_matches_snapshot_helper" \
  "hepta-memory recall context helper availability tests"
assert_file_contains "$hepta_memory_tests_dir/recall_context_helpers/coverage.rs" \
  "async fn store_recall_context_coverage_matches_snapshot_helper" \
  "hepta-memory recall context helper coverage tests"
assert_file_contains "$hepta_memory_tests_dir/recall_context_helpers/omission.rs" \
  "async fn store_recall_context_omission_counts_match_snapshot_helper" \
  "hepta-memory recall context helper omission tests"
assert_file_contains "$hepta_memory_tests_dir/recall_context_helpers/limit_pressure.rs" \
  "async fn store_recall_context_limit_pressure_matches_snapshot_helper" \
  "hepta-memory recall context helper limit pressure tests"
assert_file_not_contains "$hepta_memory_tests_dir/recall_context_quality.rs" \
  "fn store_snapshot_recall_context_coverage_matches_inspection_helper" \
  "hepta-memory recall context quality wrapper body"
assert_file_contains "$hepta_memory_tests_dir/recall_context_quality/inspection.rs" \
  "fn store_snapshot_recall_context_inspection_tracks_availability_counts" \
  "hepta-memory recall context quality inspection tests"
assert_file_contains "$hepta_memory_tests_dir/recall_context_quality/coverage.rs" \
  "fn store_snapshot_recall_context_coverage_matches_inspection_helper" \
  "hepta-memory recall context quality coverage tests"
assert_file_contains "$hepta_memory_tests_dir/recall_context_quality/availability.rs" \
  "fn store_snapshot_recall_context_source_availability_preserves_memory_scope_split" \
  "hepta-memory recall context quality availability tests"
assert_file_contains "$hepta_memory_tests_dir/recall_context_quality/omission.rs" \
  "fn store_snapshot_recall_context_with_zero_limits_reports_full_omission_pressure" \
  "hepta-memory recall context quality omission tests"
assert_file_not_contains "$hepta_memory_tests_dir/context_plane.rs" \
  "fn store_snapshot_context_plane_status_report_is_payload_light" \
  "hepta-memory context plane wrapper body"
assert_file_contains "$hepta_memory_tests_dir/context_plane/status.rs" \
  "fn store_snapshot_context_plane_status_report_is_payload_light" \
  "hepta-memory context plane status tests"
assert_file_contains "$hepta_memory_tests_dir/context_plane/activation_matrix.rs" \
  "fn store_snapshot_context_plane_activation_blocker_matrix_is_payload_light" \
  "hepta-memory context plane activation matrix tests"
assert_file_contains "$hepta_memory_tests_dir/context_plane/operator_packet.rs" \
  "fn store_snapshot_context_plane_operator_approval_packet_is_payload_light" \
  "hepta-memory context plane operator packet tests"
assert_file_contains "$hepta_memory_tests_dir/context_memory.rs" \
  "fn store_snapshot_context_memory_eval_harness_seed_is_payload_light" \
  "hepta-memory context memory tests"
assert_file_not_contains "$hepta_memory_tests_dir/recall_memory.rs" \
  "fn store_snapshot_recall_context_memory_taxonomy_maps_sources_without_payloads" \
  "hepta-memory recall memory wrapper body"
assert_file_contains "$hepta_memory_tests_dir/recall_memory/taxonomy.rs" \
  "fn store_snapshot_recall_context_memory_taxonomy_maps_sources_without_payloads" \
  "hepta-memory recall memory taxonomy tests"
assert_file_contains "$hepta_memory_tests_dir/recall_memory/formation.rs" \
  "fn store_snapshot_recall_context_memory_formation_receipts_are_payload_light" \
  "hepta-memory recall memory formation tests"
assert_file_contains "$hepta_memory_tests_dir/recall_memory/temporal.rs" \
  "fn store_snapshot_recall_context_memory_temporal_fact_graph_is_payload_light" \
  "hepta-memory recall memory temporal tests"
assert_file_contains "$hepta_memory_tests_dir/search.rs" \
  "fn search_report_suppresses_tombstone_and_conflict_control_records" \
  "hepta-memory search tests"
assert_file_not_contains "$hepta_memory_tests_dir/snapshot_inspection.rs" \
  "fn store_snapshot_audit_report_matches_clean_snapshot" \
  "hepta-memory snapshot inspection wrapper body"
assert_file_contains "$hepta_memory_tests_dir/snapshot_inspection/audit.rs" \
  "fn store_snapshot_audit_report_matches_clean_snapshot" \
  "hepta-memory snapshot inspection audit tests"
assert_file_contains "$hepta_memory_tests_dir/snapshot_inspection/drift.rs" \
  "fn store_snapshot_inspection_drift_report_tracks_section_level_drift" \
  "hepta-memory snapshot inspection drift tests"
assert_file_contains "$hepta_memory_tests_dir/snapshot_inspection/health.rs" \
  "fn inspected_store_snapshot_inspection_health_uses_embedded_bundle" \
  "hepta-memory snapshot inspection health tests"
assert_file_contains "$hepta_memory_tests_dir/snapshot_inspection/inspected.rs" \
  "fn inspected_store_snapshot_matches_snapshot_helpers" \
  "hepta-memory inspected snapshot tests"
assert_file_not_contains "$hepta_memory_tests_dir/snapshot_inventory.rs" \
  "fn snapshot_manifest_tracks_sorted_records_and_sizes" \
  "hepta-memory snapshot inventory wrapper body"
assert_file_contains "$hepta_memory_tests_dir/snapshot_inventory/manifest.rs" \
  "fn snapshot_manifest_tracks_sorted_records_and_sizes" \
  "hepta-memory snapshot inventory manifest tests"
assert_file_contains "$hepta_memory_tests_dir/snapshot_inventory/session_inventory.rs" \
  "fn session_agent_inventory_summarizes_sessions_by_agent" \
  "hepta-memory snapshot inventory session tests"
assert_file_contains "$hepta_memory_tests_dir/snapshot_inventory/stats.rs" \
  "fn snapshot_stats_summarize_active_archived_and_memory_scope_counts" \
  "hepta-memory snapshot inventory stats tests"
assert_file_not_contains "$hepta_memory_tests_dir/snapshot_restore.rs" \
  "fn store_snapshot_restore_preview_matches_core_report" \
  "hepta-memory snapshot restore wrapper body"
assert_file_contains "$hepta_memory_tests_dir/snapshot_restore/preview.rs" \
  "fn store_snapshot_restore_preview_matches_core_report" \
  "hepta-memory snapshot restore tests"
assert_file_contains "$hepta_memory_tests_dir/snapshot_restore/impact.rs" \
  "fn store_snapshot_restore_impact_matches_preview_impact" \
  "hepta-memory snapshot restore impact tests"
assert_file_contains "$hepta_memory_tests_dir/snapshot_restore/inspected.rs" \
  "fn inspected_store_snapshot_restore_helpers_delegate_to_snapshot_payload" \
  "hepta-memory snapshot restore inspected tests"
assert_file_contains "$hepta_memory_tests_dir/snapshot_restore/readiness.rs" \
  "fn store_snapshot_restore_readiness_matches_preview_and_impact_helpers" \
  "hepta-memory snapshot restore readiness tests"
assert_file_contains "$hepta_memory_tests_dir/snapshot_restore/roundtrip.rs" \
  "fn snapshot_restore_roundtrip_recovers_sessions_and_memories" \
  "hepta-memory snapshot restore roundtrip tests"
assert_file_contains "$hepta_memory_tests_dir/restore_preview.rs" \
  "fn preview_restore_safety_matches_snapshot_helper" \
  "hepta-memory restore preview tests"

assert_file_contains "$debug_gate" \
  "hepta-context-memory-test-module-boundary-gate.sh" \
  "memory test module boundary debug gate"
assert_file_contains "$preflight_script" \
  "context memory test module boundary gate" \
  "memory test module boundary preflight stage"
assert_file_contains "$front_door_gate" \
  "memory_test_module_boundary_gate_script" \
  "memory test module boundary front-door static check"
assert_file_contains "$release_manifest" \
  "codex-rs/hepta-memory/src/tests/mod.rs" \
  "memory test module boundary release manifest rust entry"
assert_file_contains "$release_manifest" \
  "scripts/hepta-context-memory-test-module-boundary-gate.sh" \
  "memory test module boundary release manifest script entry"

assert_line_before \
  "$debug_gate" \
  "hepta-context-memory-snapshot-helper-boundary-gate.sh" \
  "hepta-context-memory-test-module-boundary-gate.sh" \
  "memory test module boundary debug gate order"
assert_line_before \
  "$debug_gate" \
  "hepta-context-memory-test-module-boundary-gate.sh" \
  "hepta-context-memory-recall-helper-boundary-gate.sh" \
  "memory test module boundary debug gate order"
assert_line_before \
  "$preflight_script" \
  "context memory snapshot helper boundary gate" \
  "context memory test module boundary gate" \
  "memory test module boundary preflight order"
assert_line_before \
  "$preflight_script" \
  "context memory test module boundary gate" \
  "context memory recall helper boundary gate" \
  "memory test module boundary preflight order"

cargo test --manifest-path "$manifest" -p hepta-memory \
  store_snapshot \
  --lib --message-format=short

cargo test --manifest-path "$manifest" -p hepta-memory \
  recall_context \
  --lib --message-format=short

cargo test --manifest-path "$manifest" -p hepta-memory \
  context_plane \
  --lib --message-format=short

echo "context-memory-test-module-boundary=pass"
echo "context-memory-test-module-boundary.inline-tests=externalized"
echo "context-memory-test-module-boundary.submodules=14"
echo "context-memory-test-module-boundary.nested-submodules=30"
echo "context-memory-test-module-boundary.snapshot=pass"
echo "context-memory-test-module-boundary.recall=pass"
echo "context-memory-test-module-boundary.context-plane=pass"
echo "context-memory-test-module-boundary.runtime-activation=disabled"
echo "Hepta context memory test module boundary gate passed"
