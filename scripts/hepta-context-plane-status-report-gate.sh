#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
manifest="$repo_root/codex-rs/Cargo.toml"
hepta_core_memory="$repo_root/codex-rs/hepta-core/src/memory.rs"
hepta_core_memory_context_plane="$repo_root/codex-rs/hepta-core/src/memory/context_plane.rs"
hepta_core_memory_context_plane_status="$repo_root/codex-rs/hepta-core/src/memory/context_plane/status.rs"
hepta_core_memory_context_plane_status_entry="$repo_root/codex-rs/hepta-core/src/memory/context_plane/status/entry.rs"
hepta_core_memory_context_plane_status_report="$repo_root/codex-rs/hepta-core/src/memory/context_plane/status/report.rs"
hepta_core_memory_context_plane_status_section="$repo_root/codex-rs/hepta-core/src/memory/context_plane/status/section.rs"
hepta_core_memory_tests="$repo_root/codex-rs/hepta-core/src/memory/tests/context_plane_status.rs"
hepta_memory="$repo_root/codex-rs/hepta-memory/src/lib.rs"
hepta_memory_tests="$repo_root/codex-rs/hepta-memory/src/tests/context_plane/status.rs"
hepta_memory_context_plane_helpers="$repo_root/codex-rs/hepta-memory/src/context_plane_helpers.rs"
contracts="$repo_root/codex-rs/CONTEXT_DEBUG_CONTRACTS.md"
debug_gate="$repo_root/scripts/hepta-context-debug-gate.sh"
preflight_script="$repo_root/scripts/hepta-context-preflight.sh"
front_door_gate="$repo_root/scripts/hepta-context-source-aware-compression-front-door-gate.sh"
status_report="$repo_root/scripts/hepta-context-plane-status-report.sh"
lane="${HEPTA_CARGO_LANE:-${HEPTA_LANE:-hepta-context}}"
target_root="${HEPTA_CARGO_TARGET_ROOT:-$HOME/.openclaw/tmp/cargo-targets}"
target_leaf="$lane"
if [[ "$target_leaf" != hepta-* ]]; then
  target_leaf="hepta-$target_leaf"
fi
export CARGO_TARGET_DIR="${HEPTA_CARGO_TARGET_DIR:-$target_root/$target_leaf}"
export CARGO_INCREMENTAL="${CARGO_INCREMENTAL:-0}"

