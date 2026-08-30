#!/usr/bin/env python3

import subprocess
import sys
import unittest
from collections.abc import Sequence
from pathlib import Path
from unittest.mock import patch

SCRIPT_DIR = Path(__file__).resolve().parent
ROOT = SCRIPT_DIR.parents[1]
sys.path.insert(0, str(SCRIPT_DIR))

import run_bazel_q022_negative_targets as subject


class TargetAndMetadataQualificationTest(unittest.TestCase):
    @staticmethod
    def validate(*args: str) -> None:
        with patch.object(subject, "_validate_q021"):
            subject.validate_keyless_windows_gnullvm_final_args(args, {})

    @staticmethod
    def test_args(
        *extra_options: str,
        targets: Sequence[str] = ("//codex-rs/uds:uds-unit-tests-bin",),
    ) -> tuple[str, ...]:
        return (
            "test",
            "--config=ci-windows",
            subject.CANONICAL_TEST_TAG_FILTER,
            subject.CANONICAL_SKIP_INCOMPATIBLE,
            subject.CANONICAL_TEST_VERBOSE_TIMEOUTS,
            *extra_options,
            "--",
            *targets,
        )

    @staticmethod
    def clippy_args(
        *extra_options: str,
        targets: Sequence[str] = ("//codex-rs/uds:uds-unit-tests-bin",),
    ) -> tuple[str, ...]:
        return (
            "build",
            "--config=clippy",
            "--config=ci-windows",
            subject.CANONICAL_SKIP_INCOMPATIBLE,
            subject.CLIPPY_JOB_METADATA,
            *extra_options,
            "--",
            *targets,
        )

    @staticmethod
    def release_args(
        *extra_options: str,
        targets: Sequence[str] = subject.CANONICAL_RELEASE_TARGETS,
    ) -> tuple[str, ...]:
        return (
            "build",
            "--config=ci-windows",
            subject.RELEASE_JOB_METADATA,
            *extra_options,
            "--",
            *targets,
        )

    def test_canonical_test_targets_pass(self) -> None:
        self.validate(*self.test_args())

    def test_canonical_clippy_targets_pass(self) -> None:
        self.validate(*self.clippy_args())

    def test_exact_release_target_set_passes(self) -> None:
        self.validate(*self.release_args())

    def test_duplicate_release_job_metadata_fails_closed(self) -> None:
        with self.assertRaisesRegex(ValueError, "ambiguous TAG_job build metadata"):
            self.validate(*self.release_args(subject.RELEASE_JOB_METADATA))

    def test_release_plus_alternate_job_metadata_fails_closed(self) -> None:
        with self.assertRaisesRegex(ValueError, "ambiguous TAG_job build metadata"):
            self.validate(
                *self.release_args("--build_metadata=TAG_job=ordinary-test")
            )

    def test_split_build_metadata_fails_closed(self) -> None:
        with self.assertRaisesRegex(ValueError, "split-form --build_metadata"):
            self.validate(
                *self.release_args(
                    "--build_metadata",
                    "TAG_job=ordinary-test",
                )
            )

    def test_malformed_job_metadata_fails_closed(self) -> None:
        with self.assertRaisesRegex(ValueError, "malformed TAG_job build metadata"):
            self.validate(*self.clippy_args("--build_metadata=TAG_job"))

    def test_empty_job_metadata_fails_closed(self) -> None:
        with self.assertRaisesRegex(ValueError, "malformed TAG_job build metadata"):
            self.validate(*self.clippy_args(subject.JOB_METADATA_PREFIX))

    def test_test_lane_job_metadata_fails_closed(self) -> None:
        with self.assertRaisesRegex(ValueError, "test qualification rejects TAG_job"):
            self.validate(*self.test_args("--build_metadata=TAG_job=test"))

    def test_unrecognized_build_job_metadata_fails_closed(self) -> None:
        args = list(self.clippy_args())
        args[args.index(subject.CLIPPY_JOB_METADATA)] = (
            "--build_metadata=TAG_job=unreviewed"
        )
        with self.assertRaisesRegex(ValueError, "one recognized lane metadata tag"):
            self.validate(*args)

    def test_duplicate_clippy_job_metadata_fails_closed(self) -> None:
        with self.assertRaisesRegex(ValueError, "ambiguous TAG_job build metadata"):
            self.validate(
                *self.clippy_args("--build_metadata=TAG_job=ordinary-test")
            )

    def test_test_exclude_all_fails_closed(self) -> None:
        with self.assertRaisesRegex(ValueError, "test qualification rejects negative"):
            self.validate(*self.test_args(targets=("//codex-rs/...", "-//...")))

    def test_clippy_arbitrary_exclusion_fails_closed(self) -> None:
        with self.assertRaisesRegex(ValueError, "clippy qualification rejects negative"):
            self.validate(
                *self.clippy_args(
                    targets=("//codex-rs/...", "-//codex-rs/core:all")
                )
            )

    def test_release_target_drop_fails_closed(self) -> None:
        with self.assertRaisesRegex(ValueError, "exact canonical target set"):
            self.validate(*self.release_args(targets=("//codex-rs/...",)))

    def test_release_target_addition_fails_closed(self) -> None:
        with self.assertRaisesRegex(ValueError, "exact canonical target set"):
            self.validate(
                *self.release_args(
                    targets=(
                        *subject.CANONICAL_RELEASE_TARGETS,
                        "-//codex-rs/core:all",
                    )
                )
            )

    def test_release_metadata_on_test_command_fails_closed(self) -> None:
        with self.assertRaisesRegex(ValueError, "test qualification rejects TAG_job"):
            self.validate(*self.test_args(subject.RELEASE_JOB_METADATA))

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

    def test_wrapper_imports_composed_validator(self) -> None:
        wrapper = (SCRIPT_DIR / "run_bazel_with_buildbuddy.py").read_text(
            encoding="utf-8"
        )
        self.assertIn(
            "from run_bazel_q022_negative_targets import (",
            wrapper,
        )


if __name__ == "__main__":
    unittest.main()
