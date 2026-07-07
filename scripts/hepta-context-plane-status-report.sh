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
context-plane-status.source-aware-front-door=disabled
context-plane-status.production-write=disabled
context-plane-status.graph-write=disabled
context-plane-status.runtime-activation=disabled
context-plane-status.adaptive-allocator-runtime-activation=disabled
context-plane-status.source-aware-runtime-activation=disabled
context-plane-status.prompt-assembly-change=disabled
context-plane-status.operator-activation=disabled
STATUS
