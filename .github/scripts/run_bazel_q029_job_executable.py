"""Q0.29 fail-closed GitHub job and Bazel executable authority."""

from __future__ import annotations

import hashlib
import re
import shutil
from collections.abc import Callable, Mapping, MutableMapping, Sequence
from pathlib import Path, PureWindowsPath

from run_bazel_q022_negative_targets import CANONICAL_RELEASE_TARGETS
from run_bazel_q022_negative_targets import CLIPPY_JOB_METADATA
from run_bazel_q022_negative_targets import RELEASE_JOB_METADATA
from run_bazel_q028_startup_contract import (
    validate_keyless_windows_gnullvm_final_args as _validate_q028,
)

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

CLIPPY_TARGET_PREFIX = (
    "//codex-rs/...",
    "-//codex-rs/v8-poc:all",
)
STRICT_STARTUP_FLAGS = (
    "--nomaster_bazelrc",
    "--nosystem_rc",
    "--noworkspace_rc",
    "--nohome_rc",
)
DISABLED_REPO_CONTENTS_CACHE = "--noexperimental_remote_repo_contents_cache"
OUTPUT_USER_ROOT_PREFIX = "--output_user_root="
COMMIT_METADATA_PREFIX = "--build_metadata=COMMIT_SHA="
SHARD_METADATA_PREFIX = "--build_metadata=TAG_windows_test_shard="
WINDOWS_LOCAL_METADATA = "--build_metadata=TAG_windows_gnullvm_local=true"

BAZELISK_REQUIRED_ENV = {
    "USE_BAZEL_VERSION": BAZEL_VERSION,
    "BAZELISK_VERIFY_SHA256": BAZEL_WINDOWS_X86_64_SHA256,
    "BAZELISK_SKIP_WRAPPER": "true",
}
BAZELISK_DIRECT_OVERRIDE_ENV = frozenset(
    (
        "BAZEL_REAL",
        "BAZEL_WRAPPER",
        "CODEX_BAZEL_BIN",
        "USE_BAZEL_FALLBACK_VERSION",
    )
)
LOWERCASE_SHA1 = re.compile(r"[0-9a-f]{40}\Z")
EXECUTION_LOG_NAME = re.compile(
    r"execution-log-(build|test)-([A-Za-z0-9_.-]+)-([0-9]+)\.zst\Z"
)


def _git_blob_sha1(data: bytes) -> str:
    framed = f"blob {len(data)}\0".encode("ascii") + data
    return hashlib.sha1(framed, usedforsecurity=False).hexdigest()


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


def _windows_path(value: str) -> str:
    return str(PureWindowsPath(value)).rstrip("\\").casefold()


def _absolute_windows_path(
    value: str,
    *,
    owner: str,
    allow_drive_root_shorthand: bool = False,
) -> PureWindowsPath:
    candidate = value
    if allow_drive_root_shorthand and re.fullmatch(r"[A-Za-z]:", candidate):
        candidate += "\\"
    path = PureWindowsPath(candidate)
    if not path.is_absolute() or ".." in path.parts:
        raise ValueError(f"{owner} must be an absolute normalized Windows path")
    return path


def _split_command(
    command: Sequence[str],
) -> tuple[list[str], str, list[str], list[str]]:
    if len(command) < 3:
        raise ValueError("expected Bazel executable, command, options, and targets")
    command_idx = next(
        (
            index
            for index, arg in enumerate(command[1:], start=1)
            if not arg.startswith("-")
        ),
        len(command),
    )
    if command_idx == len(command):
        raise ValueError("expected a Bazel command")
    try:
        separator_idx = command.index("--", command_idx + 1)
    except ValueError as error:
        raise ValueError("expected one Bazel target separator") from error
    if "--" in command[separator_idx + 1 :]:
        raise ValueError("multiple Bazel target separators are forbidden")
    return (
        list(command[1:command_idx]),
        command[command_idx],
        list(command[command_idx + 1 : separator_idx]),
        list(command[separator_idx + 1 :]),
    )


