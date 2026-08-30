#!/usr/bin/env python3

from pathlib import Path
from unittest.mock import patch

from _q027_test_support import Q027TestCase
from _q027_test_support import job_policy
from _q027_test_support import path_policy
from _q027_test_support import subject


class Q027JobContractTest(Q027TestCase):
    def validate_with_digest(
        self, env: dict[str, str], command: list[str], digest: str
    ) -> None:
        with (
            patch.object(job_policy, "BAZEL_WINDOWS_X86_64_SHA256", digest),
            patch.object(
                path_policy,
                "_qualification_workspace_bazelrc",
                return_value=Path(env["GITHUB_WORKSPACE"], ".bazelrc").resolve(),
            ),
        ):
            subject.validate_keyless_windows_gnullvm_command(command, env)

    def test_all_three_real_jobs_pass(self) -> None:
        for job in sorted(subject.QUALIFYING_JOBS):
            with self.subTest(job=job):
                self.validate_job(job)

    def test_job_identity_cannot_be_spoofed_by_metadata(self) -> None:
        env, command, digest = self.validate_job(subject.CLIPPY_JOB)
        env["GITHUB_JOB"] = subject.RELEASE_JOB
        with self.assertRaisesRegex(
            ValueError, "runner-controlled root|requires exact configs"
        ):
            self.validate_with_digest(env, command, digest)

    def test_clippy_rejects_extra_allowlisted_config(self) -> None:
        env, command, digest = self.validate_job(subject.CLIPPY_JOB)
        command.insert(command.index("--config=ci-windows"), "--config=ci-v8")
        with self.assertRaisesRegex(ValueError, "requires exact configs"):
            self.validate_with_digest(env, command, digest)

    def test_release_job_rejects_test_command(self) -> None:
        env, command, digest = self.validate_job(subject.RELEASE_JOB)
        command[command.index("build")] = "test"
        with self.assertRaisesRegex(ValueError, "requires Bazel command"):
            self.validate_with_digest(env, command, digest)

    def test_output_base_escape_fails_closed(self) -> None:
        env, command, digest = self.validate_job(subject.CLIPPY_JOB)
        env["BAZEL_OUTPUT_BASE"] = "C:/attacker"
        with self.assertRaisesRegex(ValueError, "startup arguments are not exact"):
            self.validate_with_digest(env, command, digest)

    def test_unclassified_option_fails_closed(self) -> None:
        env, command, digest = self.validate_job(subject.CLIPPY_JOB)
        command.insert(command.index("--"), "--build_tag_filters=-slow")
        with self.assertRaisesRegex(ValueError, "unclassified Bazel option"):
            self.validate_with_digest(env, command, digest)


if __name__ == "__main__":
    import unittest

    unittest.main()
