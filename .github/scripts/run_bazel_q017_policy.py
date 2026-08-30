"""Q0.17 fail-closed Bazel final-command and rc-input policy."""

import hashlib
from collections.abc import Mapping, Sequence
from pathlib import Path

LOCAL_WINDOWS_CI_CONFIG = "--config=ci-windows"
LOCAL_WINDOWS_GNULLVM_TARGET_PLATFORM = "--platforms=//:windows_x86_64_gnullvm"

QUALIFICATION_BAZELRC_GIT_BLOB_SHA1 = "0736ecbb6e8183b31f0e2739abef901c47235e9d"

CI_ALLOWED_CONFIGS = {
    "ci",
    "ci-bazel",
    "ci-windows",
    "clippy",
    "argument-comment-lint",
    "ci-v8",
    "rusty-v8-upstream-libcxx",
    "v8-release-compat",
    "v8-target-x64",
    "v8-target-arm64",
}

CI_EXACT_OPTIONS = {
    "--host_platform=": "--host_platform=//:local_windows_msvc",
    "--platforms=": LOCAL_WINDOWS_GNULLVM_TARGET_PLATFORM,
    "--repo_env=BAZEL_DO_NOT_DETECT_CPP_TOOLCHAIN=": (
        "--repo_env=BAZEL_DO_NOT_DETECT_CPP_TOOLCHAIN=0"
    ),
    "--extra_execution_platforms=": (
        "--extra_execution_platforms=//:windows_x86_64_msvc"
    ),
    "--extra_toolchains=": (
        "--extra_toolchains=//:windows_gnullvm_tests_on_msvc_host_toolchain,"
        "//bazel/toolchains/windows:local_msvc_cc_toolchain"
    ),
    "--strategy=TestRunner=": "--strategy=TestRunner=local",
    "--strategy=V8Mksnapshot=": "--strategy=V8Mksnapshot=local",
    "--local_test_jobs=": "--local_test_jobs=8",
    "--jobs=": "--jobs=8",
    "--test_env=RUST_TEST_THREADS=": "--test_env=RUST_TEST_THREADS=1",
    "--test_env=CODEX_BAZEL_TEST_SKIP_FILTERS=": (
        "--test_env=CODEX_BAZEL_TEST_SKIP_FILTERS="
        "command_safety::powershell_parser::tests::,"
        "suite::code_mode::code_mode_can_call_hidden_dynamic_tools,"
        "tests::windows_tests::conpty_ctrl_c_interrupts_powershell_foreground_child"
    ),
    "--build_metadata=TAG_windows_gnullvm_local=": (
        "--build_metadata=TAG_windows_gnullvm_local=true"
    ),
}

CI_SPLIT_FORM_FORBIDDEN = {
    "--config",
    "--host_platform",
    "--platforms",
    "--repo_env",
    "--extra_execution_platforms",
    "--extra_toolchains",
    "--strategy",
    "--spawn_strategy",
    "--genrule_strategy",
    "--strategy_regexp",
    "--local_test_jobs",
    "--jobs",
    "--test_env",
    "--action_env",
    "--host_action_env",
    "--build_metadata",
    "--remote_executor",
    "--remote_cache",
    "--remote_header",
    "--bes_backend",
    "--bes_results_url",
    "--experimental_remote_downloader",
    "--shell_executable",
}

CI_REMOTE_ENDPOINT_PREFIXES = (
    "--remote_executor=",
    "--remote_cache=",
    "--remote_header=",
    "--bes_backend=",
    "--bes_results_url=",
    "--experimental_remote_downloader=",
    "--remote_proxy=",
    "--remote_instance_name=",
)

CI_REMOTE_EXECUTION_PREFIXES = (
    "--spawn_strategy=",
    "--genrule_strategy=",
    "--strategy_regexp=",
)

