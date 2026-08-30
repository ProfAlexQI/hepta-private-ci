#!/usr/bin/env python3

import json
import os
import shutil
import subprocess
import unittest
from pathlib import Path
from tempfile import TemporaryDirectory


SCRIPT = Path(__file__).with_name("run-bazel-ci.sh")

CI_HOST = "--host_platform=//:local_windows_msvc"
CI_TARGET = "--platforms=//:windows_x86_64_gnullvm"
CI_CC_DISCOVERY = "--repo_env=BAZEL_DO_NOT_DETECT_CPP_TOOLCHAIN=0"
CI_EXECUTION_PLATFORMS = "--extra_execution_platforms=//:windows_x86_64_msvc"
CI_TOOLCHAINS = (
    "--extra_toolchains=//:windows_gnullvm_tests_on_msvc_host_toolchain,"
    "//bazel/toolchains/windows:local_msvc_cc_toolchain"
)
CI_TEST_RUNNER = "--strategy=TestRunner=local"
CI_V8_RUNNER = "--strategy=V8Mksnapshot=local"
CI_LOCAL_TEST_JOBS = "--local_test_jobs=8"
CI_JOBS = "--jobs=8"
CI_TEST_THREADS = "--test_env=RUST_TEST_THREADS=1"
CI_TEST_FILTERS = (
    "--test_env=CODEX_BAZEL_TEST_SKIP_FILTERS="
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

    @staticmethod
    def prefix_values(captured: list[str], prefix: str) -> list[str]:
        return [arg for arg in captured if arg.startswith(prefix)]

    def test_keyless_github_cross_uses_exact_split_toolchain_contract(self) -> None:
        result, captured = self.run_wrapper(
            self.cross_args(), github_actions=True
        )
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertNotIn("--windows-cross-compile", captured)
        self.assertEqual(captured[0], "--print-failed-action-summary")
        for expected in (
            "--windows-msvc-host-platform",
            "--config=ci-windows",
            CI_HOST,
            CI_TARGET,
            CI_CC_DISCOVERY,
            CI_EXECUTION_PLATFORMS,
            CI_TOOLCHAINS,
            CI_TEST_RUNNER,
            CI_V8_RUNNER,
            CI_LOCAL_TEST_JOBS,
            CI_TEST_THREADS,
            CI_TEST_FILTERS,
            "--build_metadata=TAG_windows_gnullvm_local=true",
            CI_JOBS,
        ):
            self.assertIn(expected, captured)
        self.assertNotIn("--platforms=//:local_windows_msvc", captured)
        self.assertEqual(captured[-1], self.cross_args()[-1])
        for prefix in (
            "--host_platform=",
            "--platforms=",
            "--repo_env=BAZEL_DO_NOT_DETECT_CPP_TOOLCHAIN=",
            "--extra_execution_platforms=",
            "--extra_toolchains=",
            "--strategy=TestRunner=",
            "--strategy=V8Mksnapshot=",
            "--local_test_jobs=",
            "--jobs=",
            "--test_env=RUST_TEST_THREADS=",
            "--test_env=CODEX_BAZEL_TEST_SKIP_FILTERS=",
        ):
            self.assertEqual(
                len(self.prefix_values(captured, prefix)),
                1,
                (prefix, captured),
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

    def test_github_actions_rejects_conflicting_target(self) -> None:
        result, captured = self.run_wrapper(
            self.cross_args("--platforms=//:windows_x86_64_msvc"),
            github_actions=True,
        )
        self.assertNotEqual(result.returncode, 0)
        self.assertEqual(captured, [])
        self.assertIn("rejects conflicting argument", result.stderr)

    def test_github_actions_rejects_conflicting_host(self) -> None:
        result, captured = self.run_wrapper(
            self.cross_args("--host_platform=//:rbe"),
            github_actions=True,
        )
        self.assertNotEqual(result.returncode, 0)
        self.assertEqual(captured, [])
        self.assertIn("rejects conflicting argument", result.stderr)

    def test_github_actions_rejects_disabled_cc_discovery(self) -> None:
        result, captured = self.run_wrapper(
            self.cross_args(
                "--repo_env=BAZEL_DO_NOT_DETECT_CPP_TOOLCHAIN=1"
            ),
            github_actions=True,
        )
        self.assertNotEqual(result.returncode, 0)
        self.assertEqual(captured, [])
        self.assertIn("rejects conflicting argument", result.stderr)

    def test_github_actions_rejects_missing_execution_platform(self) -> None:
        result, captured = self.run_wrapper(
            self.cross_args("--extra_execution_platforms=//:custom"),
            github_actions=True,
        )
        self.assertNotEqual(result.returncode, 0)
        self.assertEqual(captured, [])
        self.assertIn("rejects non-canonical entry", result.stderr)

    def test_github_actions_rejects_missing_required_toolchain(self) -> None:
        result, captured = self.run_wrapper(
            self.cross_args(
                "--extra_toolchains=//:windows_gnullvm_tests_on_msvc_host_toolchain"
            ),
            github_actions=True,
        )
        self.assertNotEqual(result.returncode, 0)
        self.assertEqual(captured, [])
        self.assertIn(
            "missing '//bazel/toolchains/windows:local_msvc_cc_toolchain'",
            result.stderr,
        )

    def test_github_actions_rejects_additional_execution_platform(self) -> None:
        result, captured = self.run_wrapper(
            self.cross_args(
                "--extra_execution_platforms=//:custom,//:windows_x86_64_msvc"
            ),
            github_actions=True,
        )
        self.assertNotEqual(result.returncode, 0)
        self.assertEqual(captured, [])
        self.assertIn("rejects non-canonical entry '//:custom'", result.stderr)

    def test_github_actions_rejects_additional_toolchain(self) -> None:
        result, captured = self.run_wrapper(
            self.cross_args(
                "--extra_toolchains=//:custom,//:windows_gnullvm_tests_on_msvc_host_toolchain,//bazel/toolchains/windows:local_msvc_cc_toolchain"
            ),
            github_actions=True,
        )
        self.assertNotEqual(result.returncode, 0)
        self.assertEqual(captured, [])
        self.assertIn("rejects non-canonical entry '//:custom'", result.stderr)

    def test_github_actions_rejects_duplicate_execution_platform(self) -> None:
        result, captured = self.run_wrapper(
            self.cross_args(
                "--extra_execution_platforms=//:windows_x86_64_msvc",
                "--extra_execution_platforms=//:windows_x86_64_msvc",
            ),
            github_actions=True,
        )
        self.assertNotEqual(result.returncode, 0)
        self.assertEqual(captured, [])
        self.assertIn("rejects duplicate entry", result.stderr)

    def test_github_actions_rejects_duplicate_toolchain(self) -> None:
        result, captured = self.run_wrapper(
            self.cross_args(
                "--extra_toolchains=//:windows_gnullvm_tests_on_msvc_host_toolchain,//bazel/toolchains/windows:local_msvc_cc_toolchain",
                "--extra_toolchains=//:windows_gnullvm_tests_on_msvc_host_toolchain",
            ),
            github_actions=True,
        )
        self.assertNotEqual(result.returncode, 0)
        self.assertEqual(captured, [])
        self.assertIn("rejects duplicate entry", result.stderr)

    def test_github_actions_canonicalizes_exact_execution_lists(self) -> None:
        result, captured = self.run_wrapper(
            self.cross_args(
                "--extra_execution_platforms=//:windows_x86_64_msvc",
                "--extra_toolchains=//bazel/toolchains/windows:local_msvc_cc_toolchain,//:windows_gnullvm_tests_on_msvc_host_toolchain",
            ),
            github_actions=True,
        )
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(
            self.prefix_values(captured, "--extra_execution_platforms="),
            [CI_EXECUTION_PLATFORMS],
        )
        self.assertEqual(
            self.prefix_values(captured, "--extra_toolchains="),
            [CI_TOOLCHAINS],
        )

    def test_github_actions_places_canonical_contract_after_configs(self) -> None:
        result, captured = self.run_wrapper(
            self.cross_args(
                CI_HOST,
                CI_TARGET,
                CI_CC_DISCOVERY,
                CI_TEST_FILTERS,
            ),
            github_actions=True,
        )
        self.assertEqual(result.returncode, 0, result.stderr)
        last_config = max(
            index
            for index, arg in enumerate(captured)
            if arg.startswith("--config=")
        )
        for expected in (
            CI_HOST,
            CI_TARGET,
            CI_CC_DISCOVERY,
            CI_EXECUTION_PLATFORMS,
            CI_TOOLCHAINS,
            CI_TEST_RUNNER,
            CI_V8_RUNNER,
            CI_LOCAL_TEST_JOBS,
            CI_JOBS,
            CI_TEST_THREADS,
            CI_TEST_FILTERS,
        ):
            self.assertGreater(captured.index(expected), last_config, captured)

    def test_github_actions_rejects_remote_config(self) -> None:
        result, captured = self.run_wrapper(
            self.cross_args("--config=ci-windows-cross"),
            github_actions=True,
        )
        self.assertNotEqual(result.returncode, 0)
        self.assertEqual(captured, [])
        self.assertIn("rejects non-allowlisted Bazel config", result.stderr)

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

    def test_local_non_actions_late_conflicting_platform_is_rejected(self) -> None:
        result, captured = self.run_wrapper(
            self.cross_args(
                CI_HOST,
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
