from __future__ import annotations

import hashlib
import sys
import unittest
from pathlib import Path
from tempfile import TemporaryDirectory
from unittest.mock import patch

SCRIPT_DIR = Path(__file__).resolve().parent
sys.path.insert(0, str(SCRIPT_DIR))

import run_bazel_q022_negative_targets as lane_policy
import run_bazel_q027_bazelisk as bazelisk_policy
import run_bazel_q027_executable_contract as subject
import run_bazel_q027_lane as job_policy
import run_bazel_q027_paths as path_policy
import run_bazel_with_buildbuddy as wrapper


class Completed:
    def __init__(self, *, stdout: str = "", stderr: str = "", returncode: int = 0):
        self.stdout = stdout
        self.stderr = stderr
        self.returncode = returncode


class Q027TestCase(unittest.TestCase):
    def fixture(self, job: str) -> tuple[TemporaryDirectory[str], dict[str, str], Path]:
        temp = TemporaryDirectory()
        root = Path(temp.name)
        workspace = root / "workspace"
        home = root / "home"
        workspace.mkdir()
        home.mkdir()
        (workspace / ".bazelversion").write_bytes(subject.BAZELVERSION_BYTES)
        bazelrc = workspace / ".bazelrc"
        bazelrc.write_text("common --config=ci\n", encoding="utf-8")
        env = {
            "GITHUB_ACTIONS": "true",
            "RUNNER_OS": "Windows",
            "RUNNER_ARCH": "X64",
            "RUNNER_ENVIRONMENT": "github-hosted",
            "GITHUB_REPOSITORY": subject.REPOSITORY,
            "GITHUB_JOB": job,
            "GITHUB_SHA": "0123456789abcdef",
            "GITHUB_RUN_ID": "42",
            "GITHUB_WORKSPACE": str(workspace),
            "USERPROFILE": str(home),
            "CI_BUILD_ROOT": "D:",
            "BAZEL_OUTPUT_BASE": "D:/o",
            "BAZEL_OUTPUT_USER_ROOT": "D:/b",
            "BAZEL_REPOSITORY_CACHE": "D:/bazel-repository-cache",
            "BAZEL_REPO_CONTENTS_CACHE": f"D:/bazel-repo-contents-cache-42-{job}",
            "RUNNER_TEMP": "D:/runner-temp",
            "CODEX_BAZEL_EXECUTION_LOG_COMPACT_DIR": "D:/runner-temp/bazel-execution-logs",
            "CODEX_BAZEL_WINDOWS_PATH": "C:/Git/bin;C:/Windows/System32",
            "PATH": str(root / "toolcache"),
        }
        subject.prepare_bazelisk_environment(env)
        if job == subject.TEST_JOB:
            env["BAZEL_TEST_SHARD"] = "2"
            env["BAZEL_TEST_SHARD_COUNT"] = "4"
        return temp, env, bazelrc

    @staticmethod
    def digest(path: Path) -> str:
        return hashlib.sha256(path.read_bytes()).hexdigest()

    def real_bazel(self, root: Path) -> tuple[Path, str]:
        payload = b"verified bazel fixture"
        digest = hashlib.sha256(payload).hexdigest()
        path = root / "downloads" / "sha256" / digest / "bin" / "bazel.exe"
        path.parent.mkdir(parents=True)
        path.write_bytes(payload)
        return path, digest

    def options(self, env: dict[str, str], command_name: str) -> list[str]:
        job = env["GITHUB_JOB"]
        if job == subject.TEST_JOB:
            options = ["--config=ci-windows"]
        elif job == subject.CLIPPY_JOB:
            options = ["--config=clippy", "--config=ci-windows"]
        else:
            options = ["--config=ci-windows"]
        options.extend(subject.CI_EXACT_OPTIONS.values())
        options.extend(
            (
                f"--action_env=PATH={env['CODEX_BAZEL_WINDOWS_PATH']}",
                f"--host_action_env=PATH={env['CODEX_BAZEL_WINDOWS_PATH']}",
                f"--test_env=PATH={env['CODEX_BAZEL_WINDOWS_PATH']}",
                f"--repo_contents_cache={env['BAZEL_REPO_CONTENTS_CACHE']}",
                f"--repository_cache={env['BAZEL_REPOSITORY_CACHE']}",
                "--execution_log_compact_file="
                f"{env['CODEX_BAZEL_EXECUTION_LOG_COMPACT_DIR']}/"
                f"execution-log-{command_name}-{job}-123.zst",
                "--announce_rc",
            )
        )
        if job == subject.TEST_JOB:
            options.extend(sorted(subject.TEST_OPTIONS))
            options.extend(
                (
                    f"--build_metadata=COMMIT_SHA={env['GITHUB_SHA']}",
                    f"--build_metadata=TAG_windows_test_shard={env['BAZEL_TEST_SHARD']}",
                )
            )
        elif job == subject.CLIPPY_JOB:
            options.extend(subject.CLIPPY_OPTIONS)
            options.extend(
                (
                    f"--build_metadata=COMMIT_SHA={env['GITHUB_SHA']}",
                    lane_policy.CLIPPY_JOB_METADATA,
                )
            )
        else:
            options.extend(sorted(subject.RELEASE_OPTIONS))
            options.extend(
                (
                    f"--build_metadata=COMMIT_SHA={env['GITHUB_SHA']}",
                    lane_policy.RELEASE_JOB_METADATA,
                    "--build_metadata=TAG_rust_debug_assertions=off",
                )
            )
        return options

    @staticmethod
    def targets(job: str) -> list[str]:
        if job == subject.TEST_JOB:
            return ["//codex-rs/foo:test", "//sdk/python:smoke"]
        if job == subject.CLIPPY_JOB:
            return [
                "//codex-rs/...",
                lane_policy.CANONICAL_CLIPPY_NEGATIVE_TARGET,
                "//codex-rs/uds:uds-unit-tests-bin",
            ]
        return list(lane_policy.CANONICAL_RELEASE_TARGETS)

    def command(self, env: dict[str, str], executable: Path) -> list[str]:
        job = env["GITHUB_JOB"]
        command_name = "test" if job == subject.TEST_JOB else "build"
        bazelrc = Path(env["GITHUB_WORKSPACE"]) / ".bazelrc"
        startup = [
            f"--output_user_root={env['BAZEL_OUTPUT_USER_ROOT']}",
            "--noexperimental_remote_repo_contents_cache",
            f"--output_base={env['BAZEL_OUTPUT_BASE']}",
            "--nomaster_bazelrc",
            "--nosystem_rc",
            "--noworkspace_rc",
            "--nohome_rc",
            f"--bazelrc={bazelrc.resolve()}",
        ]
        return [
            str(executable),
            *startup,
            command_name,
            *self.options(env, command_name),
            "--",
            *self.targets(job),
        ]

    def validate_job(self, job: str) -> tuple[dict[str, str], list[str], str]:
        temp, env, bazelrc = self.fixture(job)
        self.addCleanup(temp.cleanup)
        executable, digest = self.real_bazel(Path(temp.name))
        command = self.command(env, executable)
        with (
            patch.object(job_policy, "BAZEL_WINDOWS_X86_64_SHA256", digest),
            patch.object(
                path_policy,
                "_qualification_workspace_bazelrc",
                return_value=bazelrc.resolve(),
            ),
        ):
            subject.validate_keyless_windows_gnullvm_command(command, env)
        return env, command, digest
