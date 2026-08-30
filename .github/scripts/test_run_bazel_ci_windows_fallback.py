#!/usr/bin/env python3

import os
import shutil
import subprocess
import unittest
from pathlib import Path
from tempfile import TemporaryDirectory

SCRIPT_DIR = Path(__file__).resolve().parent
RUN_BAZEL_CI = SCRIPT_DIR / "run-bazel-ci.sh"
FAIL_CLOSED_MESSAGE = (
    "refusing to substitute an MSVC result for a gnullvm check"
)


class RunBazelCiWindowsFallbackTest(unittest.TestCase):
    def base_env(self) -> dict[str, str]:
        return {
            "HOME": os.environ.get("HOME", str(Path.home())),
            "PATH": os.environ.get("PATH", ""),
            "RUNNER_OS": "Windows",
            "CODEX_BAZEL_WINDOWS_PATH": r"C:\Windows\System32",
        }

    def run_stubbed_fixture(
        self, extra_env: dict[str, str]
    ) -> tuple[subprocess.CompletedProcess[str], list[str]]:
        with TemporaryDirectory() as temp_dir:
            fixture_dir = Path(temp_dir)
            fixture_script = fixture_dir / "run-bazel-ci.sh"
            wrapper = fixture_dir / "run_bazel_with_buildbuddy.py"
            capture = fixture_dir / "args.txt"
            shutil.copy2(RUN_BAZEL_CI, fixture_script)
            wrapper.write_text(
                "#!/usr/bin/env bash\nprintf '%s\\n' \"$@\" > \"$ARG_CAPTURE\"\n",
                encoding="utf-8",
            )
            wrapper.chmod(0o755)

            env = self.base_env()
            env.update(
                {
                    "ARG_CAPTURE": str(capture),
                    "GITHUB_ACTIONS": "true",
                    "GITHUB_JOB": "bazel-wrapper-unit-test",
                }
            )
            env.update(extra_env)
            result = subprocess.run(
                [
                    "bash",
                    str(fixture_script),
                    "--windows-cross-compile",
                    "--",
                    "build",
                    "--",
                    "//:diagnostic-fixture",
                ],
                check=False,
                capture_output=True,
                env=env,
                text=True,
            )
            args = capture.read_text(encoding="utf-8").splitlines() if capture.exists() else []
            return result, args

    def test_keyless_cross_compile_fails_before_bazel(self) -> None:
        result = subprocess.run(
            [
                "bash",
                str(RUN_BAZEL_CI),
                "--windows-cross-compile",
                "--",
                "build",
                "--",
                "//:must-not-run",
            ],
            check=False,
            capture_output=True,
            env=self.base_env(),
            text=True,
        )

        self.assertNotEqual(result.returncode, 0)
        self.assertIn(FAIL_CLOSED_MESSAGE, result.stderr)
        self.assertNotIn("BuildBuddy API key is not available", result.stdout)

    def test_explicit_nonqualifying_msvc_diagnostic_is_abi_coherent(self) -> None:
        result, args = self.run_stubbed_fixture(
            {"ALLOW_WINDOWS_MSVC_FALLBACK": "1"}
        )

        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertIn("--host_platform=//:local_windows_msvc", args)
        self.assertIn("--platforms=//:local_windows_msvc", args)
        self.assertIn("--jobs=8", args)
        self.assertNotIn("--config=ci-windows-cross", args)

    def test_authenticated_cross_compile_preserves_rbe_semantics(self) -> None:
        result, args = self.run_stubbed_fixture(
            {"BUILDBUDDY_API_KEY": "fixture-token"}
        )

        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertIn("--config=ci-windows-cross", args)
        self.assertIn("--host_platform=//:rbe", args)
        self.assertIn("--shell_executable=/bin/bash", args)
        self.assertNotIn("--host_platform=//:local_windows_msvc", args)
        self.assertNotIn("--platforms=//:local_windows_msvc", args)


if __name__ == "__main__":
    unittest.main()
