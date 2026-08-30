#!/usr/bin/env python3

import subprocess
import sys
import unittest
from pathlib import Path
from unittest.mock import patch

SCRIPT_DIR = Path(__file__).resolve().parent
ROOT = SCRIPT_DIR.parents[1]
sys.path.insert(0, str(SCRIPT_DIR))

import run_bazel_q022_negative_targets as subject


class NegativeTargetQualificationTest(unittest.TestCase):
    @staticmethod
    def validate(*args: str) -> None:
        with patch.object(subject, "_validate_q021"):
            subject.validate_keyless_windows_gnullvm_final_args(args, {})

    def test_non_release_positive_targets_pass(self) -> None:
        self.validate("build", "--", "//codex-rs/cli:codex")

    def test_one_non_release_job_metadata_passes(self) -> None:
        self.validate(
            "test",
            "--build_metadata=TAG_job=clippy",
            "--",
            "//codex-rs/cli:codex",
        )

    def test_exact_release_target_set_passes(self) -> None:
        self.validate(
            "build",
            subject.RELEASE_JOB_METADATA,
            "--",
            *subject.CANONICAL_RELEASE_TARGETS,
        )

    def test_duplicate_release_job_metadata_fails_closed(self) -> None:
        with self.assertRaisesRegex(ValueError, "ambiguous TAG_job build metadata"):
            self.validate(
                "build",
                subject.RELEASE_JOB_METADATA,
                subject.RELEASE_JOB_METADATA,
                "--",
                *subject.CANONICAL_RELEASE_TARGETS,
            )

    def test_release_plus_alternate_job_metadata_fails_closed(self) -> None:
        with self.assertRaisesRegex(ValueError, "ambiguous TAG_job build metadata"):
            self.validate(
                "build",
                subject.RELEASE_JOB_METADATA,
                "--build_metadata=TAG_job=ordinary-test",
                "--",
                *subject.CANONICAL_RELEASE_TARGETS,
            )

    def test_split_build_metadata_fails_closed(self) -> None:
        with self.assertRaisesRegex(ValueError, "split-form --build_metadata"):
            self.validate(
                "build",
                subject.RELEASE_JOB_METADATA,
                "--build_metadata",
                "TAG_job=ordinary-test",
                "--",
                *subject.CANONICAL_RELEASE_TARGETS,
            )

    def test_malformed_job_metadata_fails_closed(self) -> None:
        with self.assertRaisesRegex(ValueError, "malformed TAG_job build metadata"):
            self.validate(
                "build",
                "--build_metadata=TAG_job",
                "--",
                "//codex-rs/cli:codex",
            )

    def test_empty_job_metadata_fails_closed(self) -> None:
        with self.assertRaisesRegex(ValueError, "malformed TAG_job build metadata"):
            self.validate(
                "build",
                subject.JOB_METADATA_PREFIX,
                "--",
                "//codex-rs/cli:codex",
            )

    def test_non_release_duplicate_job_metadata_fails_closed(self) -> None:
        with self.assertRaisesRegex(ValueError, "ambiguous TAG_job build metadata"):
            self.validate(
                "test",
                "--build_metadata=TAG_job=clippy",
                "--build_metadata=TAG_job=test",
                "--",
                "//codex-rs/cli:codex",
            )

    def test_non_release_exclude_all_fails_closed(self) -> None:
        with self.assertRaisesRegex(ValueError, "outside the release lane"):
            self.validate("build", "--", "//codex-rs/...", "-//...")

    def test_non_release_arbitrary_exclusion_fails_closed(self) -> None:
        with self.assertRaisesRegex(ValueError, "outside the release lane"):
            self.validate(
                "build",
                "--",
                "//codex-rs/...",
                "-//codex-rs/core:all",
            )

    def test_release_target_drop_fails_closed(self) -> None:
        with self.assertRaisesRegex(ValueError, "exact canonical target set"):
            self.validate(
                "build",
                subject.RELEASE_JOB_METADATA,
                "--",
                "//codex-rs/...",
            )

    def test_release_target_addition_fails_closed(self) -> None:
        with self.assertRaisesRegex(ValueError, "exact canonical target set"):
            self.validate(
                "build",
                subject.RELEASE_JOB_METADATA,
                "--",
                *subject.CANONICAL_RELEASE_TARGETS,
                "-//codex-rs/core:all",
            )

    def test_release_metadata_on_test_command_fails_closed(self) -> None:
        with self.assertRaisesRegex(ValueError, "exact canonical target set"):
            self.validate(
                "test",
                subject.RELEASE_JOB_METADATA,
                "--",
                *subject.CANONICAL_RELEASE_TARGETS,
            )

    def test_release_script_matches_policy_constant(self) -> None:
        script = ROOT / "scripts" / "list-bazel-release-targets.sh"
        completed = subprocess.run(
            ["bash", str(script)],
            cwd=ROOT,
            check=False,
            capture_output=True,
            text=True,
        )
        self.assertEqual(completed.returncode, 0, completed.stderr)
        self.assertEqual(
            tuple(completed.stdout.splitlines()),
            subject.CANONICAL_RELEASE_TARGETS,
        )

    def test_wrapper_imports_q022_validator(self) -> None:
        wrapper = (SCRIPT_DIR / "run_bazel_with_buildbuddy.py").read_text(
            encoding="utf-8"
        )
        self.assertIn(
            "from run_bazel_q022_negative_targets import (",
            wrapper,
        )


if __name__ == "__main__":
    unittest.main()
