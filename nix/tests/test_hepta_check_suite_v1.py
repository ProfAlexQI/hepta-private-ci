import copy
import json
import os
import pathlib
import subprocess
import sys
import tempfile
import unittest


ROOT = pathlib.Path(__file__).resolve().parents[1]
SCRIPT = ROOT / "hepta-check-suite-v1.py"
NEXTEST_CONFIG = (ROOT / "hepta-nextest.toml").read_bytes()
PACKAGES = (
    "codex-hepta-contracts",
    "codex-hepta-evidence",
    "codex-hepta-governance",
    "codex-hepta-memory",
    "codex-hepta-memory-extension",
    "codex-hepta-mnl-replay-v1",
    "codex-hepta-mnl-trust-v1",
    "codex-hepta-native-gateway",
    "codex-hepta-nix-mnl-v1",
    "codex-hepta-paths",
    "codex-hepta-runtime",
)
WORKSPACE_MEMBERS = {
    "codex-hepta-contracts": "hepta-contracts",
    "codex-hepta-evidence": "hepta-evidence",
    "codex-hepta-governance": "ext/hepta-governance",
    "codex-hepta-memory": "hepta-memory",
    "codex-hepta-memory-extension": "ext/hepta-memory",
    "codex-hepta-mnl-replay-v1": "hepta-mnl-replay-v1",
    "codex-hepta-mnl-trust-v1": "hepta-mnl-trust-v1",
    "codex-hepta-native-gateway": "hepta-native-gateway",
    "codex-hepta-nix-mnl-v1": "hepta-nix-mnl-v1",
    "codex-hepta-paths": "hepta-paths",
    "codex-hepta-runtime": "hepta-runtime",
}
WORKSPACE_ROOT = "/build/codex-rs"
TARGET_DIRECTORY = "/build/hepta-nextest-target"


def compact(value):
    return json.dumps(value, sort_keys=True, separators=(",", ":")).encode("ascii")


def list_document():
    suites = {}
    for index, package in enumerate(PACKAGES):
        binary = package.replace("-", "_")
        suite_id = package
        suites[suite_id] = {
            "package-name": package,
            "binary-id": suite_id,
            "binary-name": binary,
            "package-id": "path+file://{}/{}#{}@0.0.0".format(
                WORKSPACE_ROOT, WORKSPACE_MEMBERS[package], package
            ),
            "kind": "lib",
            "binary-path": (
                "{}/debug/deps/{}-{:016x}".format(
                    TARGET_DIRECTORY, binary, index + 1
                )
            ),
            "build-platform": "target",
            "cwd": "{}/{}".format(WORKSPACE_ROOT, WORKSPACE_MEMBERS[package]),
            "status": "listed",
            "testcases": {
                "tests::passes": {
                    "kind": "test",
                    "ignored": False,
                    "filter-match": {"status": "matches"},
                }
            },
        }
    return {
        "rust-build-meta": {
            "target-directory": TARGET_DIRECTORY,
            "base-output-directories": ["debug"],
            "non-test-binaries": {},
            "build-script-out-dirs": {},
            "linked-paths": [],
            "platforms": {
                "host": {
                    "platform": {
                        "triple": "x86_64-unknown-linux-gnu",
                        "target-features": "unknown",
                    },
                    "libdir": {
                        "status": "available",
                        "path": (
                            "/nix/store/00000000000000000000000000000000-"
                            "rust-minimal-1.95.0/lib/rustlib/"
                            "x86_64-unknown-linux-gnu/lib"
                        ),
                    },
                },
                "targets": [],
            },
            "target-platforms": [
                {
                    "triple": "x86_64-unknown-linux-gnu",
                    "target-features": "unknown",
                }
            ],
            "target-platform": None,
        },
        "test-count": len(PACKAGES),
        "rust-suites": suites,
    }


