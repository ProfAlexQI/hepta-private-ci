#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
exec "$ROOT/scripts/hepta-gate-pair-runner" gate "hepta-systems-work-graph-dtd8-retack-rbackacktd-rbackacktd-rcptret-rbackack-td-receipt-ack-replay-preview"
