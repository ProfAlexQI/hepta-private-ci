#!/usr/bin/env python3
from __future__ import annotations

import importlib.util
import tempfile
import unittest
from pathlib import Path
from types import ModuleType

ROOT = Path(__file__).resolve().parents[2]
TOOL = ROOT / "scripts/hepta-servo-worker-startup-descriptor.py"


def load_tool() -> ModuleType:
    specification = importlib.util.spec_from_file_location(
        "hepta_servo_worker_startup_descriptor",
        TOOL,
    )
    if specification is None or specification.loader is None:
        raise RuntimeError("cannot load startup descriptor compiler")
    module = importlib.util.module_from_spec(specification)
    specification.loader.exec_module(module)
    return module


class StartupDescriptorTests(unittest.TestCase):
    def setUp(self) -> None:
        self.tool = load_tool()

    def test_contract_keeps_launch_and_authority_closed(self) -> None:
        result = self.tool.contract()
        self.assertFalse(result["launch_authorized"])
        self.assertFalse(result["worker_executed"])
        self.assertFalse(result["real_descriptor_created"])
        self.assertFalse(result["secret_material_in_descriptor"])
        self.assertFalse(any(result["authority"].values()))

    def test_self_test_covers_seven_fail_closed_cases(self) -> None:
        result = self.tool.self_test()
        self.assertEqual(result["status"], "PASS_LOCAL_FIXTURE_ONLY")
        self.assertEqual(result["test_count"], 7)
        self.assertFalse(result["launch_authorized"])

    def test_descriptor_contains_no_secret_keys(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            paths = self.tool.fixture(root)
            descriptor = self.tool.create_descriptor(
                root,
                paths["manifest"],
                paths["verification"],
                paths["worker"],
                "ab" * 32,
                7,
                3,
                "unix_inherited_socketpair",
                "2026-08-28T00:10:00Z",
            )
            serialized = self.tool.canonical(descriptor).decode("utf-8")
            for forbidden in (
                "startup_capability",
                "host_nonce",
                "raw_capability",
                "credential",
                "secret",
            ):
                self.assertNotIn(forbidden, serialized)

    def test_only_private_transport_classes_are_accepted(self) -> None:
        for value in ("tcp_loopback", "websocket", "filesystem_socket", ""):
            with self.subTest(value=value):
                with self.assertRaises(self.tool.DescriptorError):
                    self.tool.build_descriptor(
                        {
                            "worker_relative": "bin/worker",
                            "worker_sha256": "0" * 64,
                            "worker_bytes": 1,
                            "manifest_relative": "manifest.json",
                            "manifest_sha256": "1" * 64,
                            "verification_relative": "verification.json",
                            "verification_sha256": "2" * 64,
                            "verification_receipt_id": "hepta-servo-worker-receipt-graph:v1:" + "3" * 64,
                        },
                        "ab" * 32,
                        1,
                        1,
                        value,
                        "2026-08-28T00:11:00Z",
                    )

    def test_zero_generation_or_owner_epoch_fails(self) -> None:
        packet = {
            "worker_relative": "bin/worker",
            "worker_sha256": "0" * 64,
            "worker_bytes": 1,
            "manifest_relative": "manifest.json",
            "manifest_sha256": "1" * 64,
            "verification_relative": "verification.json",
            "verification_sha256": "2" * 64,
            "verification_receipt_id": "hepta-servo-worker-receipt-graph:v1:" + "3" * 64,
        }
        for generation, owner_epoch in ((0, 1), (1, 0), (-1, 1), (1, -1)):
            with self.subTest(generation=generation, owner_epoch=owner_epoch):
                with self.assertRaises(self.tool.DescriptorError):
                    self.tool.build_descriptor(
                        packet,
                        "ab" * 32,
                        generation,
                        owner_epoch,
                        "unix_inherited_socketpair",
                        "2026-08-28T00:12:00Z",
                    )

    def test_descriptor_id_rejects_tampering(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            paths = self.tool.fixture(root)
            descriptor = self.tool.create_descriptor(
                root,
                paths["manifest"],
                paths["verification"],
                paths["worker"],
                "ab" * 32,
                7,
                3,
                "unix_inherited_socketpair",
                "2026-08-28T00:13:00Z",
            )
            descriptor["session_binding"]["generation"] = 8
            path = root / "tampered.json"
            path.write_bytes(self.tool.canonical(descriptor))
            with self.assertRaises(self.tool.DescriptorError):
                self.tool.validate_descriptor(path, root)


if __name__ == "__main__":
    unittest.main()
