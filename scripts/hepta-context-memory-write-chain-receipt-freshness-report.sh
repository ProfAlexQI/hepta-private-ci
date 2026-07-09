#!/usr/bin/env bash
set -euo pipefail

cat <<'EOF'
context-memory-write-chain-receipt-freshness=pass
context-memory-write-chain-receipt-freshness.payload-light=pass
context-memory-write-chain-receipt-freshness.schema=1
context-memory-write-chain-receipt-freshness.source-readiness-schema=1
context-memory-write-chain-receipt-freshness.namespace-count=6
context-memory-write-chain-receipt-freshness.receipt-required-count=18
context-memory-write-chain-receipt-freshness.receipt-projected-count=18
context-memory-write-chain-receipt-freshness.receipt-digest-count=6
context-memory-write-chain-receipt-freshness.freshness-pass-count=6
context-memory-write-chain-receipt-freshness.replay-guard-pass-count=6
context-memory-write-chain-receipt-freshness.stale-replay-rejected-count=6
context-memory-write-chain-receipt-freshness.recorded-receipt-count=0
context-memory-write-chain-receipt-freshness.persisted-receipt-count=0
context-memory-write-chain-receipt-freshness.production-write-count=0
context-memory-write-chain-receipt-freshness.graph-write-count=0
context-memory-write-chain-receipt-freshness.production-write=disabled
context-memory-write-chain-receipt-freshness.graph-write=disabled
context-memory-write-chain-receipt-freshness.hot-path-write=disabled
context-memory-write-chain-receipt-freshness.prompt-assembly-change=disabled
context-memory-write-chain-receipt-freshness.runtime-activation=disabled
EOF
