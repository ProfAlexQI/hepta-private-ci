#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
manifest="$repo_root/codex-rs/Cargo.toml"
hepta_core_memory="$repo_root/codex-rs/hepta-core/src/memory.rs"
hepta_core_memory_taxonomy="$repo_root/codex-rs/hepta-core/src/memory/taxonomy.rs"
hepta_core_memory_tests="$repo_root/codex-rs/hepta-core/src/memory/tests/recall_memory/taxonomy.rs"
hepta_memory_snapshot_helpers="$repo_root/codex-rs/hepta-memory/src/recall_helpers/snapshot.rs"
hepta_memory_store_helpers="$repo_root/codex-rs/hepta-memory/src/recall_helpers/store.rs"
hepta_memory_tests="$repo_root/codex-rs/hepta-memory/src/tests/recall_memory/taxonomy.rs"
contracts="$repo_root/codex-rs/CONTEXT_DEBUG_CONTRACTS.md"
debug_gate="$repo_root/scripts/hepta-context-debug-gate.sh"
preflight_script="$repo_root/scripts/hepta-context-preflight.sh"
front_door_gate="$repo_root/scripts/hepta-context-source-aware-compression-front-door-gate.sh"
release_manifest="$repo_root/codex-rs/CONTEXT_LANE_RELEASE_MANIFEST.tsv"
report_script="$repo_root/scripts/hepta-context-memory-namespace-policy-report.sh"
lane="${HEPTA_CARGO_LANE:-${HEPTA_LANE:-hepta-context}}"
target_root="${HEPTA_CARGO_TARGET_ROOT:-$HOME/.openclaw/tmp/cargo-targets}"
target_leaf="$lane"
if [[ "$target_leaf" != hepta-* ]]; then
  target_leaf="hepta-$target_leaf"
fi
export CARGO_TARGET_DIR="${HEPTA_CARGO_TARGET_DIR:-$target_root/$target_leaf}"
export CARGO_INCREMENTAL="${CARGO_INCREMENTAL:-0}"

report_output="$(mktemp -t hepta-context-memory-namespace-policy.XXXXXX)"
trap 'rm -f "$report_output"' EXIT

