"""Q0.28 fail-closed Windows runner, job, path, and Bazel executable contract."""

from __future__ import annotations

import hashlib
import re
import shutil
from collections.abc import Callable, Mapping, MutableMapping, Sequence
from pathlib import Path, PureWindowsPath

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
QUALIFYING_JOBS = frozenset((TEST_JOB, CLIPPY_JOB, RELEASE_JOB))
COMMIT_METADATA_PREFIX = "--build_metadata=COMMIT_SHA="
TEST_SHARD_METADATA_PREFIX = "--build_metadata=TAG_windows_test_shard="
CLIPPY_JOB_METADATA = "--build_metadata=TAG_job=clippy"
RELEASE_JOB_METADATA = "--build_metadata=TAG_job=verify-release-build"

FORBIDDEN_BAZELISK_ENV = frozenset(
    (
        "BAZELISK_BASE_URL",
        "BAZELISK_GITHUB_TOKEN",
        "BAZELISK_HOME",
        "BAZELISK_NOJDK",
        "BAZELISK_NOJDK_URL",
        "BAZELISK_NOJDK_VERSION",
        "BAZELISK_SHUTDOWN",
        "BAZELISK_USER_AGENT",
        "BAZEL_REAL",
        "BAZEL_WRAPPER",
    )
)
_LOG_NAME = re.compile(r"execution-log-(build|test)-([a-z0-9-]+)-[0-9]+\.zst\Z")


def _git_blob_sha1(data: bytes) -> str:
    framed = f"blob {len(data)}\0".encode("ascii") + data
    return hashlib.sha1(framed, usedforsecurity=False).hexdigest()


