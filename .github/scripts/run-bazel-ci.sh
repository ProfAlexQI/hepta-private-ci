#!/usr/bin/env bash

set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
windows_cross_compile=0

for arg in "$@"; do
  if [[ "$arg" == "--" ]]; then
    break
  fi
  if [[ "$arg" == "--windows-cross-compile" ]]; then
    windows_cross_compile=1
  fi
done

if [[ "${GITHUB_ACTIONS:-}" == "true" && "${ALLOW_WINDOWS_MSVC_FALLBACK:-}" == "1" ]]; then
  echo "ALLOW_WINDOWS_MSVC_FALLBACK is forbidden in GitHub Actions qualification jobs. Use the manual Windows MSVC non-qualifying diagnostic workflow, whose result is excluded from repository admission." >&2
  exit 1
fi

if [[ "${GITHUB_ACTIONS:-}" == "true" && "${RUNNER_OS:-}" == "Windows" && $windows_cross_compile -eq 1 && -z "${BUILDBUDDY_API_KEY:-}" ]]; then
  echo "Automated Windows gnullvm qualification requires authenticated BuildBuddy/RBE; refusing all keyless fallback before the Bazel implementation is invoked." >&2
  exit 1
fi

exec "${script_dir}/run-bazel-ci-impl.sh" "$@"
