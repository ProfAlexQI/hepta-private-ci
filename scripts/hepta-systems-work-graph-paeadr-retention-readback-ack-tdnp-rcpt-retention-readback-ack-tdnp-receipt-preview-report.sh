#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
exec "$ROOT/scripts/hepta-gate-pair-runner" report "hepta-systems-work-graph-paeadr-retention-readback-ack-tdnp-rcpt-retention-readback-ack-tdnp-receipt-preview"
