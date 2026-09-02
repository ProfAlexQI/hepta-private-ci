#!/usr/bin/env python3

import unittest

import run_bazel_with_buildbuddy


class WindowsLocalFallbackTest(unittest.TestCase):
    def test_keyless_windows_invocation_selects_local_ci_config(self) -> None:
        self.assertEqual(
            run_bazel_with_buildbuddy.bazel_args_with_remote_config(
                ["build", "--", "//codex-rs/cli:codex"],
                {"RUNNER_OS": "Windows"},
            ),
            [
                "build",
                "--config=ci-windows",
                "--",
                "//codex-rs/cli:codex",
            ],
        )

    def test_keyless_cross_fallback_uses_one_msvc_abi(self) -> None:
        self.assertEqual(
            run_bazel_with_buildbuddy.bazel_args_with_remote_config(
                [
                    "test",
                    "--host_platform=//:local_windows_msvc",
                    "--",
                    "//codex-rs/utils/rustls-provider:rustls-provider-provider-test",
                ],
                {"RUNNER_OS": "Windows"},
            ),
            [
                "test",
                "--host_platform=//:local_windows_msvc",
                "--config=ci-windows",
                "--platforms=//:windows_x86_64_msvc",
                "--",
                "//codex-rs/utils/rustls-provider:rustls-provider-provider-test",
            ],
        )

    def test_keyless_cross_config_is_replaced_not_stacked(self) -> None:
        self.assertEqual(
            run_bazel_with_buildbuddy.bazel_args_with_remote_config(
                [
                    "build",
                    "--config=ci-windows-cross",
                    "--",
                    "//codex-rs/cli:codex",
                ],
                {"RUNNER_OS": "Windows"},
            ),
            [
                "build",
                "--config=ci-windows",
                "--platforms=//:windows_x86_64_msvc",
                "--",
                "//codex-rs/cli:codex",
            ],
        )

    def test_explicit_target_platform_is_preserved(self) -> None:
        self.assertEqual(
            run_bazel_with_buildbuddy.bazel_args_with_remote_config(
                [
                    "build",
                    "--host_platform=//:local_windows_msvc",
                    "--platforms=//:custom-windows-platform",
                    "--",
                    "//codex-rs/cli:codex",
                ],
                {"RUNNER_OS": "Windows"},
            ),
            [
                "build",
                "--host_platform=//:local_windows_msvc",
                "--platforms=//:custom-windows-platform",
                "--config=ci-windows",
                "--",
                "//codex-rs/cli:codex",
            ],
        )

    def test_authenticated_cross_request_remains_rbe(self) -> None:
        args = ["build", "--config=ci-windows-cross", "//codex-rs/cli:codex"]
        self.assertEqual(
            run_bazel_with_buildbuddy.bazel_args_with_remote_config(
                args,
                {
                    "RUNNER_OS": "Windows",
                    "BUILDBUDDY_API_KEY": "fork-token",
                },
            ),
            [
                "build",
                "--config=buildbuddy-generic-rbe",
                "--remote_header=x-buildbuddy-api-key=fork-token",
                "--config=ci-windows-cross",
                "//codex-rs/cli:codex",
            ],
        )


if __name__ == "__main__":
    unittest.main()
