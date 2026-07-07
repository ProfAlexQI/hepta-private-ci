#!/usr/bin/env bash
set -euo pipefail

cat <<'STATUS'
context-plane-status=pass
context-plane-status.source-registry=ready
context-plane-status.adaptive-budget-allocation=shadow
context-plane-status.memory-taxonomy=ready
context-plane-status.memory-formation-receipts=ready
context-plane-status.memory-formation-queue=ready
context-plane-status.memory-temporal-facts=ready
context-plane-status.memory-temporal-fact-graph=ready
context-plane-status.memory-temporal-graph-shadow-eval=shadow
context-plane-status.eval-harness-seed=ready
context-plane-status.adaptive-allocator-eval-shadow=shadow
context-plane-status.recall-quality-gate=ready
context-plane-status.recall-quality-blocking-reason-count=0
context-plane-status.recall-quality-blocking-reasons=none
context-plane-status.memory-provider-boundary=shadow
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
