#!/usr/bin/env python3

from __future__ import annotations

import hashlib
import sys
import unittest
from pathlib import Path
from tempfile import TemporaryDirectory
from unittest.mock import patch

SCRIPT_DIR = Path(__file__).resolve().parent
sys.path.insert(0, str(SCRIPT_DIR))

import run_bazel_q034_workspace_targets as subject


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


class WorkspaceTargetAuthorityTest(unittest.TestCase):
    def fixture(
        self,
        *,
        job: str,
        targets: tuple[str, ...],
        shard: str = "1",
    ) -> tuple[TemporaryDirectory[str], Path, Path, dict[str, str], list[str]]:
        temporary = TemporaryDirectory()
        root = Path(temporary.name)
        workspace = root / "workspace"
        executable = root / "bazel.exe"
        workspace.mkdir()
        executable.write_bytes(b"q0.34 verified Bazel fixture")
        env = {
            "GITHUB_WORKSPACE": str(workspace),
            "GITHUB_JOB": job,
            "BAZEL_TEST_SHARD": shard,
            "BAZEL_TEST_SHARD_COUNT": "4",
            "BAZEL_REPO_CONTENTS_CACHE": "D:/repo-contents-cache",
            "BAZEL_REPOSITORY_CACHE": "D:/repository-cache",
        }
        command = [
            str(executable.resolve()),
            "--output_user_root=D:/b",
            "--noexperimental_remote_repo_contents_cache",
            "build" if job != subject.TEST_JOB else "test",
            "--",
            *targets,
        ]
        return temporary, workspace, executable, env, command

    @staticmethod
    def digest(path: Path) -> str:
        return hashlib.sha256(path.read_bytes()).hexdigest()

    def validate(
        self,
        *,
        job: str,
        targets: tuple[str, ...],
        query_stdout: str = "",
        shard: str = "1",
        query_returncode: int = 0,
        query_stderr: str = "",
    ) -> Path:
        temporary, workspace, executable, env, command = self.fixture(
            job=job,
            targets=targets,
            shard=shard,
        )
        self.addCleanup(temporary.cleanup)

        calls: list[dict[str, object]] = []

        def run(
            argv: list[str],
            *,
            cwd: Path,
            env: dict[str, str],
            capture_output: bool,
            text: bool,
            check: bool,
            timeout: int,
        ) -> Completed:
            calls.append(
                {
                    "argv": argv,
                    "cwd": cwd,
                    "env": env,
                    "capture_output": capture_output,
                    "text": text,
                    "check": check,
                    "timeout": timeout,
                }
            )
            return Completed(
                stdout=query_stdout,
                stderr=query_stderr,
                returncode=query_returncode,
            )

        with (
            patch.object(
                subject,
                "_validate_bazelisk_inputs",
                return_value=workspace.resolve(),
            ),
            patch.object(
                subject,
                "_validate_runner_identity",
                return_value=job,
            ),
            patch.object(subject, "_require_cas_identity"),
            patch.object(
                subject,
                "BAZEL_WINDOWS_X86_64_SHA256",
                self.digest(executable),
            ),
        ):
            result = subject.validate_keyless_windows_gnullvm_workspace_and_targets(
                command,
                env,
                run=run,
                digest_file=self.digest,
            )

        if job == subject.RELEASE_JOB:
            self.assertEqual(calls, [])
        else:
            self.assertEqual(len(calls), 1)
            self.assertEqual(calls[0]["cwd"], workspace.resolve())
            argv = calls[0]["argv"]
            self.assertEqual(argv[0], str(executable.resolve()))
            self.assertIn("--output=label", argv)
            self.assertIn(
                f"--repo_contents_cache={env['BAZEL_REPO_CONTENTS_CACHE']}",
                argv,
            )
            self.assertIn(
                f"--repository_cache={env['BAZEL_REPOSITORY_CACHE']}",
                argv,
            )
        return result

    def test_posix_cksum_matches_reviewed_shell_generator(self) -> None:
        self.assertEqual(subject.posix_cksum(b"abc"), 1219131554)
        self.assertEqual(subject.posix_cksum(b"//foo:bar\n"), 1487933824)

    def test_exact_test_shard_vector_is_accepted(self) -> None:
        labels = tuple(f"//pkg:case_{index}" for index in range(1, 33))
        shard = "3"
        expected = subject._expected_test_targets(labels, shard, "4")
        result = self.validate(
            job=subject.TEST_JOB,
            targets=expected,
            query_stdout="\n".join(reversed(labels)) + "\n",
            shard=shard,
        )
        self.assertTrue(result.is_absolute())

    def test_omitted_or_substituted_test_target_fails_closed(self) -> None:
        labels = tuple(f"//pkg:case_{index}" for index in range(1, 33))
        shard = "2"
        expected = subject._expected_test_targets(labels, shard, "4")
        self.assertGreaterEqual(len(expected), 2)
        for observed in (
            expected[:-1],
            (*expected[:-1], "//pkg:substituted"),
        ):
            with self.subTest(observed=observed):
                with self.assertRaisesRegex(ValueError, "target vector drifted"):
                    self.validate(
                        job=subject.TEST_JOB,
                        targets=tuple(observed),
                        query_stdout="\n".join(labels) + "\n",
                        shard=shard,
                    )

    def test_exact_clippy_vector_is_accepted(self) -> None:
        query_labels = (
            "//codex-rs/a:a-unit-tests-bin",
            "//codex-rs/b:b-test-bin",
            "//codex-rs/c:c",
        )
        expected = (
            *subject.CLIPPY_TARGET_PREFIX,
            "//codex-rs/a:a-unit-tests-bin",
            "//codex-rs/c:c",
        )
        result = self.validate(
            job=subject.CLIPPY_JOB,
            targets=expected,
            query_stdout="\n".join(query_labels) + "\n",
        )
        self.assertTrue(result.is_absolute())

    def test_omitted_clippy_target_fails_closed(self) -> None:
        query_labels = (
            "//codex-rs/a:a-unit-tests-bin",
            "//codex-rs/c:c",
        )
        expected = (
            *subject.CLIPPY_TARGET_PREFIX,
            "//codex-rs/a:a-unit-tests-bin",
            "//codex-rs/c:c",
        )
        with self.assertRaisesRegex(ValueError, "target vector drifted"):
            self.validate(
                job=subject.CLIPPY_JOB,
                targets=expected[:-1],
                query_stdout="\n".join(query_labels) + "\n",
            )

    def test_query_failure_does_not_echo_query_output(self) -> None:
        secret = "sensitive-target-or-token-output"
        with self.assertRaisesRegex(ValueError, "exit=17") as error:
            self.validate(
                job=subject.TEST_JOB,
                targets=("//pkg:case",),
                query_returncode=17,
                query_stdout=secret,
                query_stderr=secret,
            )
        self.assertNotIn(secret, str(error.exception))

    def test_release_reuses_existing_exact_payload_without_query(self) -> None:
        workspace = self.validate(
            job=subject.RELEASE_JOB,
            targets=("//codex-rs/...",),
        )
        self.assertTrue(workspace.is_absolute())

    def test_wrapper_binds_final_windows_launch_cwd_and_environment(self) -> None:
        wrapper = (SCRIPT_DIR / "run_bazel_with_buildbuddy.py").read_text(
            encoding="utf-8"
        )
        helper_call = (
            "launch_cwd = validate_keyless_windows_gnullvm_workspace_and_targets("
        )
        final_spawn = "result = subprocess.run(\n                command,\n                cwd=launch_cwd,"
        self.assertIn(helper_call, wrapper)
        self.assertIn(final_spawn, wrapper)
        self.assertIn("env=os.environ", wrapper)
        self.assertLess(wrapper.index(helper_call), wrapper.index(final_spawn))


if __name__ == "__main__":
    unittest.main()
