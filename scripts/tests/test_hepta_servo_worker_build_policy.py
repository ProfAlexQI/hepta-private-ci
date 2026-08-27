#!/usr/bin/env python3
from __future__ import annotations

import importlib.util
import json
import pathlib
import tempfile
import unittest
from types import ModuleType

ROOT = pathlib.Path(__file__).resolve().parents[2]
POLICY_PATH = ROOT / "scripts/hepta-servo-worker-build-inputs.py"


def load_policy() -> ModuleType:
    specification = importlib.util.spec_from_file_location(
        "hepta_servo_worker_build_policy",
        POLICY_PATH,
    )
    if specification is None or specification.loader is None:
        raise RuntimeError("cannot load worker build policy")
    module = importlib.util.module_from_spec(specification)
    specification.loader.exec_module(module)
    return module


class FakeEngine:
    @staticmethod
    def load_json(path: pathlib.Path, _label: str) -> tuple[dict[str, object], bytes]:
        raw = path.read_bytes()
        value = json.loads(raw)
        if raw != json.dumps(value, sort_keys=True, separators=(",", ":")).encode():
            raise RuntimeError("noncanonical fixture")
        return value, raw


class WorkerBuildPolicyTests(unittest.TestCase):
    def setUp(self) -> None:
        self.policy = load_policy()
        self.directory = tempfile.TemporaryDirectory()
        self.root = pathlib.Path(self.directory.name)
        self.command = self.root / "command.json"
        self.environment = self.root / "environment.json"
        self.write_command(
            [
                "cargo",
                "build",
                "--locked",
                "--offline",
                "--target",
                "x86_64-unknown-linux-gnu",
            ]
        )
        self.write_environment(
            {
                "CARGO_NET_OFFLINE": "true",
                "LANG": "C.UTF-8",
                "SOURCE_DATE_EPOCH": "1787836800",
                "TZ": "UTC",
            }
        )

    def tearDown(self) -> None:
        self.directory.cleanup()

    def write(self, path: pathlib.Path, value: object) -> None:
        path.write_bytes(
            json.dumps(value, sort_keys=True, separators=(",", ":")).encode()
        )

    def write_command(self, argv: list[str]) -> None:
        self.write(
            self.command,
            {
                "argv": argv,
                "network_access_during_build": False,
                "schema": "hepta.servo.worker_build_command.v1",
                "working_directory": "servo-source",
            },
        )

    def write_environment(self, variables: dict[str, str]) -> None:
        self.write(
            self.environment,
            {
                "network_access_during_build": False,
                "schema": "hepta.servo.worker_build_environment.v1",
                "variables": dict(sorted(variables.items())),
            },
        )

    def invocation(self, *extra: str) -> list[str]:
        return [
            "create",
            "--build-command",
            str(self.command),
            "--environment",
            str(self.environment),
            "--feature",
            "local-fixture",
            *extra,
        ]

    def assert_rejected(self, *extra: str) -> None:
        with self.assertRaises(RuntimeError):
            self.policy.validate_invocation(self.invocation(*extra), FakeEngine())

    def test_valid_locked_offline_cargo_build_is_accepted(self) -> None:
        self.policy.validate_invocation(self.invocation(), FakeEngine())

    def test_non_cargo_executable_is_rejected(self) -> None:
        self.write_command(["sh", "-c", "cargo build --locked --offline"])
        self.assert_rejected()

    def test_registry_or_acquisition_operation_is_rejected(self) -> None:
        self.write_command(["cargo", "publish", "--locked", "--offline"])
        self.assert_rejected()

    def test_missing_locked_is_rejected(self) -> None:
        self.write_command(["cargo", "build", "--offline"])
        self.assert_rejected()

    def test_missing_offline_is_rejected(self) -> None:
        self.write_command(["cargo", "build", "--locked"])
        self.assert_rejected()

    def test_duplicate_feature_is_rejected(self) -> None:
        self.assert_rejected("--feature", "local-fixture")

    def test_newline_in_command_is_rejected(self) -> None:
        self.write_command(["cargo", "build", "--locked", "--offline", "bad\narg"])
        self.assert_rejected()

    def test_secret_or_multiline_environment_is_rejected(self) -> None:
        self.write_environment(
            {
                "CARGO_NET_OFFLINE": "true",
                "GITHUB_TOKEN": "secret",
                "LANG": "C.UTF-8\nunsafe",
            }
        )
        self.assert_rejected()


if __name__ == "__main__":
    unittest.main()
