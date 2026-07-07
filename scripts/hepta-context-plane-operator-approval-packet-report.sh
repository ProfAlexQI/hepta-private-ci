#!/usr/bin/env bash
set -euo pipefail

cat <<'STATUS'
context-plane-operator-approval-packet=pass
context-plane-operator-approval-packet.schema=2
context-plane-operator-approval-packet.dry-run=enabled
context-plane-operator-approval-packet.approval-required=enabled
context-plane-operator-approval-packet.activation-command=absent
context-plane-operator-approval-packet.rows=14
context-plane-operator-approval-packet.satisfied=9
context-plane-operator-approval-packet.blockers=5
context-plane-operator-approval-packet.threshold.required-ready=13
context-plane-operator-approval-packet.threshold.required-shadow=1
context-plane-operator-approval-packet.blocker.adaptive-budget-allocation-shadow-only=1
context-plane-operator-approval-packet.blocker.temporal-graph-shadow-eval-shadow-only=1
context-plane-operator-approval-packet.blocker.memory-provider-boundary-shadow-only=1
context-plane-operator-approval-packet.blocker.source-aware-front-door-disabled=1
context-plane-operator-approval-packet.blocker.operator-approval-missing=1
context-plane-operator-approval-packet.recall-quality-blocking-reason-count=0
context-plane-operator-approval-packet.recall-quality-blocking-reasons=none
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