CI_WINDOWS_BUILD_ENV_NAMES = {
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

CI_RC_CONTROL_FLAGS = {
    "--system_rc",
    "--nosystem_rc",
    "--workspace_rc",
    "--noworkspace_rc",
    "--home_rc",
    "--nohome_rc",
    "--master_bazelrc",
    "--nomaster_bazelrc",
    "--ignore_all_rc_files",
    "--noignore_all_rc_files",
    "--bazelrc",
}

def _command_index(args: Sequence[str]) -> int:
    return next(
        (idx for idx, arg in enumerate(args) if not arg.startswith("-")),
        len(args),
    )

def _git_blob_sha1(data: bytes) -> str:
    framed = f"blob {len(data)}\0".encode("ascii") + data
    return hashlib.sha1(framed, usedforsecurity=False).hexdigest()

def _qualification_workspace_bazelrc(
    env: Mapping[str, str], *, expected_blob: str = QUALIFICATION_BAZELRC_GIT_BLOB_SHA1
) -> Path:
    workspace_value = env.get("GITHUB_WORKSPACE")
    if not workspace_value:
        raise ValueError(
            "credential-free Windows gnullvm qualification requires GITHUB_WORKSPACE"
        )
    workspace = Path(workspace_value)
    if not workspace.is_absolute():
        raise ValueError("GITHUB_WORKSPACE must be absolute")
    try:
        canonical_workspace = workspace.resolve(strict=True)
    except OSError as error:
        raise ValueError(f"cannot resolve GITHUB_WORKSPACE: {error}") from error
    if not canonical_workspace.is_dir():
        raise ValueError("GITHUB_WORKSPACE must resolve to a directory")

    bazelrc = canonical_workspace / ".bazelrc"
    if bazelrc.is_symlink() or not bazelrc.is_file():
        raise ValueError("qualification .bazelrc must be a regular non-symlink file")
    try:
        data = bazelrc.read_bytes()
    except OSError as error:
        raise ValueError(f"cannot read qualification .bazelrc: {error}") from error
    observed = _git_blob_sha1(data)
    if observed != expected_blob:
        raise ValueError(
            "qualification .bazelrc Git blob drifted: "
            f"expected {expected_blob}, observed {observed}"
        )

    # The pinned .bazelrc ends with a try-import of this workspace-local file.
    # Its absence is therefore part of the exact qualification input set.
    user_bazelrc = canonical_workspace / "user.bazelrc"
    if user_bazelrc.exists() or user_bazelrc.is_symlink():
        raise ValueError(
            "credential-free Windows gnullvm qualification forbids user.bazelrc"
        )
    return bazelrc

def _has_rc_control(arg: str) -> bool:
    name = arg.split("=", 1)[0]
    return name in CI_RC_CONTROL_FLAGS or arg.startswith("--bazelrc=")

def _is_keyless_windows_gnullvm(
    args: Sequence[str], env: Mapping[str, str]
) -> bool:
    if (
        env.get("GITHUB_ACTIONS") != "true"
        or env.get("RUNNER_OS") != "Windows"
        or env.get("BUILDBUDDY_API_KEY")
    ):
        return False
    try:
        separator_idx = args.index("--")
    except ValueError:
        separator_idx = len(args)
    return LOCAL_WINDOWS_GNULLVM_TARGET_PLATFORM in args[:separator_idx]

def _option_args(args: Sequence[str]) -> tuple[int, int, list[str]]:
    command_idx = _command_index(args)
    if command_idx == len(args):
        raise ValueError("expected a Bazel command")
    try:
        separator_idx = args.index("--", command_idx + 1)
    except ValueError as error:
        raise ValueError(
            "credential-free Windows gnullvm qualification requires an exact target separator"
        ) from error
    if separator_idx == len(args) - 1:
        raise ValueError("credential-free Windows gnullvm qualification requires targets")
    return command_idx, separator_idx, list(args[command_idx + 1 : separator_idx])

def _exact_option(options: Sequence[str], prefix: str, expected: str) -> None:
    observed = [arg for arg in options if arg.startswith(prefix)]
    if observed != [expected]:
        raise ValueError(
            "credential-free Windows gnullvm final command requires exactly "
            f"{expected!r}; observed {observed!r}"
        )

def _validate_windows_environment_args(
    options: Sequence[str], env: Mapping[str, str]
) -> None:
    observed_by_prefix: dict[str, dict[str, str]] = {
        "--action_env=": {},
        "--host_action_env=": {},
    }
    for prefix in observed_by_prefix:
        for arg in options:
            if not arg.startswith(prefix):
                continue
            payload = arg[len(prefix) :]
            name, separator, value = payload.partition("=")
            if name == "PATH":
                expected_path = env.get("CODEX_BAZEL_WINDOWS_PATH")
                if not separator or not expected_path or value != expected_path:
                    raise ValueError(f"invalid {prefix}PATH binding")
            elif name in CI_WINDOWS_BUILD_ENV_NAMES:
                if separator or not env.get(name):
                    raise ValueError(f"invalid inherited {prefix}{name} binding")
                value = "<inherited>"
            else:
                raise ValueError(
                    f"credential-free Windows gnullvm qualification rejects {arg!r}"
                )
            if name in observed_by_prefix[prefix]:
                raise ValueError(f"duplicate {prefix}{name} binding")
            observed_by_prefix[prefix][name] = value

    if observed_by_prefix["--action_env="] != observed_by_prefix["--host_action_env="]:
        raise ValueError("action_env and host_action_env bindings must be identical")

    test_env = [arg for arg in options if arg.startswith("--test_env=")]
    allowed_test_env = {
        CI_EXACT_OPTIONS["--test_env=RUST_TEST_THREADS="],
        CI_EXACT_OPTIONS["--test_env=CODEX_BAZEL_TEST_SKIP_FILTERS="],
    }
    windows_path = env.get("CODEX_BAZEL_WINDOWS_PATH")
    if not windows_path:
        raise ValueError("CODEX_BAZEL_WINDOWS_PATH must be set")
    allowed_test_env.add(f"--test_env=PATH={windows_path}")
    if len(test_env) != len(set(test_env)) or set(test_env) != allowed_test_env:
        raise ValueError(
            "credential-free Windows gnullvm qualification has a non-canonical "
            f"test_env set: {test_env!r}"
        )

def validate_keyless_windows_gnullvm_final_args(
    args: Sequence[str], env: Mapping[str, str]
) -> None:
    command_idx, separator_idx, options = _option_args(args)
    if args[command_idx] not in {"build", "test"}:
        raise ValueError(
            "credential-free Windows gnullvm qualification permits only build/test"
        )

    for arg in options:
        if arg in CI_SPLIT_FORM_FORBIDDEN:
            raise ValueError(
                f"credential-free Windows gnullvm qualification rejects split-form {arg!r}"
            )
        if arg.startswith(CI_REMOTE_ENDPOINT_PREFIXES):
            raise ValueError(
                f"credential-free Windows gnullvm qualification rejects remote endpoint {arg!r}"
            )
        if arg.startswith(CI_REMOTE_EXECUTION_PREFIXES):
            raise ValueError(
                f"credential-free Windows gnullvm qualification rejects execution override {arg!r}"
            )
        if arg.startswith("--strategy=") and arg not in {
            CI_EXACT_OPTIONS["--strategy=TestRunner="],
            CI_EXACT_OPTIONS["--strategy=V8Mksnapshot="],
        }:
            raise ValueError(
                f"credential-free Windows gnullvm qualification rejects strategy {arg!r}"
            )

    configs = [arg.removeprefix("--config=") for arg in options if arg.startswith("--config=")]
    if any(config not in CI_ALLOWED_CONFIGS for config in configs):
        raise ValueError(
            f"credential-free Windows gnullvm qualification has non-canonical configs: {configs!r}"
        )
    if configs.count("ci-windows") != 1:
        raise ValueError(
            "credential-free Windows gnullvm qualification requires one "
            "ci-windows config"
        )
    config_positions = [
        index for index, arg in enumerate(options) if arg.startswith("--config=")
    ]
    last_config_position = max(config_positions)
    if options[last_config_position] != LOCAL_WINDOWS_CI_CONFIG:
        raise ValueError("ci-windows must be the final command-line config")

    for prefix, expected in CI_EXACT_OPTIONS.items():
        _exact_option(options, prefix, expected)
        if options.index(expected) <= last_config_position:
            raise ValueError(f"canonical option {expected!r} must follow every config")

    repo_env = [arg for arg in options if arg.startswith("--repo_env=")]
    if repo_env != [CI_EXACT_OPTIONS["--repo_env=BAZEL_DO_NOT_DETECT_CPP_TOOLCHAIN="]]:
        raise ValueError(f"non-canonical explicit repo_env set: {repo_env!r}")

    _validate_windows_environment_args(options, env)

    for env_name, prefix in (
        ("BAZEL_REPO_CONTENTS_CACHE", "--repo_contents_cache="),
        ("BAZEL_REPOSITORY_CACHE", "--repository_cache="),
    ):
        observed = [arg for arg in options if arg.startswith(prefix)]
        expected_value = env.get(env_name)
        expected = [f"{prefix}{expected_value}"] if expected_value else []
        if observed != expected:
            raise ValueError(f"non-canonical {prefix} binding: {observed!r}")

    execution_logs = [
        arg for arg in options if arg.startswith("--execution_log_compact_file=")
    ]
    log_root = env.get("CODEX_BAZEL_EXECUTION_LOG_COMPACT_DIR")
    if log_root:
        if len(execution_logs) != 1:
            raise ValueError("one compact execution log path is required")
        log_path = Path(execution_logs[0].split("=", 1)[1])
        try:
            log_path.relative_to(Path(log_root))
        except ValueError as error:
            raise ValueError("execution log path escapes its configured root") from error
    elif execution_logs:
        raise ValueError("unexpected compact execution log path")

    # Do not allow options to be smuggled into the target payload. Bazel uses
    # a single leading dash for canonical negative target patterns, which the
    # release-build lane needs to exclude bounded first-party targets. Keep
    # those local-workspace exclusions while rejecting every other dash-led
    # payload, including option-shaped values.
    for target in args[separator_idx + 1 :]:
        if target.startswith("-") and not target.startswith("-//"):
            raise ValueError(f"invalid Bazel target payload {target!r}")

def _insert_before_separator(args: Sequence[str], value: str) -> list[str]:
    try:
        separator_idx = args.index("--")
    except ValueError:
        return [*args, value]
    return [*args[:separator_idx], value, *args[separator_idx:]]
