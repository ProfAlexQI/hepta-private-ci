#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
manifest="$repo_root/codex-rs/Cargo.toml"
hepta_core_memory="$repo_root/codex-rs/hepta-core/src/memory.rs"
hepta_core_memory_context_plane="$repo_root/codex-rs/hepta-core/src/memory/context_plane.rs"
hepta_core_memory_context_plane_operator="$repo_root/codex-rs/hepta-core/src/memory/context_plane/operator.rs"
hepta_core_memory_tests="$repo_root/codex-rs/hepta-core/src/memory/tests/context_plane_operator_packet.rs"
hepta_memory="$repo_root/codex-rs/hepta-memory/src/lib.rs"
hepta_memory_tests="$repo_root/codex-rs/hepta-memory/src/tests/context_plane/operator_packet.rs"
hepta_memory_context_plane_helpers="$repo_root/codex-rs/hepta-memory/src/context_plane_helpers.rs"
contracts="$repo_root/codex-rs/CONTEXT_DEBUG_CONTRACTS.md"
debug_gate="$repo_root/scripts/hepta-context-debug-gate.sh"
preflight_script="$repo_root/scripts/hepta-context-preflight.sh"
front_door_gate="$repo_root/scripts/hepta-context-source-aware-compression-front-door-gate.sh"
approval_report="$repo_root/scripts/hepta-context-plane-operator-approval-packet-report.sh"
lane="${HEPTA_CARGO_LANE:-${HEPTA_LANE:-hepta-context}}"
target_root="${HEPTA_CARGO_TARGET_ROOT:-$HOME/.openclaw/tmp/cargo-targets}"
target_leaf="$lane"
if [[ "$target_leaf" != hepta-* ]]; then
  target_leaf="hepta-$target_leaf"
fi
export CARGO_TARGET_DIR="${HEPTA_CARGO_TARGET_DIR:-$target_root/$target_leaf}"
export CARGO_INCREMENTAL="${CARGO_INCREMENTAL:-0}"

