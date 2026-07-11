#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
exec "$ROOT/scripts/hepta-gate-pair-runner" report "hepta-systems-work-graph-unified-projection-enforcement-readiness-work-graph-events-event-store-cutover-replay-readback-execution-closure-rerun-preview"
