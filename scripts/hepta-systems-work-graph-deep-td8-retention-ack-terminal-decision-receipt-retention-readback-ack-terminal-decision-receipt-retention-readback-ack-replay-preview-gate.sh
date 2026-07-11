#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
exec "$ROOT/scripts/hepta-gate-pair-runner" gate "hepta-systems-work-graph-deep-td8-retention-ack-terminal-decision-receipt-retention-readback-ack-terminal-decision-receipt-retention-readback-ack-replay-preview"