fail() {
  echo "hepta-context-plane-status-report-gate: $*" >&2
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
  "Context Plane status/export report" \
  "context-plane-status=pass" \
  "source_registry" \
  "adaptive_budget_allocation" \
  "memory_taxonomy" \
  "memory_formation_receipts" \
  "memory_formation_queue" \
  "memory_namespace_policy" \
  "memory_namespace_policy_namespace_count" \
  "memory_namespace_policy_shadow_wal_required_count" \
  "memory_namespace_policy_operator_approval_required_count" \
  "memory_temporal_facts" \
  "memory_temporal_fact_graph" \
  "memory_temporal_graph_shadow_eval" \
  "eval_harness_seed" \
  "adaptive_allocator_eval_shadow" \
  "recall_quality_gate" \
  "memory_ranked_recall_shadow_eval" \
  "ranked_recall_hybrid_signal_pass_count" \
  "ranked_recall_lexical_bm25_check_pass" \
  "ranked_recall_temporal_validity_check_pass" \
  "ranked_recall_positive_hybrid_signal_pass_count" \
  "ranked_recall_hybrid_regression_blocked_count" \
  "ranked_recall_routing_diff_shadow_only_count" \
  "ranked_recall_min_positive_routing_diff_delta_basis_points" \
  "ranked_recall_min_positive_routing_diff_token_tradeoff_basis_points" \
  "ranked_recall_real_workload_trace_slo_pass_count" \
  "ranked_recall_real_workload_trace_total_leak_count" \
  "ranked_recall_min_positive_real_workload_trace_coverage_basis_points" \
  "ranked_recall_real_workload_trace_operator_review_required_count" \
  "memory_provider_boundary" \
  "memory_provider_v2_boundary" \
  "memory_provider_v2_lifecycle_pass_count" \
  "memory_provider_v2_propose_write_check_pass" \
  "memory_provider_v2_close_check_pass" \
  "memory_shadow_canary_readiness" \
  "memory_shadow_canary_promotion_readiness" \
  "canary_promotion_checklist_pass_count" \
  "canary_promotion_negative_rehearsal_check_pass" \
  "canary_promotion_audit_digest_check_pass" \
  "canary_promotion_audit_freshness_check_pass" \
  "canary_promotion_rollback_rehearsal_pass_count" \
  "canary_promotion_kill_switch_rehearsal_pass_count" \
  "canary_promotion_soak_readback_pass_count" \
  "recall_quality_blocking_reason_count" \
  "recall_quality_blocking_reasons" \
  "source_aware_front_door" \
  "must not contain prompt text" \
  "must not contain transcript text" \
  "must not contain memory text" \
  "must not contain answer text" \
  "no production memory writes" \
  "no graph writes" \
  "no runtime activation" \
  "no adaptive allocator runtime activation" \
  "no source-aware runtime activation" \
  "no prompt assembly changes" \
  "no operator activation allowance" \
  "hepta-context-plane-status-report.sh" \
  "hepta-context-plane-status-report-gate.sh" \
  "context-plane-status.recall-quality-blocking-reason-count=0" \
  "context-plane-status.recall-quality-blocking-reasons=none" \
  "context-plane-status.ranked-recall.hybrid-signal-pass-count=5" \
  "context-plane-status.ranked-recall.positive-hybrid-signal-pass-count=15" \
  "context-plane-status.ranked-recall.hybrid-regression-blocked-count=1" \
  "context-plane-status.ranked-recall.routing-diff-shadow-only-count=4" \
  "context-plane-status.ranked-recall.min-positive-routing-diff-delta-basis-points=640" \
  "context-plane-status.ranked-recall.min-positive-routing-diff-token-tradeoff-basis-points=3000" \
  "context-plane-status.ranked-recall.real-workload-trace-slo-pass-count=3" \
  "context-plane-status.ranked-recall.real-workload-trace-total-leak-count=0" \
  "context-plane-status.ranked-recall.min-positive-real-workload-trace-coverage-basis-points=8000" \
  "context-plane-status.ranked-recall.canary-precondition-pass-count=4" \
  "context-plane-status.ranked-recall.canary-feature-flag-disabled-count=4" \
  "context-plane-status.ranked-recall.canary-precondition-route-opened-count=0" \
  "context-plane-status.memory-namespace-policy=shadow" \
  "context-plane-status.memory-namespace-policy.namespace-count=6" \
  "context-plane-status.memory-namespace-policy.shadow-wal-required-count=6" \
  "context-plane-status.memory-namespace-policy.production-write-count=0" \
  "runtime-activation=disabled"; do
  assert_file_contains "$contracts" "$term" "context plane status contract"
done

assert_file_contains "$hepta_core_memory" \
  "CONTEXT_PLANE_STATUS_SCHEMA_VERSION" \
  "context plane status schema version"
assert_file_contains "$hepta_core_memory_context_plane" \
  "mod status;" \
  "context plane status module declaration"
assert_file_contains "$hepta_core_memory_context_plane" \
  "pub use status::" \
  "context plane status re-export"
assert_file_contains "$hepta_core_memory_context_plane_status" \
  "mod section;" \
  "context plane status section leaf"
assert_file_contains "$hepta_core_memory_context_plane_status" \
  "mod entry;" \
  "context plane status entry leaf"
assert_file_contains "$hepta_core_memory_context_plane_status" \
  "mod report;" \
  "context plane status report leaf"
assert_file_contains "$hepta_core_memory_context_plane_status_section" \
  "ContextPlaneStatusSection" \
  "context plane status section enum"
assert_file_contains "$hepta_core_memory_context_plane_status_report" \
  "ContextPlaneStatusReport" \
  "context plane status report"
assert_file_contains "$hepta_core_memory_context_plane_status_report" \
  "pub fn from_reports(" \
  "context plane status report constructor"
assert_file_contains "$hepta_core_memory_tests" \
  "context_plane_status_report_unifies_readiness_without_payloads_or_activation" \
  "context plane status hepta-core test"
assert_file_contains "$hepta_core_memory_tests" \
  "context_plane_status_report_rolls_up_recall_quality_blockers_without_payloads" \
  "context plane status recall-quality blocker rollup test"
assert_file_contains "$hepta_core_memory_tests" \
  "context_plane_status_report_rejects_canary_promotion_checklist_false_green" \
  "context plane status canary promotion checklist false-green test"
assert_file_contains "$hepta_core_memory_context_plane_status_entry" \
  "recall_quality_blocking_reason_count" \
  "context plane status recall-quality blocker count"
assert_file_contains "$hepta_core_memory_context_plane_status_entry" \
  "recall_quality_blocking_reasons" \
  "context plane status recall-quality blocker reasons"

assert_file_contains "$hepta_memory" \
  "mod context_plane_helpers" \
  "context plane status hepta-memory helper module"
assert_file_contains "$hepta_memory_context_plane_helpers" \
  "context_plane_status_report" \
  "context plane status hepta-memory helper"
assert_file_contains "$hepta_memory_tests" \
  "store_snapshot_context_plane_status_report_is_payload_light" \
  "context plane status hepta-memory snapshot test"
assert_file_contains "$hepta_memory_tests" \
  "store_context_plane_status_report_matches_snapshot_helper" \
  "context plane status hepta-memory store test"

assert_file_contains "$debug_gate" \
  "hepta-context-plane-status-report-gate.sh" \
  "context plane status debug gate"
assert_file_contains "$preflight_script" \
  "context plane status/export report gate" \
  "context plane status preflight stage"
assert_file_contains "$front_door_gate" \
  "context_plane_status_report_gate_script" \
  "context plane status front-door static check"

assert_line_before \
  "$debug_gate" \
  "hepta-context-memory-adaptive-allocator-eval-shadow-gate.sh" \
  "hepta-context-plane-status-report-gate.sh" \
  "context plane status debug gate order"
assert_line_before \
  "$debug_gate" \
  "hepta-context-plane-status-report-gate.sh" \
  "hepta-context-source-aware-compression-front-door-report.sh" \
  "context plane status debug gate front-door order"
assert_line_before \
  "$preflight_script" \
  "context memory recall quality gate" \
  "context plane status/export report gate" \
  "context plane status preflight order"
assert_line_before \
  "$preflight_script" \
  "context plane status/export report gate" \
  "source-aware compression front-door machine-readable report" \
  "context plane status front-door preflight order"

expected_status="$(cat <<'STATUS'
context-plane-status=pass
context-plane-status.source-registry=ready
context-plane-status.adaptive-budget-allocation=shadow
context-plane-status.memory-taxonomy=ready
context-plane-status.memory-formation-receipts=ready
context-plane-status.memory-formation-queue=ready
context-plane-status.memory-namespace-policy=shadow
context-plane-status.memory-namespace-policy.namespace-count=6
context-plane-status.memory-namespace-policy.operator-approval-required-count=6
context-plane-status.memory-namespace-policy.shadow-wal-required-count=6
context-plane-status.memory-namespace-policy.readback-required-count=6
context-plane-status.memory-namespace-policy.canary-required-count=6
context-plane-status.memory-namespace-policy.rollback-supported-count=6
context-plane-status.memory-namespace-policy.production-write-count=0
context-plane-status.memory-namespace-policy.graph-write-count=0
context-plane-status.memory-temporal-facts=ready
context-plane-status.memory-temporal-fact-graph=ready
context-plane-status.memory-temporal-graph-shadow-eval=shadow
context-plane-status.eval-harness-seed=ready
context-plane-status.adaptive-allocator-eval-shadow=shadow
context-plane-status.recall-quality-gate=ready
context-plane-status.recall-quality-blocking-reason-count=0
context-plane-status.recall-quality-blocking-reasons=none
context-plane-status.memory-ranked-recall-shadow-eval=shadow
context-plane-status.ranked-recall.hybrid-signal-required-count=5
context-plane-status.ranked-recall.hybrid-signal-pass-count=5
context-plane-status.ranked-recall.lexical-bm25-check=pass
context-plane-status.ranked-recall.recency-check=pass
context-plane-status.ranked-recall.source-authority-check=pass
context-plane-status.ranked-recall.temporal-validity-check=pass
context-plane-status.ranked-recall.feedback-check=pass
context-plane-status.ranked-recall.positive-hybrid-signal-required-count=15
context-plane-status.ranked-recall.positive-hybrid-signal-pass-count=15
context-plane-status.ranked-recall.hybrid-regression-blocked-count=1
context-plane-status.ranked-recall.hybrid-signal-min-basis-points=6000
context-plane-status.ranked-recall.min-positive-hybrid-score-basis-points=7800
context-plane-status.ranked-recall.routing-diff-fixture-count=4
context-plane-status.ranked-recall.routing-diff-shadow-only-count=4
context-plane-status.ranked-recall.routing-diff-win-count=3
context-plane-status.ranked-recall.routing-diff-loss-count=1
context-plane-status.ranked-recall.routing-diff-regression-blocked-count=1
context-plane-status.ranked-recall.routing-diff-delta-min-basis-points=400
context-plane-status.ranked-recall.min-positive-routing-diff-delta-basis-points=640
context-plane-status.ranked-recall.routing-diff-latency-delta-max-ms=20
context-plane-status.ranked-recall.max-positive-routing-diff-latency-delta-ms=10
context-plane-status.ranked-recall.routing-diff-token-tradeoff-min-basis-points=1000
context-plane-status.ranked-recall.min-positive-routing-diff-token-tradeoff-basis-points=3000
context-plane-status.ranked-recall.real-workload-trace-fixture-count=4
context-plane-status.ranked-recall.real-workload-trace-shadow-only-count=4
context-plane-status.ranked-recall.real-workload-trace-slo-pass-count=3
context-plane-status.ranked-recall.real-workload-trace-win-count=3
context-plane-status.ranked-recall.real-workload-trace-loss-count=1
context-plane-status.ranked-recall.real-workload-trace-operator-review-required-count=4
context-plane-status.ranked-recall.real-workload-trace-total-leak-count=0
context-plane-status.ranked-recall.real-workload-trace-max-leak-rate-basis-points=0
context-plane-status.ranked-recall.min-positive-real-workload-trace-coverage-basis-points=8000
context-plane-status.ranked-recall.min-positive-real-workload-trace-precision-basis-points=8000
context-plane-status.ranked-recall.total-positive-real-workload-trace-token-saved=2140
context-plane-status.ranked-recall.max-positive-real-workload-trace-latency-ms=55
context-plane-status.ranked-recall.real-workload-trace-regression-loss-count=1
context-plane-status.ranked-recall.canary-precondition-fixture-count=4
context-plane-status.ranked-recall.canary-precondition-shadow-only-count=4
context-plane-status.ranked-recall.canary-precondition-pass-count=4
context-plane-status.ranked-recall.canary-feature-flag-registered-count=4
context-plane-status.ranked-recall.canary-feature-flag-disabled-count=4
context-plane-status.ranked-recall.canary-kill-switch-registered-count=4
context-plane-status.ranked-recall.canary-kill-switch-enabled-count=4
context-plane-status.ranked-recall.canary-rollback-rehearsal-covered-count=4
context-plane-status.ranked-recall.canary-activation-denial-covered-count=4
context-plane-status.ranked-recall.canary-precondition-operator-review-required-count=4
context-plane-status.ranked-recall.canary-precondition-route-opened-count=0
context-plane-status.ranked-recall.canary-precondition-rollback-write-count=0
context-plane-status.memory-provider-boundary=shadow
context-plane-status.memory-provider-v2-boundary=shadow
context-plane-status.memory-provider-v2.lifecycle-required-count=6
context-plane-status.memory-provider-v2.lifecycle-pass-count=6
context-plane-status.memory-provider-v2.query-check=pass
context-plane-status.memory-provider-v2.update-context-check=pass
context-plane-status.memory-provider-v2.propose-write-check=pass
context-plane-status.memory-provider-v2.add-check=pass
context-plane-status.memory-provider-v2.clear-check=pass
context-plane-status.memory-provider-v2.close-check=pass
context-plane-status.memory-provider-v2.candidate-count=1
context-plane-status.memory-provider-v2.operator-review-required-count=1
context-plane-status.memory-shadow-canary-readiness=shadow
context-plane-status.memory-shadow-canary-promotion-readiness=shadow
context-plane-status.canary-promotion.required-stable-window-count=1
context-plane-status.canary-promotion.observed-stable-window-count=1
context-plane-status.canary-promotion.required-pass-streak=3
context-plane-status.canary-promotion.observed-pass-streak=3
context-plane-status.canary-promotion.promotion-blocker-count=0
context-plane-status.canary-promotion.checklist-required-count=4
context-plane-status.canary-promotion.checklist-pass-count=4
context-plane-status.canary-promotion.readiness-check=pass
context-plane-status.canary-promotion.negative-rehearsal-check=pass
context-plane-status.canary-promotion.audit-digest-check=pass
context-plane-status.canary-promotion.audit-freshness-check=pass
context-plane-status.canary-promotion.rollback-rehearsal-pass-count=3
context-plane-status.canary-promotion.kill-switch-rehearsal-pass-count=3
context-plane-status.canary-promotion.soak-readback-pass-count=3
context-plane-status.source-aware-front-door=disabled
context-plane-status.production-write=disabled
context-plane-status.graph-write=disabled
context-plane-status.runtime-activation=disabled
context-plane-status.adaptive-allocator-runtime-activation=disabled
context-plane-status.source-aware-runtime-activation=disabled
context-plane-status.prompt-assembly-change=disabled
context-plane-status.operator-activation=disabled
STATUS
)"
actual_status="$(bash "$status_report")"
if [ "$actual_status" != "$expected_status" ]; then
  fail "context plane status report output changed"
fi

if printf '%s\n' "$actual_status" | grep -E 'prompt_text|transcript_text|memory_text|answer_text|source_id|session_id|memory_id|trace_id|query_text|tool_args|entity_hash|supersedes|idempotency|production-write=enabled|runtime-activation=enabled' >/dev/null; then
  fail "context plane status report leaked payload or enabled activation"
fi

cargo test --manifest-path "$manifest" -p hepta-core \
  context_plane_status \
  --lib --message-format=short

cargo test --manifest-path "$manifest" -p hepta-memory \
  context_plane_status \
  --lib --message-format=short

echo "context-plane-status-report=pass"
echo "context-plane-status-report.payload-light=pass"
echo "context-plane-status-report.runtime-activation=disabled"
echo "Hepta context plane status report gate passed"
