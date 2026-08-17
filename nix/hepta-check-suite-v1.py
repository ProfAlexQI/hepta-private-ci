#!/usr/bin/env python3
"""Fail-closed nextest inventory/result parser for the scoped Nix MNL check."""

import argparse
import hashlib
import json
import math
import os
import platform
import re
import stat
import struct
import sys


EXPECTED_INVENTORY_SCHEMA = "hepta_nix_mnl_expected_check_inventory_v1"
MANIFEST_SCHEMA = "hepta_nix_mnl_check_suite_result_v1"
SUITE_SCOPE = "hepta_nix_linux_exact_packages_v1"
TARGET_TRIPLE = "x86_64-unknown-linux-gnu"
INVENTORY_DOMAIN = b"hepta.mnl.check-suite.inventory.v1\0"
RUNNER_NAME = "cargo-nextest"
RUNNER_VERSION = "0.9.124"
CARGO_VERSION = "cargo 1.95.0 (f2d3ce0bd 2026-03-21)"
RUSTC_VERSION = "rustc 1.95.0 (59807616e 2026-04-14)"
MESSAGE_FORMAT = "libtest-json-plus"
MESSAGE_FORMAT_VERSION = "0.1"
MAX_LIST_BYTES = 64 * 1024 * 1024
MAX_EVENTS_BYTES = 256 * 1024 * 1024
MAX_INVENTORY_BYTES = 16 * 1024 * 1024
MAX_MANIFEST_BYTES = 64 * 1024 * 1024
MAX_TEST_COUNT = 65_536
MAX_CARGO_METADATA_BYTES = 64 * 1024 * 1024
MAX_VERSION_BYTES = 4096
MAX_CARGO_LOCK_BYTES = 16 * 1024 * 1024
MAX_NEXTEST_CONFIG_BYTES = 16 * 1024
EXPECTED_NEXTEST_CONFIG_BYTE_COUNT = 345
EXPECTED_NEXTEST_CONFIG_SHA256 = "ab8d3269123827f77074cd66c69c6815ee350827e5efc931d26a93cb8b0f0b41"

