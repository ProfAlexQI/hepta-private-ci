#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
exec "$ROOT/scripts/hepta-memory-live-mutation-result-receipt-gate-runner" "hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-replay-idempotency-denial"