def prepare_bazelisk_environment(env: MutableMapping[str, str]) -> None:
    """Reject ambient executable-selection controls and install exact ones."""

    for name in sorted(BAZELISK_DIRECT_OVERRIDE_ENV):
        if env.get(name):
            raise ValueError(f"Bazel executable override {name} is forbidden")

    # setup-bazel exports the GitHub token used only to authenticate its
    # official Bazelisk release lookup. It cannot select a URL or artifact,
    # and both the Bazelisk and Bazel bytes are independently digest-bound.
    allowed_bazelisk_names = {
        "BAZELISK_GITHUB_TOKEN",
        "BAZELISK_VERIFY_SHA256",
        "BAZELISK_SKIP_WRAPPER",
    }
    for name, value in sorted(env.items()):
        if (
            name.startswith("BAZELISK_")
            and name not in allowed_bazelisk_names
            and value
        ):
            raise ValueError(f"Bazelisk override {name} is forbidden")

    for name, expected in BAZELISK_REQUIRED_ENV.items():
        observed = env.get(name)
        if observed not in {None, "", expected}:
            raise ValueError(
                f"Bazelisk override {name} conflicts with required value "
                f"{expected!r}"
            )
        env[name] = expected


def _validate_bazelisk_inputs(env: Mapping[str, str]) -> Path:
    for name in sorted(BAZELISK_DIRECT_OVERRIDE_ENV):
        if env.get(name):
            raise ValueError(f"Bazel executable override {name} is forbidden")

    # setup-bazel exports the GitHub token used only to authenticate its
    # official Bazelisk release lookup. It cannot select a URL or artifact,
    # and both the Bazelisk and Bazel bytes are independently digest-bound.
    allowed_bazelisk_names = {
        "BAZELISK_GITHUB_TOKEN",
        "BAZELISK_VERIFY_SHA256",
        "BAZELISK_SKIP_WRAPPER",
    }
    for name, value in sorted(env.items()):
        if (
            name.startswith("BAZELISK_")
            and name not in allowed_bazelisk_names
            and value
        ):
            raise ValueError(f"Bazelisk override {name} is forbidden")

    for name, expected in BAZELISK_REQUIRED_ENV.items():
        if env.get(name) != expected:
            raise ValueError(f"{name} must equal {expected!r}")

    workspace = Path(_require_env(env, "GITHUB_WORKSPACE"))
    try:
        workspace = workspace.resolve(strict=True)
    except OSError as error:
        raise ValueError(f"cannot resolve GITHUB_WORKSPACE: {error}") from error
    if not workspace.is_dir():
        raise ValueError("GITHUB_WORKSPACE must resolve to a directory")

    bazelversion = workspace / ".bazelversion"
    if bazelversion.is_symlink() or not bazelversion.is_file():
        raise ValueError(".bazelversion must be a regular non-symlink file")
    data = bazelversion.read_bytes()
    if data != BAZELVERSION_BYTES:
        raise ValueError(".bazelversion bytes drifted from 9.0.0")
    if _git_blob_sha1(data) != BAZELVERSION_GIT_BLOB_SHA1:
        raise ValueError(".bazelversion Git blob identity drifted")

    for wrapper in sorted((workspace / "tools").glob("bazel*")):
        raise ValueError(
            "workspace Bazel wrapper surface is forbidden: "
            f"{wrapper.relative_to(workspace)}"
        )

    workspace_rc = workspace / ".bazeliskrc"
    if workspace_rc.exists() or workspace_rc.is_symlink():
        raise ValueError("workspace .bazeliskrc is forbidden")

    home_value = env.get("USERPROFILE") or env.get("HOME")
    if not home_value:
        raise ValueError("runner home is required to reject user .bazeliskrc")
    home_rc = Path(home_value) / ".bazeliskrc"
    if home_rc.exists() or home_rc.is_symlink():
        raise ValueError("runner-home .bazeliskrc is forbidden")
    return workspace


def bind_verified_bazelisk(
    command: Sequence[str],
    env: Mapping[str, str],
    *,
    which: Callable[..., str | None] = shutil.which,
    digest_file: Callable[[Path], str] = _sha256_file,
) -> list[str]:
    """Replace argv[0] with the verified official Bazelisk executable."""

    if not command or command[0].casefold() not in {"bazel", "bazel.exe"}:
        raise ValueError("unverified Bazel argv[0] is forbidden")
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


