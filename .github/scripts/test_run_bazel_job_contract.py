#!/usr/bin/env python3

import sys
import unittest
from pathlib import Path
from tempfile import TemporaryDirectory
from unittest.mock import patch

SCRIPT_DIR = Path(__file__).resolve().parent
sys.path.insert(0, str(SCRIPT_DIR))

import run_bazel_q023_job_contract as subject


class JobContractTest(unittest.TestCase):
    def fixture(self, job: str) -> tuple[TemporaryDirectory[str], dict[str, str]]:
        temp = TemporaryDirectory()
        root = Path(temp.name)
        workspace = root / "workspace"
        home = root / "home"
        workspace.mkdir()
        home.mkdir()
        (workspace / ".bazelversion").write_bytes(subject.BAZELVERSION_BYTES)
        (workspace / ".bazelrc").write_text("common --config=ci\n")
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
            "CI_BUILD_ROOT": "D:/ci",
            "BAZEL_OUTPUT_USER_ROOT": "D:/ci/b",
            "BAZEL_REPOSITORY_CACHE": "D:/ci/bazel-repository-cache",
            "BAZEL_REPO_CONTENTS_CACHE": f"D:/ci/bazel-repo-contents-cache-42-{job}",
            "RUNNER_TEMP": "D:/runner-temp",
            "CODEX_BAZEL_EXECUTION_LOG_COMPACT_DIR": (
                "D:/runner-temp/bazel-execution-logs"
            ),
            "CODEX_BAZEL_WINDOWS_PATH": "C:/Git/bin;C:/Windows/System32",
            "PATH": "C:/toolcache",
        }
        subject.prepare_bazelisk_environment(env)
        if job == subject.TEST_JOB:
            env["BAZEL_TEST_SHARD"] = "2"
            env["BAZEL_TEST_SHARD_COUNT"] = "4"
        return temp, env

    @staticmethod
    def targets(job: str) -> list[str]:
        if job == subject.TEST_JOB:
            return ["//codex-rs/foo:unit", "//sdk/python:smoke"]
        if job == subject.CLIPPY_JOB:
            return [
                *subject.CLIPPY_TARGET_PREFIX,
                "//codex-rs/uds:uds-unit-tests-bin",
            ]
        return list(subject.CANONICAL_RELEASE_TARGETS)

    def command(self, env: dict[str, str]) -> list[str]:
        job = env["GITHUB_JOB"]
        if job == subject.TEST_JOB:
            command_name = "test"
            configs = ["--config=ci-windows"]
            metadata = [
                f"--build_metadata=COMMIT_SHA={env['GITHUB_SHA']}",
                f"--build_metadata=TAG_windows_test_shard={env['BAZEL_TEST_SHARD']}",
            ]
            job_options = sorted(subject.TEST_OPTIONS)
        elif job == subject.CLIPPY_JOB:
            command_name = "build"
            configs = ["--config=clippy", "--config=ci-windows"]
            metadata = [
                f"--build_metadata=COMMIT_SHA={env['GITHUB_SHA']}",
                "--build_metadata=TAG_job=clippy",
            ]
            job_options = sorted(subject.CLIPPY_OPTIONS)
        else:
            command_name = "build"
            configs = ["--config=ci-windows"]
            metadata = [
                f"--build_metadata=COMMIT_SHA={env['GITHUB_SHA']}",
                "--build_metadata=TAG_job=verify-release-build",
                "--build_metadata=TAG_rust_debug_assertions=off",
            ]
            job_options = sorted(subject.RELEASE_OPTIONS)

        common = list(subject.CI_EXACT_OPTIONS.values())
        path_value = env["CODEX_BAZEL_WINDOWS_PATH"]
        dynamic = [
            f"--action_env=PATH={path_value}",
            f"--host_action_env=PATH={path_value}",
            f"--test_env=PATH={path_value}",
            f"--repo_contents_cache={env['BAZEL_REPO_CONTENTS_CACHE']}",
            f"--repository_cache={env['BAZEL_REPOSITORY_CACHE']}",
            "--execution_log_compact_file="
            f"{env['CODEX_BAZEL_EXECUTION_LOG_COMPACT_DIR']}/"
            f"execution-log-{command_name}-{job}-123.zst",
            "--announce_rc",
        ]
        startup = [
            f"--output_user_root={env['BAZEL_OUTPUT_USER_ROOT']}",
            "--noexperimental_remote_repo_contents_cache",
            "--nomaster_bazelrc",
            "--nosystem_rc",
            "--noworkspace_rc",
            "--nohome_rc",
            f"--bazelrc={Path(env['GITHUB_WORKSPACE']).resolve() / '.bazelrc'}",
        ]
        return [
            str(Path(env["GITHUB_WORKSPACE"]) / "bazel.exe"),
            *startup,
            command_name,
            *configs,
            *metadata,
            *job_options,
            *common,
            *dynamic,
            "--",
            *self.targets(job),
        ]

    def validate(self, job: str) -> tuple[dict[str, str], list[str]]:
        temp, env = self.fixture(job)
        self.addCleanup(temp.cleanup)
        command = self.command(env)
        with patch.object(subject, "_validate_q017"):
            subject.validate_keyless_windows_gnullvm_command(command, env)
        return env, command

    def test_all_three_canonical_jobs_pass(self) -> None:
        for job in sorted(subject.QUALIFYING_JOBS):
            with self.subTest(job=job):
                self.validate(job)

    def test_release_job_cannot_use_test_command(self) -> None:
        env, command = self.validate(subject.RELEASE_JOB)
        command[command.index("build")] = "test"
        with patch.object(subject, "_validate_q017"):
            with self.assertRaisesRegex(ValueError, "requires Bazel command"):
                subject.validate_keyless_windows_gnullvm_command(command, env)

    def test_clippy_cannot_claim_release_metadata(self) -> None:
        env, command = self.validate(subject.CLIPPY_JOB)
        command.insert(
            command.index("--"),
            "--build_metadata=TAG_job=verify-release-build",
        )
        with patch.object(subject, "_validate_q017"):
            with self.assertRaisesRegex(ValueError, "exact build metadata"):
                subject.validate_keyless_windows_gnullvm_command(command, env)

    def test_clippy_rejects_extra_allowlisted_config(self) -> None:
        env, command = self.validate(subject.CLIPPY_JOB)
        command.insert(command.index("--config=ci-windows"), "--config=ci-v8")
        with patch.object(subject, "_validate_q017"):
            with self.assertRaisesRegex(ValueError, "exact configs"):
                subject.validate_keyless_windows_gnullvm_command(command, env)

    def test_test_shard_rejects_negative_target(self) -> None:
        env, command = self.validate(subject.TEST_JOB)
        command.append("-//codex-rs/core:all")
        with patch.object(subject, "_validate_q017"):
            with self.assertRaisesRegex(ValueError, "non-positive Bazel target"):
                subject.validate_keyless_windows_gnullvm_command(command, env)

    def test_clippy_allows_only_its_canonical_negative_target(self) -> None:
        env, command = self.validate(subject.CLIPPY_JOB)
        command.append("-//codex-rs/core:all")
        with patch.object(subject, "_validate_q017"):
            with self.assertRaisesRegex(ValueError, "outside //codex-rs"):
                subject.validate_keyless_windows_gnullvm_command(command, env)

    def test_release_payload_is_exact(self) -> None:
        env, command = self.validate(subject.RELEASE_JOB)
        command[-1] = "-//codex-rs/core:all"
        with patch.object(subject, "_validate_q017"):
            with self.assertRaisesRegex(ValueError, "exact canonical release"):
                subject.validate_keyless_windows_gnullvm_command(command, env)

    def test_unknown_job_fails_closed(self) -> None:
        temp, env = self.fixture(subject.CLIPPY_JOB)
        self.addCleanup(temp.cleanup)
        command = self.command(env)
        env["GITHUB_JOB"] = "attacker-job"
        with self.assertRaisesRegex(ValueError, "unknown keyless"):
            subject.validate_keyless_windows_gnullvm_command(command, env)

    def test_unclassified_option_fails_closed(self) -> None:
        env, command = self.validate(subject.CLIPPY_JOB)
        command.insert(command.index("--"), "--build_tag_filters=-slow")
        with patch.object(subject, "_validate_q017"):
            with self.assertRaisesRegex(ValueError, "unclassified Bazel option"):
                subject.validate_keyless_windows_gnullvm_command(command, env)

    def test_additional_startup_option_fails_closed(self) -> None:
        env, command = self.validate(subject.CLIPPY_JOB)
        command.insert(1, "--host_jvm_args=-Xmx32g")
        with patch.object(subject, "_validate_q017"):
            with self.assertRaisesRegex(ValueError, "startup arguments are not exact"):
                subject.validate_keyless_windows_gnullvm_command(command, env)

    def test_cache_root_drift_fails_closed(self) -> None:
        env, command = self.validate(subject.CLIPPY_JOB)
        env["BAZEL_REPOSITORY_CACHE"] = "C:/attacker-cache"
        with patch.object(subject, "_validate_q017"):
            with self.assertRaisesRegex(
                ValueError, "escaped its runner-controlled root"
            ):
                subject.validate_keyless_windows_gnullvm_command(command, env)

    def test_bazelisk_override_fails_closed(self) -> None:
        temp, env = self.fixture(subject.CLIPPY_JOB)
        self.addCleanup(temp.cleanup)
        env["BAZELISK_BASE_URL"] = "https://example.invalid"
        with self.assertRaisesRegex(ValueError, "BAZELISK_BASE_URL"):
            subject.prepare_bazelisk_environment(env)

    def test_conflicting_version_override_fails_closed(self) -> None:
        temp, env = self.fixture(subject.CLIPPY_JOB)
        self.addCleanup(temp.cleanup)
        env["USE_BAZEL_VERSION"] = "latest"
        with self.assertRaisesRegex(ValueError, "conflicts"):
            subject.prepare_bazelisk_environment(env)

    def test_bazelversion_drift_fails_closed(self) -> None:
        temp, env = self.fixture(subject.CLIPPY_JOB)
        self.addCleanup(temp.cleanup)
        Path(env["GITHUB_WORKSPACE"], ".bazelversion").write_text("latest\n")
        with self.assertRaisesRegex(ValueError, "bytes drifted"):
            subject.bind_verified_bazelisk(["bazel", "build"], env)

    def test_verified_bazelisk_is_replaced_with_absolute_path(self) -> None:
        temp, env = self.fixture(subject.CLIPPY_JOB)
        self.addCleanup(temp.cleanup)
        executable = Path(temp.name) / "bazel.exe"
        executable.write_bytes(b"bazelisk fixture")
        command = ["bazel", "build"]
        with patch.object(
            subject,
            "BAZELISK_WINDOWS_X86_64_SHA256",
            subject._sha256_file(executable),
        ):
            bound = subject.bind_verified_bazelisk(
                command,
                env,
                which=lambda *_args, **_kwargs: str(executable),
            )
        self.assertEqual(bound[0], str(executable.resolve()))

    def test_bazelisk_digest_drift_fails_closed(self) -> None:
        temp, env = self.fixture(subject.CLIPPY_JOB)
        self.addCleanup(temp.cleanup)
        executable = Path(temp.name) / "bazel.exe"
        executable.write_bytes(b"not the official asset")
        with self.assertRaisesRegex(ValueError, "SHA-256 drifted"):
            subject.bind_verified_bazelisk(
                ["bazel", "build"],
                env,
                which=lambda *_args, **_kwargs: str(executable),
            )


if __name__ == "__main__":
    unittest.main()
