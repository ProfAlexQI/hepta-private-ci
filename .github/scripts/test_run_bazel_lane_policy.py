#!/usr/bin/env python3

import sys
import unittest
from pathlib import Path
from unittest.mock import patch

SCRIPT_DIR = Path(__file__).resolve().parent
sys.path.insert(0, str(SCRIPT_DIR))

import run_bazel_q022_negative_targets as subject


class LaneSelectionQualificationTest(unittest.TestCase):
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
    def test_args(cls, *extra_options: str) -> tuple[str, ...]:
        return (
            "test",
            "--config=ci-windows",
            *cls.common_options(),
            subject.CANONICAL_TEST_TAG_FILTER,
            subject.CANONICAL_SKIP_INCOMPATIBLE,
            subject.CANONICAL_TEST_VERBOSE_TIMEOUTS,
            *extra_options,
            "--",
            "//codex-rs/uds:uds-unit-tests-bin",
        )

    @classmethod
    def clippy_args(cls, *extra_options: str) -> tuple[str, ...]:
        return (
            "build",
            "--config=clippy",
            "--config=ci-windows",
            *cls.common_options(),
            subject.CANONICAL_SKIP_INCOMPATIBLE,
            subject.CLIPPY_JOB_METADATA,
            *extra_options,
            "--",
            "//codex-rs/uds:uds-unit-tests-bin",
        )

    @classmethod
    def release_args(cls, *extra_options: str) -> tuple[str, ...]:
        return (
            "build",
            "--config=ci-windows",
            *cls.common_options(),
            subject.RELEASE_JOB_METADATA,
            *extra_options,
            "--",
            *subject.CANONICAL_RELEASE_TARGETS,
        )

    def test_canonical_test_lane_passes(self) -> None:
        self.validate(*self.test_args())

    def test_canonical_clippy_lane_passes(self) -> None:
        self.validate(*self.clippy_args())

    def test_canonical_release_lane_passes(self) -> None:
        self.validate(*self.release_args())

    def test_extra_test_config_fails_closed(self) -> None:
        with self.assertRaisesRegex(ValueError, "exact configs"):
            self.validate(
                "test",
                "--config=argument-comment-lint",
                *self.test_args()[1:],
            )

    def test_test_filter_fails_closed(self) -> None:
        with self.assertRaisesRegex(ValueError, "test-selection override"):
            self.validate(*self.test_args("--test_filter=NoSuchTest"))

    def test_test_arg_fails_closed(self) -> None:
        with self.assertRaisesRegex(ValueError, "test-selection override"):
            self.validate(*self.test_args("--test_arg=NoSuchTest"))

    def test_split_selection_forms_fail_closed(self) -> None:
        split_forms = (
            ("--test_filter", "NoSuchTest"),
            ("--test_arg", "NoSuchTest"),
            ("--test_tag_filters", "-manual"),
            ("--test_lang_filters", "rust"),
            ("--test_size_filters", "small"),
            ("--test_timeout_filters", "short"),
            ("--build_tag_filters", "-required"),
            ("--build_tests_only", "true"),
            ("--nobuild_tests_only",),
        )
        for form in split_forms:
            with self.subTest(form=form):
                with self.assertRaisesRegex(
                    ValueError,
                    "test-selection override",
                ):
                    self.validate(*self.test_args(*form))

    def test_alternate_test_tag_filter_fails_closed(self) -> None:
        with self.assertRaisesRegex(ValueError, "test qualification requires exactly"):
            args = list(self.test_args())
            args[args.index(subject.CANONICAL_TEST_TAG_FILTER)] = (
                "--test_tag_filters=-manual"
            )
            self.validate(*args)

    def test_duplicate_test_tag_filter_fails_closed(self) -> None:
        with self.assertRaisesRegex(ValueError, "test qualification requires exactly"):
            self.validate(*self.test_args(subject.CANONICAL_TEST_TAG_FILTER))

    def test_build_tag_filter_fails_closed(self) -> None:
        with self.assertRaisesRegex(ValueError, "test-selection override"):
            self.validate(*self.clippy_args("--build_tag_filters=-required"))

    def test_nobuild_tests_only_equals_form_fails_closed(self) -> None:
        with self.assertRaisesRegex(ValueError, "test-selection override"):
            self.validate(*self.clippy_args("--nobuild_tests_only=false"))

    def test_unclassified_build_fails_closed(self) -> None:
        with self.assertRaisesRegex(ValueError, "one recognized lane metadata"):
            self.validate(
                "build",
                "--config=ci-windows",
                *self.common_options(),
                "--",
                "//codex-rs/uds:uds-unit-tests-bin",
            )

    def test_clippy_without_skip_incompatible_fails_closed(self) -> None:
        with self.assertRaisesRegex(
            ValueError,
            "clippy qualification requires exactly",
        ):
            args = list(self.clippy_args())
            args.remove(subject.CANONICAL_SKIP_INCOMPATIBLE)
            self.validate(*args)

    def test_clippy_skip_disable_alias_fails_closed(self) -> None:
        with self.assertRaisesRegex(
            ValueError,
            "clippy qualification requires exactly",
        ):
            self.validate(
                *self.clippy_args("--noskip_incompatible_explicit_targets")
            )

    def test_clippy_skip_false_alias_fails_closed(self) -> None:
        with self.assertRaisesRegex(
            ValueError,
            "clippy qualification requires exactly",
        ):
            self.validate(
                *self.clippy_args("--skip_incompatible_explicit_targets=false")
            )

    def test_test_skip_disable_alias_fails_closed(self) -> None:
        with self.assertRaisesRegex(
            ValueError,
            "test qualification requires exactly",
        ):
            self.validate(
                *self.test_args("--noskip_incompatible_explicit_targets")
            )

    def test_test_timeout_disable_alias_fails_closed(self) -> None:
        with self.assertRaisesRegex(
            ValueError,
            "test qualification requires exactly",
        ):
            self.validate(*self.test_args("--notest_verbose_timeout_warnings"))

    def test_test_timeout_false_alias_fails_closed(self) -> None:
        with self.assertRaisesRegex(
            ValueError,
            "test qualification requires exactly",
        ):
            self.validate(*self.test_args("--test_verbose_timeout_warnings=false"))

    def test_build_timeout_false_alias_fails_closed(self) -> None:
        with self.assertRaisesRegex(ValueError, "build qualification rejects"):
            self.validate(*self.clippy_args("--test_verbose_timeout_warnings=false"))

    def test_release_skip_true_alias_fails_closed(self) -> None:
        with self.assertRaisesRegex(ValueError, "release qualification rejects"):
            self.validate(
                *self.release_args("--skip_incompatible_explicit_targets=true")
            )

    def test_release_skip_disable_alias_fails_closed(self) -> None:
        with self.assertRaisesRegex(ValueError, "release qualification rejects"):
            self.validate(
                *self.release_args("--noskip_incompatible_explicit_targets")
            )

    def test_release_with_clippy_config_fails_closed(self) -> None:
        with self.assertRaisesRegex(ValueError, "release qualification"):
            self.validate(
                "build",
                "--config=clippy",
                *self.release_args()[1:],
            )

    def test_release_with_arbitrary_exclusion_fails_closed(self) -> None:
        with self.assertRaisesRegex(ValueError, "exact canonical target set"):
            args = list(self.release_args())
            args[-1:] = ["-//codex-rs/core:all"]
            self.validate(*args)

    def test_test_negative_target_fails_closed(self) -> None:
        with self.assertRaisesRegex(ValueError, "test qualification rejects negative"):
            self.validate(
                *self.test_args(),
                "-//codex-rs/core:all",
            )


if __name__ == "__main__":
    unittest.main()