SUITE_PACKAGES = (
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


class CheckSuiteError(Exception):
    pass


def fail(message):
    raise CheckSuiteError(message)


def exact_keys(value, expected, label):
    if not isinstance(value, dict):
        fail("{} must be an object".format(label))
    actual = set(value.keys())
    expected_set = set(expected)
    if actual != expected_set:
        fail(
            "{} fields differ: missing={} unknown={}".format(
                label,
                sorted(expected_set - actual),
                sorted(actual - expected_set),
            )
        )


def require_string(value, label):
    if not isinstance(value, str) or not value:
        fail("{} must be a non-empty string".format(label))
    return value


def require_uint(value, label):
    if type(value) is not int or value < 0:
        fail("{} must be a non-negative integer".format(label))
    return value


def require_bool(value, label):
    if type(value) is not bool:
        fail("{} must be a boolean".format(label))
    return value


def require_canonical_absolute_path(value, label):
    value = require_string(value, label)
    if (
        not value.startswith("/")
        or value.endswith("/")
        or any(segment in ("", ".", "..") for segment in value[1:].split("/"))
        or any(ord(character) < 0x21 or ord(character) == 0x7F for character in value)
    ):
        fail("{} must be a canonical absolute path".format(label))
    return value


def require_canonical_relative_path(value, label):
    require_string(value, label)
    if (
        not value
        or value.startswith("/")
        or value.endswith("/")
        or any(segment in ("", ".", "..") for segment in value.split("/"))
        or any(ord(character) < 0x21 or ord(character) == 0x7F for character in value)
    ):
        fail("{} must be a canonical relative path".format(label))
    return value


def require_nonnegative_number(value, label):
    if type(value) not in (int, float) or not math.isfinite(value) or value < 0:
        fail("{} must be a finite non-negative number".format(label))


def reject_duplicate_pairs(pairs):
    result = {}
    for key, value in pairs:
        if key in result:
            fail("JSON object contains duplicate field {!r}".format(key))
        result[key] = value
    return result


def reject_nonfinite(value):
    fail("JSON contains non-finite number {!r}".format(value))


def parse_json_bytes(raw, label):
    try:
        return json.loads(
            raw.decode("utf-8"),
            object_pairs_hook=reject_duplicate_pairs,
            parse_constant=reject_nonfinite,
        )
    except (UnicodeDecodeError, json.JSONDecodeError) as error:
        fail("{} is not strict UTF-8 JSON: {}".format(label, error))


def canonical_json_bytes(value):
    return json.dumps(
        value,
        ensure_ascii=True,
        separators=(",", ":"),
        sort_keys=True,
        allow_nan=False,
    ).encode("ascii")


def canonical_json_line_bytes(value):
    return canonical_json_bytes(value) + b"\n"


def read_regular(path, maximum, label):
    if not hasattr(os, "O_NOFOLLOW"):
        fail("{} cannot be opened fail-closed without O_NOFOLLOW".format(label))
    try:
        path_before = os.lstat(path)
    except OSError as error:
        fail("{} cannot be inspected: {}".format(label, error))
    if not stat.S_ISREG(path_before.st_mode):
        fail("{} must be a regular file, not a symlink or special file".format(label))
    if path_before.st_size < 0 or path_before.st_size > maximum:
        fail("{} byte length is outside its bound".format(label))
    flags = os.O_RDONLY | os.O_NOFOLLOW | getattr(os, "O_CLOEXEC", 0)
    try:
        descriptor = os.open(path, flags)
    except OSError as error:
        fail("{} cannot be opened without following links: {}".format(label, error))
    try:
        opened = os.fstat(descriptor)
        identity_fields = ("st_dev", "st_ino", "st_mode", "st_uid", "st_gid", "st_size")
        if (
            not stat.S_ISREG(opened.st_mode)
            or any(
                getattr(path_before, field) != getattr(opened, field)
                for field in identity_fields
            )
        ):
            fail("{} changed between path inspection and open".format(label))
        chunks = []
        total = 0
        while True:
            chunk = os.read(descriptor, min(1024 * 1024, maximum + 1 - total))
            if not chunk:
                break
            chunks.append(chunk)
            total += len(chunk)
            if total > maximum:
                fail("{} exceeds its byte bound".format(label))
        opened_after = os.fstat(descriptor)
        if any(
            getattr(opened, field) != getattr(opened_after, field)
            for field in identity_fields
        ):
            fail("{} changed while being read".format(label))
    except OSError as error:
        fail("{} cannot be read: {}".format(label, error))
    finally:
        os.close(descriptor)
    try:
        path_after = os.lstat(path)
    except OSError as error:
        fail("{} path disappeared after read: {}".format(label, error))
    if any(
        getattr(opened, field) != getattr(path_after, field)
        for field in identity_fields
    ):
        fail("{} path identity changed while being read".format(label))
    raw = b"".join(chunks)
    if len(raw) != opened.st_size:
        fail("{} length differs from its retained file identity".format(label))
    return raw, opened


def sha256(raw):
    return hashlib.sha256(raw).hexdigest()


def length_prefix(value):
    raw = value.encode("utf-8")
    return struct.pack(">Q", len(raw)) + raw


def identity_key(identity):
    return (
        identity["package"],
        identity["package_id"],
        identity["binary_id"],
        identity["target_kind"],
        identity["target_name"],
        identity["test_name"],
    )


def suite_key(suite):
    return (
        suite["package"],
        suite["package_id"],
        suite["binary_id"],
        suite["target_kind"],
        suite["target_name"],
    )


def inventory_digest(suites, tests):
    ordered_suites = sorted(suites, key=suite_key)
    ordered_tests = sorted(tests, key=identity_key)
    frame = bytearray(INVENTORY_DOMAIN)
    frame.extend(struct.pack(">Q", len(ordered_suites)))
    for suite in ordered_suites:
        for field in (
            "package",
            "package_id",
            "binary_id",
            "target_kind",
            "target_name",
        ):
            frame.extend(length_prefix(suite[field]))
        frame.extend(struct.pack(">Q", suite["test_count"]))
    frame.extend(struct.pack(">Q", len(ordered_tests)))
    for identity in ordered_tests:
        for field in (
            "package",
            "package_id",
            "binary_id",
            "target_kind",
            "target_name",
            "test_name",
        ):
            frame.extend(length_prefix(identity[field]))
        frame.extend(b"\x00")
    return sha256(bytes(frame))


def normalized_package_id(package):
    return "{} 0.0.0 (workspace-member:{})".format(
        package, WORKSPACE_MEMBERS[package]
    )


def require_cargo_workspace_package_id(raw_package_id, package, exact_source):
    cargo_package_match = re.fullmatch(
        r"path\+file://(?P<source>/[^?#\x00-\x20]+)#{}@0\.0\.0".format(
            re.escape(package)
        ),
        require_string(raw_package_id, "Cargo package id"),
    )
    if (
        cargo_package_match is None
        or cargo_package_match.group("source") != exact_source
    ):
        fail("Cargo package id is not the exact workspace member identity")


def expected_target_projection_record(package):
    member = WORKSPACE_MEMBERS[package]
    return {
        "crate_types": ["lib"],
        "doc": True,
        "doctest": False,
        "edition": "2024",
        "features": [],
        "kind": ["lib"],
        "member_path": member,
        "package": package,
        "package_id": normalized_package_id(package),
        "source_relative_path": member + "/src/lib.rs",
        "target_name": package.replace("-", "_"),
        "test": True,
        "version": "0.0.0",
    }


def validate_target_projection(value, label):
    if not isinstance(value, list):
        fail("{} must be a list".format(label))
    expected = [expected_target_projection_record(package) for package in SUITE_PACKAGES]
    if value != expected:
        fail("{} differs from the exact scoped Cargo target roster".format(label))
    return [dict(record) for record in value]


def inspect_cargo_metadata(raw):
    document = parse_json_bytes(raw, "Cargo metadata")
    exact_keys(
        document,
        (
            "build_directory",
            "metadata",
            "packages",
            "resolve",
            "target_directory",
            "version",
            "workspace_default_members",
            "workspace_members",
            "workspace_root",
        ),
        "Cargo metadata",
    )
    if require_uint(document["version"], "Cargo metadata version") != 1:
        fail("Cargo metadata format version differs")
    if document["resolve"] is not None:
        fail("Cargo metadata --no-deps resolve must be null")
    if not isinstance(document["metadata"], dict):
        fail("Cargo workspace metadata must be an object")
    workspace_root = require_canonical_absolute_path(
        document["workspace_root"], "Cargo workspace root"
    )
    target_directory = require_canonical_absolute_path(
        document["target_directory"], "Cargo target directory"
    )
    build_directory = require_canonical_absolute_path(
        document["build_directory"], "Cargo build directory"
    )
    if target_directory != build_directory:
        fail("Cargo target and build directories differ")
    if (
        not workspace_root.endswith("/codex-rs")
        or target_directory
        != workspace_root.rsplit("/", 1)[0] + "/hepta-nextest-target"
    ):
        fail("Cargo workspace and target directories differ from the wrapper layout")
    for field in ("workspace_members", "workspace_default_members"):
        if not isinstance(document[field], list):
            fail("Cargo metadata {} must be a list".format(field))
        for package_id in document[field]:
            require_string(package_id, "Cargo metadata {} package id".format(field))
        if len(document[field]) != len(set(document[field])):
            fail("Cargo metadata {} contains duplicates".format(field))
    if not isinstance(document["packages"], list):
        fail("Cargo metadata packages must be a list")

    selected = {}
    for package_document in document["packages"]:
        if not isinstance(package_document, dict):
            fail("Cargo metadata package must be an object")
        package = package_document.get("name")
        if package not in SUITE_PACKAGES:
            continue
        exact_keys(
            package_document,
            (
                "authors",
                "categories",
                "default_run",
                "dependencies",
                "description",
                "documentation",
                "edition",
                "features",
                "homepage",
                "id",
                "keywords",
                "license",
                "license_file",
                "links",
                "manifest_path",
                "metadata",
                "name",
                "publish",
                "readme",
                "repository",
                "rust_version",
                "source",
                "targets",
                "version",
            ),
            "Cargo metadata package {}".format(package),
        )
        if package in selected:
            fail("Cargo metadata contains a duplicate scoped package")
        member = WORKSPACE_MEMBERS[package]
        package_root = workspace_root + "/" + member
        require_cargo_workspace_package_id(package_document["id"], package, package_root)
        if package_document["id"] not in document["workspace_members"]:
            fail("scoped Cargo package is not a workspace member")
        if (
            package_document["version"] != "0.0.0"
            or package_document["source"] is not None
            or package_document["features"] != {}
            or package_document["manifest_path"] != package_root + "/Cargo.toml"
        ):
            fail("scoped Cargo package identity, version, features, or manifest differs")
        targets = package_document["targets"]
        if not isinstance(targets, list) or len(targets) != 1:
            fail("scoped Cargo package must have exactly one target")
        target = targets[0]
        exact_keys(
            target,
            (
                "crate_types",
                "doc",
                "doctest",
                "edition",
                "kind",
                "name",
                "src_path",
                "test",
            ),
            "Cargo metadata target {}".format(package),
        )
        if (
            target["crate_types"] != ["lib"]
            or require_bool(target["doc"], "Cargo target doc") is not True
            or require_bool(target["doctest"], "Cargo target doctest") is not False
            or target["edition"] != "2024"
            or target["kind"] != ["lib"]
            or target["name"] != package.replace("-", "_")
            or target["src_path"] != package_root + "/src/lib.rs"
            or require_bool(target["test"], "Cargo target test") is not True
        ):
            fail("scoped Cargo target axes differ")
        selected[package] = expected_target_projection_record(package)

    if set(selected) != set(SUITE_PACKAGES):
        fail("Cargo metadata scoped package roster differs")
    projection = [selected[package] for package in SUITE_PACKAGES]
    validate_target_projection(projection, "observed Cargo target projection")
    return projection, workspace_root, target_directory


def logical_check_recipe():
    package_argv = []
    for package in SUITE_PACKAGES:
        package_argv.extend(("-p", package))
    common = [
        "cargo",
        "nextest",
        "--user-config-file",
        "none",
        "--config-file",
        "nix/hepta-nextest.toml",
        "--profile",
        "default",
    ]
    list_argv = common + [
        "list",
        "--ignore-default-filter",
        "--locked",
        "--offline",
    ] + package_argv + ["--list-type", "full", "--message-format", "json"]
    run_argv = common + [
        "run",
        "--ignore-default-filter",
        "--locked",
        "--offline",
    ] + package_argv + [
        "--no-fail-fast",
        "--no-tests",
        "fail",
        "--retries",
        "0",
        "--test-threads",
        "1",
        "--message-format",
        MESSAGE_FORMAT,
        "--message-format-version",
        MESSAGE_FORMAT_VERSION,
    ]
    return {
        "archive": "none",
        "binaries_metadata": "none",
        "build_jobs": 1,
        "build_profile": "test",
        "caller_manifest_allowed": False,
        "candidate_verify_revalidates_discovered_inventory_after_run": True,
        "candidate_verify_revalidates_tool_versions_after_run": True,
        "cargo_metadata_argv": [
            "cargo",
            "metadata",
            "--locked",
            "--offline",
            "--no-deps",
            "--format-version",
            "1",
            "--manifest-path",
            "Cargo.toml",
        ],
        "expected_inventory_compare_argv": [
            "cmp",
            "--silent",
            "<run_unique_tmpdir>/hepta-check-suite-v1/discovered-inventory.json",
            "nix/hepta-expected-check-inventory-v1.json",
        ],
        "cargo_metadata_projection_preflight_before_nextest_list": True,
        "execution_order": [
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
        "expected_inventory_compared_before_nextest_run": True,
        "wrapper_explicit_environment_overrides": [
            {"name": "CARGO_BUILD_JOBS", "value": "1"},
            {"name": "CARGO_INCREMENTAL", "value": "0"},
            {"name": "CARGO_NET_OFFLINE", "value": "true"},
            {
                "name": "CARGO_TARGET_DIR",
                "value": "<run_unique_tmpdir>/hepta-nextest-target",
            },
            {"name": "CARGO_TERM_COLOR", "value": "never"},
            {"name": "NO_COLOR", "value": "1"},
            {"name": "RUST_BACKTRACE", "value": "0"},
        ],
        "list_argv": list_argv,
        "manifest_path": "codex-rs/Cargo.toml",
        "list_and_run_share_target_dir": True,
        "nextest_list_launches_test_binaries_for_enumeration": True,
        "nextest_config_preflight_before_nextest_list": True,
        "nextest_reuse_build_option": "absent",
        "no_run": False,
        "no_tests_behavior": "fail",
        "release": False,
        "run_argv": run_argv,
        "run_environment": [
            {"name": "NEXTEST_EXPERIMENTAL_LIBTEST_JSON", "value": "1"}
        ],
        "target_dir_remap": "none",
        "tool_versions_preflight_before_cargo_metadata_and_nextest_list": True,
        "workspace_remap": "none",
        "workspace_root": "codex-rs",
    }


def validate_build_meta(value):
    exact_keys(
        value,
        (
            "target-directory",
            "base-output-directories",
            "non-test-binaries",
            "build-script-out-dirs",
            "linked-paths",
            "platforms",
            "target-platforms",
            "target-platform",
        ),
        "nextest rust-build-meta",
    )
    platforms = value["platforms"]
    exact_keys(platforms, ("host", "targets"), "nextest build platforms")
    if platforms["targets"] != []:
        fail("nextest build platforms must not contain a cross target")
    host = platforms["host"]
    exact_keys(host, ("platform", "libdir"), "nextest host platform")
    exact_keys(host["platform"], ("triple", "target-features"), "nextest host triple")
    if host["platform"]["triple"] != TARGET_TRIPLE:
        fail("nextest host target triple differs")
    if host["platform"]["target-features"] != "unknown":
        fail("nextest native host target features must be the exact unknown sentinel")
    exact_keys(host["libdir"], ("status", "path"), "nextest host libdir")
    if host["libdir"]["status"] != "available":
        fail("nextest host libdir is unavailable")
    require_canonical_absolute_path(
        host["libdir"]["path"], "nextest host libdir path"
    )
    if value["base-output-directories"] != ["debug"]:
        fail("nextest base output directories differ from the exact test profile")
    linked_paths = value["linked-paths"]
    if not isinstance(linked_paths, list):
        fail("nextest linked paths must be a list")
    for index, linked_path in enumerate(linked_paths):
        require_canonical_relative_path(
            linked_path, "nextest linked path {}".format(index)
        )
        if not re.fullmatch(
            r"debug/build/[a-z0-9_+.-]+-[0-9a-f]{16}/out", linked_path
        ):
            fail("nextest linked path is outside the exact Cargo test build shape")
    if linked_paths != sorted(linked_paths) or len(linked_paths) != len(
        set(linked_paths)
    ):
        fail("nextest linked paths must be strictly sorted and unique")
    target_platforms = value["target-platforms"]
    if target_platforms != [host["platform"]]:
        fail("nextest deprecated target-platforms must exactly mirror the native host")
    for field in ("non-test-binaries", "build-script-out-dirs"):
        if value[field] != {}:
            fail("nextest rust-build-meta {} must be the exact empty object".format(field))
    target_directory = require_canonical_absolute_path(
        value["target-directory"], "nextest target directory"
    )
    if not target_directory.endswith("/hepta-nextest-target"):
        fail("nextest target directory does not use the frozen run-unique basename")
    libdir = host["libdir"]["path"]
    if not re.fullmatch(
        r"/nix/store/[0-9abcdfghijklmnpqrsvwxyz]{32}-rust-minimal-1\.95\.0/lib/rustlib/"
        r"x86_64-unknown-linux-gnu/lib",
        libdir,
    ):
        fail("nextest host libdir does not match the exact Nix Rust 1.95.0 shape")
    if value["target-platform"] is not None:
        fail("nextest target-platform must be null for this native suite")
    return target_directory


def inspect_list(raw):
    document = parse_json_bytes(raw, "nextest list")
    exact_keys(document, ("rust-build-meta", "test-count", "rust-suites"), "nextest list")
    target_directory = validate_build_meta(document["rust-build-meta"])
    declared_count = require_uint(document["test-count"], "nextest test-count")
    if declared_count == 0 or declared_count > MAX_TEST_COUNT:
        fail("nextest test-count is outside the exact bounded suite")
    suites = document["rust-suites"]
    if not isinstance(suites, dict):
        fail("nextest rust-suites must be an object")
    if len(suites) != len(SUITE_PACKAGES):
        fail("nextest suite count differs from the exact package roster")

    tests = []
    suite_records = []
    suite_counts = {}
    seen_packages = set()
    workspace_root = None
    for suite_id in sorted(suites.keys()):
        require_string(suite_id, "nextest suite id")
        suite = suites[suite_id]
        exact_keys(
            suite,
            (
                "package-name",
                "binary-id",
                "binary-name",
                "package-id",
                "kind",
                "binary-path",
                "build-platform",
                "cwd",
                "status",
                "testcases",
            ),
            "nextest suite {}".format(suite_id),
        )
        package = require_string(suite["package-name"], "nextest package name")
        if package not in SUITE_PACKAGES:
            fail("nextest listed package outside the frozen suite: {}".format(package))
        seen_packages.add(package)
        raw_package_id = require_string(suite["package-id"], "nextest package id")
        binary_id = require_string(suite["binary-id"], "nextest binary id")
        if binary_id != suite_id:
            fail("nextest suite id differs from binary-id")
        if binary_id != package:
            fail("nextest library binary-id differs from its exact package id")
        target_name = require_string(suite["binary-name"], "nextest binary name")
        target_kind = require_string(suite["kind"], "nextest target kind")
        binary_path = require_canonical_absolute_path(
            suite["binary-path"], "nextest suite binary-path"
        )
        if not binary_path.startswith(target_directory + "/"):
            fail("nextest suite binary is outside the exact Cargo target directory")
        require_string(suite["build-platform"], "nextest suite build-platform")
        require_canonical_absolute_path(suite["cwd"], "nextest suite cwd")
        member = WORKSPACE_MEMBERS[package]
        if target_kind != "lib" or target_name != package.replace("-", "_"):
            fail("nextest suite does not join the exact Cargo target projection")
        if not re.fullmatch(
            re.escape(target_directory + "/debug/deps/" + target_name)
            + r"-[0-9a-f]{16}",
            binary_path,
        ):
            fail("nextest suite binary path differs from the exact Cargo test shape")
        expected_suffix = "/" + member
        if not suite["cwd"].startswith("/") or not suite["cwd"].endswith(
            expected_suffix
        ):
            fail("nextest suite cwd does not identify the frozen workspace member")
        inferred_root = suite["cwd"][: -len(expected_suffix)]
        if not inferred_root or inferred_root.endswith("/"):
            fail("nextest suite cwd has a non-canonical workspace root")
        if workspace_root is None:
            workspace_root = inferred_root
        elif workspace_root != inferred_root:
            fail("nextest suites do not share one workspace root")
        require_cargo_workspace_package_id(raw_package_id, package, suite["cwd"])
        package_id = normalized_package_id(package)
        if suite["build-platform"] != "target":
            fail("nextest suite build-platform must be the exact native target role")
        if suite["status"] != "listed":
            fail("nextest suite was not fully listed")
        testcases = suite["testcases"]
        if not isinstance(testcases, dict):
            fail("nextest testcases must be an object")
        event_suite_key = (package, target_kind, target_name)
        if event_suite_key in suite_counts:
            fail("duplicate nextest package/kind/binary suite")
        suite_counts[event_suite_key] = len(testcases)
        suite_records.append(
            {
                "binary_id": binary_id,
                "package": package,
                "package_id": package_id,
                "target_kind": target_kind,
                "target_name": target_name,
                "test_count": len(testcases),
            }
        )
        for test_name in sorted(testcases.keys()):
            require_string(test_name, "nextest test name")
            testcase = testcases[test_name]
            exact_keys(testcase, ("filter-match", "ignored", "kind"), "nextest testcase")
            if testcase["kind"] != "test":
                fail("frozen suite contains a non-test testcase kind")
            if require_bool(testcase["ignored"], "nextest testcase ignored"):
                fail("frozen suite contains ignored test {}".format(test_name))
            exact_keys(testcase["filter-match"], ("status",), "nextest filter match")
            if testcase["filter-match"]["status"] != "matches":
                fail("frozen suite contains a filtered test {}".format(test_name))
            tests.append(
                {
                    "binary_id": binary_id,
                    "ignored": False,
                    "package": package,
                    "package_id": package_id,
                    "target_kind": target_kind,
                    "target_name": target_name,
                    "test_name": test_name,
                }
            )

    if seen_packages != set(SUITE_PACKAGES):
        fail(
            "nextest package roster differs: missing={} unknown={}".format(
                sorted(set(SUITE_PACKAGES) - seen_packages),
                sorted(seen_packages - set(SUITE_PACKAGES)),
            )
        )
    suite_records.sort(key=suite_key)
    suite_keys = [suite_key(item) for item in suite_records]
    if len(suite_keys) != len(set(suite_keys)):
        fail("nextest inventory contains duplicate suite identity")
    tests.sort(key=identity_key)
    keys = [identity_key(item) for item in tests]
    if len(keys) != len(set(keys)):
        fail("nextest inventory contains duplicate test identity")
    if declared_count != len(tests):
        fail("nextest test-count differs from the enumerated inventory")
    if not tests:
        fail("nextest inventory is empty")
    if (
        not workspace_root.endswith("/codex-rs")
        or target_directory
        != workspace_root.rsplit("/", 1)[0] + "/hepta-nextest-target"
    ):
        fail("nextest workspace and target directories differ from the wrapper layout")
    return suite_records, tests, suite_counts, workspace_root, target_directory


def expected_inventory(suites, tests, target_projection):
    target_projection = validate_target_projection(
        target_projection, "expected Cargo target projection"
    )
    return {
        "authorization_binding_observed": False,
        "cargo_metadata_format_version": 1,
        "cargo_target_count": len(target_projection),
        "cargo_target_projection": target_projection,
        "nonempty_suite_count": sum(1 for suite in suites if suite["test_count"] > 0),
        "packages": list(SUITE_PACKAGES),
        "schema": EXPECTED_INVENTORY_SCHEMA,
        "schema_version": 1,
        "required_authorization_binding": "typed_final_freeze_and_closed_plan",
        "suite_count": len(suites),
        "suite_scope": SUITE_SCOPE,
        "suites": sorted(suites, key=suite_key),
        "target_triple": TARGET_TRIPLE,
        "test_count": len(tests),
        "tests": sorted(tests, key=identity_key),
    }


def inspect_expected_inventory(raw):
    document = parse_json_bytes(raw, "expected inventory")
    if raw != canonical_json_line_bytes(document):
        fail("expected inventory is not exact canonical LF-terminated JSON")
    exact_keys(
        document,
        (
            "authorization_binding_observed",
            "cargo_metadata_format_version",
            "cargo_target_count",
            "cargo_target_projection",
            "nonempty_suite_count",
            "packages",
            "schema",
            "schema_version",
            "required_authorization_binding",
            "suite_count",
            "suite_scope",
            "suites",
            "target_triple",
            "test_count",
            "tests",
        ),
        "expected inventory",
    )
    if (
        document["schema"] != EXPECTED_INVENTORY_SCHEMA
        or require_uint(document["schema_version"], "expected inventory schema_version") != 1
    ):
        fail("expected inventory schema differs")
    if document["suite_scope"] != SUITE_SCOPE or document["target_triple"] != TARGET_TRIPLE:
        fail("expected inventory scope or target differs")
    if require_bool(
        document["authorization_binding_observed"],
        "expected inventory authorization binding observed",
    ):
        fail("expected inventory must not claim an observed authorization binding")
    if document["required_authorization_binding"] != "typed_final_freeze_and_closed_plan":
        fail("expected inventory required authorization binding differs")
    if document["packages"] != list(SUITE_PACKAGES):
        fail("expected inventory package roster differs")
    if (
        require_uint(
            document["cargo_metadata_format_version"],
            "expected inventory Cargo metadata format version",
        )
        != 1
    ):
        fail("expected inventory Cargo metadata format version differs")
    target_projection = validate_target_projection(
        document["cargo_target_projection"], "expected Cargo target projection"
    )
    if require_uint(document["cargo_target_count"], "expected Cargo target count") != len(
        target_projection
    ):
        fail("expected Cargo target count differs")
    if not isinstance(document["suites"], list) or not document["suites"]:
        fail("expected inventory suites must be a non-empty list")
    suites = []
    for index, suite in enumerate(document["suites"]):
        exact_keys(
            suite,
            (
                "binary_id",
                "package",
                "package_id",
                "target_kind",
                "target_name",
                "test_count",
            ),
            "expected suite {}".format(index),
        )
        for field in (
            "binary_id",
            "package",
            "package_id",
            "target_kind",
            "target_name",
        ):
            require_string(suite[field], "expected suite {} {}".format(index, field))
        if suite["package"] not in SUITE_PACKAGES:
            fail("expected suite package is outside the frozen suite")
        require_uint(suite["test_count"], "expected suite test_count")
        suites.append(dict(suite))
    if suites != sorted(suites, key=suite_key):
        fail("expected inventory suites are not strictly sorted")
    suite_keys = [suite_key(item) for item in suites]
    if len(suite_keys) != len(set(suite_keys)):
        fail("expected inventory contains duplicate suite identity")
    if require_uint(document["suite_count"], "expected suite_count") != len(suites):
        fail("expected suite_count differs from suite inventory")
    if len(suites) != len(SUITE_PACKAGES):
        fail("expected suite count differs from the exact package roster")
    nonempty_suite_count = sum(1 for suite in suites if suite["test_count"] > 0)
    if (
        require_uint(document["nonempty_suite_count"], "expected nonempty_suite_count")
        != nonempty_suite_count
    ):
        fail("expected nonempty_suite_count differs from suite inventory")
    if not isinstance(document["tests"], list) or not document["tests"]:
        fail("expected inventory tests must be a non-empty list")
    tests = []
    for index, identity in enumerate(document["tests"]):
        exact_keys(
            identity,
            (
                "binary_id",
                "ignored",
                "package",
                "package_id",
                "target_kind",
                "target_name",
                "test_name",
            ),
            "expected test {}".format(index),
        )
        for field in (
            "binary_id",
            "package",
            "package_id",
            "target_kind",
            "target_name",
            "test_name",
        ):
            require_string(identity[field], "expected test {} {}".format(index, field))
        if require_bool(identity["ignored"], "expected test ignored"):
            fail("expected inventory contains ignored test")
        if identity["package"] not in SUITE_PACKAGES:
            fail("expected test package is outside the frozen suite")
        tests.append(dict(identity))
    if tests != sorted(tests, key=identity_key):
        fail("expected inventory tests are not strictly sorted")
    keys = [identity_key(item) for item in tests]
    if len(keys) != len(set(keys)):
        fail("expected inventory contains duplicate test identity")
    if require_uint(document["test_count"], "expected test_count") != len(tests):
        fail("expected test_count differs from test inventory")
    if len(tests) == 0 or len(tests) > MAX_TEST_COUNT:
        fail("expected test count is outside the exact bounded suite")
    tests_per_suite = {key: 0 for key in suite_keys}
    for identity in tests:
        key = (
            identity["package"],
            identity["package_id"],
            identity["binary_id"],
            identity["target_kind"],
            identity["target_name"],
        )
        if key not in tests_per_suite:
            fail("expected test has no matching expected suite")
        tests_per_suite[key] += 1
    for suite in suites:
        if tests_per_suite[suite_key(suite)] != suite["test_count"]:
            fail("expected suite test_count differs from its tests")
    return document, suites, tests, target_projection


def suite_from_nextest(value, label):
    exact_keys(value, ("crate", "test_binary", "kind"), label)
    return (
        require_string(value["crate"], label + " crate"),
        require_string(value["kind"], label + " kind"),
        require_string(value["test_binary"], label + " binary"),
    )


def inspect_events(raw, tests, suite_counts):
    if not raw.endswith(b"\n") or b"\r" in raw or b"\x00" in raw:
        fail("nextest event stream must be canonical LF-terminated JSONL")
    expected_names = {}
    expected_tests_by_suite = {key: set() for key in suite_counts}
    for identity in tests:
        emitted = "{}::{}${}".format(
            identity["package"], identity["target_name"], identity["test_name"]
        )
        if emitted in expected_names:
            fail("two inventory tests map to the same nextest event name")
        expected_names[emitted] = identity
        expected_tests_by_suite[
            (identity["package"], identity["target_kind"], identity["target_name"])
        ].add(emitted)

    suite_started = set()
    suite_finished = set()
    test_started = set()
    test_finished = set()
    lines = raw[:-1].split(b"\n")
    if not lines:
        fail("nextest event stream is empty")
    for line_number, line in enumerate(lines, 1):
        if not line:
            fail("nextest event stream contains a blank line")
        event = parse_json_bytes(line, "nextest event line {}".format(line_number))
        if not isinstance(event, dict):
            fail("nextest event line {} is not an object".format(line_number))
        event_type = event.get("type")
        disposition = event.get("event")
        if event_type == "suite" and disposition == "started":
            exact_keys(event, ("type", "event", "test_count", "nextest"), "suite started")
            suite_key = suite_from_nextest(event["nextest"], "suite started nextest")
            if suite_key not in suite_counts:
                fail("nextest started an unknown suite")
            if suite_key in suite_started:
                fail("nextest started a suite more than once")
            if require_uint(event["test_count"], "suite test_count") != suite_counts[suite_key]:
                fail("nextest suite test_count differs from inventory")
            suite_started.add(suite_key)
        elif event_type == "suite" and disposition == "ok":
            exact_keys(
                event,
                (
                    "type",
                    "event",
                    "passed",
                    "failed",
                    "ignored",
                    "measured",
                    "filtered_out",
                    "exec_time",
                    "nextest",
                ),
                "suite finished",
            )
            suite_key = suite_from_nextest(event["nextest"], "suite finished nextest")
            if suite_key not in suite_started or suite_key in suite_finished:
                fail("nextest finished a suite without exactly one start")
            require_nonnegative_number(event["exec_time"], "suite exec_time")
            if require_uint(event["passed"], "suite passed") != suite_counts[suite_key]:
                fail("nextest suite passed count differs from inventory")
            for field in ("failed", "ignored", "measured", "filtered_out"):
                if require_uint(event[field], "suite " + field) != 0:
                    fail("nextest suite {} is nonzero".format(field))
            if not expected_tests_by_suite[suite_key].issubset(test_finished):
                fail("nextest finished a suite before every listed test passed")
            suite_finished.add(suite_key)
        elif event_type == "test" and disposition == "started":
            exact_keys(event, ("type", "event", "name"), "test started")
            name = require_string(event["name"], "started test name")
            if name not in expected_names:
                fail("nextest started an unknown or retried test {}".format(name))
            identity = expected_names[name]
            suite_key = (
                identity["package"],
                identity["target_kind"],
                identity["target_name"],
            )
            if suite_key not in suite_started or suite_key in suite_finished:
                fail("nextest test start occurred outside its suite lifetime")
            if name in test_started:
                fail("nextest started a test more than once")
            test_started.add(name)
        elif event_type == "test" and disposition == "ok":
            exact_keys(event, ("type", "event", "name", "exec_time"), "test finished")
            name = require_string(event["name"], "finished test name")
            require_nonnegative_number(event["exec_time"], "test exec_time")
            if name not in expected_names:
                fail("nextest finished an unknown or retried test {}".format(name))
            identity = expected_names[name]
            suite_key = (
                identity["package"],
                identity["target_kind"],
                identity["target_name"],
            )
            if suite_key not in suite_started or suite_key in suite_finished:
                fail("nextest test finish occurred outside its suite lifetime")
            if name not in test_started or name in test_finished:
                fail("nextest finished a test without exactly one start")
            test_finished.add(name)
        elif event_type == "test" and disposition in ("ignored", "failed"):
            fail("nextest emitted forbidden test disposition {}".format(disposition))
        elif event_type == "suite" and disposition == "failed":
            fail("nextest emitted a failed suite")
        else:
            fail(
                "nextest emitted unknown event type/disposition {!r}/{!r}".format(
                    event_type, disposition
                )
            )

    expected_suite_set = {key for key, count in suite_counts.items() if count > 0}
    if suite_started != expected_suite_set or suite_finished != expected_suite_set:
        fail("nextest did not start and finish every non-empty suite exactly once")
    if test_started != set(expected_names) or test_finished != set(expected_names):
        fail("nextest did not start and pass every discovered test exactly once")
    return [
        dict(identity, result="PASS")
        for identity in sorted(tests, key=identity_key)
    ]


def read_version(path, label, maximum=MAX_VERSION_BYTES):
    raw, _ = read_regular(path, maximum, label)
    try:
        text = raw.decode("utf-8")
    except UnicodeDecodeError as error:
        fail("{} is not UTF-8: {}".format(label, error))
    if (
        not text.endswith("\n")
        or any(ord(character) < 0x20 and character != "\n" for character in text)
    ):
        fail("{} is not canonical LF-terminated text".format(label))
    return raw, text[:-1].split("\n")


def inspect_nextest_config(raw):
    digest = sha256(raw)
    if (
        len(raw) != EXPECTED_NEXTEST_CONFIG_BYTE_COUNT
        or digest != EXPECTED_NEXTEST_CONFIG_SHA256
    ):
        fail("nextest config bytes differ from the frozen one-shot profile")
    return digest


def write_new(path, raw, label):
    try:
        with open(path, "xb") as handle:
            handle.write(raw)
            handle.flush()
    except FileExistsError:
        fail("{} output already exists".format(label))
    except OSError as error:
        fail("{} output cannot be written: {}".format(label, error))


def discover(args):
    list_raw, _ = read_regular(args.list_json, MAX_LIST_BYTES, "nextest list")
    metadata_raw, _ = read_regular(
        args.cargo_metadata_json, MAX_CARGO_METADATA_BYTES, "Cargo metadata"
    )
    suites, tests, _, list_root, list_target_directory = inspect_list(list_raw)
    target_projection, metadata_root, metadata_target_directory = inspect_cargo_metadata(
        metadata_raw
    )
    if list_root != metadata_root or list_target_directory != metadata_target_directory:
        fail("Cargo metadata and nextest list roots or target directories differ")
    output = canonical_json_line_bytes(expected_inventory(suites, tests, target_projection))
    if len(output) > MAX_INVENTORY_BYTES:
        fail("canonical discovered inventory exceeds its retained byte bound")
    write_new(args.output, output, "discovery")


def preflight_metadata(args):
    metadata_raw, _ = read_regular(
        args.cargo_metadata_json, MAX_CARGO_METADATA_BYTES, "Cargo metadata"
    )
    expected_raw, _ = read_regular(
        args.expected_inventory, MAX_INVENTORY_BYTES, "expected inventory"
    )
    target_projection, _, _ = inspect_cargo_metadata(metadata_raw)
    _, _, _, expected_target_projection = inspect_expected_inventory(expected_raw)
    if target_projection != expected_target_projection:
        fail("Cargo metadata target projection differs from the frozen inventory")


def preflight_config(args):
    nextest_config_raw, _ = read_regular(
        args.nextest_config, MAX_NEXTEST_CONFIG_BYTES, "nextest config"
    )
    inspect_nextest_config(nextest_config_raw)


def inspect_tool_versions(cargo_version_file, rustc_version_file, runner_version_file):
    _, cargo_lines = read_version(cargo_version_file, "Cargo version")
    _, rustc_lines = read_version(rustc_version_file, "rustc version")
    _, runner_lines = read_version(runner_version_file, "runner version")
    if cargo_lines != [CARGO_VERSION]:
        fail("Cargo version differs from exact Rust 1.95.0 toolchain")
    if rustc_lines != [RUSTC_VERSION]:
        fail("rustc version differs from exact Rust 1.95.0 toolchain")
    if len(runner_lines) != 3:
        fail("cargo-nextest version output has an unexpected line count")
    if runner_lines[0] != "cargo-nextest " + RUNNER_VERSION:
        fail("cargo-nextest version differs")
    if runner_lines[1] != "release: " + RUNNER_VERSION:
        fail("cargo-nextest release differs")
    if runner_lines[2] != "host: " + TARGET_TRIPLE:
        fail("cargo-nextest host differs")
    return cargo_lines, rustc_lines, runner_lines


def preflight_tool_versions(args):
    inspect_tool_versions(
        args.cargo_version_file,
        args.rustc_version_file,
        args.runner_version_file,
    )


def verify(args):
    list_raw, _ = read_regular(args.list_json, MAX_LIST_BYTES, "nextest list")
    metadata_raw, _ = read_regular(
        args.cargo_metadata_json, MAX_CARGO_METADATA_BYTES, "Cargo metadata"
    )
    events_raw, _ = read_regular(args.events_jsonl, MAX_EVENTS_BYTES, "nextest events")
    expected_raw, _ = read_regular(
        args.expected_inventory, MAX_INVENTORY_BYTES, "expected inventory"
    )
    discovered_raw, _ = read_regular(
        args.discovered_inventory, MAX_INVENTORY_BYTES, "discovered inventory"
    )
    cargo_lock_raw, _ = read_regular(args.cargo_lock, MAX_CARGO_LOCK_BYTES, "Cargo.lock")
    nextest_config_raw, _ = read_regular(
        args.nextest_config, MAX_NEXTEST_CONFIG_BYTES, "nextest config"
    )
    nextest_config_sha256 = inspect_nextest_config(nextest_config_raw)

    (
        suites,
        tests,
        suite_counts,
        list_root,
        list_target_directory,
    ) = inspect_list(list_raw)
    target_projection, metadata_root, metadata_target_directory = inspect_cargo_metadata(
        metadata_raw
    )
    if list_root != metadata_root or list_target_directory != metadata_target_directory:
        fail("Cargo metadata and nextest list roots or target directories differ")
    (
        expected_document,
        expected_suites,
        expected_tests,
        expected_target_projection,
    ) = inspect_expected_inventory(expected_raw)
    discovered_document = expected_inventory(suites, tests, target_projection)
    recomputed_discovered_raw = canonical_json_line_bytes(discovered_document)
    if (
        expected_document != discovered_document
        or expected_suites != suites
        or expected_tests != tests
        or expected_target_projection != target_projection
        or discovered_raw != expected_raw
        or discovered_raw != recomputed_discovered_raw
    ):
        fail(
            "post-run discovered inventory, recomputed inventory, and frozen expected inventory differ"
        )
    outcomes = inspect_events(events_raw, tests, suite_counts)

    cargo_lines, rustc_lines, runner_lines = inspect_tool_versions(
        args.cargo_version_file,
        args.rustc_version_file,
        args.runner_version_file,
    )

    inventory_sha256 = inventory_digest(suites, tests)
    manifest = {
        "authorizes_pass": False,
        "cargo_lock_sha256": sha256(cargo_lock_raw),
        "cargo_target_count": len(target_projection),
        "cargo_target_projection_sha256": sha256(
            canonical_json_bytes(target_projection)
        ),
        "cargo_version": cargo_lines[0],
        "candidate_observed_scoped_inventory_complete_and_passed": True,
        "discovered_count": len(tests),
        "executed_count": len(tests),
        "failed_count": 0,
        "filtered_count": 0,
        "filters": "exact_package_allowlist_only",
        "ignored_count": 0,
        "message_format": MESSAGE_FORMAT,
        "message_format_version": MESSAGE_FORMAT_VERSION,
        "measured_count": 0,
        "nextest_config": {
            "byte_count": len(nextest_config_raw),
            "relative_path": "nix/hepta-nextest.toml",
            "sha256": nextest_config_sha256,
        },
        "package_count": len(SUITE_PACKAGES),
        "packages": list(SUITE_PACKAGES),
        "partition": "none",
        "passed_count": len(tests),
        "python_version": platform.python_version(),
        "retried_count": 0,
        "retries": 0,
        "runner_host": TARGET_TRIPLE,
        "runner_name": RUNNER_NAME,
        "runner_version": RUNNER_VERSION,
        "recipe": logical_check_recipe(),
        "rustc_version": rustc_lines[0],
        "schema": MANIFEST_SCHEMA,
        "schema_version": 1,
        "skipped_count": 0,
        "selection": {
            "all": False,
            "all_features": False,
            "benchmark_mode": (
                "nextest_list_kind_lib_only;"
                "reject_any_additional_target_or_suite"
            ),
            "build_target": "native_x86_64-unknown-linux-gnu",
            "cargo_target_selection_mode": (
                "exact_cargo_metadata_single_lib_roster_joined_to_"
                "nextest_list_kind_lib_v1"
            ),
            "doctests": "cargo_metadata_roster_doctest_false;nextest_no_doctests",
            "exclude": [],
            "features": [],
            "filter_expression": "none",
            "ignored_test_policy": "reject_inventory_and_do_not_run",
            "nextest_list_suites_must_join_cargo_metadata_projection": True,
            "no_default_features": False,
            "package_selection_mode": "explicit_exact_allowlist",
            "packages": list(SUITE_PACKAGES),
            "partition": "none",
            "target_selector_argv": [],
            "test_name_filters": [],
            "workspace": False,
        },
        "source_workspace_check_only": True,
        "subject_product_executed_by_workspace_check": False,
        "suite_count": len(suites),
        "nonempty_suite_count": sum(1 for suite in suites if suite["test_count"] > 0),
        "suite_inventory_sha256": inventory_sha256,
        "suite_scope": SUITE_SCOPE,
        "target_triple": TARGET_TRIPLE,
        "test_count": len(tests),
        "test_outcomes": outcomes,
        "timed_out_count": 0,
        "trust_boundaries": {
            "candidate_owned_raw_material_authoritative": False,
            "candidate_raw_material_retained_in_check_output": False,
            "candidate_reported_recipe_authoritative": False,
            "candidate_summary_authoritative": False,
            "trusted_supervisor_exact_check_derivation_and_wrapper_binding_required": True,
            "trusted_supervisor_independent_raw_capture_required": True,
            "trusted_supervisor_reparse_required": True,
        },
        "expected_inventory_sha256": sha256(expected_raw),
    }
    manifest_raw = canonical_json_bytes(manifest)
    if len(manifest_raw) > MAX_MANIFEST_BYTES:
        fail("canonical candidate summary exceeds its retained byte bound")
    write_new(args.output, manifest_raw, "check manifest")


def parser():
    root = argparse.ArgumentParser()
    commands = root.add_subparsers(dest="command", required=True)
    discover_parser = commands.add_parser("discover")
    discover_parser.add_argument("--cargo-metadata-json", required=True)
    discover_parser.add_argument("--list-json", required=True)
    discover_parser.add_argument("--output", required=True)

    preflight_parser = commands.add_parser("preflight-metadata")
    preflight_parser.add_argument("--cargo-metadata-json", required=True)
    preflight_parser.add_argument("--expected-inventory", required=True)

    preflight_config_parser = commands.add_parser("preflight-config")
    preflight_config_parser.add_argument("--nextest-config", required=True)

    preflight_tools_parser = commands.add_parser("preflight-tool-versions")
    preflight_tools_parser.add_argument("--cargo-version-file", required=True)
    preflight_tools_parser.add_argument("--rustc-version-file", required=True)
    preflight_tools_parser.add_argument("--runner-version-file", required=True)

    verify_parser = commands.add_parser("verify")
    for option in (
        "list-json",
        "cargo-metadata-json",
        "events-jsonl",
        "discovered-inventory",
        "expected-inventory",
        "cargo-lock",
        "nextest-config",
        "cargo-version-file",
        "rustc-version-file",
        "runner-version-file",
        "output",
    ):
        verify_parser.add_argument("--" + option, required=True)
    return root


def main():
    args = parser().parse_args()
    try:
        if args.command == "discover":
            discover(args)
        elif args.command == "preflight-metadata":
            preflight_metadata(args)
        elif args.command == "preflight-config":
            preflight_config(args)
        elif args.command == "preflight-tool-versions":
            preflight_tool_versions(args)
        else:
            verify(args)
    except CheckSuiteError as error:
        print("hepta-check-suite-v1: {}".format(error), file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
