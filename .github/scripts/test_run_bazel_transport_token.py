#!/usr/bin/env python3

import hashlib
import sys
import unittest
from pathlib import Path
from tempfile import TemporaryDirectory
from unittest.mock import patch

SCRIPT_DIR = Path(__file__).resolve().parent
sys.path.insert(0, str(SCRIPT_DIR))

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


class BazeliskTransportTokenTest(unittest.TestCase):
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
            "GITHUB_WORKSPACE": str(workspace),
            "USERPROFILE": str(home),
            "PATH": str(toolcache),
            subject.SETUP_BAZEL_TRANSPORT_TOKEN: "github-actions-transport",
        }
        subject.prepare_bazelisk_environment(env)
        return temporary, env, bazelisk, real_bazel

    @staticmethod
    def digest(path: Path) -> str:
        return hashlib.sha256(path.read_bytes()).hexdigest()

    def resolve(
        self,
        *,
        stdout: str | None = None,
        returncode: int = 0,
    ) -> tuple[dict[str, str], dict[str, str], list[str], Path]:
        temporary, env, bazelisk, real_bazel = self.fixture()
        self.addCleanup(temporary.cleanup)
        child_path = f"{real_bazel.parent};C:/Windows/System32"
        if stdout is None:
            stdout = f"PATH={child_path}\n"
        observed: dict[str, str] = {}

        def resolver(*_args: object, **kwargs: object) -> Completed:
            observed.update(dict(kwargs["env"]))
            return Completed(stdout=stdout or "", returncode=returncode)

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
                run=resolver,
            )
        return env, observed, command, real_bazel

    def test_transport_token_is_resolver_only(self) -> None:
        env, resolver_env, _command, _real_bazel = self.resolve()
        self.assertEqual(
            resolver_env[subject.SETUP_BAZEL_TRANSPORT_TOKEN],
            "github-actions-transport",
        )
        self.assertNotIn(subject.SETUP_BAZEL_TRANSPORT_TOKEN, env)

    def test_print_env_transport_token_leak_fails_closed(self) -> None:
        with self.assertRaisesRegex(ValueError, "leaked the setup-only"):
            self.resolve(
                stdout=(
                    "BAZELISK_GITHUB_TOKEN=leaked\n"
                    "PATH=C:/verified/bin;C:/Windows/System32\n"
                )
            )

    def test_resolution_failure_still_scrubs_transport_token(self) -> None:
        temporary, env, bazelisk, _real_bazel = self.fixture()
        self.addCleanup(temporary.cleanup)
        with (
            patch.object(
                subject,
                "BAZELISK_WINDOWS_X86_64_SHA256",
                self.digest(bazelisk),
            ),
            self.assertRaisesRegex(ValueError, "failed to resolve"),
        ):
            subject.resolve_verified_bazel_command(
                ["bazel", "build"],
                env,
                which=lambda *_args, **_kwargs: str(bazelisk),
                run=lambda *_args, **_kwargs: Completed(returncode=9),
            )
        self.assertNotIn(subject.SETUP_BAZEL_TRANSPORT_TOKEN, env)

    def test_direct_launch_rejects_reintroduced_transport_token(self) -> None:
        env, _resolver_env, command, real_bazel = self.resolve()
        env[subject.SETUP_BAZEL_TRANSPORT_TOKEN] = "reintroduced"
        with (
            patch.object(
                subject,
                "BAZEL_WINDOWS_X86_64_SHA256",
                self.digest(real_bazel),
            ),
            self.assertRaisesRegex(ValueError, "reached direct Bazel launch"),
        ):
            subject.validate_keyless_windows_gnullvm_command(command, env)


if __name__ == "__main__":
    unittest.main()
