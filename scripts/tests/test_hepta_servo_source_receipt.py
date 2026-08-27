from __future__ import annotations

import importlib.util
import json
import pathlib
import subprocess
import tempfile
import unittest

ROOT = pathlib.Path(__file__).resolve().parents[2]
SCRIPT = ROOT / "scripts" / "hepta-servo-source-receipt.py"
SPEC = importlib.util.spec_from_file_location("hepta_servo_source_receipt", SCRIPT)
if SPEC is None or SPEC.loader is None:
    raise RuntimeError("cannot load Servo source receipt module")
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)


class ServoSourceReceiptTests(unittest.TestCase):
    def setUp(self) -> None:
        self.temporary = tempfile.TemporaryDirectory()
        self.addCleanup(self.temporary.cleanup)
        self.checkout = pathlib.Path(self.temporary.name) / "servo"
        self.checkout.mkdir()
        self.git("init")
        self.git("config", "user.name", "Hepta Qualification")
        self.git("config", "user.email", "qualification@example.invalid")
        self.git("remote", "add", "origin", "git@github.com:servo/servo.git")
        (self.checkout / "LICENSE").write_text(
            "Mozilla Public License Version 2.0\nfixture only\n",
            encoding="utf-8",
        )
        (self.checkout / "README.md").write_text("fixture\n", encoding="utf-8")
        source = self.checkout / "components" / "webdriver_server"
        source.mkdir(parents=True)
        (source / "lib.rs").write_text("pub fn fixture() {}\n", encoding="utf-8")
        self.git("add", ".")
        self.git("commit", "-m", "fixture Servo source")
        self.commit = self.git("rev-parse", "HEAD").strip()
        self.tree = self.git("rev-parse", "HEAD^{tree}").strip()
        self.expectation = MODULE.SourceExpectation(
            repository=MODULE.EXPECTED_REPOSITORY,
            commit=self.commit,
            tree=self.tree,
        )

    def git(self, *arguments: str) -> str:
        return subprocess.run(
            ["git", "-C", str(self.checkout), *arguments],
            check=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
        ).stdout

    def write_receipt(self, value: object, name: str = "receipt.json") -> pathlib.Path:
        path = pathlib.Path(self.temporary.name) / name
        path.write_bytes(MODULE.canonical_bytes(value))
        return path

    def test_collect_source_binds_exact_clean_tree_and_license(self) -> None:
        source = MODULE.collect_source(self.checkout, self.expectation)
        self.assertEqual(source["repository"], MODULE.EXPECTED_REPOSITORY)
        self.assertEqual(source["commit"], self.commit)
        self.assertEqual(source["tree"], self.tree)
        self.assertTrue(source["clean_worktree"])
        self.assertFalse(source["embedded_commit_signature"])
        self.assertEqual(source["license"]["spdx_id"], "MPL-2.0")
        self.assertEqual(source["tree_manifest"]["entry_count"], 3)
        self.assertEqual(source["tree_manifest"]["blob_count"], 3)
        self.assertRegex(source["tree_manifest"]["sha256"], r"^[0-9a-f]{64}$")

    def test_receipt_is_canonical_self_bound_and_verifiable(self) -> None:
        receipt = MODULE.receipt_payload(
            self.checkout,
            self.expectation,
            "2026-08-27T00:00:00Z",
        )
        path = self.write_receipt(receipt)
        verified = MODULE.verify_receipt(self.checkout, path, self.expectation)
        self.assertEqual(verified, receipt)
        self.assertEqual(
            path.read_bytes(),
            json.dumps(receipt, sort_keys=True, separators=(",", ":")).encode(),
        )
        self.assertEqual(receipt["decision"], "SOURCE_PIN_VERIFIED_BUILD_NOT_QUALIFIED")
        self.assertTrue(all(value is False for value in receipt["authority"].values()))
        self.assertFalse(receipt["artifact"]["worker_artifact_built"])

    def test_dirty_or_wrong_head_fails_closed(self) -> None:
        (self.checkout / "untracked.txt").write_text("dirty", encoding="utf-8")
        with self.assertRaisesRegex(MODULE.ReceiptError, "dirty"):
            MODULE.collect_source(self.checkout, self.expectation)

        (self.checkout / "untracked.txt").unlink()
        wrong = MODULE.SourceExpectation(
            repository=MODULE.EXPECTED_REPOSITORY,
            commit="0" * 40,
            tree=self.tree,
        )
        with self.assertRaisesRegex(MODULE.ReceiptError, "pin mismatch"):
            MODULE.collect_source(self.checkout, wrong)

    def test_unexpected_origin_fails_closed(self) -> None:
        self.git("remote", "set-url", "origin", "https://github.com/example/not-servo.git")
        with self.assertRaisesRegex(MODULE.ReceiptError, "unexpected Servo origin"):
            MODULE.collect_source(self.checkout, self.expectation)

    def test_noncanonical_or_tampered_receipt_fails_closed(self) -> None:
        receipt = MODULE.receipt_payload(
            self.checkout,
            self.expectation,
            "2026-08-27T00:00:00Z",
        )
        pretty = pathlib.Path(self.temporary.name) / "pretty.json"
        pretty.write_text(json.dumps(receipt, indent=2), encoding="utf-8")
        with self.assertRaisesRegex(MODULE.ReceiptError, "not compact canonical"):
            MODULE.parse_canonical_receipt(pretty)

        tampered = json.loads(MODULE.canonical_bytes(receipt))
        tampered["authority"]["runtime_authority"] = True
        tampered_path = self.write_receipt(tampered, "tampered.json")
        with self.assertRaisesRegex(MODULE.ReceiptError, "positive"):
            MODULE.verify_receipt(self.checkout, tampered_path, self.expectation)

    def test_receipt_id_detects_payload_change(self) -> None:
        receipt = MODULE.receipt_payload(
            self.checkout,
            self.expectation,
            "2026-08-27T00:00:00Z",
        )
        receipt["captured_at_utc"] = "2026-08-27T00:00:01Z"
        path = self.write_receipt(receipt)
        with self.assertRaisesRegex(MODULE.ReceiptError, "ID does not match"):
            MODULE.verify_receipt(self.checkout, path, self.expectation)


if __name__ == "__main__":
    unittest.main()