def metadata_document():
    workspace_root = WORKSPACE_ROOT
    packages = []
    workspace_members = []
    for package in PACKAGES:
        member = WORKSPACE_MEMBERS[package]
        package_root = workspace_root + "/" + member
        package_id = "path+file://{}#{}@0.0.0".format(package_root, package)
        workspace_members.append(package_id)
        packages.append(
            {
                "authors": [],
                "categories": [],
                "default_run": None,
                "dependencies": [],
                "description": None,
                "documentation": None,
                "edition": "2024",
                "features": {},
                "homepage": None,
                "id": package_id,
                "keywords": [],
                "license": None,
                "license_file": None,
                "links": None,
                "manifest_path": package_root + "/Cargo.toml",
                "metadata": {},
                "name": package,
                "publish": None,
                "readme": None,
                "repository": None,
                "rust_version": None,
                "source": None,
                "targets": [
                    {
                        "crate_types": ["lib"],
                        "doc": True,
                        "doctest": False,
                        "edition": "2024",
                        "kind": ["lib"],
                        "name": package.replace("-", "_"),
                        "src_path": package_root + "/src/lib.rs",
                        "test": True,
                    }
                ],
                "version": "0.0.0",
            }
        )
    return {
        "build_directory": TARGET_DIRECTORY,
        "metadata": {},
        "packages": packages,
        "resolve": None,
        "target_directory": TARGET_DIRECTORY,
        "version": 1,
        "workspace_default_members": workspace_members,
        "workspace_members": workspace_members,
        "workspace_root": workspace_root,
    }


def event_documents(document):
    events = []
    for suite_id in sorted(document["rust-suites"]):
        suite = document["rust-suites"][suite_id]
        test_names = sorted(suite["testcases"])
        if not test_names:
            continue
        nextest = {
            "crate": suite["package-name"],
            "test_binary": suite["binary-name"],
            "kind": suite["kind"],
        }
        events.append(
            {
                "type": "suite",
                "event": "started",
                "test_count": len(test_names),
                "nextest": nextest,
            }
        )
        for test_name in test_names:
            name = "{}::{}${}".format(
                suite["package-name"], suite["binary-name"], test_name
            )
            events.append({"type": "test", "event": "started", "name": name})
            events.append(
                {"type": "test", "event": "ok", "name": name, "exec_time": 0.01}
            )
        events.append(
            {
                "type": "suite",
                "event": "ok",
                "passed": len(test_names),
                "failed": 0,
                "ignored": 0,
                "measured": 0,
                "filtered_out": 0,
                "exec_time": 0.01,
                "nextest": nextest,
            }
        )
    return events


def events_bytes(events):
    return b"".join(compact(event) + b"\n" for event in events)


