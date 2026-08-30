#!/usr/bin/env bash

set -euo pipefail

python3 .github/scripts/test_run_bazel_ci_wrapper.py
python3 .github/scripts/test_run_bazel_local_windows_gnullvm.py
python3 .github/scripts/test_run_bazel_option_grammar.py
python3 .github/scripts/test_run_bazel_final_command.py
python3 .github/scripts/test_run_bazel_negative_targets.py
python3 .github/scripts/test_run_bazel_lane_policy.py
python3 .github/scripts/test_run_bazel_lane_semantics.py
python3 .github/scripts/test_run_bazel_execution_context.py
python3 scripts/verify-windows-gnullvm-final-command.py
python3 scripts/verify-windows-gnullvm-lane-policy.py
python3 scripts/verify-windows-gnullvm-lane-semantics.py
python3 scripts/verify-windows-gnullvm-execution-context.py

echo "PASS_WINDOWS_GNULLVM_QUALIFICATION_BOUNDARY_FIXTURES"
