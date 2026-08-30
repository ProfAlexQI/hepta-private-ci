#!/usr/bin/env python3

import json
import os
import shutil
import subprocess
import unittest
from pathlib import Path
from tempfile import TemporaryDirectory


SCRIPT = Path(__file__).with_name("run-bazel-ci.sh")
CANONICAL_SKIP = (
    "command_safety::powershell_parser::tests::,"
    "suite::code_mode::code_mode_can_call_hidden_dynamic_tools,"
    "tests::windows_tests::conpty_ctrl_c_interrupts_powershell_foreground_child"
)


class RunBazelCiQualificationWrapperTest(unittest.TestCase):
    def run_wrapper(
        self,
        args: list[str],
        *,
        runner_os: str = "Windows",
        buildbuddy_key: str | None = None,
        allow_msvc_fallback: bool = False,
        github_actions: bool = False,
    ) -> tuple[subprocess.CompletedProcess[str], list[str]]:
        with TemporaryDirectory() as temp_dir:
            root = Path(temp_dir)
            wrapper = root / "run-bazel-ci.sh"
            delegate = root / "run-bazel-ci-impl.sh"
            capture = root / "capture.json"
            shutil.copy2(SCRIPT, wrapper)
            delegate.write_text(
                "#!/usr/bin/env python3\n"
                "import json, os, sys\n"
                "from pathlib import Path\n"
                "Path(os.environ['CAPTURE']).write_text(json.dumps(sys.argv[1:]), encoding='utf-8')\n",
                encoding="utf-8",
            )
            wrapper.chmod(0o755)
            delegate.chmod(0o755)

            env = os.environ.copy()
            env["CAPTURE"] = str(capture)
            env["RUNNER_OS"] = runner_os
            if buildbuddy_key is None:
                env.pop("BUILDBUDDY_API_KEY", None)
            else:
                env["BUILDBUDDY_API_KEY"] = buildbuddy_key
            if allow_msvc_fallback:
                env["ALLOW_WINDOWS_MSVC_FALLBACK"] = "1"
            else:
                env.pop("ALLOW_WINDOWS_MSVC_FALLBACK", None)
            if github_actions:
                env["GITHUB_ACTIONS"] = "true"
            else:
                env.pop("GITHUB_ACTIONS", None)

            bash = shutil.which("bash")
            if bash is None:
                self.skipTest("bash is required to exercise run-bazel-ci.sh")

            result = subprocess.run(
                [bash, str(wrapper), *args],
                env=env,
                check=False,
                capture_output=True,
                text=True,
            )
            captured = (
                json.loads(capture.read_text(encoding="utf-8"))
                if capture.exists()
                else []
            )
            return result, captured

    @staticmethod
    def cross_args(*extra: str) -> list[str]:
        return [
            "--print-failed-action-summary",
            "--windows-cross-compile",
            "--",
            "build",
            "--config=clippy",
            *extra,
            "--",
            "//codex-rs/utils/rustls-provider:rustls-provider-provider-test",
        ]

    def test_keyless_github_cross_uses_exact_split_toolchain_contract(self) -> None:
        result, captured = self.run_wrapper(
            self.cross_args(), github_actions=True
        )
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertNotIn("--windows-cross-compile", captured)
        self.assertIn("--windows-local-gnullvm", captured)
        self.assertNotIn("--windows-msvc-host-platform", captured)
        self.assertEqual(captured[0], "--print-failed-action-summary")
        expected_options = (
            "--config=ci-windows",
            "--host_platform=//:local_windows_msvc",
            "--platforms=//:windows_x86_64_gnullvm",
            "--repo_env=BAZEL_DO_NOT_DETECT_CPP_TOOLCHAIN=0",
            "--extra_execution_platforms=//:windows_x86_64_msvc",
            "--extra_toolchains=//bazel/toolchains/windows:local_msvc_cc_toolchain",
            "--extra_toolchains=//:windows_gnullvm_tests_on_msvc_host_toolchain",
            "--strategy=TestRunner=local",
            "--strategy=V8Mksnapshot=local",
            "--local_test_jobs=8",
            "--test_env=RUST_TEST_THREADS=1",
            f"--test_env=CODEX_BAZEL_TEST_SKIP_FILTERS={CANONICAL_SKIP}",
            "--build_metadata=TAG_windows_gnullvm_local=true",
            "--jobs=8",
        )
        for expected in expected_options:
            self.assertIn(expected, captured)
        self.assertNotIn("--platforms=//:local_windows_msvc", captured)
        self.assertEqual(captured[-1], self.cross_args()[-1])

        config_index = captured.index("--config=ci-windows")
        for option in expected_options[1:]:
            self.assertGreater(
                captured.index(option),
                config_index,
                f"canonical override must follow ci-windows: {option}",
            )
        for prefix in (
            "--host_platform=",
            "--platforms=",
            "--repo_env=BAZEL_DO_NOT_DETECT_CPP_TOOLCHAIN=",
            "--extra_execution_platforms=",
            "--strategy=TestRunner=",
            "--strategy=V8Mksnapshot=",
            "--local_test_jobs=",
            "--jobs=",
            "--test_env=RUST_TEST_THREADS=",
            "--test_env=CODEX_BAZEL_TEST_SKIP_FILTERS=",
        ):
            self.assertEqual(
                sum(arg.startswith(prefix) for arg in captured),
                1,
                f"expected one canonical {prefix} option: {captured}",
            )
        self.assertEqual(
            sum(arg.startswith("--extra_toolchains=") for arg in captured),
            2,
            captured,
        )

    def test_authenticated_windows_cross_delegates_unchanged(self) -> None:
        args = self.cross_args()
        result, captured = self.run_wrapper(
            args, buildbuddy_key="token", github_actions=True
        )
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(captured, args)

    def test_local_explicit_msvc_diagnostic_is_preserved(self) -> None:
        result, captured = self.run_wrapper(
            self.cross_args(), allow_msvc_fallback=True
        )
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertIn("--windows-msvc-host-platform", captured)
        self.assertNotIn("--windows-local-gnullvm", captured)
        self.assertIn("--platforms=//:local_windows_msvc", captured)
        self.assertIn(
            "--extra_toolchains=//bazel/toolchains/windows:local_msvc_cc_toolchain",
            captured,
        )
        self.assertNotIn(
            "--extra_toolchains=//:windows_gnullvm_tests_on_msvc_host_toolchain",
            captured,
        )

    def test_github_actions_rejects_ambient_msvc_fallback(self) -> None:
        result, captured = self.run_wrapper(
            self.cross_args(),
            allow_msvc_fallback=True,
            github_actions=True,
        )
        self.assertNotEqual(result.returncode, 0)
        self.assertEqual(captured, [])
        self.assertIn("forbidden in GitHub Actions", result.stderr)

    def test_github_actions_rejects_every_caller_owned_critical_option(self) -> None:
        critical_options = (
            "--host_platform=//:local_windows_msvc",
            "--platforms=//:windows_x86_64_gnullvm",
            "--repo_env=BAZEL_DO_NOT_DETECT_CPP_TOOLCHAIN=0",
            "--extra_execution_platforms=//:windows_x86_64_msvc",
            "--extra_toolchains=//:windows_gnullvm_tests_on_msvc_host_toolchain",
            "--strategy=TestRunner=local",
            "--strategy=V8Mksnapshot=local",
            "--local_test_jobs=8",
            "--jobs=8",
            "--test_env=RUST_TEST_THREADS=1",
            f"--test_env=CODEX_BAZEL_TEST_SKIP_FILTERS={CANONICAL_SKIP}",
            "--action_env=INCLUDE",
            "--host_action_env=INCLUDE",
        )
        for option in critical_options:
            with self.subTest(option=option):
                result, captured = self.run_wrapper(
                    self.cross_args(option), github_actions=True
                )
                self.assertNotEqual(result.returncode, 0)
                self.assertEqual(captured, [])
                self.assertIn("qualification owns", result.stderr)

    def test_github_actions_rejects_custom_execution_platform_even_with_required_one(self) -> None:
        result, captured = self.run_wrapper(
            self.cross_args(
                "--extra_execution_platforms=//:custom,//:windows_x86_64_msvc"
            ),
            github_actions=True,
        )
        self.assertNotEqual(result.returncode, 0)
        self.assertEqual(captured, [])
        self.assertIn("qualification owns --extra_execution_platforms=", result.stderr)

    def test_github_actions_rejects_remote_or_platform_configs(self) -> None:
        for config in (
            "ci-windows",
            "ci-windows-cross",
            "remote",
            "buildbuddy-generic-rbe",
            "buildbuddy-openai-rbe",
        ):
            with self.subTest(config=config):
                result, captured = self.run_wrapper(
                    self.cross_args(f"--config={config}"), github_actions=True
                )
                self.assertNotEqual(result.returncode, 0)
                self.assertEqual(captured, [])
                self.assertIn(f"forbids --config={config}", result.stderr)

    def test_local_non_actions_conflicting_platform_is_rejected(self) -> None:
        result, captured = self.run_wrapper(
            self.cross_args(
                "--host_platform=//:custom_host",
                "--platforms=//:custom_target",
                "--jobs=3",
            )
        )
        self.assertNotEqual(result.returncode, 0)
        self.assertEqual(captured, [])
        self.assertIn("requires --host_platform=//:local_windows_msvc", result.stderr)

    def test_local_non_actions_later_conflicting_duplicate_is_rejected(self) -> None:
        result, captured = self.run_wrapper(
            self.cross_args(
                "--host_platform=//:local_windows_msvc",
                "--host_platform=//:custom_host",
            )
        )
        self.assertNotEqual(result.returncode, 0)
        self.assertEqual(captured, [])
        self.assertIn("requires --host_platform=//:local_windows_msvc", result.stderr)

    def test_non_windows_cross_delegates_unchanged(self) -> None:
        args = self.cross_args()
        result, captured = self.run_wrapper(args, runner_os="Linux")
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(captured, args)


if __name__ == "__main__":
    unittest.main()
