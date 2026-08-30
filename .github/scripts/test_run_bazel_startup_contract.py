#!/usr/bin/env python3

import sys
import unittest
from pathlib import Path
from tempfile import TemporaryDirectory
from unittest.mock import patch

SCRIPT_DIR = Path(__file__).resolve().parent
sys.path.insert(0, str(SCRIPT_DIR))

import run_bazel_q028_startup_contract as startup_contract
import run_bazel_with_buildbuddy as subject


class ExactStartupVectorQualificationTest(unittest.TestCase):
    def fixture(self) -> tuple[TemporaryDirectory[str], Path, dict[str, str]]:
        temporary = TemporaryDirectory()
        workspace = Path(temporary.name)
        bazelrc = workspace / ".bazelrc"
        bazelrc.write_text(
            "common --enable_platform_specific_config\n"
            "common:ci-windows --config=ci\n"
            "try-import %workspace%/user.bazelrc\n",
            encoding="utf-8",
        )
        env = {
            "GITHUB_ACTIONS": "true",
            "RUNNER_OS": "Windows",
            "GITHUB_WORKSPACE": str(workspace),
            "CODEX_BAZEL_BIN": "bazel-test",
        }
        return temporary, bazelrc, env

    def command(
        self,
        *startup: str,
        env_updates: dict[str, str] | None = None,
    ) -> list[str]:
        temporary, bazelrc, env = self.fixture()
        self.addCleanup(temporary.cleanup)
        if env_updates:
            env.update(env_updates)
        expected_blob = subject._git_blob_sha1(bazelrc.read_bytes())
        with (
            patch.object(
                subject,
                "QUALIFICATION_BAZELRC_GIT_BLOB_SHA1",
                expected_blob,
            ),
            patch.object(startup_contract, "_validate_q027"),
        ):
            return subject.bazel_command(
                *startup,
                "build",
                "--platforms=//:windows_x86_64_gnullvm",
                "--",
                "//codex-rs/uds:uds-unit-tests-bin",
                env=env,
            )

    def test_canonical_startup_vector_passes(self) -> None:
        command = self.command()
        command_idx = command.index("build")
        startup = command[1:command_idx]
        self.assertEqual(
            startup[:-1],
            [
                startup_contract.DISABLED_REPO_CONTENTS_CACHE,
                *startup_contract.STRICT_STARTUP_FLAGS,
            ],
        )
        self.assertTrue(startup[-1].startswith("--bazelrc="))
        self.assertTrue(startup[-1].endswith("/.bazelrc"))

    def test_exact_output_root_from_environment_passes(self) -> None:
        output_root = str(Path.cwd() / "bazel-output")
        command = self.command(
            env_updates={"BAZEL_OUTPUT_USER_ROOT": output_root}
        )
        self.assertEqual(command[1], f"--output_user_root={output_root}")

    def test_startup_jvm_option_fails_closed(self) -> None:
        with self.assertRaisesRegex(ValueError, "exact startup vector"):
            self.command("--host_jvm_args=-Xmx4g")

    def test_positive_repository_contents_cache_fails_closed(self) -> None:
        with self.assertRaisesRegex(ValueError, "exact startup vector"):
            self.command("--experimental_remote_repo_contents_cache")

    def test_duplicate_negative_repository_contents_cache_fails_closed(self) -> None:
        with self.assertRaisesRegex(ValueError, "exact startup vector"):
            self.command(
                startup_contract.DISABLED_REPO_CONTENTS_CACHE,
                startup_contract.DISABLED_REPO_CONTENTS_CACHE,
            )

    def test_output_root_drift_fails_closed(self) -> None:
        expected_root = str(Path.cwd() / "expected")
        with self.assertRaisesRegex(ValueError, "exact startup vector"):
            self.command(
                "--output_user_root=C:/attacker",
                env_updates={"BAZEL_OUTPUT_USER_ROOT": expected_root},
            )


if __name__ == "__main__":
    unittest.main()
