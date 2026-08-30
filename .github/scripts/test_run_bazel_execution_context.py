#!/usr/bin/env python3

import sys
import unittest
from pathlib import Path
from tempfile import TemporaryDirectory
from unittest.mock import patch

SCRIPT_DIR = Path(__file__).resolve().parent
sys.path.insert(0, str(SCRIPT_DIR))

import run_bazel_q029_execution_context as subject
import run_bazel_with_buildbuddy as launcher


class ExecutionContextTest(unittest.TestCase):
    def fixture(self, job: str, *, bare_drive: bool = False):
        temp = TemporaryDirectory()
        root = Path(temp.name)
        workspace = root / "workspace"
        home = root / "home"
        workspace.mkdir()
        home.mkdir()
        (workspace / ".bazelversion").write_bytes(subject.BAZELVERSION_BYTES)
        (workspace / ".bazelrc").write_text("common --config=ci\n", encoding="utf-8")
        build_root = "D:" if bare_drive else "D:/ci"
        prefix = "D:" if bare_drive else "D:/ci"
        env = {
            "GITHUB_ACTIONS": "true",
            "GITHUB_REPOSITORY": subject.REPOSITORY,
            "GITHUB_JOB": job,
            "GITHUB_SHA": "0123456789abcdef0123456789abcdef01234567",
            "GITHUB_RUN_ID": "42",
            "GITHUB_WORKSPACE": str(workspace),
            "RUNNER_OS": "Windows",
            "RUNNER_ARCH": "X64",
            "RUNNER_ENVIRONMENT": "github-hosted",
            "RUNNER_TEMP": "D:/runner-temp",
            "USERPROFILE": str(home),
            "CI_BUILD_ROOT": build_root,
            "BAZEL_OUTPUT_USER_ROOT": f"{prefix}/b",
            "BAZEL_REPOSITORY_CACHE": f"{prefix}/bazel-repository-cache",
            "BAZEL_REPO_CONTENTS_CACHE": (
                f"{prefix}/bazel-repo-contents-cache-42-{job}"
            ),
            "CODEX_BAZEL_EXECUTION_LOG_COMPACT_DIR": (
                "D:/runner-temp/bazel-execution-logs"
            ),
            "PATH": str(root),
        }
        if job == subject.TEST_JOB:
            env["BAZEL_TEST_SHARD"] = "2"
            env["BAZEL_TEST_SHARD_COUNT"] = "4"
        subject.prepare_bazelisk_environment(env)
        executable = root / "bazel.exe"
        executable.write_bytes(b"official bazelisk fixture")
        return temp, env, executable

    def command(self, env, executable):
        job = env["GITHUB_JOB"]
        command_name = "test" if job == subject.TEST_JOB else "build"
        metadata = [f"{subject.COMMIT_METADATA_PREFIX}{env['GITHUB_SHA']}"]
        if job == subject.TEST_JOB:
            metadata.append(
                f"{subject.TEST_SHARD_METADATA_PREFIX}{env['BAZEL_TEST_SHARD']}"
            )
        elif job == subject.CLIPPY_JOB:
            metadata.append(subject.CLIPPY_JOB_METADATA)
        else:
            metadata.append(subject.RELEASE_JOB_METADATA)
        options = [
            *metadata,
            f"--repo_contents_cache={env['BAZEL_REPO_CONTENTS_CACHE']}",
            f"--repository_cache={env['BAZEL_REPOSITORY_CACHE']}",
            "--execution_log_compact_file="
            f"{env['CODEX_BAZEL_EXECUTION_LOG_COMPACT_DIR']}/"
            f"execution-log-{command_name}-{job}-123.zst",
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
            str(executable.resolve()),
            *startup,
            command_name,
            *options,
            "--",
            "//codex-rs/foo:unit",
        ]

    def validate(self, job, *, bare_drive=False):
        temp, env, executable = self.fixture(job, bare_drive=bare_drive)
        self.addCleanup(temp.cleanup)
        command = self.command(env, executable)
        with patch.object(
            subject,
            "BAZELISK_WINDOWS_X86_64_SHA256",
            subject._sha256_file(executable),
        ):
            subject.validate_keyless_windows_gnullvm_execution_context(command, env)
        return env, executable, command

    def test_wrapper_launch_order_binds_context_after_q027(self):
        env = {}
        raw = [
            "candidate-bazel",
            "build",
            "--platforms=//:windows_x86_64_gnullvm",
            "--",
            "//codex-rs/cli:codex",
        ]
        verified = ["verified-bazelisk", *raw[1:]]
        events = []

        def prepare(observed_env):
            self.assertIs(observed_env, env)
            events.append("prepare")

        def bind(command, observed_env):
            self.assertEqual(command, raw)
            self.assertIs(observed_env, env)
            events.append("bind")
            return verified

        def validate(command, observed_env):
            self.assertEqual(command, verified)
            self.assertIs(observed_env, env)
            events.append("validate")

        with (
            patch.object(launcher, "bazel_command", return_value=raw),
            patch.object(launcher, "_is_keyless_windows_gnullvm", return_value=True),
            patch.object(launcher, "prepare_bazelisk_environment", side_effect=prepare),
            patch.object(launcher, "bind_verified_bazelisk", side_effect=bind),
            patch.object(
                launcher,
                "validate_keyless_windows_gnullvm_execution_context",
                side_effect=validate,
            ),
        ):
            result = launcher.executable_command("build", env=env)

        self.assertEqual(result, verified)
        self.assertEqual(events, ["prepare", "bind", "validate"])

    def test_wrapper_non_keyless_path_remains_passthrough(self):
        env = {}
        raw = ["bazel", "build", "--", "//codex-rs/cli:codex"]
        with (
            patch.object(launcher, "bazel_command", return_value=raw),
            patch.object(launcher, "_is_keyless_windows_gnullvm", return_value=False),
            patch.object(launcher, "prepare_bazelisk_environment") as prepare,
            patch.object(launcher, "bind_verified_bazelisk") as bind,
        ):
            result = launcher.executable_command("build", env=env)

        self.assertEqual(result, raw)
        prepare.assert_not_called()
        bind.assert_not_called()

    def test_all_three_canonical_jobs_pass(self):
        for job in sorted(subject.QUALIFYING_JOBS):
            with self.subTest(job=job):
                self.validate(job)

    def test_bare_dev_drive_root_is_canonicalized(self):
        self.validate(subject.CLIPPY_JOB, bare_drive=True)

    def test_drive_relative_subdirectory_fails_closed(self):
        env, executable, command = self.validate(subject.CLIPPY_JOB)
        env["CI_BUILD_ROOT"] = "D:ci"
        with patch.object(
            subject,
            "BAZELISK_WINDOWS_X86_64_SHA256",
            subject._sha256_file(executable),
        ):
            with self.assertRaisesRegex(ValueError, "absolute Windows path"):
                subject.validate_keyless_windows_gnullvm_execution_context(command, env)

    def test_unknown_job_fails_closed(self):
        env, executable, command = self.validate(subject.CLIPPY_JOB)
        env["GITHUB_JOB"] = "attacker-job"
        with patch.object(
            subject,
            "BAZELISK_WINDOWS_X86_64_SHA256",
            subject._sha256_file(executable),
        ):
            with self.assertRaisesRegex(ValueError, "unknown keyless"):
                subject.validate_keyless_windows_gnullvm_execution_context(command, env)

    def test_self_hosted_runner_fails_closed(self):
        env, executable, command = self.validate(subject.CLIPPY_JOB)
        env["RUNNER_ENVIRONMENT"] = "self-hosted"
        with patch.object(
            subject,
            "BAZELISK_WINDOWS_X86_64_SHA256",
            subject._sha256_file(executable),
        ):
            with self.assertRaisesRegex(ValueError, "GitHub-hosted"):
                subject.validate_keyless_windows_gnullvm_execution_context(command, env)

    def test_job_command_mismatch_fails_closed(self):
        env, executable, command = self.validate(subject.RELEASE_JOB)
        command[command.index("build")] = "test"
        with patch.object(
            subject,
            "BAZELISK_WINDOWS_X86_64_SHA256",
            subject._sha256_file(executable),
        ):
            with self.assertRaisesRegex(ValueError, "requires Bazel command"):
                subject.validate_keyless_windows_gnullvm_execution_context(command, env)

    def test_job_metadata_mismatch_fails_closed(self):
        env, executable, command = self.validate(subject.CLIPPY_JOB)
        command[command.index(subject.CLIPPY_JOB_METADATA)] = (
            subject.RELEASE_JOB_METADATA
        )
        with patch.object(
            subject,
            "BAZELISK_WINDOWS_X86_64_SHA256",
            subject._sha256_file(executable),
        ):
            with self.assertRaisesRegex(ValueError, "job metadata"):
                subject.validate_keyless_windows_gnullvm_execution_context(command, env)

    def test_commit_metadata_drift_fails_closed(self):
        env, executable, command = self.validate(subject.CLIPPY_JOB)
        index = next(
            i
            for i, value in enumerate(command)
            if value.startswith(subject.COMMIT_METADATA_PREFIX)
        )
        command[index] = f"{subject.COMMIT_METADATA_PREFIX}{'f' * 40}"
        with patch.object(
            subject,
            "BAZELISK_WINDOWS_X86_64_SHA256",
            subject._sha256_file(executable),
        ):
            with self.assertRaisesRegex(ValueError, "COMMIT_SHA"):
                subject.validate_keyless_windows_gnullvm_execution_context(command, env)

    def test_shard_topology_drift_fails_closed(self):
        env, executable, command = self.validate(subject.TEST_JOB)
        env["BAZEL_TEST_SHARD_COUNT"] = "8"
        with patch.object(
            subject,
            "BAZELISK_WINDOWS_X86_64_SHA256",
            subject._sha256_file(executable),
        ):
            with self.assertRaisesRegex(ValueError, "must equal 4"):
                subject.validate_keyless_windows_gnullvm_execution_context(command, env)

    def test_additional_startup_option_fails_closed(self):
        env, executable, command = self.validate(subject.CLIPPY_JOB)
        command.insert(1, "--host_jvm_args=-Xmx32g")
        with patch.object(
            subject,
            "BAZELISK_WINDOWS_X86_64_SHA256",
            subject._sha256_file(executable),
        ):
            with self.assertRaisesRegex(ValueError, "startup arguments are not exact"):
                subject.validate_keyless_windows_gnullvm_execution_context(command, env)

    def test_cache_root_drift_fails_closed(self):
        env, executable, command = self.validate(subject.CLIPPY_JOB)
        env["BAZEL_REPOSITORY_CACHE"] = "C:/attacker-cache"
        with patch.object(
            subject,
            "BAZELISK_WINDOWS_X86_64_SHA256",
            subject._sha256_file(executable),
        ):
            with self.assertRaisesRegex(ValueError, "runner-controlled root"):
                subject.validate_keyless_windows_gnullvm_execution_context(command, env)

    def test_execution_log_escape_fails_closed(self):
        env, executable, command = self.validate(subject.CLIPPY_JOB)
        index = next(
            i
            for i, value in enumerate(command)
            if value.startswith("--execution_log_compact_file=")
        )
        command[index] = "--execution_log_compact_file=C:/attacker/log.zst"
        with patch.object(
            subject,
            "BAZELISK_WINDOWS_X86_64_SHA256",
            subject._sha256_file(executable),
        ):
            with self.assertRaisesRegex(ValueError, "escaped RUNNER_TEMP"):
                subject.validate_keyless_windows_gnullvm_execution_context(command, env)

    def test_execution_log_job_mismatch_fails_closed(self):
        env, executable, command = self.validate(subject.CLIPPY_JOB)
        index = next(
            i
            for i, value in enumerate(command)
            if value.startswith("--execution_log_compact_file=")
        )
        command[index] = (
            "--execution_log_compact_file="
            f"{env['CODEX_BAZEL_EXECUTION_LOG_COMPACT_DIR']}/"
            "execution-log-build-test-windows-shard-123.zst"
        )
        with patch.object(
            subject,
            "BAZELISK_WINDOWS_X86_64_SHA256",
            subject._sha256_file(executable),
        ):
            with self.assertRaisesRegex(ValueError, "non-canonical compact"):
                subject.validate_keyless_windows_gnullvm_execution_context(command, env)

    def test_bazelisk_override_fails_closed(self):
        temp, env, _ = self.fixture(subject.CLIPPY_JOB)
        self.addCleanup(temp.cleanup)
        env["BAZELISK_BASE_URL"] = "https://example.invalid"
        with self.assertRaisesRegex(ValueError, "BAZELISK_BASE_URL"):
            subject.prepare_bazelisk_environment(env)

    def test_bazelisk_format_url_override_fails_closed(self):
        temp, env, _ = self.fixture(subject.CLIPPY_JOB)
        self.addCleanup(temp.cleanup)
        env["BAZELISK_FORMAT_URL"] = "https://example.invalid/%v/%h"
        with self.assertRaisesRegex(ValueError, "BAZELISK_FORMAT_URL"):
            subject.prepare_bazelisk_environment(env)

    def test_codex_bazel_override_fails_closed(self):
        temp, env, _ = self.fixture(subject.CLIPPY_JOB)
        self.addCleanup(temp.cleanup)
        env["CODEX_BAZEL_BIN"] = "attacker.exe"
        with self.assertRaisesRegex(ValueError, "CODEX_BAZEL_BIN"):
            subject.prepare_bazelisk_environment(env)

    def test_conflicting_version_override_fails_closed(self):
        temp, env, _ = self.fixture(subject.CLIPPY_JOB)
        self.addCleanup(temp.cleanup)
        env["USE_BAZEL_VERSION"] = "latest"
        with self.assertRaisesRegex(ValueError, "conflicts"):
            subject.prepare_bazelisk_environment(env)

    def test_bazelversion_drift_fails_closed(self):
        temp, env, executable = self.fixture(subject.CLIPPY_JOB)
        self.addCleanup(temp.cleanup)
        Path(env["GITHUB_WORKSPACE"], ".bazelversion").write_text("latest\n")
        with self.assertRaisesRegex(ValueError, "bytes drifted"):
            subject.bind_verified_bazelisk(
                ["bazel", "build"],
                env,
                which=lambda *_args, **_kwargs: str(executable),
            )

    def test_workspace_bazeliskrc_fails_closed(self):
        temp, env, executable = self.fixture(subject.CLIPPY_JOB)
        self.addCleanup(temp.cleanup)
        Path(env["GITHUB_WORKSPACE"], ".bazeliskrc").write_text(
            "USE_BAZEL_VERSION=latest\n"
        )
        with self.assertRaisesRegex(ValueError, "workspace .bazeliskrc"):
            subject.bind_verified_bazelisk(
                ["bazel", "build"],
                env,
                which=lambda *_args, **_kwargs: str(executable),
            )

    def test_verified_bazelisk_replaces_argv_zero(self):
        temp, env, executable = self.fixture(subject.CLIPPY_JOB)
        self.addCleanup(temp.cleanup)
        with patch.object(
            subject,
            "BAZELISK_WINDOWS_X86_64_SHA256",
            subject._sha256_file(executable),
        ):
            bound = subject.bind_verified_bazelisk(
                ["attacker-stub", "build"],
                env,
                which=lambda *_args, **_kwargs: str(executable),
            )
        self.assertEqual(bound[0], str(executable.resolve()))

    def test_bazelisk_digest_drift_fails_closed(self):
        temp, env, executable = self.fixture(subject.CLIPPY_JOB)
        self.addCleanup(temp.cleanup)
        with self.assertRaisesRegex(ValueError, "SHA-256 drifted"):
            subject.bind_verified_bazelisk(
                ["bazel", "build"],
                env,
                which=lambda *_args, **_kwargs: str(executable),
            )


if __name__ == "__main__":
    unittest.main()