def _sha256_file(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def _require_env(env: Mapping[str, str], name: str) -> str:
    value = env.get(name)
    if not value:
        raise ValueError(
            "credential-free Windows gnullvm qualification "
            f"requires {name}"
        )
    return value


def _absolute_windows_path(value: str, *, owner: str) -> PureWindowsPath:
    path = PureWindowsPath(value)
    # PowerShell's setup-dev-drive script intentionally exports a bare drive
    # root such as ``D:``. PureWindowsPath treats that spelling as drive-relative;
    # qualification treats this one root spelling as the canonical ``D:\\``.
    if path.drive and not path.root and len(path.parts) == 1:
        path = PureWindowsPath(f"{path.drive}\\")
    if not path.is_absolute():
        raise ValueError(f"{owner} must be an absolute Windows path: {value!r}")
    if any(part in {".", ".."} for part in path.parts):
        raise ValueError(f"{owner} contains a non-canonical path segment: {value!r}")
    return path


def _path_key(path: PureWindowsPath) -> str:
    return str(path).replace("/", "\\").rstrip("\\").casefold()


def _require_path(env: Mapping[str, str], name: str, expected: PureWindowsPath) -> None:
    observed = _absolute_windows_path(_require_env(env, name), owner=name)
    if _path_key(observed) != _path_key(expected):
        raise ValueError(
            f"{name} escaped its runner-controlled root: "
            f"expected {str(expected)!r}, observed {str(observed)!r}"
        )


def prepare_bazelisk_environment(env: MutableMapping[str, str]) -> None:
    """Reject ambient executable controls and install exact Bazelisk inputs."""

    if env.get("CODEX_BAZEL_BIN"):
        raise ValueError("CODEX_BAZEL_BIN is forbidden in repository qualification")
    for name in sorted(FORBIDDEN_BAZELISK_ENV):
        if env.get(name):
            raise ValueError(f"{name} is forbidden in repository qualification")

    configured = env.get("USE_BAZEL_VERSION")
    if configured and configured != BAZEL_VERSION:
        raise ValueError(
            f"USE_BAZEL_VERSION={configured!r} conflicts with Bazel {BAZEL_VERSION}"
        )
    configured_digest = env.get("BAZELISK_VERIFY_SHA256")
    if configured_digest and configured_digest != BAZEL_WINDOWS_X86_64_SHA256:
        raise ValueError(
            "BAZELISK_VERIFY_SHA256 conflicts with the reviewed Bazel asset"
        )
    configured_skip = env.get("BAZELISK_SKIP_WRAPPER")
    if configured_skip and configured_skip.lower() != "true":
        raise ValueError("BAZELISK_SKIP_WRAPPER must remain true")

    env["USE_BAZEL_VERSION"] = BAZEL_VERSION
    env["BAZELISK_VERIFY_SHA256"] = BAZEL_WINDOWS_X86_64_SHA256
    env["BAZELISK_SKIP_WRAPPER"] = "true"


def _validate_bazelisk_inputs(env: Mapping[str, str]) -> None:
    if env.get("USE_BAZEL_VERSION") != BAZEL_VERSION:
        raise ValueError("USE_BAZEL_VERSION is not bound to Bazel 9.0.0")
    if env.get("BAZELISK_VERIFY_SHA256") != BAZEL_WINDOWS_X86_64_SHA256:
        raise ValueError(
            "BAZELISK_VERIFY_SHA256 is not bound to the reviewed Bazel asset"
        )
    if env.get("BAZELISK_SKIP_WRAPPER", "").lower() != "true":
        raise ValueError("BAZELISK_SKIP_WRAPPER is not enabled")

    workspace = Path(_require_env(env, "GITHUB_WORKSPACE")).resolve(strict=True)
    bazelversion = workspace / ".bazelversion"
    if bazelversion.is_symlink() or not bazelversion.is_file():
        raise ValueError(".bazelversion must be a regular non-symlink file")
    data = bazelversion.read_bytes()
    if data != BAZELVERSION_BYTES:
        raise ValueError(".bazelversion bytes drifted from 9.0.0")
    if _git_blob_sha1(data) != BAZELVERSION_GIT_BLOB_SHA1:
        raise ValueError(".bazelversion Git blob identity drifted")
    workspace_rc = workspace / ".bazeliskrc"
    if workspace_rc.exists() or workspace_rc.is_symlink():
        raise ValueError("workspace .bazeliskrc is forbidden")

    home_value = env.get("USERPROFILE") or env.get("HOME")
    if not home_value:
        raise ValueError("runner home is required to reject user .bazeliskrc")
    home_rc = Path(home_value) / ".bazeliskrc"
    if home_rc.exists() or home_rc.is_symlink():
        raise ValueError("runner-home .bazeliskrc is forbidden")


def bind_verified_bazelisk(
    command: Sequence[str],
    env: Mapping[str, str],
    *,
    which: Callable[..., str | None] = shutil.which,
    digest_file: Callable[[Path], str] = _sha256_file,
) -> list[str]:
    """Replace argv[0] with the verified official Bazelisk executable."""

    if not command:
        raise ValueError("cannot bind an empty Bazel command")
    _validate_bazelisk_inputs(env)
    resolved = which("bazel", path=env.get("PATH"))
    if not resolved:
        raise ValueError("official Bazelisk executable was not found on PATH")
    unresolved = Path(resolved)
    if unresolved.is_symlink():
        raise ValueError("Bazelisk PATH entry must not be a symlink")
    try:
        executable = unresolved.resolve(strict=True)
    except OSError as error:
        raise ValueError(f"cannot resolve Bazelisk executable: {error}") from error
    if not executable.is_file():
        raise ValueError("Bazelisk PATH entry is not a regular file")
    observed = digest_file(executable)
    if observed != BAZELISK_WINDOWS_X86_64_SHA256:
        raise ValueError(
            "Bazelisk executable SHA-256 drifted: "
            f"expected {BAZELISK_WINDOWS_X86_64_SHA256}, observed {observed}"
        )
    return [str(executable), *command[1:]]


def _split_command(
    command: Sequence[str],
) -> tuple[list[str], str, list[str], list[str]]:
    if len(command) < 3:
        raise ValueError("incomplete Bazel command")
    command_idx = next(
        (
            index
            for index, arg in enumerate(command[1:], start=1)
            if not arg.startswith("-")
        ),
        None,
    )
    if command_idx is None:
        raise ValueError("Bazel command verb is missing")
    separators = [
        index
        for index, arg in enumerate(
            command[command_idx + 1 :], start=command_idx + 1
        )
        if arg == "--"
    ]
    if len(separators) != 1:
        raise ValueError("qualification requires exactly one Bazel target separator")
    separator_idx = separators[0]
    return (
        list(command[1:command_idx]),
        command[command_idx],
        list(command[command_idx + 1 : separator_idx]),
        list(command[separator_idx + 1 :]),
    )


def _validate_runner_and_job(
    command_name: str,
    options: Sequence[str],
    env: Mapping[str, str],
) -> str:
    if env.get("GITHUB_ACTIONS") != "true":
        raise ValueError("qualification requires GITHUB_ACTIONS=true")
    if env.get("GITHUB_REPOSITORY") != REPOSITORY:
        raise ValueError(f"qualification repository must be {REPOSITORY}")
    if env.get("RUNNER_OS") != "Windows":
        raise ValueError("qualification requires a Windows runner")
    if env.get("RUNNER_ENVIRONMENT") != "github-hosted":
        raise ValueError("qualification requires a GitHub-hosted runner")
    if env.get("RUNNER_ARCH") != "X64":
        raise ValueError("qualification requires an X64 runner")
    if env.get("BUILDBUDDY_API_KEY"):
        raise ValueError(
            "execution-context contract applies only to keyless qualification"
        )

    job = _require_env(env, "GITHUB_JOB")
    if job not in QUALIFYING_JOBS:
        raise ValueError(f"unknown keyless Windows gnullvm job {job!r}")

    expected_command = "test" if job == TEST_JOB else "build"
    if command_name != expected_command:
        raise ValueError(
            f"job {job} requires Bazel command {expected_command!r}, "
            f"observed {command_name!r}"
        )

    job_metadata = [
        option
        for option in options
        if option.startswith("--build_metadata=TAG_job=")
    ]
    expected_job_metadata = {
        TEST_JOB: [],
        CLIPPY_JOB: [CLIPPY_JOB_METADATA],
        RELEASE_JOB: [RELEASE_JOB_METADATA],
    }[job]
    if job_metadata != expected_job_metadata:
        raise ValueError(
            f"job {job} has non-canonical job metadata: {job_metadata!r}"
        )

    sha = _require_env(env, "GITHUB_SHA")
    commit_metadata = [
        option for option in options if option.startswith(COMMIT_METADATA_PREFIX)
    ]
    if commit_metadata != [f"{COMMIT_METADATA_PREFIX}{sha}"]:
        raise ValueError(
            f"job {job} requires exact COMMIT_SHA metadata; "
            f"observed {commit_metadata!r}"
        )

    shard_metadata = [
        option for option in options if option.startswith(TEST_SHARD_METADATA_PREFIX)
    ]
    if job == TEST_JOB:
        shard = _require_env(env, "BAZEL_TEST_SHARD")
        if shard not in {"1", "2", "3", "4"}:
            raise ValueError("BAZEL_TEST_SHARD must be one of 1, 2, 3, or 4")
        if env.get("BAZEL_TEST_SHARD_COUNT") != "4":
            raise ValueError("BAZEL_TEST_SHARD_COUNT must equal 4")
        expected_shard = f"{TEST_SHARD_METADATA_PREFIX}{shard}"
        if shard_metadata != [expected_shard]:
            raise ValueError(
                f"test job requires exact shard metadata {expected_shard!r}; "
                f"observed {shard_metadata!r}"
            )
    elif shard_metadata:
        raise ValueError(f"build job {job} rejects test-shard metadata")

    return job


def _validate_startup(startup: Sequence[str], env: Mapping[str, str]) -> None:
    output_root = _require_env(env, "BAZEL_OUTPUT_USER_ROOT")
    workspace = Path(_require_env(env, "GITHUB_WORKSPACE")).resolve(strict=True)
    bazelrc = workspace / ".bazelrc"
    expected = [
        f"--output_user_root={output_root}",
        "--noexperimental_remote_repo_contents_cache",
        "--nomaster_bazelrc",
        "--nosystem_rc",
        "--noworkspace_rc",
        "--nohome_rc",
        f"--bazelrc={bazelrc}",
    ]
    if list(startup) != expected:
        raise ValueError(
            "keyless Windows gnullvm startup arguments are not exact: "
            f"expected {expected!r}, observed {list(startup)!r}"
        )


def _validate_paths(
    command_name: str,
    options: Sequence[str],
    env: Mapping[str, str],
    job: str,
) -> None:
    build_root = _absolute_windows_path(
        _require_env(env, "CI_BUILD_ROOT"), owner="CI_BUILD_ROOT"
    )
    run_id = _require_env(env, "GITHUB_RUN_ID")
    runner_temp = _absolute_windows_path(
        _require_env(env, "RUNNER_TEMP"), owner="RUNNER_TEMP"
    )

    expected_output_root = build_root / "b"
    expected_repository_cache = build_root / "bazel-repository-cache"
    expected_repo_contents = build_root / f"bazel-repo-contents-cache-{run_id}-{job}"
    expected_log_root = runner_temp / "bazel-execution-logs"

    _require_path(env, "BAZEL_OUTPUT_USER_ROOT", expected_output_root)
    _require_path(env, "BAZEL_REPOSITORY_CACHE", expected_repository_cache)
    _require_path(env, "BAZEL_REPO_CONTENTS_CACHE", expected_repo_contents)
    _require_path(
        env,
        "CODEX_BAZEL_EXECUTION_LOG_COMPACT_DIR",
        expected_log_root,
    )

    for name, prefix in (
        ("BAZEL_REPO_CONTENTS_CACHE", "--repo_contents_cache="),
        ("BAZEL_REPOSITORY_CACHE", "--repository_cache="),
    ):
        observed = [option for option in options if option.startswith(prefix)]
        expected = f"{prefix}{env[name]}"
        if observed != [expected]:
            raise ValueError(f"non-canonical {prefix} option: {observed!r}")

    execution_logs = [
        option
        for option in options
        if option.startswith("--execution_log_compact_file=")
    ]
    if len(execution_logs) != 1:
        raise ValueError("exactly one compact execution log path is required")
    log_path = _absolute_windows_path(
        execution_logs[0].split("=", 1)[1],
        owner="compact execution log",
    )
    try:
        relative = log_path.relative_to(expected_log_root)
    except ValueError as error:
        raise ValueError("compact execution log escaped RUNNER_TEMP") from error
    match = _LOG_NAME.fullmatch(relative.name) if len(relative.parts) == 1 else None
    if not match or match.group(1) != command_name or match.group(2) != job:
        raise ValueError(f"non-canonical compact execution log name {relative!s}")


def validate_keyless_windows_gnullvm_execution_context(
    command: Sequence[str],
    env: Mapping[str, str],
    *,
    digest_file: Callable[[Path], str] = _sha256_file,
) -> None:
    """Validate immutable CI context immediately before process launch."""

    _validate_bazelisk_inputs(env)
    startup, command_name, options, targets = _split_command(command)
    if not targets:
        raise ValueError("qualification requires at least one Bazel target")

    job = _validate_runner_and_job(command_name, options, env)
    _validate_startup(startup, env)
    _validate_paths(command_name, options, env, job)

    executable = Path(command[0])
    if not executable.is_absolute():
        raise ValueError("Bazelisk executable path must be absolute")
    if executable.is_symlink() or not executable.is_file():
        raise ValueError("Bazelisk executable must be a regular non-symlink file")
    observed = digest_file(executable)
    if observed != BAZELISK_WINDOWS_X86_64_SHA256:
        raise ValueError(
            "Bazelisk executable SHA-256 drifted before launch: "
            f"expected {BAZELISK_WINDOWS_X86_64_SHA256}, observed {observed}"
        )
