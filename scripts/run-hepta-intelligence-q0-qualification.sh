#!/usr/bin/env bash
set -euo pipefail

script_dir=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
for stage in \
  00-prepare.sh \
  10-source-gates.sh \
  20-rust-matrix.sh \
  30-receipt.sh; do
  # The stages intentionally share one shell so exact-candidate state and functions stay bound.
  source "$script_dir/q0-qualification/$stage"
done
