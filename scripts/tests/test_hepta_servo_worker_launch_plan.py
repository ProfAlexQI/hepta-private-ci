#!/usr/bin/env python3
from __future__ import annotations

import importlib.util
import tempfile
import unittest
from pathlib import Path
from types import ModuleType
from typing import Any

ROOT = Path(__file__).resolve().parents[2]
TOOL = ROOT / "scripts/hepta-servo-worker-launch-plan.py"


def load_tool() -> ModuleType:
    specification = importlib.util.spec_from_file_location(
        "hepta_servo_worker_launch_plan",
        TOOL,
    )
    if specification is None or specification.loader is None:
        raise RuntimeError("cannot load qualification launch plan compiler")
    module = importlib.util.module_from_spec(specification)
    specification.loader.exec_module(module)
    return module


def collect_keys(value: Any) -> set[str]:
    keys: set[str] = set()
    if isinstance(value, dict):
        for key, item in value.items():
            keys.add(key)
            keys.update(collect_keys(item))
    elif isinstance(value, list):
        for item in value:
            keys.update(collect_keys(item))
    return keys


class QualificationLaunchPlanTests(unittest.TestCase):
    def setUp(self) -> None:
        self.tool = load_tool()

    def test_contract_keeps_launch_and_authority_closed(self) -> None:
        result = self.tool.contract()
        self.assertEqual(result["supported_platforms"], sorted(self.tool.PLATFORMS))
        self.assertFalse(result["launch_authorized"])
        self.assertFalse(result["worker_executed"])
        self.assertFalse(result["real_plan_created"])
        self.assertFalse(any(result["authority"].values()))

    def test_self_test_covers_seven_fail_closed_cases(self) -> None:
        result = self.tool.self_test()
        self.assertEqual(result["status"], "PASS_LOCAL_FIXTURE_ONLY")
        self.assertEqual(result["test_count"], 7)
        self.assertFalse(result["launch_authorized"])

    def test_fixture_plan_has_no_secret_or_execution_keys(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            paths = self.tool.fixture(root)
            plan = self.tool.create_plan(
                root,
                paths["descriptor"],
                paths["policy"],
                "2026-08-28T01:00:00Z",
            )
            keys = collect_keys(plan)
            self.assertTrue(self.tool.FORBIDDEN_KEYS.isdisjoint(keys))
            self.assertEqual(plan["admission_state"], "BLOCKED_EVIDENCE_REQUIRED")
            self.assertFalse(plan["runtime"]["launch_authorized"])
            self.assertTrue(plan["required_evidence"]["graph_reverification_at_launch"])
            self.assertTrue(
                all(
                    value is None
                    for key, value in plan["required_evidence"].items()
                    if key != "graph_reverification_at_launch"
                )
            )

    def test_platform_policy_mapping_is_exact(self) -> None:
        for platform, expected in self.tool.PLATFORMS.items():
            policy = {
                "schema": self.tool.POLICY_SCHEMA,
                "schema_version": 1,
                "phase": "DEVELOPMENT",
                "platform": platform,
                "target_triple": expected["target_triple"],
                "transport": expected["transport"],
                "enforcement": {
                    "executable_binding": expected["executable_binding"],
                    "peer_identity": expected["peer_identity"],
                    "parent_death": expected["parent_death"],
                    "descendant_cleanup": expected["descendant_cleanup"],
                    "sandbox_required": True,
                    "egress_denial_required": True,
                    "network_listener": False,
                    "filesystem_endpoint": False,
                    "external_network": False,
                },
                "policy_digests": {
                    "sandbox_policy_sha256": "1" * 64,
                    "egress_policy_sha256": "2" * 64,
                    "profile_root_policy_sha256": "3" * 64,
                    "resource_policy_sha256": "4" * 64,
                },
                "limits": {
                    "startup_timeout_ms": 5_000,
                    "command_timeout_ms": 30_000,
                    "teardown_timeout_ms": 5_000,
                    "memory_limit_bytes": 512 * 1024 * 1024,
                    "cpu_time_limit_ms": 60_000,
                    "process_limit": 16,
                    "open_file_limit": 256,
                    "output_limit_bytes": 16 * 1024 * 1024,
                },
                "admission": {
                    "launch_authorized": False,
                    "evidence_complete": False,
                    "operator_acceptance_required": False,
                    "promotion_required": False,
                },
                "authority": self.tool.AUTHORITY,
            }
            with self.subTest(platform=platform):
                self.assertEqual(self.tool.validate_policy(policy), expected)

    def test_platform_policy_rejects_wrong_enforcement_strategy(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            paths = self.tool.fixture(root)
            policy_path = root / paths["policy"]
            policy, _ = self.tool.load_json(policy_path, "policy fixture")
            policy["enforcement"]["peer_identity"] = "trust_reported_pid"
            policy_path.write_bytes(self.tool.canonical(policy))
            with self.assertRaises(self.tool.LaunchPlanError):
                self.tool.create_plan(
                    root,
                    paths["descriptor"],
                    paths["policy"],
                    "2026-08-28T01:01:00Z",
                )

    def test_policy_limits_reject_bool_and_ordering_errors(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            paths = self.tool.fixture(root)
            policy_path = root / paths["policy"]
            policy, _ = self.tool.load_json(policy_path, "policy fixture")
            policy["limits"]["process_limit"] = True
            policy_path.write_bytes(self.tool.canonical(policy))
            with self.assertRaises(self.tool.LaunchPlanError):
                self.tool.create_plan(
                    root,
                    paths["descriptor"],
                    paths["policy"],
                    "2026-08-28T01:02:00Z",
                )
            policy["limits"]["process_limit"] = 16
            policy["limits"]["teardown_timeout_ms"] = 60_000
            policy["limits"]["command_timeout_ms"] = 30_000
            policy_path.write_bytes(self.tool.canonical(policy))
            with self.assertRaises(self.tool.LaunchPlanError):
                self.tool.create_plan(
                    root,
                    paths["descriptor"],
                    paths["policy"],
                    "2026-08-28T01:03:00Z",
                )

    def test_plan_id_rejects_tampering(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            paths = self.tool.fixture(root)
            plan = self.tool.create_plan(
                root,
                paths["descriptor"],
                paths["policy"],
                "2026-08-28T01:04:00Z",
            )
            plan["platform_policy"]["limits"]["process_limit"] = 17
            path = root / "tampered-plan.json"
            path.write_bytes(self.tool.canonical(plan))
            with self.assertRaises(self.tool.LaunchPlanError):
                self.tool.validate_plan(path, root)


if __name__ == "__main__":
    unittest.main()
