#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
exec "$ROOT/scripts/hepta-route-gate-runner" "hepta-first-model-invocation-operator-approval-final-authorization-dry-run-result-receipt-replay-idempotency-denial"
