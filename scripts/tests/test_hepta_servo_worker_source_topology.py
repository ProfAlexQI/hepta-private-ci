#!/usr/bin/env python3
from __future__ import annotations

import importlib.util
import io
import json
import os
import pathlib
import tarfile
import tempfile
import unittest
from types import ModuleType

ROOT = pathlib.Path(__file__).resolve().parents[2]
TOOL_PATH = ROOT / "scripts/hepta-servo-worker-source-topology.py"
TOPOLOGY_PATH = ROOT / "docs/hepta-vnext/browser/SERVO_WORKER_SOURCE_TOPOLOGY_V1.json"


def load_tool() -> ModuleType:
    specification = importlib.util.spec_from_file_location(
        "hepta_servo_worker_source_topology",
        TOOL_PATH,
    )
    if specification is None or specification.loader is None:
        raise RuntimeError("cannot load source-topology tool")
    module = importlib.util.module_from_spec(specification)
    specification.loader.exec_module(module)
    return module


class ServoWorkerSourceTopologyTests(unittest.TestCase):
    def setUp(self) -> None:
        self.tool = load_tool()
        self.temporary = tempfile.TemporaryDirectory()
        self.addCleanup(self.temporary.cleanup)
        self.root = pathlib.Path(self.temporary.name)
        self.prefix = f"servo-{self.tool.SERVO_COMMIT}/"
        self.contents = {
            "Cargo.toml": b"[workspace]\n",
            "components/servo/Cargo.toml": b'name = "servo"\n',
            "components/servo/examples/winit_minimal.rs": b"ServoBuilder::default()\n",
            "components/servo/lib.rs": b"pub use ServoBuilder;\n",
            "components/webdriver_server/lib.rs": b'pub fn start_server() {}\n0.0.0.0\n',
            "ports/servoshell/Cargo.toml": b"webdriver_server = true\n",
            "ports/servoshell/lib.rs": b"mod webdriver;\n",
            "ports/servoshell/running_app_state.rs": b"webdriver_server::start_server(\n",
            "ports/servoshell/webdriver.rs": b"WebDriverCommandMsg::ScriptCommand\n",
        }
        self.synthetic = self._synthetic_topology(self.contents)

    def _entry(self, path: str, content: bytes, role: str) -> dict[str, object]:
        return {
            "path": path,
            "git_blob_sha1": self.tool.git_blob_id(content),
            "max_bytes": max(1024, len(content) + 1024),
            "role": role,
            "required_anchors": [content.decode("utf-8").splitlines()[0]],
        }

    def _synthetic_topology(self, contents: dict[str, bytes]) -> dict[str, object]:
        selected_paths = {
            "Cargo.toml",
            "components/servo/Cargo.toml",
            "components/servo/examples/winit_minimal.rs",
            "components/servo/lib.rs",
        }
        selected = [
            self._entry(path, contents[path], "selected fixture")
            for path in sorted(selected_paths)
        ]
        reference = [
            self._entry(path, contents[path], "reference-only fixture")
            for path in sorted(set(contents) - selected_paths)
        ]
        return {
            "selected_files": selected,
            "reference_only_files": reference,
        }

    def _write_tar(
        self,
        contents: dict[str, bytes],
        *,
        symlink_path: str | None = None,
        hardlink_path: str | None = None,
    ) -> pathlib.Path:
        path = self.root / "source.tar"
        with tarfile.open(path, "w") as archive:
            root = tarfile.TarInfo(self.prefix.rstrip("/"))
            root.type = tarfile.DIRTYPE
            root.mode = 0o755
            root.mtime = 0
            archive.addfile(root)
            for relative, content in sorted(contents.items()):
                info = tarfile.TarInfo(self.prefix + relative)
                info.mode = 0o644
                info.mtime = 0
                if relative == symlink_path:
                    info.type = tarfile.SYMTYPE
                    info.linkname = "target"
                    info.size = 0
                    archive.addfile(info)
                elif relative == hardlink_path:
                    info.type = tarfile.LNKTYPE
                    info.linkname = self.prefix + "Cargo.toml"
                    info.size = 0
                    archive.addfile(info)
                else:
                    info.size = len(content)
                    archive.addfile(info, io.BytesIO(content))
        return path

    def test_canonical_topology_is_self_bound_and_closed(self) -> None:
        topology, raw = self.tool.load_topology(TOPOLOGY_PATH)
        self.assertEqual(raw, self.tool.canonical(topology))
        self.assertEqual(topology["authority"], self.tool.AUTHORITY)
        self.assertFalse(topology["decision"]["servoshell_build_root"])
        self.assertFalse(topology["decision"]["webdriver_server_dependency"])
        self.assertEqual(
            topology["decision"]["embedder_strategy"],
            "out_of_tree_hepta_worker_using_public_servo_embedding_api",
        )

    def test_exact_projection_scans_selected_and_reference_files(self) -> None:
        result = self.tool.scan_tar(
            self._write_tar(self.contents),
            self.prefix,
            self.synthetic,
        )
        self.assertEqual(result["selected_file_count"], 4)
        self.assertEqual(result["reference_only_file_count"], 5)
        self.assertEqual(len(result["files"]), 9)
        self.assertRegex(result["file_projection_sha256"], r"^[0-9a-f]{64}$")

    def test_missing_required_anchor_fails_closed(self) -> None:
        topology = self._synthetic_topology(self.contents)
        topology["selected_files"][0]["required_anchors"] = ["not-present"]
        with self.assertRaisesRegex(self.tool.TopologyError, "anchor is missing"):
            self.tool.scan_tar(self._write_tar(self.contents), self.prefix, topology)

    def test_blob_drift_fails_closed(self) -> None:
        changed = dict(self.contents)
        changed["components/servo/lib.rs"] += b"drift\n"
        with self.assertRaisesRegex(self.tool.TopologyError, "Git blob drifted"):
            self.tool.scan_tar(self._write_tar(changed), self.prefix, self.synthetic)

    def test_required_symlink_fails_closed(self) -> None:
        with self.assertRaisesRegex(self.tool.TopologyError, "must be a regular file"):
            self.tool.scan_tar(
                self._write_tar(
                    self.contents,
                    symlink_path="components/servo/lib.rs",
                ),
                self.prefix,
                self.synthetic,
            )

    def test_any_hardlink_fails_closed(self) -> None:
        with self.assertRaisesRegex(self.tool.TopologyError, "hard link"):
            self.tool.scan_tar(
                self._write_tar(
                    self.contents,
                    hardlink_path="ports/servoshell/webdriver.rs",
                ),
                self.prefix,
                self.synthetic,
            )

    def test_duplicate_json_keys_fail_closed(self) -> None:
        path = self.root / "duplicate.json"
        path.write_text('{"schema":"a","schema":"b"}', encoding="utf-8")
        os.chmod(path, 0o600)
        with self.assertRaisesRegex(self.tool.TopologyError, "duplicate JSON key"):
            self.tool.load_json(path, "duplicate", maximum_bytes=1024)

    def test_output_is_create_only_and_private(self) -> None:
        path = self.root / "receipt.json"
        self.tool.write_new(path, b"{}")
        self.assertEqual(path.read_bytes(), b"{}")
        self.assertEqual(path.stat().st_mode & 0o777, 0o600)
        with self.assertRaisesRegex(self.tool.TopologyError, "create-only"):
            self.tool.write_new(path, b"{}")

    def test_servoshell_conflicts_are_explicitly_frozen(self) -> None:
        topology, _raw = self.tool.load_topology(TOPOLOGY_PATH)
        anchors = {
            entry["path"]: set(entry["required_anchors"])
            for entry in topology["reference_only_files"]
        }
        self.assertIn(
            'servo = { workspace = true, features = ["background_hang_monitor", "bluetooth", "testbinding"], default-features = false }',
            anchors["ports/servoshell/Cargo.toml"],
        )
        self.assertIn(
            "webdriver_server = { workspace = true }",
            anchors["ports/servoshell/Cargo.toml"],
        )
        self.assertIn(
            'SocketAddrV4::new("0.0.0.0".parse().unwrap(), port);',
            anchors["components/webdriver_server/lib.rs"],
        )

    def test_contract_command_never_claims_source_or_runtime(self) -> None:
        self.assertEqual(
            self.tool.main(
                [
                    "contract",
                    "--topology",
                    str(TOPOLOGY_PATH),
                ]
            ),
            0,
        )


if __name__ == "__main__":
    unittest.main()
