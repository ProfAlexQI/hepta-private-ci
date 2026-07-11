#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
exec "$ROOT/scripts/hepta-gate-pair-runner" report "hepta-systems-wg-events-cutover-terminal-no-cutover-receipt-ack-replay-idempotency-closeout-receipt-ack-replay-idempotency-closeout-receipt-receipt-receipt-receipt-receipt-receipt-preview"