fail() {
  echo "hepta-context-plane-operator-approval-packet-gate: $*" >&2
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
  "Context Plane operator approval packet dry-run" \
  "context-plane-operator-approval-packet=pass" \
  "approval_required" \
  "dry_run_only" \
  "activation_command_present" \
  "matrix row counts" \
  "blocker reason counts" \
  "recall_quality_blocking_reason_count" \
  "recall_quality_blocking_reason_counts" \
  "context-plane-operator-approval-packet.recall-quality-blocking-reason-count=0" \
  "context-plane-operator-approval-packet.recall-quality-blocking-reasons=none" \
  "threshold snapshot" \
  "memory_formation_queue" \
  "memory_temporal_fact_graph" \
  "memory_temporal_graph_shadow_eval" \
  "recall_quality_gate" \
  "memory_provider_v2_boundary" \
  "memory_provider_v2_lifecycle_pass_count" \
  "memory_provider_v2_propose_write_check_pass" \
  "memory_provider_v2_close_check_pass" \
  "memory_shadow_canary_promotion_readiness" \
  "canary_promotion_checklist_pass_count" \
  "canary_promotion_negative_rehearsal_check_pass" \
  "canary_promotion_audit_digest_check_pass" \
  "canary_promotion_audit_freshness_check_pass" \
  "canary_promotion_rollback_rehearsal_pass_count" \
  "canary_promotion_kill_switch_rehearsal_pass_count" \
  "canary_promotion_soak_readback_pass_count" \
  "required approval scopes" \
  "adaptive_budget_allocation_runtime" \
  "source_aware_runtime_activation" \
  "production_memory_write" \
  "graph_write" \
  "prompt_assembly_change" \
  "operator_activation" \
  "adaptive_budget_allocation_shadow_only" \
  "temporal_graph_shadow_eval_shadow_only" \
  "memory_provider_boundary_shadow_only" \
  "memory_provider_v2_boundary_shadow_only" \
  "memory_shadow_canary_readiness_shadow_only" \
  "memory_shadow_canary_promotion_readiness_shadow_only" \
  "source_aware_front_door_disabled" \
  "operator_approval_missing" \
  "must not contain prompt text" \
  "must not contain transcript text" \
  "must not contain memory text" \
  "must not contain answer text" \
  "must not include activation commands" \
  "no production memory writes" \
  "no graph writes" \
  "no runtime activation" \
  "no adaptive allocator runtime activation" \
  "no source-aware runtime activation" \
  "no prompt assembly changes" \
  "no operator activation allowance" \
  "hepta-context-plane-operator-approval-packet-report.sh" \
  "hepta-context-plane-operator-approval-packet-gate.sh" \
  "runtime-activation=disabled"; do
  assert_file_contains "$contracts" "$term" "context plane operator approval packet contract"
done

assert_file_contains "$hepta_core_memory" \
  "CONTEXT_PLANE_OPERATOR_APPROVAL_PACKET_SCHEMA_VERSION" \
  "context plane operator approval packet schema version"
assert_file_contains "$hepta_core_memory_context_plane" \
  "mod operator" \
  "context plane operator approval module declaration"
assert_file_contains "$hepta_core_memory_context_plane" \
  "pub use operator::" \
  "context plane operator approval re-export"
assert_file_contains "$hepta_core_memory_context_plane_operator" \
  "ContextPlaneOperatorApprovalScope" \
  "context plane operator approval scope enum"
assert_file_contains "$hepta_core_memory_context_plane_operator" \
  "ContextPlaneOperatorApprovalThresholdSnapshot" \
  "context plane operator approval threshold snapshot"
assert_file_contains "$hepta_core_memory_context_plane_operator" \
  "ContextPlaneOperatorApprovalPacket" \
  "context plane operator approval packet"
assert_file_contains "$hepta_core_memory_context_plane_operator" \
  "ContextPlaneOperatorApprovalRecallQualityBlockerReasonCount" \
  "context plane operator approval recall-quality blocker reason count"
assert_file_contains "$hepta_core_memory_context_plane_operator" \
  "pub fn from_matrix(matrix: &ContextPlaneActivationBlockerMatrix) -> Self" \
  "context plane operator approval packet constructor"
assert_file_contains "$hepta_core_memory_tests" \
  "context_plane_operator_approval_packet_is_payload_light_dry_run" \
  "context plane operator approval hepta-core test"
assert_file_contains "$hepta_core_memory_tests" \
  "context_plane_operator_approval_packet_rolls_up_recall_quality_blockers_without_payloads" \
  "context plane operator approval recall-quality no-payload rollup hepta-core test"
assert_file_contains "$hepta_core_memory_tests" \
  "context_plane_operator_approval_packet_rejects_canary_promotion_checklist_false_green" \
  "context plane operator approval canary promotion checklist false-green test"

assert_file_contains "$hepta_memory" \
  "mod context_plane_helpers" \
  "context plane operator approval hepta-memory helper module"
assert_file_contains "$hepta_memory_context_plane_helpers" \
  "context_plane_operator_approval_packet" \
  "context plane operator approval hepta-memory helper"
assert_file_contains "$hepta_memory_tests" \
  "store_snapshot_context_plane_operator_approval_packet_is_payload_light" \
  "context plane operator approval hepta-memory snapshot test"
assert_file_contains "$hepta_memory_tests" \
  "store_context_plane_operator_approval_packet_matches_snapshot_helper" \
  "context plane operator approval hepta-memory store test"

assert_file_contains "$debug_gate" \
  "hepta-context-plane-operator-approval-packet-gate.sh" \
  "context plane operator approval packet debug gate"
assert_file_contains "$preflight_script" \
  "context plane operator approval packet dry-run gate" \
  "context plane operator approval packet preflight stage"
assert_file_contains "$front_door_gate" \
  "context_plane_operator_approval_packet_gate_script" \
  "context plane operator approval packet front-door static check"

assert_line_before \
  "$debug_gate" \
  "hepta-context-plane-activation-blocker-matrix-gate.sh" \
  "hepta-context-plane-operator-approval-packet-gate.sh" \
  "context plane operator approval debug gate order"
assert_line_before \
  "$debug_gate" \
  "hepta-context-plane-operator-approval-packet-gate.sh" \
  "hepta-context-source-aware-compression-front-door-report.sh" \
  "context plane operator approval debug front-door order"
assert_line_before \
  "$preflight_script" \
  "context plane activation blocker matrix gate" \
  "context plane operator approval packet dry-run gate" \
  "context plane operator approval preflight order"
assert_line_before \
  "$preflight_script" \
  "context plane operator approval packet dry-run gate" \
  "source-aware compression front-door machine-readable report" \
  "context plane operator approval front-door preflight order"

expected_status="$(cat <<'STATUS'
context-plane-operator-approval-packet=pass
context-plane-operator-approval-packet.schema=6
context-plane-operator-approval-packet.dry-run=enabled
context-plane-operator-approval-packet.approval-required=enabled
context-plane-operator-approval-packet.activation-command=absent
context-plane-operator-approval-packet.rows=17
context-plane-operator-approval-packet.satisfied=9
context-plane-operator-approval-packet.blockers=8
context-plane-operator-approval-packet.threshold.required-ready=16
context-plane-operator-approval-packet.threshold.required-shadow=1
context-plane-operator-approval-packet.blocker.adaptive-budget-allocation-shadow-only=1
context-plane-operator-approval-packet.blocker.temporal-graph-shadow-eval-shadow-only=1
context-plane-operator-approval-packet.blocker.memory-provider-boundary-shadow-only=1
context-plane-operator-approval-packet.blocker.memory-provider-v2-boundary-shadow-only=1
context-plane-operator-approval-packet.blocker.memory-shadow-canary-readiness-shadow-only=1
context-plane-operator-approval-packet.blocker.memory-shadow-canary-promotion-readiness-shadow-only=1
context-plane-operator-approval-packet.blocker.source-aware-front-door-disabled=1
context-plane-operator-approval-packet.blocker.operator-approval-missing=1
context-plane-operator-approval-packet.recall-quality-blocking-reason-count=0
context-plane-operator-approval-packet.recall-quality-blocking-reasons=none
context-plane-operator-approval-packet.canary-promotion.required-stable-window-count=1
context-plane-operator-approval-packet.canary-promotion.observed-stable-window-count=1
context-plane-operator-approval-packet.canary-promotion.required-pass-streak=3
context-plane-operator-approval-packet.canary-promotion.observed-pass-streak=3
context-plane-operator-approval-packet.canary-promotion.promotion-blocker-count=0
context-plane-operator-approval-packet.canary-promotion.checklist-required-count=4
context-plane-operator-approval-packet.canary-promotion.checklist-pass-count=4
context-plane-operator-approval-packet.canary-promotion.readiness-check=pass
context-plane-operator-approval-packet.canary-promotion.negative-rehearsal-check=pass
context-plane-operator-approval-packet.canary-promotion.audit-digest-check=pass
context-plane-operator-approval-packet.canary-promotion.audit-freshness-check=pass
context-plane-operator-approval-packet.canary-promotion.rollback-rehearsal-pass-count=3
context-plane-operator-approval-packet.canary-promotion.kill-switch-rehearsal-pass-count=3
context-plane-operator-approval-packet.canary-promotion.soak-readback-pass-count=3
context-plane-operator-approval-packet.memory-provider-v2.lifecycle-required-count=6
context-plane-operator-approval-packet.memory-provider-v2.lifecycle-pass-count=6
context-plane-operator-approval-packet.memory-provider-v2.query-check=pass
context-plane-operator-approval-packet.memory-provider-v2.update-context-check=pass
context-plane-operator-approval-packet.memory-provider-v2.propose-write-check=pass
context-plane-operator-approval-packet.memory-provider-v2.add-check=pass
context-plane-operator-approval-packet.memory-provider-v2.clear-check=pass
context-plane-operator-approval-packet.memory-provider-v2.close-check=pass
context-plane-operator-approval-packet.memory-provider-v2.candidate-count=1
context-plane-operator-approval-packet.memory-provider-v2.operator-review-required-count=1
context-plane-operator-approval-packet.required-scopes=6
context-plane-operator-approval-packet.scope.adaptive-budget-allocation-runtime=required
context-plane-operator-approval-packet.scope.source-aware-runtime-activation=required
context-plane-operator-approval-packet.scope.production-memory-write=required
context-plane-operator-approval-packet.scope.graph-write=required
context-plane-operator-approval-packet.scope.prompt-assembly-change=required
context-plane-operator-approval-packet.scope.operator-activation=required
context-plane-operator-approval-packet.runtime-activation=disabled
context-plane-operator-approval-packet.adaptive-allocator-runtime-activation=disabled
context-plane-operator-approval-packet.source-aware-runtime-activation=disabled
context-plane-operator-approval-packet.production-write=disabled
context-plane-operator-approval-packet.graph-write=disabled
context-plane-operator-approval-packet.prompt-assembly-change=disabled
context-plane-operator-approval-packet.operator-activation=disabled
STATUS
)"
actual_status="$(bash "$approval_report")"
if [ "$actual_status" != "$expected_status" ]; then
  fail "context plane operator approval packet report output changed"
fi

if printf '%s\n' "$actual_status" | grep -E 'prompt_text|transcript_text|memory_text|answer_text|source_id|session_id|memory_id|trace_id|query_text|tool_args|entity_hash|supersedes|idempotency|fixture_hash|activation-command=(run|enabled|present)|runtime-activation=enabled|production-write=enabled|graph-write=enabled' >/dev/null; then
  fail "context plane operator approval packet report leaked payload or enabled activation"
fi

cargo test --manifest-path "$manifest" -p hepta-core \
  context_plane_operator_approval \
  --lib --message-format=short

cargo test --manifest-path "$manifest" -p hepta-memory \
  context_plane_operator_approval \
  --lib --message-format=short

echo "context-plane-operator-approval-packet=pass"
echo "context-plane-operator-approval-packet.payload-light=pass"
echo "context-plane-operator-approval-packet.runtime-activation=disabled"
echo "Hepta context plane operator approval packet gate passed"
