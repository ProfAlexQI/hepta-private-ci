#!/usr/bin/env python3

import hashlib
import sys
import unittest
from pathlib import Path
from tempfile import TemporaryDirectory
from unittest.mock import patch

SCRIPT_DIR = Path(__file__).resolve().parent
sys.path.insert(0, str(SCRIPT_DIR))

import run_bazel_q022_negative_targets as q026
import run_bazel_q030_direct_bazel as subject


class Completed:
    def __init__(
        self,
        *,
        stdout: str = "",
        stderr: str = "",
        returncode: int = 0,
    ) -> None:
        self.stdout = stdout
        self.stderr = stderr
        self.returncode = returncode


class DirectBazelCasQualificationTest(unittest.TestCase):
    def fixture(self) -> tuple[TemporaryDirectory[str], dict[str, str], Path, Path]:
        temporary = TemporaryDirectory()
        root = Path(temporary.name)
        workspace = root / "workspace"
        home = root / "home"
        toolcache = root / "toolcache"
        workspace.mkdir()
        home.mkdir()
        toolcache.mkdir()
        (workspace / ".bazelversion").write_bytes(b"9.0.0\n")
        (workspace / ".bazelrc").write_text(
            "common --config=ci\n",
            encoding="utf-8",
        )

        bazelisk = toolcache / "bazel.exe"
        bazelisk.write_bytes(b"verified bazelisk fixture")
        real_bazel = (
            root
            / "downloads"
            / "sha256"
            / hashlib.sha256(b"verified bazel fixture").hexdigest()
            / "bin"
            / "bazel.exe"
        )
        real_bazel.parent.mkdir(parents=True)
        real_bazel.write_bytes(b"verified bazel fixture")

        env = {
            "GITHUB_ACTIONS": "true",
            "GITHUB_EVENT_NAME": "pull_request",
            "RUNNER_OS": "Windows",
            "RUNNER_ARCH": "X64",
            "RUNNER_ENVIRONMENT": "github-hosted",
            "GITHUB_REPOSITORY": "ProfHepta/hepta-private-ci",
            "GITHUB_JOB": "clippy",
            "GITHUB_SHA": "0123456789abcdef0123456789abcdef01234567",
            "GITHUB_RUN_ID": "42",
            "GITHUB_WORKSPACE": str(workspace),
            "USERPROFILE": str(home),
            "CI_BUILD_ROOT": "D:",
            "BAZEL_OUTPUT_BASE": "D:/o",
            "BAZEL_OUTPUT_USER_ROOT": "D:/b",
            "BAZEL_REPOSITORY_CACHE": "D:/bazel-repository-cache",
            "BAZEL_REPO_CONTENTS_CACHE": (
                "D:/bazel-repo-contents-cache-42-clippy"
            ),
            "CARGO_TARGET_DIR": "D:/cargo-target",
            "TEMP": "D:/tmp",
            "TMP": "D:/tmp",
            "RUNNER_TEMP": "D:/a/_temp",
            "CODEX_BAZEL_EXECUTION_LOG_COMPACT_DIR": (
                "D:/a/_temp/bazel-execution-logs"
            ),
            "PATH": str(toolcache),
        }
        return temporary, env, bazelisk, real_bazel

    @staticmethod
    def digest(path: Path) -> str:
        return hashlib.sha256(path.read_bytes()).hexdigest()

    def resolve(
        self,
        *,
        stdout: str | None = None,
        returncode: int = 0,
        stderr: str = "",
    ) -> tuple[dict[str, str], list[str], Path, Path]:
        temporary, env, bazelisk, real_bazel = self.fixture()
        self.addCleanup(temporary.cleanup)
        subject.prepare_bazelisk_environment(env)
        child_path = f"{real_bazel.parent};C:/Windows/System32"
        if stdout is None:
            stdout = f"PATH={child_path}\n"
        with (
            patch.object(
                subject,
                "BAZELISK_WINDOWS_X86_64_SHA256",
                self.digest(bazelisk),
            ),
            patch.object(
                subject,
                "BAZEL_WINDOWS_X86_64_SHA256",
                self.digest(real_bazel),
            ),
        ):
            command = subject.resolve_verified_bazel_command(
                ["bazel", "build"],
                env,
                which=lambda *_args, **_kwargs: str(bazelisk),
                run=lambda *_args, **_kwargs: Completed(
                    stdout=stdout or "",
                    stderr=stderr,
                    returncode=returncode,
                ),
            )
        return env, command, bazelisk, real_bazel

    def test_resolver_verifies_bazelisk_and_cached_bazel(self) -> None:
        env, command, _bazelisk, real_bazel = self.resolve()
        self.assertEqual(command[0], str(real_bazel.resolve()))
        self.assertEqual(
            env["PATH"].split(";", 1)[0],
            str(real_bazel.parent),
        )

    def test_cached_bazel_is_rehashed_even_when_bazelisk_succeeds(self) -> None:
        temporary, env, bazelisk, _real_bazel = self.fixture()
        self.addCleanup(temporary.cleanup)
        subject.prepare_bazelisk_environment(env)
        expected_digest = hashlib.sha256(b"expected bazel").hexdigest()
        tampered = (
            Path(temporary.name)
            / "downloads"
            / "sha256"
            / expected_digest
            / "bin"
            / "bazel.exe"
        )
        tampered.parent.mkdir(parents=True)
        tampered.write_bytes(b"tampered cached bazel")
        with (
            patch.object(
                subject,
                "BAZELISK_WINDOWS_X86_64_SHA256",
                self.digest(bazelisk),
            ),
            patch.object(
                subject,
                "BAZEL_WINDOWS_X86_64_SHA256",
                expected_digest,
            ),
            self.assertRaisesRegex(
                ValueError,
                "cached Bazel executable SHA-256 drifted",
            ),
        ):
            subject.resolve_verified_bazel_command(
                ["bazel", "build"],
                env,
                which=lambda *_args, **_kwargs: str(bazelisk),
                run=lambda *_args, **_kwargs: Completed(
                    stdout=f"PATH={tampered.parent};C:/Windows/System32\n"
                ),
            )

    def test_cached_bazel_must_use_content_addressed_path(self) -> None:
        temporary, env, bazelisk, real_bazel = self.fixture()
        self.addCleanup(temporary.cleanup)
        subject.prepare_bazelisk_environment(env)
        escaped = Path(temporary.name) / "attacker" / "bazel.exe"
        escaped.parent.mkdir()
        escaped.write_bytes(real_bazel.read_bytes())
        with (
            patch.object(
                subject,
                "BAZELISK_WINDOWS_X86_64_SHA256",
                self.digest(bazelisk),
            ),
            patch.object(
                subject,
                "BAZEL_WINDOWS_X86_64_SHA256",
                self.digest(escaped),
            ),
            self.assertRaisesRegex(ValueError, "content-addressed store"),
        ):
            subject.resolve_verified_bazel_command(
                ["bazel", "build"],
                env,
                which=lambda *_args, **_kwargs: str(bazelisk),
                run=lambda *_args, **_kwargs: Completed(
                    stdout=f"PATH={escaped.parent};C:/Windows/System32\n"
                ),
            )

    def test_print_env_failure_fails_closed(self) -> None:
        with self.assertRaisesRegex(ValueError, "failed to resolve"):
            self.resolve(returncode=9, stderr="offline")

    def test_missing_or_duplicate_path_binding_fails_closed(self) -> None:
        for stdout in (
            "HOME=C:/Users/runner\n",
            "PATH=C:/one\nPath=C:/two\n",
            "PATH=\n",
        ):
            with self.subTest(stdout=stdout):
                with self.assertRaises(ValueError):
                    self.resolve(stdout=stdout)

    def test_bare_bazelisk_override_fails_closed(self) -> None:
        temporary, env, _bazelisk, _real_bazel = self.fixture()
        self.addCleanup(temporary.cleanup)
        env["BAZELISK"] = "attacker"
        with self.assertRaisesRegex(ValueError, "BAZELISK is forbidden"):
            subject.prepare_bazelisk_environment(env)

    def test_unverified_initial_argv0_fails_closed(self) -> None:
        temporary, env, bazelisk, _real_bazel = self.fixture()
        self.addCleanup(temporary.cleanup)
        subject.prepare_bazelisk_environment(env)
        with self.assertRaisesRegex(ValueError, "argv"):
            subject.resolve_verified_bazel_command(
                ["attacker.exe", "build"],
                env,
                which=lambda *_args, **_kwargs: str(bazelisk),
            )

    def test_direct_bazel_is_rehashed_immediately_before_launch(self) -> None:
        env, command, _bazelisk, real_bazel = self.resolve()
        with (
            patch.object(subject, "_validate_q028"),
            patch.object(subject, "_validate_runner_identity", return_value="clippy"),
            patch.object(subject, "_validate_paths"),
            patch.object(subject, "_validate_job_binding"),
            patch.object(
                subject,
                "_validate_bazelisk_inputs",
                return_value=Path(env["GITHUB_WORKSPACE"]),
            ),
            patch.object(
                subject,
                "BAZEL_WINDOWS_X86_64_SHA256",
                self.digest(real_bazel),
            ),
        ):
            direct = [
                command[0],
                "--output_user_root=D:/b",
                "--noexperimental_remote_repo_contents_cache",
                "--output_base=D:/o",
                "--nomaster_bazelrc",
                "--nosystem_rc",
                "--noworkspace_rc",
                "--nohome_rc",
                f"--bazelrc={Path(env['GITHUB_WORKSPACE']) / '.bazelrc'}",
                "build",
                "--",
                "//codex-rs/...",
            ]
            subject.validate_keyless_windows_gnullvm_command(direct, env)
            real_bazel.write_bytes(b"replaced after resolution")
            with self.assertRaisesRegex(ValueError, "changed before launch"):
                subject.validate_keyless_windows_gnullvm_command(direct, env)

    def test_q026_canonical_clippy_negative_target_passes(self) -> None:
        args = (
            "build",
            "--config=clippy",
            "--config=ci-windows",
            q026.CANONICAL_SKIP_INCOMPATIBLE,
            q026.CLIPPY_JOB_METADATA,
            "--",
            "//codex-rs/...",
            q026.CANONICAL_CLIPPY_NEGATIVE_TARGET,
        )
        with patch.object(q026, "_validate_q021"):
            q026.validate_keyless_windows_gnullvm_final_args(args, {})

    def test_q026_arbitrary_clippy_negative_target_fails_closed(self) -> None:
        args = (
            "build",
            "--config=clippy",
            "--config=ci-windows",
            q026.CANONICAL_SKIP_INCOMPATIBLE,
            q026.CLIPPY_JOB_METADATA,
            "--",
            "//codex-rs/...",
            "-//codex-rs/core:all",
        )
        with (
            patch.object(q026, "_validate_q021"),
            self.assertRaisesRegex(ValueError, "outside the canonical"),
        ):
            q026.validate_keyless_windows_gnullvm_final_args(args, {})


if __name__ == "__main__":
    unittest.main()
