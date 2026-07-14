#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
exec "$ROOT/scripts/hepta-durable-memory-dry-run-result-receipt-boundary-gate-runner" "hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-replay-idempotency-denial-boundary"
