#!/usr/bin/env bash

set -euo pipefail

while IFS= read -r name; do
  if [[ "${name,,}" != "bazelisk_github_token" ]]; then
    continue
  fi
  if [[ -n "${!name}" ]]; then
    echo "setup-only Bazelisk GitHub token remained nonempty after scrub" >&2
    exit 1
  fi
done < <(compgen -e)

echo "PASS_SETUP_BAZEL_TOKEN_SCRUBBED"
