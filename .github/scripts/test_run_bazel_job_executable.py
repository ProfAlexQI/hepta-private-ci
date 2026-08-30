#!/usr/bin/env python3

import sys
import unittest
from pathlib import Path
from tempfile import TemporaryDirectory
from unittest.mock import patch

SCRIPT_DIR = Path(__file__).resolve().parent
sys.path.insert(0, str(SCRIPT_DIR))

import run_bazel_q029_job_executable as subject


GITHUB_SHA = "0123456789abcdef0123456789abcdef01234567"


class JobExecutableContractTest(unittest.TestCase):
    def fixture(self, job: str) -> tuple[TemporaryDirectory[str], dict[str, str], Path]:
        temp = TemporaryDirectory()
        root = Path(temp.name)
        workspace = root / "workspace"
        home = root / "home"
        workspace.mkdir()
        home.mkdir()
        (workspace / ".bazelversion").write_bytes(subject.BAZELVERSION_BYTES)
        (workspace / ".bazelrc").write_text("common --config=ci\n", encoding="utf-8")
        executable = root / "bazel.exe"
        executable.write_bytes(b"official Bazelisk fixture")
        env = {
            "GITHUB_ACTIONS": "true",
            "GITHUB_EVENT_NAME": "pull_request",
            "RUNNER_OS": "Windows",
            "RUNNER_ARCH": "X64",
            "RUNNER_ENVIRONMENT": "github-hosted",
            "GITHUB_REPOSITORY": subject.REPOSITORY,
            "GITHUB_JOB": job,
            "GITHUB_SHA": GITHUB_SHA,
            "GITHUB_RUN_ID": "42",
            "GITHUB_WORKSPACE": str(workspace),
            "USERPROFILE": str(home),
            "CI_BUILD_ROOT": "D:",
            "BAZEL_OUTPUT_BASE": "D:/o",
            "BAZEL_OUTPUT_USER_ROOT": "D:/b",
            "BAZEL_REPOSITORY_CACHE": "D:/bazel-repository-cache",
            "BAZEL_REPO_CONTENTS_CACHE": (
                f"D:/bazel-repo-contents-cache-42-{job}"
            ),
            "CARGO_TARGET_DIR": "D:/cargo-target",
            "TEMP": "D:/tmp",
            "TMP": "D:/tmp",
            "RUNNER_TEMP": "D:/a/_temp",
            "CODEX_BAZEL_EXECUTION_LOG_COMPACT_DIR": (
                "D:/a/_temp/bazel-execution-logs"
            ),
            "PATH": str(root),
        }
        if job == subject.TEST_JOB:
            env["BAZEL_TEST_SHARD"] = "2"
            env["BAZEL_TEST_SHARD_COUNT"] = "4"
        subject.prepare_bazelisk_environment(env)
        return temp, env, executable

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

    def command(self, env: dict[str, str], executable: Path) -> list[str]:
        job = env["GITHUB_JOB"]
        if job == subject.TEST_JOB:
            command_name = "test"
            configs = ["--config=ci-windows"]
            metadata = [
                f"--build_metadata=COMMIT_SHA={env['GITHUB_SHA']}",
                "--build_metadata=TAG_windows_gnullvm_local=true",
                f"--build_metadata=TAG_windows_test_shard={env['BAZEL_TEST_SHARD']}",
            ]
        elif job == subject.CLIPPY_JOB:
            command_name = "build"
            configs = ["--config=clippy", "--config=ci-windows"]
            metadata = [
                f"--build_metadata=COMMIT_SHA={env['GITHUB_SHA']}",
                "--build_metadata=TAG_windows_gnullvm_local=true",
                subject.CLIPPY_JOB_METADATA,
            ]
        else:
            command_name = "build"
            configs = ["--config=ci-windows"]
            metadata = [
                f"--build_metadata=COMMIT_SHA={env['GITHUB_SHA']}",
                "--build_metadata=TAG_windows_gnullvm_local=true",
                subject.RELEASE_JOB_METADATA,
                "--build_metadata=TAG_rust_debug_assertions=off",
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
        dynamic = [
            f"--repo_contents_cache={env['BAZEL_REPO_CONTENTS_CACHE']}",
            f"--repository_cache={env['BAZEL_REPOSITORY_CACHE']}",
            "--execution_log_compact_file="
            f"{env['CODEX_BAZEL_EXECUTION_LOG_COMPACT_DIR']}/"
            f"execution-log-{command_name}-{job}-123.zst",
        ]
        return [
            str(executable.resolve()),
            *startup,
            command_name,
            *configs,
            *metadata,
            *dynamic,
            "--",
            *self.targets(job),
        ]

    def validate(self, job: str) -> tuple[dict[str, str], list[str], Path]:
        temp, env, executable = self.fixture(job)
        self.addCleanup(temp.cleanup)
        command = self.command(env, executable)
        digest = subject._sha256_file(executable)
        with (
            patch.object(subject, "BAZELISK_WINDOWS_X86_64_SHA256", digest),
            patch.object(subject, "_validate_q028") as q028,
        ):
            subject.validate_keyless_windows_gnullvm_command(command, env)
        q028.assert_called_once_with(command[1:], env)
        return env, command, executable

    def test_all_three_canonical_jobs_pass(self) -> None:
        for job in sorted(subject.QUALIFYING_JOBS):
            with self.subTest(job=job):
                self.validate(job)

    def test_drive_root_shorthand_is_canonical(self) -> None:
        env, command, executable = self.validate(subject.CLIPPY_JOB)
        self.assertEqual(env["CI_BUILD_ROOT"], "D:")
        self.assertIn("--output_user_root=D:/b", command)
        self.assertTrue(executable.is_file())

    def test_unknown_job_fails_closed(self) -> None:
        temp, env, executable = self.fixture(subject.CLIPPY_JOB)
        self.addCleanup(temp.cleanup)
        command = self.command(env, executable)
        env["GITHUB_JOB"] = "attacker"
        with self.assertRaisesRegex(ValueError, "unknown keyless"):
            subject.validate_keyless_windows_gnullvm_command(
                command,
                env,
                digest_file=lambda _path: subject.BAZELISK_WINDOWS_X86_64_SHA256,
            )

    def test_repository_runner_and_event_identity_fail_closed(self) -> None:
        mutations = {
            "GITHUB_REPOSITORY": "attacker/repo",
            "RUNNER_OS": "Linux",
            "RUNNER_ARCH": "ARM64",
            "RUNNER_ENVIRONMENT": "self-hosted",
            "GITHUB_EVENT_NAME": "workflow_dispatch",
        }
        for name, value in mutations.items():
            with self.subTest(name=name):
                temp, env, executable = self.fixture(subject.CLIPPY_JOB)
                self.addCleanup(temp.cleanup)
                command = self.command(env, executable)
                env[name] = value
                with self.assertRaises(ValueError):
                    subject.validate_keyless_windows_gnullvm_command(
                        command,
                        env,
                        digest_file=lambda _path: (
                            subject.BAZELISK_WINDOWS_X86_64_SHA256
                        ),
                    )

    def test_github_sha_must_be_canonical(self) -> None:
        temp, env, executable = self.fixture(subject.CLIPPY_JOB)
        self.addCleanup(temp.cleanup)
        command = self.command(env, executable)
        env["GITHUB_SHA"] = "ABC"
        with self.assertRaisesRegex(ValueError, "lowercase 40-hex"):
            subject.validate_keyless_windows_gnullvm_command(
                command,
                env,
                digest_file=lambda _path: subject.BAZELISK_WINDOWS_X86_64_SHA256,
            )

    def test_cache_and_build_root_drift_fail_closed(self) -> None:
        for name, value in (
            ("BAZEL_OUTPUT_BASE", "C:/attacker/o"),
            ("BAZEL_REPOSITORY_CACHE", "C:/attacker/cache"),
            ("BAZEL_REPO_CONTENTS_CACHE", "D:/wrong"),
            ("CARGO_TARGET_DIR", "D:/wrong"),
            ("TEMP", "C:/temp"),
            ("TMP", "D:/tmp2"),
        ):
            with self.subTest(name=name):
                temp, env, executable = self.fixture(subject.CLIPPY_JOB)
                self.addCleanup(temp.cleanup)
                command = self.command(env, executable)
                env[name] = value
                with (
                    patch.object(subject, "_validate_q028"),
                    self.assertRaisesRegex(
                        ValueError,
                        "escaped its runner-controlled root",
                    ),
                ):
                    subject.validate_keyless_windows_gnullvm_command(
                        command,
                        env,
                        digest_file=lambda _path: (
                            subject.BAZELISK_WINDOWS_X86_64_SHA256
                        ),
                    )

    def test_drive_relative_and_nested_build_roots_fail_closed(self) -> None:
        for value in ("relative", "D:/nested"):
            with self.subTest(value=value):
                temp, env, executable = self.fixture(subject.CLIPPY_JOB)
                self.addCleanup(temp.cleanup)
                command = self.command(env, executable)
                env["CI_BUILD_ROOT"] = value
                with self.assertRaises(ValueError):
                    subject.validate_keyless_windows_gnullvm_command(
                        command,
                        env,
                        digest_file=lambda _path: (
                            subject.BAZELISK_WINDOWS_X86_64_SHA256
                        ),
                    )

    def test_execution_log_escape_and_wrong_job_fail_closed(self) -> None:
        mutations = (
            "C:/escape/execution-log-build-clippy-1.zst",
            "D:/a/_temp/bazel-execution-logs/"
            "execution-log-build-test-windows-shard-1.zst",
            "D:/a/_temp/bazel-execution-logs/nested/execution-log-build-clippy-1.zst",
        )
        for value in mutations:
            with self.subTest(value=value):
                env, command, executable = self.validate(subject.CLIPPY_JOB)
                idx = next(
                    i
                    for i, arg in enumerate(command)
                    if arg.startswith("--execution_log_compact_file=")
                )
                command[idx] = f"--execution_log_compact_file={value}"
                with (
                    patch.object(subject, "_validate_q028"),
                    self.assertRaises(ValueError),
                ):
                    subject.validate_keyless_windows_gnullvm_command(
                        command,
                        env,
                        digest_file=lambda _path: (
                            subject.BAZELISK_WINDOWS_X86_64_SHA256
                        ),
                    )

    def test_job_metadata_cannot_spoof_another_lane(self) -> None:
        env, command, executable = self.validate(subject.CLIPPY_JOB)
        idx = command.index(subject.CLIPPY_JOB_METADATA)
        command[idx] = subject.RELEASE_JOB_METADATA
        with (
            patch.object(subject, "_validate_q028"),
            self.assertRaisesRegex(ValueError, "exact build metadata"),
        ):
            subject.validate_keyless_windows_gnullvm_command(
                command,
                env,
                digest_file=lambda _path: subject.BAZELISK_WINDOWS_X86_64_SHA256,
            )

    def test_test_and_clippy_targets_are_bound(self) -> None:
        env, command, executable = self.validate(subject.TEST_JOB)
        command.append("-//codex-rs/core:all")
        with (
            patch.object(subject, "_validate_q028"),
            self.assertRaisesRegex(ValueError, "positive workspace targets"),
        ):
            subject.validate_keyless_windows_gnullvm_command(
                command,
                env,
                digest_file=lambda _path: subject.BAZELISK_WINDOWS_X86_64_SHA256,
            )

        env, command, executable = self.validate(subject.CLIPPY_JOB)
        command[command.index("//codex-rs/...")] = "//other/..."
        with (
            patch.object(subject, "_validate_q028"),
            self.assertRaisesRegex(ValueError, "target prefix"),
        ):
            subject.validate_keyless_windows_gnullvm_command(
                command,
                env,
                digest_file=lambda _path: subject.BAZELISK_WINDOWS_X86_64_SHA256,
            )

    def test_release_target_payload_is_exact(self) -> None:
        env, command, executable = self.validate(subject.RELEASE_JOB)
        command[-1] = "-//codex-rs/core:all"
        with (
            patch.object(subject, "_validate_q028"),
            self.assertRaisesRegex(ValueError, "exact canonical release"),
        ):
            subject.validate_keyless_windows_gnullvm_command(
                command,
                env,
                digest_file=lambda _path: subject.BAZELISK_WINDOWS_X86_64_SHA256,
            )

    def test_bazel_executable_overrides_fail_closed(self) -> None:
        for name in (
            "CODEX_BAZEL_BIN",
            "BAZEL_REAL",
            "BAZEL_WRAPPER",
            "USE_BAZEL_FALLBACK_VERSION",
            "BAZELISK_BASE_URL",
            "BAZELISK_FORMAT_URL",
            "BAZELISK_HOME",
            "BAZELISK_NOJDK",
            "BAZELISK_USER_AGENT",
        ):
            with self.subTest(name=name):
                temp, env, _executable = self.fixture(subject.CLIPPY_JOB)
                self.addCleanup(temp.cleanup)
                env[name] = "attacker"
                with self.assertRaisesRegex(ValueError, "forbidden"):
                    subject.prepare_bazelisk_environment(env)

    def test_setup_bazel_github_token_is_transport_only(self) -> None:
        temp, env, _executable = self.fixture(subject.CLIPPY_JOB)
        self.addCleanup(temp.cleanup)
        env["BAZELISK_GITHUB_TOKEN"] = "github-actions-token"
        subject.prepare_bazelisk_environment(env)
        self.assertEqual(
            env["BAZELISK_GITHUB_TOKEN"],
            "github-actions-token",
        )

    def test_conflicting_required_bazelisk_control_fails_closed(self) -> None:
        for name in subject.BAZELISK_REQUIRED_ENV:
            with self.subTest(name=name):
                temp, env, _executable = self.fixture(subject.CLIPPY_JOB)
                self.addCleanup(temp.cleanup)
                env[name] = "attacker"
                with self.assertRaisesRegex(ValueError, "conflicts"):
                    subject.prepare_bazelisk_environment(env)

    def test_bazelversion_and_bazeliskrc_drift_fail_closed(self) -> None:
        temp, env, executable = self.fixture(subject.CLIPPY_JOB)
        self.addCleanup(temp.cleanup)
        Path(env["GITHUB_WORKSPACE"], ".bazelversion").write_text(
            "latest\n", encoding="utf-8"
        )
        with self.assertRaisesRegex(ValueError, "bytes drifted"):
            subject.bind_verified_bazelisk(["bazel", "build"], env)

        temp, env, executable = self.fixture(subject.CLIPPY_JOB)
        self.addCleanup(temp.cleanup)
        Path(env["GITHUB_WORKSPACE"], ".bazeliskrc").write_text(
            "USE_BAZEL_VERSION=latest\n", encoding="utf-8"
        )
        with self.assertRaisesRegex(ValueError, "workspace .bazeliskrc"):
            subject.bind_verified_bazelisk(["bazel", "build"], env)

        temp, env, executable = self.fixture(subject.CLIPPY_JOB)
        self.addCleanup(temp.cleanup)
        Path(env["USERPROFILE"], ".bazeliskrc").write_text(
            "USE_BAZEL_VERSION=latest\n", encoding="utf-8"
        )
        with self.assertRaisesRegex(ValueError, "runner-home .bazeliskrc"):
            subject.bind_verified_bazelisk(["bazel", "build"], env)

    def test_workspace_bazel_wrapper_is_forbidden(self) -> None:
        temp, env, executable = self.fixture(subject.CLIPPY_JOB)
        self.addCleanup(temp.cleanup)
        tools = Path(env["GITHUB_WORKSPACE"]) / "tools"
        tools.mkdir()
        (tools / "bazel.exe").write_bytes(b"wrapper")
        with self.assertRaisesRegex(ValueError, "wrapper surface"):
            subject.bind_verified_bazelisk(["bazel", "build"], env)

    def test_verified_bazelisk_replaces_argv0_with_absolute_path(self) -> None:
        temp, env, executable = self.fixture(subject.CLIPPY_JOB)
        self.addCleanup(temp.cleanup)
        digest = subject._sha256_file(executable)
        with patch.object(subject, "BAZELISK_WINDOWS_X86_64_SHA256", digest):
            bound = subject.bind_verified_bazelisk(
                ["bazel", "build"],
                env,
                which=lambda *_args, **_kwargs: str(executable),
            )
        self.assertEqual(bound[0], str(executable.resolve()))

    def test_unverified_argv0_symlink_and_digest_fail_closed(self) -> None:
        temp, env, executable = self.fixture(subject.CLIPPY_JOB)
        self.addCleanup(temp.cleanup)
        with self.assertRaisesRegex(ValueError, "argv"):
            subject.bind_verified_bazelisk(
                ["attacker.exe", "build"],
                env,
                which=lambda *_args, **_kwargs: str(executable),
            )

        symlink = Path(temp.name) / "bazel-link.exe"
        try:
            symlink.symlink_to(executable)
        except OSError:
            pass
        else:
            with self.assertRaisesRegex(ValueError, "symlink"):
                subject.bind_verified_bazelisk(
                    ["bazel", "build"],
                    env,
                    which=lambda *_args, **_kwargs: str(symlink),
                )

        with self.assertRaisesRegex(ValueError, "SHA-256 drifted"):
            subject.bind_verified_bazelisk(
                ["bazel", "build"],
                env,
                which=lambda *_args, **_kwargs: str(executable),
                digest_file=lambda _path: "0" * 64,
            )

    def test_executable_is_rehashed_immediately_before_launch(self) -> None:
        env, command, executable = self.validate(subject.CLIPPY_JOB)
        with (
            patch.object(subject, "_validate_q028"),
            self.assertRaisesRegex(ValueError, "before launch"),
        ):
            subject.validate_keyless_windows_gnullvm_command(
                command,
                env,
                digest_file=lambda _path: "0" * 64,
            )


if __name__ == "__main__":
    unittest.main()
