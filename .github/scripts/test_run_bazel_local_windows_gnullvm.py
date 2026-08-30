#!/usr/bin/env python3

import json
import os
import shutil
import subprocess
import tempfile
import unittest
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[2]
SOURCE_LAUNCHER = REPO_ROOT / ".github" / "scripts" / "run-bazel-ci.sh"
TARGET = "//codex-rs/uds:uds-unit-tests-bin"


class LocalWindowsGnullvmFallbackTest(unittest.TestCase):
    def setUp(self) -> None:
        bash = shutil.which("bash")
        if bash is None:
            self.skipTest("bash is required for the Bazel launcher contract")
        self.bash = bash
        self.tempdir = tempfile.TemporaryDirectory()
        self.addCleanup(self.tempdir.cleanup)
        root = Path(self.tempdir.name)
        scripts = root / ".github" / "scripts"
        scripts.mkdir(parents=True)
        self.launcher = scripts / "run-bazel-ci.sh"
        shutil.copy2(SOURCE_LAUNCHER, self.launcher)
        self.launcher.chmod(0o755)
        recorder = scripts / "run_bazel_with_buildbuddy.py"
        recorder.write_text(
            """#!/usr/bin/env python3
import json
import os
import sys
from pathlib import Path

Path(os.environ[\"ARG_CAPTURE\"]).write_text(
    json.dumps(sys.argv[1:], indent=2) + \"\\n\", encoding=\"utf-8\"
)
""",
            encoding="utf-8",
        )
        recorder.chmod(0o755)
        self.root = root

    def run_launcher(
        self,
        *,
        extra_env: dict[str, str] | None = None,
        bazel_args: list[str] | None = None,
    ) -> tuple[subprocess.CompletedProcess[str], list[str] | None]:
        capture = self.root / "args.json"
        capture.unlink(missing_ok=True)
        env = os.environ.copy()
        env.pop("BUILDBUDDY_API_KEY", None)
        env.pop("ALLOW_WINDOWS_MSVC_FALLBACK", None)
        env.update(
            {
                "RUNNER_OS": "Windows",
                "CODEX_BAZEL_WINDOWS_PATH": r"C:\Windows\System32",
                "ARG_CAPTURE": str(capture),
                "INCLUDE": r"C:\msvc\include",
                "LIB": r"C:\msvc\lib",
            }
        )
        if extra_env:
            env.update(extra_env)
        args = bazel_args or ["build", "--config=clippy"]
        completed = subprocess.run(
            [
                self.bash,
                str(self.launcher),
                "--windows-cross-compile",
                "--",
                *args,
                "--",
                TARGET,
            ],
            cwd=self.root,
            env=env,
            text=True,
            capture_output=True,
            check=False,
        )
        captured = (
            json.loads(capture.read_text(encoding="utf-8"))
            if capture.exists()
            else None
        )
        return completed, captured

    def test_keyless_default_executes_exact_local_gnullvm_platform(self) -> None:
        completed, args = self.run_launcher()
        self.assertEqual(completed.returncode, 0, completed.stderr)
        self.assertIsNotNone(args)
        assert args is not None
        self.assertIn("--config=ci-windows", args)
        self.assertIn("--host_platform=//:local_windows", args)
        self.assertIn("--platforms=//:windows_x86_64_gnullvm", args)
        self.assertIn("--jobs=8", args)
        self.assertNotIn("--host_platform=//:local_windows_msvc", args)
        self.assertNotIn("--platforms=//:local_windows_msvc", args)
        self.assertNotIn("--config=ci-windows-cross", args)
        self.assertFalse(any(arg.startswith("--action_env=INCLUDE=") for arg in args))
        self.assertFalse(any(arg.startswith("--action_env=LIB=") for arg in args))
        self.assertIn(r"--action_env=PATH=C:\Windows\System32", args)

    def test_explicit_msvc_opt_in_remains_non_qualifying_and_coherent(self) -> None:
        completed, args = self.run_launcher(
            extra_env={"ALLOW_WINDOWS_MSVC_FALLBACK": "1"}
        )
        self.assertEqual(completed.returncode, 0, completed.stderr)
        self.assertIsNotNone(args)
        assert args is not None
        self.assertIn("--config=ci-windows", args)
        self.assertIn("--host_platform=//:local_windows_msvc", args)
        self.assertIn("--platforms=//:local_windows_msvc", args)
        self.assertNotIn("--platforms=//:windows_x86_64_gnullvm", args)
        self.assertTrue(any(arg.startswith("--action_env=INCLUDE=") for arg in args))
        self.assertTrue(any(arg.startswith("--action_env=LIB=") for arg in args))

    def test_authenticated_path_preserves_remote_gnullvm_contract(self) -> None:
        completed, args = self.run_launcher(
            extra_env={"BUILDBUDDY_API_KEY": "fixture-key"}
        )
        self.assertEqual(completed.returncode, 0, completed.stderr)
        self.assertIsNotNone(args)
        assert args is not None
        self.assertIn("--config=ci-windows-cross", args)
        self.assertIn("--host_platform=//:rbe", args)
        self.assertIn("--shell_executable=/bin/bash", args)
        self.assertNotIn("--host_platform=//:local_windows", args)
        self.assertNotIn("--platforms=//:local_windows_msvc", args)
        self.assertIn("--action_env=PATH=/usr/bin:/bin", args)

    def test_keyless_gnullvm_rejects_incompatible_explicit_target(self) -> None:
        completed, args = self.run_launcher(
            bazel_args=["build", "--platforms=//:windows_x86_64_msvc"]
        )
        self.assertEqual(completed.returncode, 1)
        self.assertIsNone(args)
        self.assertIn(
            "refusing incompatible override: --platforms=//:windows_x86_64_msvc",
            completed.stderr,
        )

    def test_keyless_gnullvm_accepts_exact_explicit_platforms(self) -> None:
        completed, args = self.run_launcher(
            bazel_args=[
                "build",
                "--host_platform=//:local_windows",
                "--platforms=//:windows_x86_64_gnullvm",
            ]
        )
        self.assertEqual(completed.returncode, 0, completed.stderr)
        self.assertIsNotNone(args)
        assert args is not None
        self.assertEqual(args.count("--host_platform=//:local_windows"), 1)
        self.assertEqual(args.count("--platforms=//:windows_x86_64_gnullvm"), 1)


if __name__ == "__main__":
    unittest.main()
