#!/usr/bin/env bash
set -euo pipefail

cat <<'STATUS'
context-plane-activation-blockers=pass
context-plane-activation-blockers.schema=2
context-plane-activation-blockers.rows=13
context-plane-activation-blockers.satisfied=9
context-plane-activation-blockers.blockers=4
context-plane-activation-blockers.source-registry=ready
context-plane-activation-blockers.adaptive-budget-allocation=blocked:adaptive_budget_allocation_shadow_only
context-plane-activation-blockers.memory-taxonomy=ready
context-plane-activation-blockers.memory-formation-receipts=ready
context-plane-activation-blockers.memory-formation-queue=ready
context-plane-activation-blockers.memory-temporal-facts=ready
context-plane-activation-blockers.memory-temporal-fact-graph=ready
context-plane-activation-blockers.eval-harness-seed=ready
context-plane-activation-blockers.adaptive-allocator-eval-shadow=shadow-threshold-pass
context-plane-activation-blockers.recall-quality-gate=ready
context-plane-activation-blockers.recall-quality-blocking-reason-count=0
context-plane-activation-blockers.recall-quality-blocking-reasons=none
context-plane-activation-blockers.memory-provider-boundary=blocked:memory_provider_boundary_shadow_only
context-plane-activation-blockers.source-aware-front-door=blocked:source_aware_front_door_disabled
context-plane-activation-blockers.operator-approval=blocked:operator_approval_missing
context-plane-activation-blockers.activation-allowed=disabled
context-plane-activation-blockers.runtime-activation=disabled
context-plane-activation-blockers.adaptive-allocator-runtime-activation=disabled
context-plane-activation-blockers.source-aware-runtime-activation=disabled
context-plane-activation-blockers.production-write=disabled
context-plane-activation-blockers.graph-write=disabled
context-plane-activation-blockers.prompt-assembly-change=disabled
context-plane-activation-blockers.operator-activation=disabled
STATUS
