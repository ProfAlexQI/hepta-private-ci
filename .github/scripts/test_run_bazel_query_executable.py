#!/usr/bin/env python3

from __future__ import annotations

import hashlib
import importlib.util
import os
import sys
import unittest
from contextlib import ExitStack
from pathlib import Path
from tempfile import TemporaryDirectory
from unittest.mock import patch


ROOT = Path(__file__).resolve().parents[2]
FILE = ROOT / "scripts" / "verify-windows-gnullvm-bazel-query-executable.py"
SPEC = importlib.util.spec_from_file_location("q041_query_executable", FILE)
if SPEC is None or SPEC.loader is None:
    raise RuntimeError(f"cannot load {FILE}")
subject = importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name] = subject
SPEC.loader.exec_module(subject)


class Result:
    def __init__(self, stdout: str = "", code: int = 0) -> None:
        self.stdout = stdout
        self.stderr = ""
        self.returncode = code


class QueryExecutableTest(unittest.TestCase):
    def fixture(self):
        temporary = TemporaryDirectory()
        self.addCleanup(temporary.cleanup)
        root = Path(temporary.name)
        workspace, home, tools = (
            root / "workspace",
            root / "home",
            root / "tools-cache",
        )
        for path in (workspace, home, tools):
            path.mkdir()
        bazelisk = tools / "bazel"
        bazelisk.write_bytes(b"bazelisk")
        bazelisk_sha = hashlib.sha256(bazelisk.read_bytes()).hexdigest()
        payload = b"bazel"
        bazel_sha = hashlib.sha256(payload).hexdigest()
        bazel = root / "downloads" / "sha256" / bazel_sha / "bin" / "bazel"
        bazel.parent.mkdir(parents=True)
        bazel.write_bytes(payload)
        env = {
            "PATH": str(tools),
            "HOME": str(home),
            "BAZELISK_GITHUB_TOKEN": "setup-only",
        }
        return workspace, env, bazelisk, bazel, bazelisk_sha, bazel_sha

    def patch_hashes(self, bazelisk_sha: str, bazel_sha: str) -> ExitStack:
        stack = ExitStack()
        stack.enter_context(
            patch.object(subject, "BAZELISK_LINUX_X86_64_SHA256", bazelisk_sha)
        )
        stack.enter_context(
            patch.object(subject, "BAZEL_LINUX_X86_64_SHA256", bazel_sha)
        )
        self.addCleanup(stack.close)
        return stack

    def test_resolver_and_direct_launch_contract(self) -> None:
        workspace, env, bazelisk, bazel, one, two = self.fixture()
        self.patch_hashes(one, two)
        child = f"{bazel.parent}{os.pathsep}/usr/bin"
        resolved, direct = subject.resolve_verified_linux_bazel(
            workspace,
            base_env=env,
            which=lambda *_args, **_kwargs: str(bazelisk),
            run=lambda *_args, **_kwargs: Result(f"PATH={child}\n"),
        )
        self.assertEqual(resolved, bazel.resolve())
        self.assertFalse(
            any(name.casefold().startswith("bazelisk_") for name in direct)
        )
        subject._validate_direct_bazel(resolved, direct)
        bazel.write_bytes(b"changed")
        with self.assertRaisesRegex(SystemExit, "changed before parser launch"):
            subject._validate_direct_bazel(resolved, direct)

    def test_bazelisk_and_child_identity_fail_closed(self) -> None:
        workspace, env, bazelisk, bazel, one, two = self.fixture()
        poisoned = dict(env)
        poisoned["BAZELISK_USER_AGENT"] = "unreviewed"
        with self.assertRaisesRegex(SystemExit, "unreviewed Bazelisk control"):
            subject.resolve_verified_linux_bazel(
                workspace,
                base_env=poisoned,
                which=lambda *_args, **_kwargs: str(bazelisk),
            )
        with self.assertRaisesRegex(SystemExit, "Bazelisk executable SHA-256"):
            subject.resolve_verified_linux_bazel(
                workspace,
                base_env=env,
                which=lambda *_args, **_kwargs: str(bazelisk),
            )
        self.patch_hashes(one, two)
        for stdout in (
            "HOME=/home/runner\n",
            "PATH=/one\nPath=/two\n",
            "PATH=\n",
            "PATH=relative\n",
            "BAZELISK_GITHUB_TOKEN=leak\nPATH=/one\n",
        ):
            with self.subTest(stdout=stdout), self.assertRaises(SystemExit):
                subject.resolve_verified_linux_bazel(
                    workspace,
                    base_env=env,
                    which=lambda *_args, **_kwargs: str(bazelisk),
                    run=lambda *_args, stdout=stdout, **_kwargs: Result(stdout),
                )
        escaped = workspace.parent / "escaped" / "bazel"
        escaped.parent.mkdir()
        escaped.write_bytes(bazel.read_bytes())
        with self.assertRaisesRegex(SystemExit, "content-addressed layout"):
            subject.resolve_verified_linux_bazel(
                workspace,
                base_env=env,
                which=lambda *_args, **_kwargs: str(bazelisk),
                run=lambda *_args, **_kwargs: Result(
                    f"PATH={escaped.parent}{os.pathsep}/usr/bin\n"
                ),
            )

    def test_bazelisk_mutation_and_direct_execution(self) -> None:
        workspace, env, bazelisk, bazel, one, two = self.fixture()
        self.patch_hashes(one, two)

        def mutate(*_args, **_kwargs):
            bazelisk.write_bytes(b"changed")
            return Result(f"PATH={bazel.parent}{os.pathsep}/usr/bin\n")

        with self.assertRaisesRegex(SystemExit, "changed during child resolution"):
            subject.resolve_verified_linux_bazel(
                workspace,
                base_env=env,
                which=lambda *_args, **_kwargs: str(bazelisk),
                run=mutate,
            )
        bazelisk.write_bytes(b"bazelisk")
        calls = []

        def run(command, **kwargs):
            calls.append((list(command), dict(kwargs["env"])))
            if command[-1] == "--print_env":
                return Result(f"PATH={bazel.parent}{os.pathsep}/usr/bin\n")
            self.assertEqual(Path(command[0]), bazel.resolve())
            self.assertEqual(command[-3:], ["--output=label", "--", "//:probe"])
            return Result("//:probe\n")

        subject.execute_parser_smoke(
            base_env=env,
            which=lambda *_args, **_kwargs: str(bazelisk),
            run=run,
        )
        self.assertEqual(len(calls), 2)
        self.assertFalse(
            any(
                name.casefold().startswith("bazelisk_")
                for name in calls[1][1]
            )
        )


if __name__ == "__main__":
    unittest.main()
