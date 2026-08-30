#!/usr/bin/env bash

set -euo pipefail

python3 .github/scripts/test_run_bazel_ci_wrapper.py
python3 .github/scripts/test_run_bazel_local_windows_gnullvm.py

echo "PASS_WINDOWS_GNULLVM_QUALIFICATION_BOUNDARY_FIXTURES"