def _validate_runner_identity(env: Mapping[str, str]) -> str:
    if env.get("GITHUB_ACTIONS") != "true":
        raise ValueError("qualification requires GitHub Actions")
    if env.get("GITHUB_REPOSITORY") != REPOSITORY:
        raise ValueError(f"qualification repository must be {REPOSITORY}")
    if env.get("RUNNER_OS") != "Windows":
        raise ValueError("qualification requires a Windows runner")
    if env.get("RUNNER_ENVIRONMENT") != "github-hosted":
        raise ValueError("qualification requires a GitHub-hosted runner")
    if env.get("RUNNER_ARCH") != "X64":
        raise ValueError("qualification requires an X64 runner")
    if env.get("GITHUB_EVENT_NAME") not in {"pull_request", "push"}:
        raise ValueError("qualification requires pull_request or push execution")

    sha = _require_env(env, "GITHUB_SHA")
    if not LOWERCASE_SHA1.fullmatch(sha):
        raise ValueError("GITHUB_SHA must be one lowercase 40-hex Git object ID")

    job = _require_env(env, "GITHUB_JOB")
    if job not in QUALIFYING_JOBS:
        raise ValueError(f"unknown keyless Windows gnullvm job {job!r}")
    return job


def _validate_startup(
    startup: Sequence[str],
    env: Mapping[str, str],
    workspace: Path,
) -> None:
    output_root = _require_env(env, "BAZEL_OUTPUT_USER_ROOT")
    expected = [
        f"{OUTPUT_USER_ROOT_PREFIX}{output_root}",
        DISABLED_REPO_CONTENTS_CACHE,
        *STRICT_STARTUP_FLAGS,
        f"--bazelrc={workspace / '.bazelrc'}",
    ]
    if list(startup) != expected:
        raise ValueError(
            "keyless Windows gnullvm startup arguments are not exact: "
            f"expected {expected!r}, observed {list(startup)!r}"
        )


def _validate_paths(
    options: Sequence[str],
    env: Mapping[str, str],
    job: str,
) -> None:
    build_root = _absolute_windows_path(
        _require_env(env, "CI_BUILD_ROOT"),
        owner="CI_BUILD_ROOT",
        allow_drive_root_shorthand=True,
    )
    if len(build_root.parts) != 1:
        raise ValueError("CI_BUILD_ROOT must be a dedicated Windows drive root")

    run_id = _require_env(env, "GITHUB_RUN_ID")
    if not run_id.isdecimal():
        raise ValueError("GITHUB_RUN_ID must be decimal")
    runner_temp = _absolute_windows_path(
        _require_env(env, "RUNNER_TEMP"),
        owner="RUNNER_TEMP",
    )

    expected_env = {
        "BAZEL_OUTPUT_BASE": str(build_root / "o"),
        "BAZEL_OUTPUT_USER_ROOT": str(build_root / "b"),
        "BAZEL_REPOSITORY_CACHE": str(
            build_root / "bazel-repository-cache"
        ),
        "BAZEL_REPO_CONTENTS_CACHE": str(
            build_root / f"bazel-repo-contents-cache-{run_id}-{job}"
        ),
        "CARGO_TARGET_DIR": str(build_root / "cargo-target"),
        "TEMP": str(build_root / "tmp"),
        "TMP": str(build_root / "tmp"),
        "CODEX_BAZEL_EXECUTION_LOG_COMPACT_DIR": str(
            runner_temp / "bazel-execution-logs"
        ),
    }
    for name, expected in expected_env.items():
        actual = _require_env(env, name)
        if _windows_path(actual) != _windows_path(expected):
            raise ValueError(
                f"{name} escaped its runner-controlled root: "
                f"expected {expected!r}, observed {actual!r}"
            )

    expected_options = {
        "--repo_contents_cache=": env["BAZEL_REPO_CONTENTS_CACHE"],
        "--repository_cache=": env["BAZEL_REPOSITORY_CACHE"],
    }
    for prefix, value in expected_options.items():
        observed = [option for option in options if option.startswith(prefix)]
        if observed != [f"{prefix}{value}"]:
            raise ValueError(f"non-canonical {prefix} option: {observed!r}")

    execution_logs = [
        option
        for option in options
        if option.startswith("--execution_log_compact_file=")
    ]
    if len(execution_logs) != 1:
        raise ValueError("exactly one compact execution log path is required")
    log_value = execution_logs[0].split("=", 1)[1]
    log_path = _absolute_windows_path(
        log_value,
        owner="compact execution log",
    )
    log_root = _absolute_windows_path(
        env["CODEX_BAZEL_EXECUTION_LOG_COMPACT_DIR"],
        owner="compact execution log root",
    )
    try:
        relative = log_path.relative_to(log_root)
    except ValueError as error:
        raise ValueError("compact execution log escaped RUNNER_TEMP") from error
    match = EXECUTION_LOG_NAME.fullmatch(relative.name)
    if (
        len(relative.parts) != 1
        or match is None
        or match.group(2) != job
    ):
        raise ValueError(
            f"non-canonical compact execution log name {relative!s}"
        )


