#!/usr/bin/env python3

import unittest

import run_bazel_with_buildbuddy


class LocalWindowsMsvcFallbackTest(unittest.TestCase):
    def test_keyless_windows_msvc_fallback_keeps_local_ci_policy(self) -> None:
        args = [
            "build",
            "--config=clippy",
            "--host_platform=//:local_windows_msvc",
            "--platforms=//:local_windows_msvc",
            "--jobs=8",
            "--",
            "//codex-rs/uds:uds-unit-tests-bin",
        ]

        self.assertEqual(
            run_bazel_with_buildbuddy.bazel_args_with_remote_config(
                args,
                {"RUNNER_OS": "Windows"},
            ),
            [
                "build",
                "--config=ci-windows",
                "--config=clippy",
                "--host_platform=//:local_windows_msvc",
                "--platforms=//:local_windows_msvc",
                "--jobs=8",
                "--",
                "//codex-rs/uds:uds-unit-tests-bin",
            ],
        )

    def test_local_policy_is_not_injected_for_non_windows_hosts(self) -> None:
        args = [
            "build",
            "--host_platform=//:local_windows_msvc",
            "--platforms=//:local_windows_msvc",
            "--",
            "//codex-rs/uds:uds-unit-tests-bin",
        ]

        self.assertEqual(
            run_bazel_with_buildbuddy.bazel_args_with_remote_config(
                args,
                {"RUNNER_OS": "Linux"},
            ),
            args,
        )

    def test_program_arguments_cannot_select_local_windows_policy(self) -> None:
        args = [
            "run",
            "//codex-rs/cli:codex",
            "--",
            "--host_platform=//:local_windows_msvc",
            "--platforms=//:local_windows_msvc",
        ]

        self.assertEqual(
            run_bazel_with_buildbuddy.bazel_args_with_remote_config(
                args,
                {"RUNNER_OS": "Windows"},
            ),
            args,
        )


if __name__ == "__main__":
    unittest.main()
