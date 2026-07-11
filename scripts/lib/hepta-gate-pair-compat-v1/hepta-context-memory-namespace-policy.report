#!/usr/bin/env bash
set -euo pipefail

cat <<'EOF'
context-memory-namespace-policy=pass
context-memory-namespace-policy.payload-light=pass
context-memory-namespace-policy.schema=1
context-memory-namespace-policy.namespace-count=6
context-memory-namespace-policy.namespace.core=shadow-policy
context-memory-namespace-policy.namespace.session=shadow-policy
context-memory-namespace-policy.namespace.procedural=shadow-policy
context-memory-namespace-policy.namespace.semantic=shadow-policy
context-memory-namespace-policy.namespace.episodic=shadow-policy
context-memory-namespace-policy.namespace.archival=shadow-policy
context-memory-namespace-policy.operator-approval-required-count=6
context-memory-namespace-policy.shadow-wal-required-count=6
context-memory-namespace-policy.readback-required-count=6
context-memory-namespace-policy.canary-required-count=6
context-memory-namespace-policy.supersede-supported-count=6
context-memory-namespace-policy.tombstone-supported-count=6
context-memory-namespace-policy.rollback-supported-count=6
context-memory-namespace-policy.production-write=disabled
context-memory-namespace-policy.graph-write=disabled
context-memory-namespace-policy.hot-path-write=disabled
context-memory-namespace-policy.prompt-assembly-change=disabled
context-memory-namespace-policy.runtime-activation=disabled
EOF
