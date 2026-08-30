#!/usr/bin/env python3

import hashlib
import subprocess
import sys
import unittest
from pathlib import Path
from tempfile import TemporaryDirectory
from unittest.mock import patch

SCRIPT_DIR = Path(__file__).resolve().parent
sys.path.insert(0, str(SCRIPT_DIR))

import run_bazel_q030_bazel_child as subject


class BazelChildContractTest(unittest.TestCase):
    def test_setup_bazel_transport_token_is_preserved(self) -> None:
        env = {subject.BAZELISK_TRANSPORT_TOKEN_ENV: "transport-token"}
        observed = []

        def prepare(base_env):
            observed.append(dict(base_env))
            self.assertNotIn(subject.BAZELISK_TRANSPORT_TOKEN_ENV, base_env)
            base_env["USE_BAZEL_VERSION"] = "9.0.0"

        with patch.object(subject, "_prepare_q029", side_effect=prepare):
            subject.prepare_bazelisk_environment(env)

        self.assertEqual(
            env[subject.BAZELISK_TRANSPORT_TOKEN_ENV],
            "transport-token",
        )
        self.assertEqual(env["USE_BAZEL_VERSION"], "9.0.0")
        self.assertEqual(len(observed), 1)

    def test_missing_transport_token_remains_absent(self) -> None:
        env = {}
        with patch.object(subject, "_prepare_q029"):
            subject.prepare_bazelisk_environment(env)
        self.assertNotIn(subject.BAZELISK_TRANSPORT_TOKEN_ENV, env)

    def test_transport_token_is_restored_when_q029_rejects(self) -> None:
        env = {subject.BAZELISK_TRANSPORT_TOKEN_ENV: "transport-token"}
        with patch.object(
            subject,
            "_prepare_q029",
            side_effect=ValueError("base rejection"),
        ):
            with self.assertRaisesRegex(ValueError, "base rejection"):
                subject.prepare_bazelisk_environment(env)
        self.assertEqual(
            env[subject.BAZELISK_TRANSPORT_TOKEN_ENV],
            "transport-token",
        )

    def test_nontransport_bazelisk_override_still_fails_closed(self) -> None:
        env = {
            subject.BAZELISK_TRANSPORT_TOKEN_ENV: "transport-token",
            "BAZELISK_BASE_URL": "https://example.invalid",
        }
        with self.assertRaisesRegex(ValueError, "BAZELISK_BASE_URL"):
            subject.prepare_bazelisk_environment(env)
        self.assertEqual(
            env[subject.BAZELISK_TRANSPORT_TOKEN_ENV],
            "transport-token",
        )

    def fixture(self):
        temp = TemporaryDirectory()
        root = Path(temp.name)
        workspace = root / "workspace"
        workspace.mkdir()
        bazelisk = root / "bazelisk" / "bazel.exe"
        bazelisk.parent.mkdir()
        bazelisk.write_bytes(b"bazelisk fixture")
        bazel_digest = hashlib.sha256(b"bazel fixture").hexdigest()
        real_bazel = (
            root
            / "cache"
            / "downloads"
            / "sha256"
            / bazel_digest
            / "bin"
            / "bazel.exe"
        )
        real_bazel.parent.mkdir(parents=True)
        real_bazel.write_bytes(b"bazel fixture")
        env = {
            "GITHUB_WORKSPACE": str(workspace),
            "PATH": str(bazelisk.parent),
            subject.BAZELISK_TRANSPORT_TOKEN_ENV: "transport-token",
        }
        return temp, env, bazelisk, real_bazel, bazel_digest

    @staticmethod
    def completed(path_value: str, *, returncode: int = 0, stderr: str = ""):
        return subprocess.CompletedProcess(
            args=["bazelisk", "--print_env"],
            returncode=returncode,
            stdout=f"PATH={path_value}\nOTHER=value\n",
            stderr=stderr,
        )

    def test_cached_bazel_is_rehashed_and_launched_directly(self) -> None:
        temp, env, bazelisk, real_bazel, bazel_digest = self.fixture()
        self.addCleanup(temp.cleanup)
        path_value = f"{real_bazel.parent};C:\\Windows\\System32"
        with (
            patch.object(
                subject,
                "BAZELISK_WINDOWS_X86_64_SHA256",
                hashlib.sha256(bazelisk.read_bytes()).hexdigest(),
            ),
            patch.object(subject, "BAZEL_WINDOWS_X86_64_SHA256", bazel_digest),
        ):
            command = subject.resolve_verified_bazel_command(
                [str(bazelisk.resolve()), "build", "--", "//codex-rs/..."],
                env,
                run=lambda *args, **kwargs: self.completed(path_value),
            )
            subject.validate_verified_bazel_prelaunch(command, env)

        self.assertEqual(command[0], str(real_bazel.resolve()))
        self.assertEqual(command[1:], ["build", "--", "//codex-rs/..."])
        self.assertEqual(env["PATH"], path_value)
        self.assertNotIn(subject.BAZELISK_TRANSPORT_TOKEN_ENV, env)

    def test_cached_bazel_digest_drift_fails_closed(self) -> None:
        temp, env, bazelisk, real_bazel, bazel_digest = self.fixture()
        self.addCleanup(temp.cleanup)
        path_value = f"{real_bazel.parent};C:\\Windows\\System32"
        with patch.object(
            subject,
            "BAZELISK_WINDOWS_X86_64_SHA256",
            hashlib.sha256(bazelisk.read_bytes()).hexdigest(),
        ):
            with self.assertRaisesRegex(ValueError, "cached Bazel executable SHA-256"):
                subject.resolve_verified_bazel_command(
                    [str(bazelisk.resolve()), "build", "--", "//codex-rs/..."],
                    env,
                    run=lambda *args, **kwargs: self.completed(path_value),
                )

    def test_cached_bazel_outside_cas_fails_closed(self) -> None:
        temp, env, bazelisk, real_bazel, bazel_digest = self.fixture()
        self.addCleanup(temp.cleanup)
        outside = Path(temp.name) / "outside" / "bazel.exe"
        outside.parent.mkdir()
        outside.write_bytes(real_bazel.read_bytes())
        path_value = f"{outside.parent};C:\\Windows\\System32"
        with (
            patch.object(
                subject,
                "BAZELISK_WINDOWS_X86_64_SHA256",
                hashlib.sha256(bazelisk.read_bytes()).hexdigest(),
            ),
            patch.object(subject, "BAZEL_WINDOWS_X86_64_SHA256", bazel_digest),
        ):
            with self.assertRaisesRegex(
                ValueError,
                "outside the reviewed Bazelisk CAS",
            ):
                subject.resolve_verified_bazel_command(
                    [str(bazelisk.resolve()), "build", "--", "//codex-rs/..."],
                    env,
                    run=lambda *args, **kwargs: self.completed(path_value),
                )

    def test_bazelisk_print_env_failure_fails_closed(self) -> None:
        temp, env, bazelisk, _, _ = self.fixture()
        self.addCleanup(temp.cleanup)
        with patch.object(
            subject,
            "BAZELISK_WINDOWS_X86_64_SHA256",
            hashlib.sha256(bazelisk.read_bytes()).hexdigest(),
        ):
            with self.assertRaisesRegex(ValueError, "failed to resolve"):
                subject.resolve_verified_bazel_command(
                    [str(bazelisk.resolve()), "build", "--", "//codex-rs/..."],
                    env,
                    run=lambda *args, **kwargs: self.completed(
                        "",
                        returncode=7,
                        stderr="resolution failed",
                    ),
                )

    def test_duplicate_print_env_path_fails_closed(self) -> None:
        temp, env, bazelisk, real_bazel, _ = self.fixture()
        self.addCleanup(temp.cleanup)
        result = subprocess.CompletedProcess(
            args=["bazelisk", "--print_env"],
            returncode=0,
            stdout=f"PATH={real_bazel.parent}\nPath={real_bazel.parent}\n",
            stderr="",
        )
        with patch.object(
            subject,
            "BAZELISK_WINDOWS_X86_64_SHA256",
            hashlib.sha256(bazelisk.read_bytes()).hexdigest(),
        ):
            with self.assertRaisesRegex(ValueError, "exactly one PATH"):
                subject.resolve_verified_bazel_command(
                    [str(bazelisk.resolve()), "build", "--", "//codex-rs/..."],
                    env,
                    run=lambda *args, **kwargs: result,
                )

    def test_prelaunch_digest_replacement_fails_closed(self) -> None:
        temp, env, _, real_bazel, bazel_digest = self.fixture()
        self.addCleanup(temp.cleanup)
        env["PATH"] = f"{real_bazel.parent};C:\\Windows\\System32"
        real_bazel.write_bytes(b"replaced")
        with patch.object(subject, "BAZEL_WINDOWS_X86_64_SHA256", bazel_digest):
            with self.assertRaisesRegex(ValueError, "SHA-256 drifted"):
                subject.validate_verified_bazel_prelaunch(
                    [str(real_bazel.resolve()), "build", "--", "//codex-rs/..."],
                    env,
                )

    def test_prelaunch_path_mismatch_fails_closed(self) -> None:
        temp, env, _, real_bazel, bazel_digest = self.fixture()
        self.addCleanup(temp.cleanup)
        env["PATH"] = f"{Path(temp.name) / 'other'};C:\\Windows\\System32"
        env.pop(subject.BAZELISK_TRANSPORT_TOKEN_ENV, None)
        with patch.object(subject, "BAZEL_WINDOWS_X86_64_SHA256", bazel_digest):
            with self.assertRaisesRegex(ValueError, "PATH"):
                subject.validate_verified_bazel_prelaunch(
                    [str(real_bazel.resolve()), "build", "--", "//codex-rs/..."],
                    env,
                )


if __name__ == "__main__":
    unittest.main()
