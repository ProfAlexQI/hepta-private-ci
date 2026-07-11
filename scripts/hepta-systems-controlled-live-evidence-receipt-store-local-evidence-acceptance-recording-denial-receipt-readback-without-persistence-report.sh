#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
exec "$ROOT/scripts/hepta-gate-pair-runner" report "hepta-systems-controlled-live-evidence-receipt-store-local-evidence-acceptance-recording-denial-receipt-readback-without-persistence"