def _validate_job_binding(
    command_name: str,
    options: Sequence[str],
    targets: Sequence[str],
    env: Mapping[str, str],
    job: str,
) -> None:
    configs = tuple(
        option.removeprefix("--config=")
        for option in options
        if option.startswith("--config=")
    )
    metadata = [
        option for option in options if option.startswith("--build_metadata=")
    ]
    common_metadata = {
        f"{COMMIT_METADATA_PREFIX}{env['GITHUB_SHA']}",
        WINDOWS_LOCAL_METADATA,
    }

    if job == TEST_JOB:
        if command_name != "test" or configs != ("ci-windows",):
            raise ValueError("test-windows-shard requires test and exact ci-windows")
        shard = _require_env(env, "BAZEL_TEST_SHARD")
        if shard not in {"1", "2", "3", "4"}:
            raise ValueError("BAZEL_TEST_SHARD must be one of 1, 2, 3, or 4")
        if env.get("BAZEL_TEST_SHARD_COUNT") != "4":
            raise ValueError("BAZEL_TEST_SHARD_COUNT must equal 4")
        expected_metadata = common_metadata | {
            f"{SHARD_METADATA_PREFIX}{shard}"
        }
        if not targets or any(
            target.startswith("-") or not target.startswith("//")
            for target in targets
        ):
            raise ValueError("test shard requires positive workspace targets")
    elif job == CLIPPY_JOB:
        if command_name != "build" or configs != ("clippy", "ci-windows"):
            raise ValueError("clippy job requires build and exact clippy,ci-windows")
        expected_metadata = common_metadata | {CLIPPY_JOB_METADATA}
        if tuple(targets[:2]) != CLIPPY_TARGET_PREFIX:
            raise ValueError(
                f"clippy requires target prefix {CLIPPY_TARGET_PREFIX!r}"
            )
        for target in targets[2:]:
            if (
                target.startswith("-")
                or not target.startswith("//codex-rs/")
                or "/v8-poc:" in target
            ):
                raise ValueError(f"clippy rejects target {target!r}")
    else:
        if command_name != "build" or configs != ("ci-windows",):
            raise ValueError(
                "verify-release-build requires build and exact ci-windows"
            )
        expected_metadata = common_metadata | {
            RELEASE_JOB_METADATA,
            "--build_metadata=TAG_rust_debug_assertions=off",
        }
        if tuple(targets) != tuple(CANONICAL_RELEASE_TARGETS):
            raise ValueError(
                "release job requires the exact canonical release target payload"
            )

    if len(metadata) != len(set(metadata)) or set(metadata) != expected_metadata:
        raise ValueError(
            f"job {job} requires exact build metadata "
            f"{sorted(expected_metadata)!r}; observed {metadata!r}"
        )
    if len(targets) != len(set(targets)):
        raise ValueError(f"job {job} rejects duplicate Bazel targets")


def validate_keyless_windows_gnullvm_command(
    command: Sequence[str],
    env: Mapping[str, str],
    *,
    digest_file: Callable[[Path], str] = _sha256_file,
) -> None:
    """Validate the complete executable command immediately before launch."""

    workspace = _validate_bazelisk_inputs(env)
    job = _validate_runner_identity(env)
    startup, command_name, options, targets = _split_command(command)

    executable = Path(command[0])
    if not executable.is_absolute():
        raise ValueError("Bazelisk executable path must be absolute")
    if executable.is_symlink() or not executable.is_file():
        raise ValueError(
            "Bazelisk executable must be a regular non-symlink file"
        )
    observed = digest_file(executable)
    if observed != BAZELISK_WINDOWS_X86_64_SHA256:
        raise ValueError(
            "Bazelisk executable SHA-256 drifted before launch: "
            f"expected {BAZELISK_WINDOWS_X86_64_SHA256}, observed {observed}"
        )

    _validate_q028(command[1:], env)
    _validate_startup(startup, env, workspace)
    _validate_paths(options, env, job)
    _validate_job_binding(command_name, options, targets, env, job)
