#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
exec "$ROOT/scripts/hepta-gate-pair-runner" report "hepta-systems-work-graph-dtd8-retack-td-rback-ack-td-rcpt-ret-rback-ack-td-rcpt-ret-readback-ack-preview"
