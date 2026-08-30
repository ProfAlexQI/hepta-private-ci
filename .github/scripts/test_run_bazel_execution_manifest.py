#!/usr/bin/env python3

from __future__ import annotations

import sys
import unittest
from pathlib import Path
from tempfile import TemporaryDirectory
from unittest.mock import Mock, patch

SCRIPT_DIR = Path(__file__).resolve().parent
sys.path.insert(0, str(SCRIPT_DIR))

import run_bazel_q034_execution_manifest as subject
import run_bazel_with_buildbuddy as wrapper


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


class ExecutionManifestTest(unittest.TestCase):
    def fixture(self, job: str) -> tuple[TemporaryDirectory[str], dict[str, str]]:
        temporary = TemporaryDirectory()
        workspace = Path(temporary.name) / "workspace"
        workspace.mkdir()
        env = {
            "GITHUB_WORKSPACE": str(workspace),
            "GITHUB_JOB": job,
            "BAZEL_TEST_SHARD": "1",
            "BAZEL_TEST_SHARD_COUNT": "4",
        }
        return temporary, env

    @staticmethod
    def command(targets: list[str]) -> list[str]:
        return [
            "C:/verified/bazel.exe",
            "--output_user_root=D:/b",
            "--output_base=D:/o",
            "build",
            "--config=ci-windows",
            "--",
            *targets,
        ]

    @staticmethod
    def labels(count: int = 64) -> list[str]:
        return [f"//codex-rs/pkg_{index}:test" for index in range(count)]

    def test_posix_cksum_matches_reviewed_shell_algorithm(self) -> None:
        self.assertEqual(subject._posix_cksum(b"hello\n"), 3015617425)
        self.assertEqual(
            subject._posix_cksum(b"//codex-rs/foo:test\n"),
            3472389055,
        )

    def test_exact_windows_test_manifest_and_workspace_pass(self) -> None:
        temporary, env = self.fixture(subject.TEST_JOB)
        self.addCleanup(temporary.cleanup)
        labels = self.labels()
        expected = subject._windows_test_targets(labels, env)
        runner = Mock(return_value=Completed(stdout="\n".join(reversed(labels)) + "\n"))
        with patch.object(subject, "_validate_q032") as q032:
            workspace = subject.validate_keyless_windows_gnullvm_execution(
                self.command(expected),
                env,
                run=runner,
            )
        self.assertEqual(workspace, Path(env["GITHUB_WORKSPACE"]).resolve())
        self.assertEqual(q032.call_count, 2)
        query_command = runner.call_args.args[0]
        self.assertEqual(query_command[0], "C:/verified/bazel.exe")
        self.assertEqual(query_command[-1], subject.WINDOWS_TEST_QUERY)
        self.assertEqual(runner.call_args.kwargs["cwd"], workspace)

    def test_test_target_omission_addition_and_reorder_fail_closed(self) -> None:
        temporary, env = self.fixture(subject.TEST_JOB)
        self.addCleanup(temporary.cleanup)
        labels = self.labels()
        expected = subject._windows_test_targets(labels, env)
        self.assertGreaterEqual(len(expected), 3)
        variants = (
            expected[:-1],
            [*expected, "//codex-rs/attacker:extra"],
            [expected[1], expected[0], *expected[2:]],
        )
        for targets in variants:
            with self.subTest(targets=targets):
                runner = Mock(return_value=Completed(stdout="\n".join(labels) + "\n"))
                with (
                    patch.object(subject, "_validate_q032"),
                    self.assertRaisesRegex(ValueError, "target manifest is not exact"),
                ):
                    subject.validate_keyless_windows_gnullvm_execution(
                        self.command(targets),
                        env,
                        run=runner,
                    )

    def test_exact_clippy_manifest_filters_only_native_test_helpers(self) -> None:
        temporary, env = self.fixture(subject.CLIPPY_JOB)
        self.addCleanup(temporary.cleanup)
        labels = [
            "//codex-rs/foo:foo-windows-cross-bin",
            "//codex-rs/foo:foo-test-bin",
            "//codex-rs/foo:foo-unit-tests-bin",
        ]
        expected = [
            *subject.CLIPPY_TARGET_PREFIX,
            "//codex-rs/foo:foo-unit-tests-bin",
            "//codex-rs/foo:foo-windows-cross-bin",
        ]
        runner = Mock(return_value=Completed(stdout="\n".join(labels) + "\n"))
        with patch.object(subject, "_validate_q032"):
            subject.validate_keyless_windows_gnullvm_execution(
                self.command(expected),
                env,
                run=runner,
            )
        self.assertEqual(runner.call_args.args[0][-1], subject.CLIPPY_MANUAL_TEST_QUERY)

    def test_clippy_target_omission_addition_and_reorder_fail_closed(self) -> None:
        temporary, env = self.fixture(subject.CLIPPY_JOB)
        self.addCleanup(temporary.cleanup)
        labels = [
            "//codex-rs/foo:foo-unit-tests-bin",
            "//codex-rs/foo:foo-windows-cross-bin",
        ]
        expected = [*subject.CLIPPY_TARGET_PREFIX, *labels]
        variants = (
            expected[:-1],
            [*expected, "//codex-rs/attacker:extra"],
            [expected[1], expected[0], *expected[2:]],
        )
        for targets in variants:
            with self.subTest(targets=targets):
                runner = Mock(return_value=Completed(stdout="\n".join(labels) + "\n"))
                with (
                    patch.object(subject, "_validate_q032"),
                    self.assertRaisesRegex(ValueError, "target manifest is not exact"),
                ):
                    subject.validate_keyless_windows_gnullvm_execution(
                        self.command(targets),
                        env,
                        run=runner,
                    )

    def test_release_target_manifest_remains_exact_without_query(self) -> None:
        temporary, env = self.fixture(subject.RELEASE_JOB)
        self.addCleanup(temporary.cleanup)
        from run_bazel_q022_negative_targets import CANONICAL_RELEASE_TARGETS

        runner = Mock()
        with patch.object(subject, "_validate_q032"):
            subject.validate_keyless_windows_gnullvm_execution(
                self.command(list(CANONICAL_RELEASE_TARGETS)),
                env,
                run=runner,
            )
        runner.assert_not_called()

        reordered = list(CANONICAL_RELEASE_TARGETS)
        reordered[0], reordered[1] = reordered[1], reordered[0]
        with (
            patch.object(subject, "_validate_q032"),
            self.assertRaisesRegex(ValueError, "target manifest is not exact"),
        ):
            subject.validate_keyless_windows_gnullvm_execution(
                self.command(reordered),
                env,
                run=runner,
            )

    def test_query_failure_fails_closed_without_echoing_output(self) -> None:
        temporary, env = self.fixture(subject.CLIPPY_JOB)
        self.addCleanup(temporary.cleanup)
        runner = Mock(
            return_value=Completed(
                stdout="sensitive stdout",
                stderr="sensitive stderr",
                returncode=9,
            )
        )
        with (
            patch.object(subject, "_validate_q032"),
            self.assertRaisesRegex(ValueError, "exit=9") as raised,
        ):
            subject.validate_keyless_windows_gnullvm_execution(
                self.command(list(subject.CLIPPY_TARGET_PREFIX)),
                env,
                run=runner,
            )
        message = str(raised.exception)
        self.assertNotIn("sensitive stdout", message)
        self.assertNotIn("sensitive stderr", message)

    def test_workspace_symlink_fails_closed(self) -> None:
        temporary, env = self.fixture(subject.RELEASE_JOB)
        self.addCleanup(temporary.cleanup)
        real_workspace = Path(env["GITHUB_WORKSPACE"])
        linked_workspace = real_workspace.parent / "linked-workspace"
        try:
            linked_workspace.symlink_to(real_workspace, target_is_directory=True)
        except OSError as error:
            self.skipTest(f"symlink unavailable: {error}")
        env["GITHUB_WORKSPACE"] = str(linked_workspace)
        from run_bazel_q022_negative_targets import CANONICAL_RELEASE_TARGETS

        with (
            patch.object(subject, "_validate_q032"),
            self.assertRaisesRegex(ValueError, "must not be a symlink"),
        ):
            subject.validate_keyless_windows_gnullvm_execution(
                self.command(list(CANONICAL_RELEASE_TARGETS)),
                env,
            )

    def test_padded_duplicate_and_nonworkspace_query_output_fail_closed(self) -> None:
        temporary, env = self.fixture(subject.CLIPPY_JOB)
        self.addCleanup(temporary.cleanup)
        outputs = (
            " //codex-rs/foo:test\n",
            "//codex-rs/foo:test\n//codex-rs/foo:test\n",
            "@repo//foo:test\n",
        )
        for stdout in outputs:
            with self.subTest(stdout=stdout):
                with (
                    patch.object(subject, "_validate_q032"),
                    self.assertRaises(ValueError),
                ):
                    subject.validate_keyless_windows_gnullvm_execution(
                        self.command(list(subject.CLIPPY_TARGET_PREFIX)),
                        env,
                        run=Mock(return_value=Completed(stdout=stdout)),
                    )


    def test_qualifying_windows_launch_uses_canonical_workspace(self) -> None:
        temporary, env = self.fixture(subject.RELEASE_JOB)
        self.addCleanup(temporary.cleanup)
        workspace = Path(env["GITHUB_WORKSPACE"]).resolve()
        command = [
            "C:/verified/bazel.exe",
            "build",
            "--",
            "//codex-rs/...",
        ]
        runner = Mock(return_value=Completed(returncode=0))
        with (
            patch.object(wrapper, "remote_config", return_value=None),
            patch.object(wrapper, "bazel_command", return_value=["bazel", "build"]),
            patch.object(
                wrapper,
                "_is_keyless_windows_gnullvm",
                return_value=True,
            ),
            patch.object(wrapper, "prepare_bazelisk_environment"),
            patch.object(
                wrapper,
                "resolve_verified_bazel_command",
                return_value=command,
            ),
            patch.object(
                wrapper,
                "validate_keyless_windows_gnullvm_command",
            ) as q032,
            patch.object(
                wrapper,
                "validate_keyless_windows_gnullvm_execution",
                return_value=workspace,
            ),
            patch.object(wrapper.subprocess, "run", runner),
            patch.object(wrapper.os, "name", "nt"),
            patch.object(wrapper.os, "environ", env),
            patch.object(wrapper.sys, "argv", ["wrapper.py", "build"]),
            self.assertRaises(SystemExit) as raised,
        ):
            wrapper.main()
        self.assertEqual(raised.exception.code, 0)
        q032.assert_called_once_with(command, env)
        runner.assert_called_once_with(command, check=False, cwd=workspace)

    def test_nonqualifying_windows_launch_retains_legacy_cwd(self) -> None:
        command = ["bazel", "build"]
        runner = Mock(return_value=Completed(returncode=0))
        with (
            patch.object(wrapper, "remote_config", return_value=None),
            patch.object(wrapper, "bazel_command", return_value=command),
            patch.object(
                wrapper,
                "_is_keyless_windows_gnullvm",
                return_value=False,
            ),
            patch.object(wrapper.subprocess, "run", runner),
            patch.object(wrapper.os, "name", "nt"),
            patch.object(wrapper.sys, "argv", ["wrapper.py", "build"]),
            self.assertRaises(SystemExit) as raised,
        ):
            wrapper.main()
        self.assertEqual(raised.exception.code, 0)
        runner.assert_called_once_with(command, check=False)


if __name__ == "__main__":
    unittest.main()
