#!/usr/bin/env bash

set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
source_dir="${repo_root}/.github/scripts"
tmp="$(mktemp -d)"
trap 'rm -rf "${tmp}"' EXIT

cp "${source_dir}/run-bazel-ci.sh" "${tmp}/run-bazel-ci.sh"
cp "${source_dir}/run-bazel-ci-impl.sh" "${tmp}/run-bazel-ci-impl.sh"
cp "${source_dir}/run_bazel_with_buildbuddy.py" "${tmp}/run_bazel_with_buildbuddy.py"
chmod +x "${tmp}/run-bazel-ci.sh" "${tmp}/run-bazel-ci-impl.sh" "${tmp}/run_bazel_with_buildbuddy.py"

cat > "${tmp}/fake-bazel.py" <<'PY'
#!/usr/bin/env python3
import json
import os
import sys
from pathlib import Path

record = Path(os.environ["BAZEL_RECORD"])
record.write_text(json.dumps(sys.argv[1:]) + "\n", encoding="utf-8")
PY
chmod +x "${tmp}/fake-bazel.py"

common_args=(
  --windows-cross-compile
  --
  build
  --config=clippy
  --
  //codex-rs/uds:uds-unit-tests-bin
)
common_env=(
  RUNNER_OS=Windows
  CODEX_BAZEL_WINDOWS_PATH=/usr/bin:/bin
  CODEX_BAZEL_BIN="${tmp}/fake-bazel.py"
)

run_expect_failure_without_bazel() {
  local stderr_path="$1"
  local record_path="$2"
  shift 2
  rm -f "${record_path}"
  set +e
  env "${common_env[@]}" BAZEL_RECORD="${record_path}" "$@" \
    "${tmp}/run-bazel-ci.sh" "${common_args[@]}" \
    > /dev/null 2> "${stderr_path}"
  local status=$?
  set -e
  if [[ ${status} -eq 0 ]]; then
    echo "Expected qualification boundary failure." >&2
    exit 1
  fi
  if [[ -e "${record_path}" ]]; then
    echo "Bazel implementation was invoked despite fail-closed boundary." >&2
    exit 1
  fi
}

run_expect_failure_without_bazel \
  "${tmp}/default.stderr" \
  "${tmp}/default.json" \
  GITHUB_ACTIONS=true

grep -Fq "Automated Windows gnullvm qualification requires authenticated BuildBuddy/RBE" \
  "${tmp}/default.stderr"

run_expect_failure_without_bazel \
  "${tmp}/ambient.stderr" \
  "${tmp}/ambient.json" \
  GITHUB_ACTIONS=true \
  ALLOW_WINDOWS_MSVC_FALLBACK=1

grep -Fq "ALLOW_WINDOWS_MSVC_FALLBACK is forbidden in GitHub Actions qualification jobs" \
  "${tmp}/ambient.stderr"

local_record="${tmp}/local.json"
env "${common_env[@]}" \
  BAZEL_RECORD="${local_record}" \
  ALLOW_WINDOWS_MSVC_FALLBACK=1 \
  "${tmp}/run-bazel-ci.sh" "${common_args[@]}" > /dev/null

python3 - "${local_record}" <<'PY'
import json
import sys
from pathlib import Path

args = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
required = {
    "--config=ci-windows",
    "--host_platform=//:local_windows_msvc",
    "--platforms=//:local_windows_msvc",
    "--jobs=8",
    "//codex-rs/uds:uds-unit-tests-bin",
}
missing = sorted(required.difference(args))
if missing:
    raise SystemExit(f"local diagnostic missing arguments: {missing}; args={args}")
if "--config=ci-windows-cross" in args:
    raise SystemExit(f"local diagnostic retained remote-only config: {args}")
PY

authenticated_record="${tmp}/authenticated.json"
env "${common_env[@]}" \
  BAZEL_RECORD="${authenticated_record}" \
  GITHUB_ACTIONS=true \
  GITHUB_REPOSITORY=ProfHepta/hepta-private-ci \
  GITHUB_EVENT_NAME=pull_request \
  BUILDBUDDY_API_KEY=fixture-key \
  "${tmp}/run-bazel-ci.sh" "${common_args[@]}" > /dev/null

python3 - "${authenticated_record}" <<'PY'
import json
import sys
from pathlib import Path

args = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
required = {
    "--config=ci-windows-cross",
    "--config=buildbuddy-generic-rbe",
    "--host_platform=//:rbe",
    "--shell_executable=/bin/bash",
    "//codex-rs/uds:uds-unit-tests-bin",
}
missing = sorted(required.difference(args))
if missing:
    raise SystemExit(f"authenticated gnullvm path missing arguments: {missing}; args={args}")
for forbidden in (
    "--config=ci-windows",
    "--host_platform=//:local_windows_msvc",
    "--platforms=//:local_windows_msvc",
    "--jobs=8",
):
    if forbidden in args:
        raise SystemExit(f"authenticated gnullvm path contains {forbidden}: {args}")
PY

echo "PASS_WINDOWS_GNULLVM_QUALIFICATION_BOUNDARY_FIXTURES"