fail() {
  echo "hepta-context-memory-namespace-policy-gate: $*" >&2
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
  "Memory namespace policy shadow report" \
  "memory_namespace_policy" \
  "core" \
  "session" \
  "procedural" \
  "semantic" \
  "episodic" \
  "archival" \
  "owner" \
  "ttl_policy" \
  "ttl_turns" \
  "privacy_tier" \
  "redaction_policy" \
  "write_policy" \
  "propose_write_required" \
  "policy_approval_required" \
  "operator_approval_required" \
  "shadow_wal_required" \
  "readback_required" \
  "canary_required" \
  "supersede_supported" \
  "tombstone_supported" \
  "rollback_supported" \
  "production_write=false" \
  "graph_write=false" \
  "hot_path_write=false" \
  "must not write production memory" \
  "must not write graph facts" \
  "must not alter prompt assembly" \
  "hepta-context-memory-namespace-policy-report.sh" \
  "hepta-context-memory-namespace-policy-gate.sh" \
  "runtime-activation=disabled"; do
  assert_file_contains "$contracts" "$term" "memory namespace policy contract"
done

assert_file_contains "$hepta_core_memory" \
  "CONTEXT_MEMORY_NAMESPACE_POLICY_SCHEMA_VERSION" \
  "memory namespace policy schema version"
assert_file_contains "$hepta_core_memory_taxonomy" \
  "ContextMemoryNamespacePolicyBlock" \
  "memory namespace policy block"
assert_file_contains "$hepta_core_memory_taxonomy" \
  "ContextMemoryNamespacePolicyReport" \
  "memory namespace policy report"
assert_file_contains "$hepta_core_memory_taxonomy" \
  "ContextMemoryNamespace::REQUIRED" \
  "memory namespace policy required set"
assert_file_contains "$hepta_core_memory_taxonomy" \
  "ShadowProposalOnly" \
  "memory namespace policy write mode"
assert_file_contains "$hepta_core_memory_tests" \
  "context_memory_namespace_policy_report_defines_shadow_blocks_without_payloads" \
  "memory namespace policy hepta-core payload-light test"
assert_file_contains "$hepta_core_memory_tests" \
  "context_memory_namespace_policy_report_rejects_write_or_namespace_drift" \
  "memory namespace policy hepta-core negative test"

assert_file_contains "$hepta_memory_snapshot_helpers" \
  "context_memory_namespace_policy_report" \
  "memory namespace policy hepta-memory snapshot helper"
assert_file_contains "$hepta_memory_store_helpers" \
  "context_memory_namespace_policy_report" \
  "memory namespace policy hepta-memory store helper"
assert_file_contains "$hepta_memory_tests" \
  "store_snapshot_context_memory_namespace_policy_is_payload_light" \
  "memory namespace policy hepta-memory snapshot test"
assert_file_contains "$hepta_memory_tests" \
  "store_context_memory_namespace_policy_matches_snapshot_helper" \
  "memory namespace policy hepta-memory store test"

assert_file_contains "$debug_gate" \
  "hepta-context-memory-namespace-policy-gate.sh" \
  "memory namespace policy debug gate"
assert_file_contains "$preflight_script" \
  "context memory namespace policy shadow gate" \
  "memory namespace policy preflight stage"
assert_file_contains "$front_door_gate" \
  "memory_namespace_policy_gate_script" \
  "memory namespace policy front-door static check"
assert_file_contains "$release_manifest" \
  "scripts/hepta-context-memory-namespace-policy-gate.sh" \
  "memory namespace policy release manifest gate entry"
assert_file_contains "$release_manifest" \
  "scripts/hepta-context-memory-namespace-policy-report.sh" \
  "memory namespace policy release manifest report entry"
assert_file_contains "$report_script" \
  "context-memory-namespace-policy.namespace-count=6" \
  "memory namespace policy report namespace count"

assert_line_before \
  "$debug_gate" \
  "hepta-context-memory-formation-queue-gate.sh" \
  "hepta-context-memory-namespace-policy-gate.sh" \
  "memory namespace policy debug gate order"
assert_line_before \
  "$debug_gate" \
  "hepta-context-memory-namespace-policy-gate.sh" \
  "hepta-context-memory-formation-candidate-no-leak-export-gate.sh" \
  "memory namespace policy debug gate order"
assert_line_before \
  "$preflight_script" \
  "context memory formation queue dry-run gate" \
  "context memory namespace policy shadow gate" \
  "memory namespace policy preflight order"
assert_line_before \
  "$preflight_script" \
  "context memory namespace policy shadow gate" \
  "context memory formation candidate no-leak export gate" \
  "memory namespace policy preflight order"

cargo test --manifest-path "$manifest" -p hepta-core \
  context_memory_namespace_policy \
  --lib --message-format=short

cargo test --manifest-path "$manifest" -p hepta-memory \
  context_memory_namespace_policy \
  --lib --message-format=short

bash "$report_script" >"$report_output"

for line in \
  "context-memory-namespace-policy=pass" \
  "context-memory-namespace-policy.payload-light=pass" \
  "context-memory-namespace-policy.schema=1" \
  "context-memory-namespace-policy.namespace-count=6" \
  "context-memory-namespace-policy.namespace.core=shadow-policy" \
  "context-memory-namespace-policy.namespace.session=shadow-policy" \
  "context-memory-namespace-policy.namespace.procedural=shadow-policy" \
  "context-memory-namespace-policy.namespace.semantic=shadow-policy" \
  "context-memory-namespace-policy.namespace.episodic=shadow-policy" \
  "context-memory-namespace-policy.namespace.archival=shadow-policy" \
  "context-memory-namespace-policy.operator-approval-required-count=6" \
  "context-memory-namespace-policy.shadow-wal-required-count=6" \
  "context-memory-namespace-policy.readback-required-count=6" \
  "context-memory-namespace-policy.canary-required-count=6" \
  "context-memory-namespace-policy.supersede-supported-count=6" \
  "context-memory-namespace-policy.tombstone-supported-count=6" \
  "context-memory-namespace-policy.rollback-supported-count=6" \
  "context-memory-namespace-policy.production-write=disabled" \
  "context-memory-namespace-policy.graph-write=disabled" \
  "context-memory-namespace-policy.hot-path-write=disabled" \
  "context-memory-namespace-policy.prompt-assembly-change=disabled" \
  "context-memory-namespace-policy.runtime-activation=disabled"; do
  assert_file_contains "$report_output" "$line" "memory namespace policy report"
done

if rg -n 'candidate_text|transcript_text|memory_text|source_id|memory_id|query_text|raw_|entity_hash|fact_hash|edge_hash' "$report_output"; then
  fail "memory namespace policy report exposed payload-shaped fields"
fi

echo "context-memory-namespace-policy=pass"
echo "context-memory-namespace-policy.payload-light=pass"
echo "context-memory-namespace-policy.production-write=disabled"
echo "context-memory-namespace-policy.graph-write=disabled"
echo "context-memory-namespace-policy.runtime-activation=disabled"
echo "Hepta context memory namespace policy gate passed"