class ParserTest(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.root = pathlib.Path(self.temp.name)
        self.document = list_document()
        self.metadata = metadata_document()
        self.events = event_documents(self.document)
        self.sequence = 0

    def tearDown(self):
        self.temp.cleanup()

    def write(self, name, raw, mode=None):
        path = self.root / name
        path.write_bytes(raw)
        if mode is not None:
            path.chmod(mode)
        return path

    def command(self, *arguments, success=True):
        result = subprocess.run(
            [sys.executable, str(SCRIPT), *map(str, arguments)],
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            check=False,
        )
        if success and result.returncode != 0:
            self.fail(result.stderr.decode("utf-8", "replace"))
        if not success and result.returncode == 0:
            self.fail("parser unexpectedly accepted invalid input")
        return result

    def discover(self, document=None, metadata=None, success=True):
        self.sequence += 1
        suffix = str(self.sequence)
        list_path = self.write("list-{}.json".format(suffix), compact(document or self.document))
        metadata_path = self.write(
            "metadata-{}.json".format(suffix), compact(metadata or self.metadata)
        )
        inventory = self.root / "inventory-{}.json".format(suffix)
        self.command(
            "discover",
            "--cargo-metadata-json",
            metadata_path,
            "--list-json",
            list_path,
            "--output",
            inventory,
            success=success,
        )
        return list_path, metadata_path, inventory

    def verify(
        self,
        document=None,
        metadata=None,
        events=None,
        events_raw=None,
        discovered_mutator=None,
        expected_mutator=None,
        config=NEXTEST_CONFIG,
        cargo_version_bytes=b"cargo 1.95.0 (f2d3ce0bd 2026-03-21)\n",
        rustc_version_bytes=b"rustc 1.95.0 (59807616e 2026-04-14)\n",
        runner_version_bytes=(
            b"cargo-nextest 0.9.124\n"
            b"release: 0.9.124\n"
            b"host: x86_64-unknown-linux-gnu\n"
        ),
        success=True,
    ):
        list_path, metadata_path, inventory = self.discover(document, metadata)
        suffix = str(self.sequence)
        expected_inventory = self.write(
            "expected-inventory-{}.json".format(suffix), inventory.read_bytes()
        )
        if discovered_mutator is not None:
            inventory.write_bytes(discovered_mutator(inventory.read_bytes()))
        if expected_mutator is not None:
            expected_inventory.write_bytes(
                expected_mutator(expected_inventory.read_bytes())
            )
        if events_raw is None:
            events_raw = events_bytes(self.events if events is None else events)
        events_path = self.write("events-{}.jsonl".format(suffix), events_raw)
        cargo_lock = self.write("Cargo-{}.lock".format(suffix), b"version = 4\n")
        nextest_config = self.write("nextest-{}.toml".format(suffix), config)
        cargo_version = self.write(
            "cargo-version-{}".format(suffix), cargo_version_bytes
        )
        rustc_version = self.write(
            "rustc-version-{}".format(suffix), rustc_version_bytes
        )
        runner_version = self.write(
            "runner-version-{}".format(suffix),
            runner_version_bytes,
        )
        output = self.root / "manifest-{}.json".format(suffix)
        result = self.command(
            "verify",
            "--cargo-metadata-json",
            metadata_path,
            "--list-json",
            list_path,
            "--events-jsonl",
            events_path,
            "--discovered-inventory",
            inventory,
            "--expected-inventory",
            expected_inventory,
            "--cargo-lock",
            cargo_lock,
            "--nextest-config",
            nextest_config,
            "--cargo-version-file",
            cargo_version,
            "--rustc-version-file",
            rustc_version,
            "--runner-version-file",
            runner_version,
            "--output",
            output,
            success=success,
        )
        return result, output

    def test_discover_and_verify_exact_positive(self):
        _, _, inventory = self.discover()
        raw = inventory.read_bytes()
        self.assertTrue(raw.endswith(b"\n"))
        self.assertEqual(raw, compact(json.loads(raw)) + b"\n")
        _, output = self.verify()
        manifest_raw = output.read_bytes()
        manifest = json.loads(manifest_raw)
        self.assertEqual(manifest_raw, compact(manifest))
        self.assertFalse(manifest["authorizes_pass"])
        self.assertTrue(
            manifest["candidate_observed_scoped_inventory_complete_and_passed"]
        )
        self.assertNotIn("all_passed", manifest)
        self.assertEqual(manifest["discovered_count"], len(PACKAGES))
        self.assertEqual(manifest["executed_count"], len(PACKAGES))
        self.assertEqual(manifest["passed_count"], len(PACKAGES))
        self.assertEqual(manifest["cargo_target_count"], len(PACKAGES))
        self.assertEqual(len(manifest["cargo_target_projection_sha256"]), 64)
        self.assertFalse(manifest["trust_boundaries"]["candidate_summary_authoritative"])
        self.assertFalse(
            manifest["trust_boundaries"]["candidate_owned_raw_material_authoritative"]
        )
        self.assertFalse(
            manifest["trust_boundaries"]["candidate_reported_recipe_authoritative"]
        )
        self.assertFalse(
            manifest["trust_boundaries"][
                "candidate_raw_material_retained_in_check_output"
            ]
        )
        self.assertTrue(
            manifest["trust_boundaries"][
                "trusted_supervisor_independent_raw_capture_required"
            ]
        )
        self.assertTrue(
            manifest["trust_boundaries"][
                "trusted_supervisor_exact_check_derivation_and_wrapper_binding_required"
            ]
        )
        self.assertNotIn("transcripts", manifest)
        self.assertNotIn("tool_version_files", manifest)
        self.assertTrue(
            manifest["trust_boundaries"]["trusted_supervisor_reparse_required"]
        )
        self.assertTrue(
            manifest["selection"][
                "nextest_list_suites_must_join_cargo_metadata_projection"
            ]
        )
        self.assertEqual(manifest["selection"]["packages"], list(PACKAGES))
        self.assertIn("--ignore-default-filter", manifest["recipe"]["list_argv"])
        self.assertEqual(
            manifest["recipe"]["execution_order"],
            [
                "capture_exact_tool_versions",
                "validate_exact_tool_versions",
                "cargo_metadata",
                "validate_exact_nextest_config",
                "parse_and_compare_cargo_target_projection",
                "nextest_list",
                "canonicalize_discovered_inventory",
                "compare_expected_inventory",
                "nextest_run",
                "verify_candidate_summary_and_discovered_inventory",
            ],
        )
        self.assertTrue(
            manifest["recipe"][
                "tool_versions_preflight_before_cargo_metadata_and_nextest_list"
            ]
        )
        self.assertTrue(
            manifest["recipe"][
                "candidate_verify_revalidates_tool_versions_after_run"
            ]
        )
        self.assertEqual(
            manifest["selection"]["benchmark_mode"],
            (
                "nextest_list_kind_lib_only;"
                "reject_any_additional_target_or_suite"
            ),
        )
        self.assertEqual(
            manifest["selection"]["cargo_target_selection_mode"],
            (
                "exact_cargo_metadata_single_lib_roster_joined_to_"
                "nextest_list_kind_lib_v1"
            ),
        )
        self.assertFalse(manifest["subject_product_executed_by_workspace_check"])
        self.assertTrue(manifest["source_workspace_check_only"])
        self.assertNotIn("subject_product", manifest)
        for field in (
            "failed_count",
            "ignored_count",
            "filtered_count",
            "skipped_count",
            "retried_count",
            "timed_out_count",
        ):
            self.assertEqual(manifest[field], 0)
        self.assertEqual(len(manifest["test_outcomes"]), len(PACKAGES))

    def test_unknown_list_field_is_rejected(self):
        document = copy.deepcopy(self.document)
        document["unknown"] = True
        self.discover(document=document, success=False)

    def test_ignored_and_filtered_inventory_are_rejected(self):
        for mutation in ("ignored", "filtered"):
            with self.subTest(mutation=mutation):
                document = copy.deepcopy(self.document)
                first = document["rust-suites"][sorted(document["rust-suites"])[0]]
                testcase = first["testcases"]["tests::passes"]
                if mutation == "ignored":
                    testcase["ignored"] = True
                else:
                    testcase["filter-match"]["status"] = "mismatch"
                self.discover(document=document, success=False)

    def test_benchmark_target_is_rejected(self):
        document = copy.deepcopy(self.document)
        first = document["rust-suites"][sorted(document["rust-suites"])[0]]
        first["kind"] = "bench"
        self.discover(document=document, success=False)

    def test_suite_and_test_count_bounds_are_rejected_before_enumeration(self):
        missing_suite = copy.deepcopy(self.document)
        missing_suite["rust-suites"].pop(sorted(missing_suite["rust-suites"])[0])
        missing_suite["test-count"] -= 1
        self.discover(document=missing_suite, success=False)

        excessive_tests = copy.deepcopy(self.document)
        excessive_tests["test-count"] = 65_537
        self.discover(document=excessive_tests, success=False)

    def test_nextest_build_meta_and_binary_path_drift_are_rejected(self):
        mutations = []

        base_output = copy.deepcopy(self.document)
        base_output["rust-build-meta"]["base-output-directories"] = ["release"]
        mutations.append(base_output)

        linked_path = copy.deepcopy(self.document)
        linked_path["rust-build-meta"]["linked-paths"] = ["../escape/out"]
        mutations.append(linked_path)

        non_string_linked_path = copy.deepcopy(self.document)
        non_string_linked_path["rust-build-meta"]["linked-paths"] = [1]
        mutations.append(non_string_linked_path)

        libdir = copy.deepcopy(self.document)
        libdir["rust-build-meta"]["platforms"]["host"]["libdir"]["path"] = (
            "/nix/store/00000000000000000000000000000000-rust-1.95.0/lib"
        )
        mutations.append(libdir)

        binary_path = copy.deepcopy(self.document)
        first = binary_path["rust-suites"][sorted(binary_path["rust-suites"])[0]]
        first["binary-path"] = TARGET_DIRECTORY + "/debug/deps/unbound"
        mutations.append(binary_path)

        for index, document in enumerate(mutations):
            with self.subTest(index=index):
                self.discover(document=document, success=False)

    def test_cargo_metadata_target_roster_drift_is_rejected(self):
        mutations = []

        doctest = copy.deepcopy(self.metadata)
        doctest["packages"][0]["targets"][0]["doctest"] = True
        mutations.append(doctest)

        feature = copy.deepcopy(self.metadata)
        feature["packages"][0]["features"] = {"unexpected": []}
        mutations.append(feature)

        extra_target = copy.deepcopy(self.metadata)
        extra_target["packages"][0]["targets"].append(
            copy.deepcopy(extra_target["packages"][0]["targets"][0])
        )
        mutations.append(extra_target)

        wrong_manifest = copy.deepcopy(self.metadata)
        wrong_manifest["packages"][0]["manifest_path"] = WORKSPACE_ROOT + "/wrong/Cargo.toml"
        mutations.append(wrong_manifest)

        for index, metadata in enumerate(mutations):
            with self.subTest(index=index):
                self.discover(metadata=metadata, success=False)

    def test_metadata_preflight_and_cross_input_roots_are_fail_closed(self):
        _, metadata_path, inventory = self.discover()
        self.command(
            "preflight-metadata",
            "--cargo-metadata-json",
            metadata_path,
            "--expected-inventory",
            inventory,
        )

        drift = copy.deepcopy(self.metadata)
        drift["packages"][0]["targets"][0]["doctest"] = True
        drift_path = self.write("metadata-preflight-drift.json", compact(drift))
        self.command(
            "preflight-metadata",
            "--cargo-metadata-json",
            drift_path,
            "--expected-inventory",
            inventory,
            success=False,
        )

        transplanted_root = copy.deepcopy(self.metadata)
        old_root = transplanted_root["workspace_root"]
        new_root = "/transplanted-workspace"
        transplanted_root["workspace_root"] = new_root
        for field in ("workspace_members", "workspace_default_members"):
            transplanted_root[field] = [
                value.replace("path+file://" + old_root, "path+file://" + new_root)
                for value in transplanted_root[field]
            ]
        for package in transplanted_root["packages"]:
            package["id"] = package["id"].replace(
                "path+file://" + old_root, "path+file://" + new_root
            )
            package["manifest_path"] = package["manifest_path"].replace(
                old_root, new_root, 1
            )
            package["targets"][0]["src_path"] = package["targets"][0][
                "src_path"
            ].replace(old_root, new_root, 1)
        self.discover(metadata=transplanted_root, success=False)

        transplanted_target = copy.deepcopy(self.metadata)
        transplanted_target["build_directory"] = "/other-target"
        transplanted_target["target_directory"] = "/other-target"
        self.discover(metadata=transplanted_target, success=False)

    def test_non_workspace_package_identity_is_rejected(self):
        document = copy.deepcopy(self.document)
        first = document["rust-suites"][sorted(document["rust-suites"])[0]]
        first["package-id"] = "registry+https://example.invalid#index@0.0.0"
        self.discover(document=document, success=False)

        document = copy.deepcopy(self.document)
        first = document["rust-suites"][sorted(document["rust-suites"])[0]]
        first["package-id"] = "path+file:///different/source#{}@0.0.0".format(
            first["package-name"]
        )
        self.discover(document=document, success=False)

    def test_cargo_1_95_workspace_package_id_is_normalized(self):
        document = copy.deepcopy(self.document)
        first = document["rust-suites"][sorted(document["rust-suites"])[0]]
        package = first["package-name"]
        first["package-id"] = "path+file://{}#{}@0.0.0".format(
            first["cwd"], package
        )
        _, _, inventory = self.discover(document)
        parsed = json.loads(inventory.read_bytes())
        normalized = "{} 0.0.0 (workspace-member:{})".format(
            package, WORKSPACE_MEMBERS[package]
        )
        self.assertIn(normalized, [item["package_id"] for item in parsed["suites"]])

    def test_wrong_member_path_and_mixed_workspace_roots_are_rejected(self):
        document = copy.deepcopy(self.document)
        first = document["rust-suites"][sorted(document["rust-suites"])[0]]
        first["cwd"] = WORKSPACE_ROOT + "/wrong-member"
        first["package-id"] = "path+file://{}/wrong-member#{}@0.0.0".format(
            WORKSPACE_ROOT,
            first["package-name"]
        )
        self.discover(document=document, success=False)

        document = copy.deepcopy(self.document)
        first = document["rust-suites"][sorted(document["rust-suites"])[0]]
        first["cwd"] = "/other-root/{}".format(
            WORKSPACE_MEMBERS[first["package-name"]]
        )
        first["package-id"] = "path+file://{}#{}@0.0.0".format(
            first["cwd"], first["package-name"]
        )
        self.discover(document=document, success=False)

    def test_undocumented_human_and_bare_package_ids_are_rejected(self):
        for package_id in (
            "{package} 0.0.0 (path+file://{cwd})",
            "path+file://{cwd}#0.0.0",
        ):
            with self.subTest(package_id=package_id):
                document = copy.deepcopy(self.document)
                first = document["rust-suites"][sorted(document["rust-suites"])[0]]
                first["package-id"] = package_id.format(
                    package=first["package-name"], cwd=first["cwd"]
                )
                self.discover(document=document, success=False)

    def test_retry_duplicate_and_missing_terminal_are_rejected(self):
        ok_index = next(
            index
            for index, event in enumerate(self.events)
            if event["type"] == "test" and event["event"] == "ok"
        )
        retry = copy.deepcopy(self.events)
        retry[ok_index]["name"] += "#2"
        self.verify(events=retry, success=False)

        duplicate = copy.deepcopy(self.events)
        duplicate.insert(ok_index + 1, copy.deepcopy(duplicate[ok_index]))
        self.verify(events=duplicate, success=False)

        missing = copy.deepcopy(self.events)
        del missing[ok_index]
        self.verify(events=missing, success=False)

    def test_noncanonical_expected_inventory_is_rejected(self):
        self.verify(expected_mutator=lambda raw: raw + b"\n", success=False)

        def boolean_schema_version(raw):
            document = json.loads(raw)
            document["schema_version"] = True
            return compact(document) + b"\n"

        self.verify(expected_mutator=boolean_schema_version, success=False)

        def unknown_field(raw):
            document = json.loads(raw)
            document["unknown"] = True
            return compact(document) + b"\n"

        self.verify(expected_mutator=unknown_field, success=False)

    def test_canonical_expected_inventory_identity_drift_is_rejected(self):
        def remove_one_test(raw):
            document = json.loads(raw)
            removed = document["tests"].pop()
            document["test_count"] -= 1
            for suite in document["suites"]:
                if all(
                    suite[field] == removed[field]
                    for field in (
                        "package",
                        "package_id",
                        "binary_id",
                        "target_kind",
                        "target_name",
                    )
                ):
                    suite["test_count"] -= 1
                    if suite["test_count"] == 0:
                        document["nonempty_suite_count"] -= 1
                    break
            return compact(document) + b"\n"

        self.verify(expected_mutator=remove_one_test, success=False)

        def alter_target_projection(raw):
            document = json.loads(raw)
            document["cargo_target_projection"][0]["doctest"] = True
            return compact(document) + b"\n"

        self.verify(expected_mutator=alter_target_projection, success=False)

    def test_duplicate_json_key_is_rejected(self):
        raw = compact(self.document)
        duplicate = b'{"test-count":0,' + raw[1:]
        list_path = self.write("duplicate-key-list.json", duplicate)
        metadata_path = self.write("duplicate-key-metadata.json", compact(self.metadata))
        self.command(
            "discover",
            "--cargo-metadata-json",
            metadata_path,
            "--list-json",
            list_path,
            "--output",
            self.root / "duplicate-key-inventory.json",
            success=False,
        )

    def test_symlink_special_and_oversize_inputs_are_rejected(self):
        metadata_path = self.write("identity-metadata.json", compact(self.metadata))
        real_list = self.write("identity-list-real.json", compact(self.document))
        symlink_list = self.root / "identity-list-symlink.json"
        os.symlink(real_list.name, symlink_list)
        self.command(
            "discover",
            "--cargo-metadata-json",
            metadata_path,
            "--list-json",
            symlink_list,
            "--output",
            self.root / "symlink-inventory.json",
            success=False,
        )

        fifo_list = self.root / "identity-list-fifo.json"
        os.mkfifo(fifo_list)
        self.command(
            "discover",
            "--cargo-metadata-json",
            metadata_path,
            "--list-json",
            fifo_list,
            "--output",
            self.root / "fifo-inventory.json",
            success=False,
        )

        self.verify(cargo_version_bytes=b"x" * 4097, success=False)

    def test_noncanonical_parent_workspace_path_is_rejected(self):
        document = copy.deepcopy(self.document)
        first = document["rust-suites"][sorted(document["rust-suites"])[0]]
        member = WORKSPACE_MEMBERS[first["package-name"]]
        first["cwd"] = "{}/../codex-rs/{}".format(WORKSPACE_ROOT, member)
        first["package-id"] = "path+file://{}#{}@0.0.0".format(
            first["cwd"], first["package-name"]
        )
        self.discover(document=document, success=False)

    def test_candidate_summary_is_stable_across_timing_only_drift(self):
        _, first_output = self.verify()
        varied = copy.deepcopy(self.events)
        for event in varied:
            if "exec_time" in event:
                event["exec_time"] = 123.456
        _, second_output = self.verify(events=varied)
        self.assertEqual(first_output.read_bytes(), second_output.read_bytes())

    def test_post_run_discovered_inventory_tamper_is_rejected(self):
        def tamper(raw):
            document = json.loads(raw)
            document["authorization_binding_observed"] = True
            return compact(document) + b"\n"

        self.verify(discovered_mutator=tamper, success=False)

    def test_unknown_event_field_is_rejected(self):
        events = copy.deepcopy(self.events)
        events[0]["unknown"] = 1
        self.verify(events=events, success=False)

    def test_failed_ignored_timeout_and_leak_events_are_rejected(self):
        ok_index = next(
            index
            for index, event in enumerate(self.events)
            if event["type"] == "test" and event["event"] == "ok"
        )
        for disposition in ("failed", "ignored", "timed_out", "leaked"):
            with self.subTest(disposition=disposition):
                events = copy.deepcopy(self.events)
                events[ok_index]["event"] = disposition
                self.verify(events=events, success=False)

    def test_nonfinite_event_duration_is_rejected(self):
        events = copy.deepcopy(self.events)
        ok_event = next(
            event
            for event in events
            if event["type"] == "test" and event["event"] == "ok"
        )
        ok_event["exec_time"] = float("nan")
        self.verify(events_raw=events_bytes(events), success=False)

    def test_nextest_config_semantic_drift_is_rejected(self):
        exact_config = self.write("preflight-nextest.toml", NEXTEST_CONFIG)
        self.command(
            "preflight-config", "--nextest-config", exact_config
        )
        drifted_config = self.write(
            "preflight-nextest-drift.toml",
            NEXTEST_CONFIG.replace(b"retries = 0", b"retries = 1"),
        )
        self.command(
            "preflight-config",
            "--nextest-config",
            drifted_config,
            success=False,
        )
        self.verify(
            config=NEXTEST_CONFIG.replace(b"retries = 0", b"retries = 1"),
            success=False,
        )
        self.verify(
            config=NEXTEST_CONFIG + b"default-filter = 'none()'\n",
            success=False,
        )

    def test_version_files_reject_non_lf_control_separators(self):
        result, _ = self.verify(
            cargo_version_bytes=b"cargo 1.95.0 (test)\vcounterfeit\n",
            success=False,
        )
        self.assertIn(b"not canonical LF-terminated text", result.stderr)
        result, _ = self.verify(
            runner_version_bytes=(
                b"cargo-nextest 0.9.124\vrelease: 0.9.124\n"
                b"host: x86_64-unknown-linux-gnu\n"
            ),
            success=False,
        )
        self.assertIn(b"not canonical LF-terminated text", result.stderr)

    def test_tool_version_preflight_is_exact(self):
        cargo_version = self.write(
            "preflight-cargo-version", b"cargo 1.95.0 (f2d3ce0bd 2026-03-21)\n"
        )
        rustc_version = self.write(
            "preflight-rustc-version", b"rustc 1.95.0 (59807616e 2026-04-14)\n"
        )
        runner_version = self.write(
            "preflight-runner-version",
            b"cargo-nextest 0.9.124\nrelease: 0.9.124\nhost: x86_64-unknown-linux-gnu\n",
        )
        arguments = (
            "preflight-tool-versions",
            "--cargo-version-file",
            cargo_version,
            "--rustc-version-file",
            rustc_version,
            "--runner-version-file",
            runner_version,
        )
        self.command(*arguments)
        cargo_version.write_bytes(b"cargo 1.95.1 (wrong)\n")
        self.command(*arguments, success=False)

    def test_test_events_must_remain_inside_their_suite_lifetime(self):
        before_start = copy.deepcopy(self.events)
        before_start[0], before_start[1] = before_start[1], before_start[0]
        self.verify(events=before_start, success=False)

        finish_before_test = copy.deepcopy(self.events)
        suite_finish = finish_before_test.pop(3)
        finish_before_test.insert(2, suite_finish)
        self.verify(events=finish_before_test, success=False)

    def test_zero_test_suite_is_frozen_and_does_not_emit_events(self):
        document = copy.deepcopy(self.document)
        suite = document["rust-suites"][sorted(document["rust-suites"])[0]]
        suite["testcases"] = {}
        document["test-count"] -= 1
        _, _, inventory_path = self.discover(document)
        inventory = json.loads(inventory_path.read_bytes())
        self.assertEqual(inventory["suite_count"], len(PACKAGES))
        self.assertEqual(inventory["nonempty_suite_count"], len(PACKAGES) - 1)
        self.assertIn(0, [item["test_count"] for item in inventory["suites"]])
        self.verify(document=document, events=event_documents(document))

    def test_cross_package_target_name_transplant_is_rejected(self):
        document = copy.deepcopy(self.document)
        for suite in document["rust-suites"].values():
            suite["binary-name"] = "shared_binary"
        self.discover(document=document, success=False)

    def test_event_stream_without_final_newline_is_rejected(self):
        raw = events_bytes(self.events).rstrip(b"\n")
        list_path, metadata_path, inventory = self.discover()
        suffix = str(self.sequence)
        events_path = self.write("events-no-newline-{}.jsonl".format(suffix), raw)
        cargo_lock = self.write("Cargo-no-newline.lock", b"version = 4\n")
        nextest_config = self.write("nextest-no-newline.toml", NEXTEST_CONFIG)
        cargo_version = self.write(
            "cargo-no-newline", b"cargo 1.95.0 (f2d3ce0bd 2026-03-21)\n"
        )
        rustc_version = self.write(
            "rustc-no-newline", b"rustc 1.95.0 (59807616e 2026-04-14)\n"
        )
        runner_version = self.write(
            "runner-no-newline",
            b"cargo-nextest 0.9.124\nrelease: 0.9.124\nhost: x86_64-unknown-linux-gnu\n",
        )
        self.command(
            "verify",
            "--cargo-metadata-json", metadata_path,
            "--list-json", list_path,
            "--events-jsonl", events_path,
            "--discovered-inventory", inventory,
            "--expected-inventory", inventory,
            "--cargo-lock", cargo_lock,
            "--nextest-config", nextest_config,
            "--cargo-version-file", cargo_version,
            "--rustc-version-file", rustc_version,
            "--runner-version-file", runner_version,
            "--output", self.root / "no-newline-manifest.json",
            success=False,
        )

    def test_non_lf_event_record_separators_are_rejected(self):
        raw = events_bytes(self.events)
        first_lf = raw.index(b"\n")
        for separator in (b"\v", b"\f", b"\x1c"):
            with self.subTest(separator=separator):
                malformed = raw[:first_lf] + separator + raw[first_lf + 1 :]
                self.verify(events_raw=malformed, success=False)


if __name__ == "__main__":
    unittest.main()
