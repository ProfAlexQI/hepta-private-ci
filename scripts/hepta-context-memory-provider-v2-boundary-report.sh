#!/usr/bin/env bash
set -euo pipefail

echo "memory-provider-v2-boundary=pass"
echo "memory-provider-v2-boundary.payload-light=pass"
echo "memory-provider-v2-boundary.mode=shadow-only"
echo "memory-provider-v2-boundary.lifecycle=query+update_context+propose_write+add+clear+close"
echo "memory-provider-v2-boundary.propose-write=shadow-proposal"
echo "memory-provider-v2-boundary.add=dry-run-or-blocked"
echo "memory-provider-v2-boundary.clear=dry-run-or-blocked"
echo "memory-provider-v2-boundary.close=noop-close-report"
echo "memory-provider-v2-boundary.operator-approval=required"
echo "memory-provider-v2-boundary.production-route=disabled"
echo "memory-provider-v2-boundary.write=disabled"
echo "memory-provider-v2-boundary.graph-write=disabled"
echo "memory-provider-v2-boundary.runtime-activation=disabled"
