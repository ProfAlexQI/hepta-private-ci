#!/usr/bin/env python3

import sys
import unittest
from pathlib import Path
from unittest.mock import patch

SCRIPT_DIR = Path(__file__).resolve().parent
sys.path.insert(0, str(SCRIPT_DIR))

import run_bazel_q022_negative_targets as subject


class TargetAndJobMetadataQualificationTest(unittest.TestCase):
    @staticmethod
    def common_options() -> list[str]:
        return [
            "--host_platform=//:local_windows_msvc",
            "--platforms=//:windows_x86_64_gnullvm",
        ]

    def validate(self, *args: str) -> None:
        with patch.object(subject, "_validate_q021"):
            subject.validate_keyless_windows_gnullvm_final_args(args, {})

    @classmethod
    def test_args(
        cls,
        *extra_options: str,
        targets: tuple[str, ...] = ("//codex-rs/cli:codex",),
    ) -> tuple[str, ...]:
        return (
            "test",
            "--config=ci-windows",
            *cls.common_options(),
            subject.CANONICAL_TEST_TAG_FILTER,
            subject.CANONICAL_SKIP_INCOMPATIBLE,
            subject.CANONICAL_TEST_VERBOSE_TIMEOUTS,
            *extra_options,
            "--",
            *targets,
        )

    @classmethod
    def clippy_args(
        cls,
        *extra_options: str,
        targets: tuple[str, ...] = ("//codex-rs/cli:codex",),
    ) -> tuple[str, ...]:
        return (
            "build",
            "--config=clippy",
            "--config=ci-windows",
            *cls.common_options(),
            subject.CANONICAL_SKIP_INCOMPATIBLE,
            subject.CLIPPY_JOB_METADATA,
            *extra_options,
            "--",
            *targets,
        )

    @classmethod
    def release_args(
        cls,
        *extra_options: str,
        targets: tuple[str, ...] = subject.CANONICAL_RELEASE_TARGETS,
    ) -> tuple[str, ...]:
        return (
            "build",
            "--config=ci-windows",
            *cls.common_options(),
            subject.RELEASE_JOB_METADATA,
            *extra_options,
            "--",
            *targets,
        )

    def test_canonical_test_lane_passes(self) -> None:
        self.validate(*self.test_args())

    def test_canonical_clippy_lane_passes(self) -> None:
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
        args = list(self.clippy_args())
        args.insert(args.index("--"), "--build_metadata=TAG_job")
        with self.assertRaisesRegex(ValueError, "malformed TAG_job build metadata"):
            self.validate(*args)

    def test_empty_job_metadata_fails_closed(self) -> None:
        args = list(self.clippy_args())
        args.insert(args.index("--"), subject.JOB_METADATA_PREFIX)
        with self.assertRaisesRegex(ValueError, "malformed TAG_job build metadata"):
            self.validate(*args)

    def test_duplicate_clippy_job_metadata_fails_closed(self) -> None:
        with self.assertRaisesRegex(ValueError, "ambiguous TAG_job build metadata"):
            self.validate(*self.clippy_args(subject.CLIPPY_JOB_METADATA))

    def test_test_lane_rejects_job_metadata(self) -> None:
        with self.assertRaisesRegex(
            ValueError,
            "test qualification rejects build-lane",
        ):
            self.validate(*self.test_args(subject.CLIPPY_JOB_METADATA))

    def test_test_exclude_all_fails_closed(self) -> None:
        with self.assertRaisesRegex(ValueError, "test qualification rejects negative"):
            self.validate(*self.test_args(targets=("//codex-rs/...", "-//...")))

    def test_clippy_arbitrary_exclusion_fails_closed(self) -> None:
        with self.assertRaisesRegex(
            ValueError,
            "clippy qualification rejects negative",
        ):
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

    def test_release_target_reorder_fails_closed(self) -> None:
        reordered = (
            subject.CANONICAL_RELEASE_TARGETS[0],
            subject.CANONICAL_RELEASE_TARGETS[2],
            subject.CANONICAL_RELEASE_TARGETS[1],
        )
        with self.assertRaisesRegex(ValueError, "exact canonical target set"):
            self.validate(*self.release_args(targets=reordered))

    def test_release_metadata_on_test_command_fails_closed(self) -> None:
        with self.assertRaisesRegex(
            ValueError,
            "test qualification rejects build-lane",
        ):
            self.validate(*self.test_args(subject.RELEASE_JOB_METADATA))

    def test_unclassified_build_fails_closed(self) -> None:
        with self.assertRaisesRegex(ValueError, "one recognized lane metadata"):
            self.validate(
                "build",
                "--config=ci-windows",
                *self.common_options(),
                "--",
                "//codex-rs/cli:codex",
            )


if __name__ == "__main__":
    unittest.main()
