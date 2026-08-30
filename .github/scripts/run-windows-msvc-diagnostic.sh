#!/usr/bin/env bash

set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
receipt_dir="${HEPTA_WINDOWS_MSVC_DIAGNOSTIC_RECEIPT_DIR:-${RUNNER_TEMP:-${TMPDIR:-/tmp}}/hepta-windows-msvc-diagnostic}"
receipt_path="${receipt_dir}/receipt.json"
python_bin="${PYTHON:-python3}"

if [[ "${HEPTA_QUALIFICATION_MODE:-}" != "non_qualifying_msvc_diagnostic" ]]; then
  echo "HEPTA_QUALIFICATION_MODE must be non_qualifying_msvc_diagnostic." >&2
  exit 1
fi

if [[ "${RUNNER_OS:-}" != "Windows" ]]; then
  echo "The MSVC diagnostic must execute on a Windows runner." >&2
  exit 1
fi

if [[ -n "${BUILDBUDDY_API_KEY:-}" ]]; then
  echo "The non-qualifying MSVC diagnostic must not consume BuildBuddy/RBE credentials." >&2
  exit 1
fi

if [[ "${GITHUB_ACTIONS:-}" == "true" ]]; then
  if [[ "${GITHUB_EVENT_NAME:-}" != "workflow_dispatch" || "${GITHUB_WORKFLOW:-}" != "Windows MSVC non-qualifying diagnostic" || "${GITHUB_JOB:-}" != "msvc-diagnostic" ]]; then
    echo "GitHub Actions may invoke this diagnostic only from the dedicated workflow_dispatch job." >&2
    exit 1
  fi
fi

mkdir -p "${receipt_dir}"

set +e
ALLOW_WINDOWS_MSVC_FALLBACK=1 \
  "${script_dir}/run-bazel-ci-impl.sh" \
  --windows-cross-compile \
  "$@"
bazel_status=$?
set -e

"${python_bin}" - "${receipt_path}" "${bazel_status}" "$@" <<'PY'
import hashlib
import json
import os
import sys
from pathlib import Path

receipt_path = Path(sys.argv[1])
bazel_status = int(sys.argv[2])
bazel_args = sys.argv[3:]
receipt = {
    "schema": "hepta_windows_msvc_non_qualifying_diagnostic_v1",
    "status": (
        "PASS_NON_QUALIFYING_MSVC_DIAGNOSTIC"
        if bazel_status == 0
        else "FAIL_NON_QUALIFYING_MSVC_DIAGNOSTIC"
    ),
    "qualification_mode": "non_qualifying_msvc_diagnostic",
    "repository": os.environ.get("GITHUB_REPOSITORY"),
    "workflow": os.environ.get("GITHUB_WORKFLOW"),
    "workflow_run_id": os.environ.get("GITHUB_RUN_ID"),
    "workflow_run_attempt": os.environ.get("GITHUB_RUN_ATTEMPT"),
    "workflow_job": os.environ.get("GITHUB_JOB"),
    "event_name": os.environ.get("GITHUB_EVENT_NAME"),
    "commit": os.environ.get("GITHUB_SHA"),
    "bazel_exit_code": bazel_status,
    "bazel_args": bazel_args,
    "bazel_args_sha256": hashlib.sha256(
        b"\0".join(arg.encode("utf-8") for arg in bazel_args)
    ).hexdigest(),
    "eligible_for_repository_admission": False,
    "gnullvm_evidence": False,
    "required_check": False,
    "runtime_authority": False,
    "production_authority": False,
    "operator_acceptance": False,
    "promotion": False,
    "release_authority": False,
    "callers_ratchet": False,
}
canonical = json.dumps(receipt, sort_keys=True, separators=(",", ":")).encode()
receipt["receipt_binding_sha256"] = hashlib.sha256(canonical).hexdigest()
receipt_path.write_text(
    json.dumps(receipt, indent=2, sort_keys=True) + "\n", encoding="utf-8"
)
print(json.dumps(receipt, indent=2, sort_keys=True))
PY

if [[ -n "${GITHUB_STEP_SUMMARY:-}" ]]; then
  {
    echo '```json'
    cat "${receipt_path}"
    echo '```'
  } >> "${GITHUB_STEP_SUMMARY}"
fi

exit "${bazel_status}"
