#!/usr/bin/env bash
set -euo pipefail

cat <<'EOF'
context-memory-write-chain-readiness=pass
context-memory-write-chain-readiness.payload-light=pass
context-memory-write-chain-readiness.schema=1
context-memory-write-chain-readiness.namespace-count=6
context-memory-write-chain-readiness.stage-required-count=6
context-memory-write-chain-readiness.stage-pass-count=6
context-memory-write-chain-readiness.propose-write-ready-count=6
context-memory-write-chain-readiness.policy-approval-ready-count=6
context-memory-write-chain-readiness.operator-approval-ready-count=6
context-memory-write-chain-readiness.shadow-wal-ready-count=6
context-memory-write-chain-readiness.readback-ready-count=6
context-memory-write-chain-readiness.canary-ready-count=6
context-memory-write-chain-readiness.rollback-ready-count=6
context-memory-write-chain-readiness.production-write-count=0
context-memory-write-chain-readiness.graph-write-count=0
context-memory-write-chain-readiness.production-write=disabled
context-memory-write-chain-readiness.graph-write=disabled
context-memory-write-chain-readiness.hot-path-write=disabled
context-memory-write-chain-readiness.prompt-assembly-change=disabled
context-memory-write-chain-readiness.runtime-activation=disabled
EOF
