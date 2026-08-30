#!/usr/bin/env python3

import json
import os
import shutil
import subprocess
import unittest
from pathlib import Path
from tempfile import TemporaryDirectory


SCRIPT = Path(__file__).with_name("run-bazel-ci.sh")


class RunBazelCiWrapperTest(unittest.TestCase):
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
            delegate = root / "run-bazel-ci-core.sh"
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

    def test_keyless_windows_cross_uses_real_local_gnullvm_target(self) -> None:
        args = self.cross_args()
        result, captured = self.run_wrapper(args, github_actions=True)

        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertNotIn("--windows-cross-compile", captured)
        self.assertEqual(captured[0], "--print-failed-action-summary")
        self.assertIn("--config=ci-windows", captured)
        self.assertIn("--config=clippy", captured)
        self.assertLess(
            captured.index("--config=ci-windows"), captured.index("--config=clippy")
        )
        self.assertIn("--host_platform=//:local_windows_msvc", captured)
        self.assertIn("--platforms=//:windows_x86_64_gnullvm", captured)
        self.assertIn(
            "--extra_execution_platforms=//:windows_x86_64_msvc", captured
        )
        self.assertIn(
            "--extra_toolchains=//:windows_gnullvm_tests_on_msvc_host_toolchain",
            captured,
        )
        self.assertIn("--build_metadata=TAG_windows_gnullvm_local=true", captured)
        self.assertIn("--jobs=8", captured)
        self.assertEqual(captured[-1], args[-1])

    def test_authenticated_windows_cross_delegates_unchanged(self) -> None:
        args = self.cross_args()
        result, captured = self.run_wrapper(
            args, buildbuddy_key="token", github_actions=True
        )
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(captured, args)

    def test_local_explicit_msvc_diagnostic_delegates_unchanged(self) -> None:
        args = self.cross_args()
        result, captured = self.run_wrapper(args, allow_msvc_fallback=True)
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(captured, args)

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

    def test_github_actions_rejects_missing_required_execution_platform(self) -> None:
        result, captured = self.run_wrapper(
            self.cross_args("--extra_execution_platforms=//:custom"),
            github_actions=True,
        )
        self.assertNotEqual(result.returncode, 0)
        self.assertEqual(captured, [])
        self.assertIn("requires '//:windows_x86_64_msvc'", result.stderr)

    def test_github_actions_accepts_required_execution_lists(self) -> None:
        result, captured = self.run_wrapper(
            self.cross_args(
                "--extra_execution_platforms=//:custom,//:windows_x86_64_msvc",
                "--extra_toolchains=//:custom,//:windows_gnullvm_tests_on_msvc_host_toolchain",
            ),
            github_actions=True,
        )
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertIn(
            "--extra_execution_platforms=//:custom,//:windows_x86_64_msvc",
            captured,
        )

    def test_explicit_local_caller_platform_and_concurrency_remain_authoritative(self) -> None:
        args = self.cross_args(
            "--host_platform=//:custom_host",
            "--platforms=//:custom_target",
            "--jobs=3",
        )
        result, captured = self.run_wrapper(args)
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertIn("--host_platform=//:custom_host", captured)
        self.assertIn("--platforms=//:custom_target", captured)
        self.assertIn("--jobs=3", captured)
        self.assertNotIn("--host_platform=//:local_windows_msvc", captured)
        self.assertNotIn("--platforms=//:windows_x86_64_gnullvm", captured)
        self.assertNotIn("--jobs=8", captured)

    def test_non_windows_cross_delegates_unchanged(self) -> None:
        args = self.cross_args()
        result, captured = self.run_wrapper(args, runner_os="Linux")
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(captured, args)


if __name__ == "__main__":
    unittest.main()
