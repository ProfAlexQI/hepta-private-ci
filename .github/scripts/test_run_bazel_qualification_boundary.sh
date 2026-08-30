#!/usr/bin/env bash

set -euo pipefail

python3 .github/scripts/test_run_bazel_ci_wrapper.py
python3 .github/scripts/test_run_bazel_local_windows_gnullvm.py
python3 .github/scripts/test_run_bazel_option_grammar.py
python3 .github/scripts/test_run_bazel_final_command.py
python3 .github/scripts/test_run_bazel_negative_targets.py
python3 .github/scripts/test_run_bazel_lane_policy.py
python3 .github/scripts/test_run_bazel_lane_semantics.py
python3 .github/scripts/test_run_bazel_startup_contract.py
python3 .github/scripts/test_run_bazel_job_executable.py
python3 .github/scripts/test_run_bazel_direct_bazel.py
python3 .github/scripts/test_run_bazel_setup_token_boundary.py
python3 .github/scripts/test_run_bazel_execution_manifest.py
python3 .github/scripts/test_run_bazel_setup_action_yaml.py
python3 .github/scripts/test_run_bazel_query_vector.py
python3 .github/scripts/test_run_bazel_query_executable.py
python3 scripts/verify-windows-gnullvm-final-command.py
python3 scripts/verify-windows-gnullvm-lane-policy.py
python3 scripts/verify-windows-gnullvm-lane-semantics.py
python3 scripts/verify-windows-gnullvm-startup-contract.py
python3 scripts/verify-windows-gnullvm-job-executable.py
python3 scripts/verify-windows-gnullvm-direct-bazel.py
python3 scripts/verify-windows-gnullvm-setup-action-yaml.py
python3 scripts/verify-windows-gnullvm-bazel-query-vector.py
python3 scripts/verify-windows-gnullvm-bazel-query-executable.py
python3 scripts/verify-windows-gnullvm-setup-token-receipt-truth.py

echo "PASS_WINDOWS_GNULLVM_QUALIFICATION_BOUNDARY_FIXTURES"
