"""Shared constants and identity helpers for Q0.27."""

from __future__ import annotations

import hashlib
from collections.abc import Mapping, Sequence
from pathlib import Path, PureWindowsPath

from run_bazel_q017_policy import CI_EXACT_OPTIONS
from run_bazel_q017_policy import _qualification_workspace_bazelrc
from run_bazel_q022_negative_targets import CANONICAL_CLIPPY_NEGATIVE_TARGET
from run_bazel_q022_negative_targets import CANONICAL_RELEASE_TARGETS
from run_bazel_q022_negative_targets import CANONICAL_SKIP_INCOMPATIBLE
from run_bazel_q022_negative_targets import CANONICAL_TEST_TAG_FILTER
from run_bazel_q022_negative_targets import CANONICAL_TEST_VERBOSE_TIMEOUTS
from run_bazel_q022_negative_targets import CLIPPY_JOB_METADATA
from run_bazel_q022_negative_targets import RELEASE_JOB_METADATA


REPOSITORY = "ProfHepta/hepta-private-ci"
BAZEL_VERSION = "9.0.0"
BAZELVERSION_BYTES = b"9.0.0\n"
BAZELVERSION_GIT_BLOB_SHA1 = "f7ee06693c17a06e2a0f51ef7eb2a61866e77b8e"
BAZELISK_VERSION = "1.28.1"
BAZELISK_WINDOWS_X86_64_SHA256 = (
    "b9d65a1f7c2d7af885a96a4fd5aa36b40fb41816d30944390569eef908bdc954"
)
BAZEL_WINDOWS_X86_64_SHA256 = (
    "463faee497df2913854d80776784137cb47f42960b4ef4e4f85068c8da4849a8"
)

TEST_JOB = "test-windows-shard"
CLIPPY_JOB = "clippy"
RELEASE_JOB = "verify-release-build"
QUALIFYING_JOBS = {TEST_JOB, CLIPPY_JOB, RELEASE_JOB}

TEST_OPTIONS = {
    CANONICAL_SKIP_INCOMPATIBLE,
    CANONICAL_TEST_TAG_FILTER,
    CANONICAL_TEST_VERBOSE_TIMEOUTS,
    "--remote_download_toplevel",
}
CLIPPY_OPTIONS = {CANONICAL_SKIP_INCOMPATIBLE}
RELEASE_OPTIONS = {
    "--compilation_mode=fastbuild",
    "--@rules_rust//rust/settings:extra_rustc_flag=-Cdebug-assertions=no",
    "--@rules_rust//rust/settings:extra_exec_rustc_flag=-Cdebug-assertions=no",
}

BAZELISK_REQUIRED_ENV = {
    "USE_BAZEL_VERSION": BAZEL_VERSION,
    "BAZELISK_VERIFY_SHA256": BAZEL_WINDOWS_X86_64_SHA256,
    "BAZELISK_SKIP_WRAPPER": "true",
}
BAZELISK_FORBIDDEN_ENV = {
    "BAZELISK",
    "BAZEL_REAL",
    "BAZELISK_BASE_URL",
    "BAZELISK_CLEAN",
    "BAZELISK_FORMAT_URL",
    "BAZELISK_HOME",
    "BAZELISK_HOME_DARWIN",
    "BAZELISK_HOME_LINUX",
    "BAZELISK_HOME_WINDOWS",
    "BAZELISK_INCOMPATIBLE_FLAGS",
    "BAZELISK_NOJDK",
    "BAZELISK_SHUTDOWN",
    "USE_BAZEL_FALLBACK_VERSION",
}

_DYNAMIC_OPTION_PREFIXES = (
    "--action_env=",
    "--host_action_env=",
    "--test_env=",
    "--repo_contents_cache=",
    "--repository_cache=",
    "--execution_log_compact_file=",
)
_WINDOWS_INHERITED_ENV_NAMES = {
    "INCLUDE",
    "LIB",
    "LIBPATH",
    "UCRTVersion",
    "UniversalCRTSdkDir",
    "VCINSTALLDIR",
    "VCToolsInstallDir",
    "WindowsLibPath",
    "WindowsSdkBinPath",
    "WindowsSdkDir",
    "WindowsSDKLibVersion",
    "WindowsSDKVersion",
}


def _git_blob_sha1(data: bytes) -> str:
    payload = f"blob {len(data)}\0".encode("ascii") + data
    return hashlib.sha1(payload, usedforsecurity=False).hexdigest()


def _sha256_file(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as source:
        for chunk in iter(lambda: source.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def _require_env(env: Mapping[str, str], name: str) -> str:
    value = env.get(name)
    if not value:
        raise ValueError(f"keyless Windows gnullvm qualification requires {name}")
    return value


def _normal_windows_path(value: str) -> str:
    return str(PureWindowsPath(value)).rstrip("\\").casefold()


def _expected_under(root: str, child: str) -> str:
    return _normal_windows_path(f"{root.rstrip('/\\')}/{child}")



def _validate_runner_identity(env: Mapping[str, str]) -> str:
    if env.get("GITHUB_ACTIONS") != "true":
        raise ValueError("qualification requires GitHub Actions")
    if env.get("RUNNER_OS") != "Windows":
        raise ValueError("qualification requires a Windows runner")
    if env.get("BUILDBUDDY_API_KEY"):
        raise ValueError("keyless qualification must not receive BuildBuddy credentials")
    if env.get("GITHUB_REPOSITORY") != REPOSITORY:
        raise ValueError(f"qualification repository must be {REPOSITORY}")
    if env.get("RUNNER_ENVIRONMENT") != "github-hosted":
        raise ValueError("qualification requires a GitHub-hosted runner")
    if env.get("RUNNER_ARCH") != "X64":
        raise ValueError("qualification requires an X64 runner")
    job = _require_env(env, "GITHUB_JOB")
    if job not in QUALIFYING_JOBS:
        raise ValueError(f"unknown keyless Windows gnullvm job {job!r}")
    return job



def _validate_environment_roots(env: Mapping[str, str], job: str) -> None:
    build_root = _require_env(env, "CI_BUILD_ROOT")
    run_id = _require_env(env, "GITHUB_RUN_ID")
    runner_temp = _require_env(env, "RUNNER_TEMP")
    expected = {
        "BAZEL_OUTPUT_BASE": _expected_under(build_root, "o"),
        "BAZEL_OUTPUT_USER_ROOT": _expected_under(build_root, "b"),
        "BAZEL_REPOSITORY_CACHE": _expected_under(
            build_root, "bazel-repository-cache"
        ),
        "BAZEL_REPO_CONTENTS_CACHE": _expected_under(
            build_root, f"bazel-repo-contents-cache-{run_id}-{job}"
        ),
        "CODEX_BAZEL_EXECUTION_LOG_COMPACT_DIR": _expected_under(
            runner_temp, "bazel-execution-logs"
        ),
    }
    for name, expected_value in expected.items():
        observed = _normal_windows_path(_require_env(env, name))
        if observed != expected_value:
            raise ValueError(
                f"{name} escaped its runner-controlled root: "
                f"expected {expected_value!r}, observed {observed!r}"
            )


