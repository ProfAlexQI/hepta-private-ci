#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
exec "$ROOT/scripts/hepta-gate-pair-runner" gate "hepta-systems-work-graph-live-attachment-terminal-closeout-readback-audit-index-non-persistence-final-closeout-readback-audit-index-non-persistence-final-closeout-readback-audit-index-non-persistence-final-closeout-readback"
