"""Fail-closed Q0.23 job, target, and Bazel executable contract."""

from __future__ import annotations

import hashlib
import re
import shutil
from collections.abc import Callable, Mapping, MutableMapping, Sequence
from pathlib import Path, PureWindowsPath

from run_bazel_q017_policy import CI_EXACT_OPTIONS
from run_bazel_q017_policy import _qualification_workspace_bazelrc
from run_bazel_q017_policy import (
    validate_keyless_windows_gnullvm_final_args as _validate_q017,
)
from run_bazel_q022_negative_targets import CANONICAL_RELEASE_TARGETS

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

CLIPPY_TARGET_PREFIX = (
    "//codex-rs/...",
    "-//codex-rs/v8-poc:all",
)
TEST_OPTIONS = {
    "--skip_incompatible_explicit_targets",
    "--test_tag_filters=-argument-comment-lint",
    "--test_verbose_timeout_warnings",
    "--remote_download_toplevel",
}
CLIPPY_OPTIONS = {"--skip_incompatible_explicit_targets"}
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


def _git_blob_sha1(data: bytes) -> str:
    payload = f"blob {len(data)}\0".encode("ascii") + data
    return hashlib.sha1(payload, usedforsecurity=False).hexdigest()


def _sha256_file(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as source:
        for chunk in iter(lambda: source.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def _windows_path(value: str) -> str:
    return str(PureWindowsPath(value)).rstrip("\\").casefold()


def _require_env(env: Mapping[str, str], name: str) -> str:
    value = env.get(name)
    if not value:
        raise ValueError(f"keyless Windows gnullvm qualification requires {name}")
    return value


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


def legacy_policy_args(args: Sequence[str], env: Mapping[str, str]) -> list[str]:
    """Return a Q0.17-compatible view while Q0.23 checks the real target payload."""
    try:
        separator_idx = args.index("--")
    except ValueError:
        return list(args)
    if env.get("GITHUB_JOB") == RELEASE_JOB:
        return list(args)
    return [
        *args[: separator_idx + 1],
        *(
            target
            for target in args[separator_idx + 1 :]
            if not target.startswith("-//")
        ),
    ]


def prepare_bazelisk_environment(env: MutableMapping[str, str]) -> None:
    """Install exact Bazelisk controls after rejecting ambient overrides."""
    if env.get("CODEX_BAZEL_BIN"):
        raise ValueError("CODEX_BAZEL_BIN is forbidden in qualifying GitHub jobs")
    for name in sorted(BAZELISK_FORBIDDEN_ENV):
        if env.get(name):
            raise ValueError(f"Bazelisk override {name} is forbidden")
    for name, expected in BAZELISK_REQUIRED_ENV.items():
        observed = env.get(name)
        if observed not in {None, "", expected}:
            raise ValueError(
                f"Bazelisk override {name} conflicts with required value {expected!r}"
            )
        env[name] = expected


def _validate_bazelisk_inputs(env: Mapping[str, str]) -> None:
    if env.get("CODEX_BAZEL_BIN"):
        raise ValueError("CODEX_BAZEL_BIN is forbidden in qualifying GitHub jobs")
    for name, expected in BAZELISK_REQUIRED_ENV.items():
        if env.get(name) != expected:
            raise ValueError(f"{name} must equal {expected!r}")
    for name in sorted(BAZELISK_FORBIDDEN_ENV):
        if env.get(name):
            raise ValueError(f"Bazelisk override {name} is forbidden")

    workspace = Path(_require_env(env, "GITHUB_WORKSPACE")).resolve(strict=True)
    bazelversion = workspace / ".bazelversion"
    if bazelversion.is_symlink() or not bazelversion.is_file():
        raise ValueError(".bazelversion must be a regular non-symlink file")
    data = bazelversion.read_bytes()
    if data != BAZELVERSION_BYTES:
        raise ValueError(".bazelversion bytes drifted from 9.0.0")
    if _git_blob_sha1(data) != BAZELVERSION_GIT_BLOB_SHA1:
        raise ValueError(".bazelversion Git blob identity drifted")
    if (workspace / ".bazeliskrc").exists():
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


def _validate_startup(startup: Sequence[str], env: Mapping[str, str]) -> None:
    output_root = _require_env(env, "BAZEL_OUTPUT_USER_ROOT")
    bazelrc = _qualification_workspace_bazelrc(env)
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


def _validate_paths(options: Sequence[str], env: Mapping[str, str], job: str) -> None:
    build_root = _require_env(env, "CI_BUILD_ROOT")
    run_id = _require_env(env, "GITHUB_RUN_ID")
    runner_temp = _require_env(env, "RUNNER_TEMP")
    expected_env = {
        "BAZEL_OUTPUT_USER_ROOT": str(PureWindowsPath(build_root) / "b"),
        "BAZEL_REPOSITORY_CACHE": str(
            PureWindowsPath(build_root) / "bazel-repository-cache"
        ),
        "BAZEL_REPO_CONTENTS_CACHE": str(
            PureWindowsPath(build_root)
            / f"bazel-repo-contents-cache-{run_id}-{job}"
        ),
        "CODEX_BAZEL_EXECUTION_LOG_COMPACT_DIR": str(
            PureWindowsPath(runner_temp) / "bazel-execution-logs"
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
    log_path = PureWindowsPath(log_value)
    log_root = PureWindowsPath(env["CODEX_BAZEL_EXECUTION_LOG_COMPACT_DIR"])
    try:
        relative = log_path.relative_to(log_root)
    except ValueError as error:
        raise ValueError("compact execution log escaped RUNNER_TEMP") from error
    expected_name = re.compile(
        rf"execution-log-(build|test)-{re.escape(job)}-[0-9]+\.zst"
    )
    if len(relative.parts) != 1 or not expected_name.fullmatch(relative.name):
        raise ValueError(f"non-canonical compact execution log name {relative!s}")


def _job_spec(
    job: str,
    env: Mapping[str, str],
) -> tuple[str, tuple[str, ...], set[str], set[str]]:
    sha = _require_env(env, "GITHUB_SHA")
    common_metadata = {
        f"--build_metadata=COMMIT_SHA={sha}",
        "--build_metadata=TAG_windows_gnullvm_local=true",
    }
    if job == TEST_JOB:
        shard = _require_env(env, "BAZEL_TEST_SHARD")
        if shard not in {"1", "2", "3", "4"}:
            raise ValueError("BAZEL_TEST_SHARD must be one of 1, 2, 3, or 4")
        if env.get("BAZEL_TEST_SHARD_COUNT") != "4":
            raise ValueError("BAZEL_TEST_SHARD_COUNT must equal 4")
        metadata = common_metadata | {
            f"--build_metadata=TAG_windows_test_shard={shard}"
        }
        return "test", ("ci-windows",), metadata, TEST_OPTIONS
    if job == CLIPPY_JOB:
        metadata = common_metadata | {"--build_metadata=TAG_job=clippy"}
        return "build", ("clippy", "ci-windows"), metadata, CLIPPY_OPTIONS
    metadata = common_metadata | {
        "--build_metadata=TAG_job=verify-release-build",
        "--build_metadata=TAG_rust_debug_assertions=off",
    }
    return "build", ("ci-windows",), metadata, RELEASE_OPTIONS


def _validate_job_options(
    command_name: str,
    options: Sequence[str],
    env: Mapping[str, str],
    job: str,
) -> None:
    expected_command, expected_configs, expected_metadata, job_options = _job_spec(
        job, env
    )
    if command_name != expected_command:
        raise ValueError(
            f"job {job} requires Bazel command {expected_command!r}, "
            f"observed {command_name!r}"
        )

    configs = tuple(
        option.removeprefix("--config=")
        for option in options
        if option.startswith("--config=")
    )
    if configs != expected_configs:
        raise ValueError(
            f"job {job} requires exact configs {expected_configs!r}, "
            f"observed {configs!r}"
        )

    metadata = [
        option for option in options if option.startswith("--build_metadata=")
    ]
    if len(metadata) != len(set(metadata)) or set(metadata) != expected_metadata:
        raise ValueError(
            f"job {job} requires exact build metadata {sorted(expected_metadata)!r}, "
            f"observed {metadata!r}"
        )

    common_options = set(CI_EXACT_OPTIONS.values()) | {"--announce_rc"}
    allowed_exact = common_options | job_options | expected_metadata
    for option in options:
        if option.startswith("--config="):
            continue
        if option in allowed_exact:
            continue
        if option.startswith(_DYNAMIC_OPTION_PREFIXES):
            continue
        raise ValueError(f"job {job} rejects unclassified Bazel option {option!r}")

    for required in job_options:
        if options.count(required) != 1:
            raise ValueError(f"job {job} requires exactly one {required!r}")


def _validate_targets(targets: Sequence[str], job: str) -> None:
    if not targets:
        raise ValueError(f"job {job} requires at least one Bazel target")
    if len(targets) != len(set(targets)):
        raise ValueError(f"job {job} rejects duplicate Bazel targets")

    if job == RELEASE_JOB:
        if tuple(targets) != CANONICAL_RELEASE_TARGETS:
            raise ValueError(
                "release job requires the exact canonical release target payload"
            )
        return
    if job == TEST_JOB:
        invalid = [
            target
            for target in targets
            if target.startswith("-") or not target.startswith("//")
        ]
        if invalid:
            raise ValueError(
                f"test shard rejects non-positive Bazel target {invalid!r}"
            )
        return

    if tuple(targets[:2]) != CLIPPY_TARGET_PREFIX:
        raise ValueError(
            f"clippy requires target prefix {CLIPPY_TARGET_PREFIX!r}"
        )
    for target in targets[2:]:
        if not target.startswith("//codex-rs/"):
            raise ValueError(f"clippy rejects target outside //codex-rs: {target!r}")
        if "/v8-poc:" in target:
            raise ValueError(f"clippy rejects direct v8-poc target {target!r}")
        if target.startswith("-"):
            raise ValueError(f"clippy rejects additional negative target {target!r}")


def validate_keyless_windows_gnullvm_command(
    command: Sequence[str], env: Mapping[str, str]
) -> None:
    """Validate the exact executable command immediately before process launch."""
    _validate_bazelisk_inputs(env)
    job = _validate_runner_identity(env)
    startup, command_name, options, targets = _split_command(command)
    executable = Path(command[0])
    if not executable.is_absolute():
        raise ValueError("Bazelisk executable path must be absolute")
    _validate_startup(startup, env)
    _validate_q017(legacy_policy_args(command[1:], env), env)
    _validate_paths(options, env, job)
    _validate_job_options(command_name, options, env, job)
    _validate_targets(targets, job)
