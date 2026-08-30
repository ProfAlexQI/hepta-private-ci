#!/usr/bin/env python3

import json
import os
import shutil
import subprocess
import unittest
from pathlib import Path
from tempfile import TemporaryDirectory


SCRIPT = Path(__file__).with_name("run-bazel-ci.sh")


class RunBazelOptionGrammarTest(unittest.TestCase):
    @staticmethod
    def cross_args(option: str, value: str) -> list[str]:
        return [
            "--print-failed-action-summary",
            "--windows-cross-compile",
            "--",
            "build",
            "--config=clippy",
            option,
            value,
            "--",
            "//codex-rs/utils/rustls-provider:rustls-provider-provider-test",
        ]

    def run_wrapper(
        self, option: str, value: str
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
                "Path(os.environ['CAPTURE']).write_text("
                "json.dumps(sys.argv[1:]), encoding='utf-8')\n",
                encoding="utf-8",
            )
            wrapper.chmod(0o755)
            delegate.chmod(0o755)

            env = os.environ.copy()
            env["CAPTURE"] = str(capture)
            env["RUNNER_OS"] = "Windows"
            env["GITHUB_ACTIONS"] = "true"
            env.pop("BUILDBUDDY_API_KEY", None)
            env.pop("ALLOW_WINDOWS_MSVC_FALLBACK", None)

            bash = shutil.which("bash")
            if bash is None:
                self.skipTest("bash is required to exercise run-bazel-ci.sh")

            result = subprocess.run(
                [bash, str(wrapper), *self.cross_args(option, value)],
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

    def test_github_actions_rejects_separated_protected_values(self) -> None:
        protected_values = (
            ("--config", "ci-windows-cross"),
            ("--host_platform", "//:rbe"),
            ("--platforms", "//:windows_x86_64_msvc"),
            ("--repo_env", "BAZEL_DO_NOT_DETECT_CPP_TOOLCHAIN=1"),
            ("--extra_execution_platforms", "//:custom"),
            ("--extra_toolchains", "//:untrusted_toolchain"),
            ("--strategy", "TestRunner=remote"),
            ("--local_test_jobs", "99"),
            ("--jobs", "99"),
            ("--test_env", "RUST_TEST_THREADS=99"),
        )
        for option, value in protected_values:
            with self.subTest(option=option):
                result, captured = self.run_wrapper(option, value)
                self.assertNotEqual(result.returncode, 0)
                self.assertEqual(captured, [])
                self.assertIn(
                    "requires protected Bazel options in --option=value form",
                    result.stderr,
                )


if __name__ == "__main__":
    unittest.main()
