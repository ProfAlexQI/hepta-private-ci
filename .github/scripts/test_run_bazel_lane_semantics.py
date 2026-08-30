#!/usr/bin/env python3

import sys
import unittest
from pathlib import Path
from unittest.mock import patch

SCRIPT_DIR = Path(__file__).resolve().parent
sys.path.insert(0, str(SCRIPT_DIR))

import run_bazel_q027_lane_semantics as subject


GITHUB_SHA = "a" * 40
TAG_WINDOWS_LOCAL = "--build_metadata=TAG_windows_gnullvm_local=true"
COMMIT_METADATA = f"--build_metadata=COMMIT_SHA={GITHUB_SHA}"
SHARD_METADATA = "--build_metadata=TAG_windows_test_shard=1"


class ExactLaneSemanticsQualificationTest(unittest.TestCase):
    @staticmethod
    def common_options() -> tuple[str, ...]:
        return (
            "--host_platform=//:local_windows_msvc",
            "--platforms=//:windows_x86_64_gnullvm",
            TAG_WINDOWS_LOCAL,
            COMMIT_METADATA,
            subject.CANONICAL_ANNOUNCE_RC,
        )

    @staticmethod
    def env(**overrides: str) -> dict[str, str]:
        env = {
            "GITHUB_ACTIONS": "true",
            "GITHUB_SHA": GITHUB_SHA,
        }
        env.update(overrides)
        return env

    @classmethod
    def _test_args(cls, *extra: str) -> tuple[str, ...]:
        return (
            "test",
            "--config=ci-windows",
            *cls.common_options(),
            subject.CANONICAL_TEST_TAG_FILTER,
            subject.CANONICAL_SKIP_INCOMPATIBLE,
            subject.CANONICAL_TEST_VERBOSE_TIMEOUTS,
            subject.CANONICAL_REMOTE_DOWNLOAD_TOPLEVEL,
            SHARD_METADATA,
            *extra,
            "--",
            "//codex-rs/uds:uds-unit-tests-bin",
        )

    @classmethod
    def _clippy_args(cls, *extra: str) -> tuple[str, ...]:
        return (
            "build",
            "--config=clippy",
            "--config=ci-windows",
            *cls.common_options(),
            subject.CANONICAL_SKIP_INCOMPATIBLE,
            subject.CLIPPY_JOB_METADATA,
            *extra,
            "--",
            "//codex-rs/uds:uds-unit-tests-bin",
        )

    @classmethod
    def _release_args(cls, *extra: str) -> tuple[str, ...]:
        return (
            "build",
            "--config=ci-windows",
            *cls.common_options(),
            subject.RELEASE_COMPILATION_MODE,
            subject.RELEASE_RUSTC_FLAG,
            subject.RELEASE_EXEC_RUSTC_FLAG,
            subject.RELEASE_JOB_METADATA,
            subject.RELEASE_DEBUG_METADATA,
            *extra,
            "--",
            *subject.CANONICAL_RELEASE_TARGETS,
        )

    def validate(
        self,
        args: tuple[str, ...],
        *,
        env: dict[str, str] | None = None,
    ) -> None:
        with patch.object(subject, "_validate_q026"):
            subject.validate_keyless_windows_gnullvm_final_args(
                args,
                env or self.env(),
            )

    def test_canonical_test_lane_passes(self) -> None:
        self.validate(
            self._test_args(),
            env=self.env(
                BAZEL_TEST_SHARD="1",
                BAZEL_TEST_SHARD_COUNT="4",
            ),
        )

    def test_canonical_clippy_lane_passes(self) -> None:
        self.validate(self._clippy_args())

    def test_canonical_release_lane_passes(self) -> None:
        self.validate(self._release_args())

    def test_release_semantics_are_all_required(self) -> None:
        for missing in (
            subject.RELEASE_COMPILATION_MODE,
            subject.RELEASE_RUSTC_FLAG,
            subject.RELEASE_EXEC_RUSTC_FLAG,
            subject.RELEASE_DEBUG_METADATA,
        ):
            with self.subTest(missing=missing):
                args = list(self._release_args())
                args.remove(missing)
                with self.assertRaisesRegex(
                    ValueError,
                    "requires exactly|non-canonical build metadata",
                ):
                    self.validate(tuple(args))

    def test_release_semantic_override_fails_closed(self) -> None:
        with self.assertRaisesRegex(ValueError, "unreviewed explicit options"):
            self.validate(self._release_args("--compilation_mode=opt"))

    def test_clippy_config_cannot_be_neutralized(self) -> None:
        overrides = (
            "--aspects=",
            "--output_groups=default",
            "--@rules_rust//rust/settings:clippy_flag=--allow=warnings",
            "--@rules_rust//rust/settings:clippy.toml=//attacker:clippy.toml",
        )
        for option in overrides:
            with self.subTest(option=option):
                with self.assertRaisesRegex(
                    ValueError,
                    "unreviewed explicit options",
                ):
                    self.validate(self._clippy_args(option))

    def test_test_execution_cannot_be_weakened(self) -> None:
        overrides = (
            "--runs_per_test=0",
            "--flaky_test_attempts=0",
            "--test_sharding_strategy=disabled",
            "--cache_test_results=yes",
            "--nocheck_visibility",
        )
        for option in overrides:
            with self.subTest(option=option):
                with self.assertRaisesRegex(
                    ValueError,
                    "unreviewed explicit options",
                ):
                    self.validate(
                        self._test_args(option),
                        env=self.env(
                            BAZEL_TEST_SHARD="1",
                            BAZEL_TEST_SHARD_COUNT="4",
                        ),
                    )

    def test_arbitrary_build_settings_fail_closed(self) -> None:
        overrides = (
            "--define=disable_checks=true",
            "--features=attacker",
            "--copt=-w",
            "--linkopt=/FORCE",
            "--//attacker:mode=unsafe",
            "--@rules_rust//rust/settings:extra_rustc_flag=-Aprops",
        )
        for option in overrides:
            with self.subTest(option=option):
                with self.assertRaisesRegex(
                    ValueError,
                    "unreviewed explicit options",
                ):
                    self.validate(self._clippy_args(option))

    def test_announce_rc_is_exactly_once(self) -> None:
        args = list(self._clippy_args())
        args.remove(subject.CANONICAL_ANNOUNCE_RC)
        with self.assertRaisesRegex(ValueError, "requires exactly"):
            self.validate(tuple(args))

        with self.assertRaisesRegex(ValueError, "requires exactly"):
            self.validate(
                self._clippy_args(subject.CANONICAL_ANNOUNCE_RC)
            )

        with self.assertRaisesRegex(
            ValueError,
            "unreviewed explicit options",
        ):
            self.validate(self._clippy_args("--noannounce_rc"))

    def test_test_remote_download_contract_is_exact(self) -> None:
        args = list(self._test_args())
        args.remove(subject.CANONICAL_REMOTE_DOWNLOAD_TOPLEVEL)
        with self.assertRaisesRegex(ValueError, "requires exactly"):
            self.validate(
                tuple(args),
                env=self.env(
                    BAZEL_TEST_SHARD="1",
                    BAZEL_TEST_SHARD_COUNT="4",
                ),
            )

        with self.assertRaisesRegex(
            ValueError,
            "unreviewed explicit options",
        ):
            self.validate(
                self._test_args("--noremote_download_toplevel"),
                env=self.env(
                    BAZEL_TEST_SHARD="1",
                    BAZEL_TEST_SHARD_COUNT="4",
                ),
            )

    def test_commit_metadata_is_bound_to_github_sha(self) -> None:
        args = [
            option
            for option in self._clippy_args()
            if option != COMMIT_METADATA
        ]
        args.insert(
            args.index("--"),
            f"--build_metadata=COMMIT_SHA={'b' * 40}",
        )
        with self.assertRaisesRegex(ValueError, "does not match GITHUB_SHA"):
            self.validate(tuple(args))

    def test_test_shard_metadata_is_bound(self) -> None:
        with self.assertRaisesRegex(ValueError, "does not match BAZEL_TEST_SHARD"):
            self.validate(
                self._test_args(),
                env=self.env(
                    BAZEL_TEST_SHARD="2",
                    BAZEL_TEST_SHARD_COUNT="4",
                ),
            )

        with self.assertRaisesRegex(ValueError, "four-shard topology"):
            self.validate(
                self._test_args(),
                env=self.env(
                    BAZEL_TEST_SHARD="1",
                    BAZEL_TEST_SHARD_COUNT="8",
                ),
            )

    def test_unknown_metadata_fails_closed(self) -> None:
        with self.assertRaisesRegex(ValueError, "non-canonical build metadata"):
            self.validate(
                self._clippy_args(
                    "--build_metadata=TAG_job=clippy-shadow",
                )
            )


if __name__ == "__main__":
    unittest.main()
