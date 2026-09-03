#!/usr/bin/env python3

import unittest

import run_bazel_with_buildbuddy as wrapper


class WindowsLocalFallbackTest(unittest.TestCase):
    def test_keyless_windows_invocation_selects_local_ci_config(self) -> None:
        self.assertEqual(
            wrapper.bazel_args_with_remote_config(
                ["build", "--", "//codex-rs/cli:codex"], {"RUNNER_OS": "Windows"}
            ),
            ["build", "--config=ci-windows", "--", "//codex-rs/cli:codex"],
        )

    def test_keyless_cross_fallback_uses_one_gnullvm_abi(self) -> None:
        for command in ("build", "test", "cquery", "aquery", "info"):
            with self.subTest(command=command):
                self.assertEqual(
                    wrapper.bazel_args_with_remote_config(
                        [command, "--config=ci-windows-cross", "--", "target"],
                        {"RUNNER_OS": "Windows"},
                    ),
                    [
                        command,
                        "--config=ci-windows",
                        "--host_platform=//:local_windows",
                        "--platforms=//:windows_x86_64_gnullvm",
                        "--",
                        "target",
                    ],
                )

    def test_clippy_does_not_hide_required_local_platform_config(self) -> None:
        self.assertEqual(
            wrapper.bazel_args_with_remote_config(
                ["build", "--config=clippy", "--config=ci-windows-cross", "//..."],
                {"RUNNER_OS": "Windows"},
            ),
            [
                "build",
                "--config=ci-windows",
                "--host_platform=//:local_windows",
                "--platforms=//:windows_x86_64_gnullvm",
                "--config=clippy",
                "//...",
            ],
        )

    def test_defaults_precede_explicit_caches_and_preserve_startup_options(self) -> None:
        args = [
            "--output_user_root=build root",
            "build",
            "--config=ci-windows-cross",
            "--repo_contents_cache=job cache",
            "--repository_cache=download cache",
            "--",
            "//...",
        ]
        self.assertEqual(
            wrapper.bazel_args_with_remote_config(args, {"RUNNER_OS": "Windows"}),
            [
                args[0],
                "build",
                "--config=ci-windows",
                "--host_platform=//:local_windows",
                "--platforms=//:windows_x86_64_gnullvm",
                *args[3:],
            ],
        )

    def test_explicit_host_and_target_platforms_are_preserved(self) -> None:
        for platforms in (
            ["--host_platform=//:custom-host", "--platforms=//:custom-target"],
            ["--host_platform", "//:custom-host", "--platforms", "//:custom-target"],
        ):
            with self.subTest(platforms=platforms):
                self.assertEqual(
                    wrapper.bazel_args_with_remote_config(
                        ["build", "--config=ci-windows-cross", *platforms, "//..."],
                        {"RUNNER_OS": "Windows"},
                    ),
                    ["build", "--config=ci-windows", *platforms, "//..."],
                )

    def test_explicit_msvc_host_is_not_silently_reinterpreted(self) -> None:
        self.assertEqual(
            wrapper.bazel_args_with_remote_config(
                ["build", "--host_platform=//:local_windows_msvc", "//..."],
                {"RUNNER_OS": "Windows"},
            ),
            [
                "build",
                "--config=ci-windows",
                "--host_platform=//:local_windows_msvc",
                "//...",
            ],
        )

    def test_existing_local_config_is_not_duplicated(self) -> None:
        args = ["build", "--config=ci-windows", "--config=ci-windows-cross", "//..."]
        self.assertEqual(
            wrapper.bazel_args_with_remote_config(args, {"RUNNER_OS": "Windows"}),
            [
                "build",
                "--host_platform=//:local_windows",
                "--platforms=//:windows_x86_64_gnullvm",
                "--config=ci-windows",
                "//...",
            ],
        )

    def test_target_discovery_does_not_inject_build_configuration(self) -> None:
        args = ["query", "--output=label", 'kind("rust_test rule", //codex-rs/...)']
        self.assertEqual(
            wrapper.bazel_args_with_remote_config(args, {"RUNNER_OS": "Windows"}),
            args,
        )

    def test_non_build_commands_do_not_gain_ci_flags(self) -> None:
        for command in ("query", "version", "help", "shutdown", "clean"):
            with self.subTest(command=command):
                self.assertEqual(
                    wrapper.bazel_args_with_remote_config(
                        [command], {"RUNNER_OS": "Windows"}
                    ),
                    [command],
                )

    def test_arguments_after_separator_are_neither_interpreted_nor_removed(self) -> None:
        payload = ["--config=ci-windows-cross", "--platforms=//:payload", "spaced value"]
        args = ["run", "--config=ci-windows-cross", "//:tool", "--", *payload]
        self.assertEqual(
            wrapper.bazel_args_with_remote_config(args, {"RUNNER_OS": "Windows"}),
            [
                "run",
                "--config=ci-windows",
                "--host_platform=//:local_windows",
                "--platforms=//:windows_x86_64_gnullvm",
                "//:tool",
                "--",
                *payload,
            ],
        )

    def test_non_windows_local_invocation_stays_native(self) -> None:
        args = ["build", "--config=ci-linux", "--", "//..."]
        self.assertEqual(
            wrapper.bazel_args_with_remote_config(args, {"RUNNER_OS": "Linux"}),
            ["build", "--", "//..."],
        )

    def test_authenticated_cross_request_remains_rbe(self) -> None:
        args = ["build", "--config=ci-windows-cross", "//codex-rs/cli:codex"]
        self.assertEqual(
            wrapper.bazel_args_with_remote_config(
                args, {"RUNNER_OS": "Windows", "BUILDBUDDY_API_KEY": "fork-token"}
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
