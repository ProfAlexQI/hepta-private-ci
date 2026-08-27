#!/usr/bin/env python3
"""Fail-closed WEB-C1 Servo source-bundle tooling.

This tool compares two independently generated source receipts, creates and
reproduces an exact-tree source archive, seals the MPL-2.0/patch packet, and
recomputes the bundle. It never builds or executes Servo and never grants
runtime, network, product, operator, promotion, or release authority.
"""
from __future__ import annotations

import argparse
import datetime as dt
import gzip
import hashlib
import json
import os
import pathlib
import re
import shutil
import stat
import subprocess
import sys
import tarfile
import tempfile
from typing import Any

REPOSITORY = "https://github.com/servo/servo"
COMMIT = "0a48e298482659817eb50097df23841f2b8e3044"
TREE = "b04d2f75b3217374d079d579c270177b57fa1389"
LICENSE = "MPL-2.0"
SOURCE_SCHEMA = "hepta.servo.source_receipt.v1"
COMPARE_SCHEMA = "hepta.servo.source_comparison_receipt.v1"
ARCHIVE_SCHEMA = "hepta.servo.source_archive_receipt.v1"
ARCHIVE_COMPARE_SCHEMA = "hepta.servo.source_archive_comparison.v1"
LICENSE_SCHEMA = "hepta.servo.license_packet.v1"
PATCH_SCHEMA = "hepta.servo.patch_inventory.v1"
DISTRIBUTION_SCHEMA = "hepta.servo.source_distribution_receipt.v1"
BUNDLE_SCHEMA = "hepta.servo.source_bundle_verification.v1"

SOURCE_ID_DOMAIN = b"hepta.servo.source-receipt.v1"
SOURCE_FACTS_DOMAIN = b"hepta.servo.source-facts.v1"
COMPARE_ID_DOMAIN = b"hepta.servo.source-comparison-receipt.v1"
ARCHIVE_ID_DOMAIN = b"hepta.servo.source-archive-receipt.v1"
ARCHIVE_COMPARE_ID_DOMAIN = b"hepta.servo.source-archive-comparison.v1"
DISTRIBUTION_ID_DOMAIN = b"hepta.servo.source-distribution-receipt.v1"
BUNDLE_ID_DOMAIN = b"hepta.servo.source-bundle-verification.v1"
ARCHIVE_INVENTORY_DOMAIN = b"hepta.servo.source-archive-inventory.v1"

SHA40 = re.compile(r"^[0-9a-f]{40}$")
SHA64 = re.compile(r"^[0-9a-f]{64}$")
STABLE_ID = re.compile(r"^[A-Za-z0-9][A-Za-z0-9._:+@=-]{0,127}$")
UTC = re.compile(r"^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z$")

SOURCE_AUTHORITY = {
    "machine_authority": False,
    "runtime_authority": False,
    "production_caller": False,
    "production_writer": False,
    "effect_authority": False,
    "external_effect": False,
    "operator_acceptance": False,
    "promotion": False,
    "release_qualified": False,
}
AUTHORITY = {
    **SOURCE_AUTHORITY,
    "external_network_allowed": False,
    "credential_export_allowed": False,
}
SOURCE_ARTIFACT = {
    "source_archive_created": False,
    "source_archive_sha256": None,
    "worker_artifact_built": False,
    "worker_artifact_sha256": None,
    "sbom_created": False,
}


class Error(RuntimeError):
    pass


def fail(message: str) -> None:
    raise Error(message)


def duplicate_pairs(pairs: list[tuple[str, Any]]) -> dict[str, Any]:
    out: dict[str, Any] = {}
    for key, value in pairs:
        if key in out:
            fail(f"duplicate JSON key {key!r}")
        out[key] = value
    return out


def canonical(value: object) -> bytes:
    return json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=False).encode()


def load(path: pathlib.Path, label: str, canonical_required: bool = True) -> tuple[dict[str, Any], bytes]:
    try:
        raw = path.read_bytes()
        value = json.loads(raw, object_pairs_hook=duplicate_pairs)
    except (OSError, UnicodeError, json.JSONDecodeError) as error:
        fail(f"cannot decode {label}: {error}")
    if not isinstance(value, dict):
        fail(f"{label} must be an object")
    if canonical_required and raw != canonical(value):
        fail(f"{label} is not compact canonical JSON")
    return value, raw


def framed(domain: bytes, *fields: bytes) -> str:
    digest = hashlib.sha256()
    digest.update(len(domain).to_bytes(8, "big")); digest.update(domain)
    for field in fields:
        digest.update(len(field).to_bytes(8, "big")); digest.update(field)
    return digest.hexdigest()


def sha_file(path: pathlib.Path) -> tuple[str, int]:
    digest = hashlib.sha256(); size = 0
    with path.open("rb") as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(block); size += len(block)
    return digest.hexdigest(), size


def write_new(path: pathlib.Path, data: bytes) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    fd = os.open(path, os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o600)
    try:
        with os.fdopen(fd, "wb") as stream:
            stream.write(data); stream.flush(); os.fsync(stream.fileno())
    except Exception:
        path.unlink(missing_ok=True); raise


def timestamp(value: str | None) -> str:
    value = value or dt.datetime.now(dt.timezone.utc).replace(microsecond=0).strftime("%Y-%m-%dT%H:%M:%SZ")
    if not UTC.fullmatch(value):
        fail("timestamp must be whole-second RFC3339 UTC")
    dt.datetime.strptime(value, "%Y-%m-%dT%H:%M:%SZ")
    return value


def exact_id(value: str, label: str) -> str:
    if not STABLE_ID.fullmatch(value):
        fail(f"{label} is noncanonical")
    return value


def validate_sha(value: Any, label: str, length: int = 64) -> str:
    pattern = SHA64 if length == 64 else SHA40
    if not isinstance(value, str) or not pattern.fullmatch(value):
        fail(f"{label} is invalid")
    return value


def receipt_id(value: dict[str, Any], prefix: str, domain: bytes) -> None:
    identifier = value.get("receipt_id")
    if not isinstance(identifier, str) or not identifier.startswith(prefix):
        fail("receipt ID prefix is invalid")
    digest = identifier.removeprefix(prefix)
    validate_sha(digest, "receipt ID")
    copy = dict(value); copy.pop("receipt_id")
    if digest != framed(domain, canonical(copy)):
        fail("receipt ID does not bind its payload")


def safe_path(name: str) -> str:
    if not name or "\x00" in name or "\\" in name:
        fail("archive path is empty or platform-ambiguous")
    path = pathlib.PurePosixPath(name)
    if path.is_absolute() or any(part in {"", ".", ".."} for part in path.parts):
        fail("archive path is unsafe")
    return name


def validate_source(path: pathlib.Path) -> tuple[dict[str, Any], bytes, str]:
    value, raw = load(path, "source receipt")
    if value.get("schema") != SOURCE_SCHEMA or value.get("phase") != "DEVELOPMENT":
        fail("source receipt schema/phase is invalid")
    if value.get("claim_level") != "SOURCE_PIN_AND_TREE_ONLY" or value.get("decision") != "SOURCE_PIN_VERIFIED_BUILD_NOT_QUALIFIED":
        fail("source receipt overclaims")
    timestamp(value.get("captured_at_utc"))
    if value.get("artifact") != SOURCE_ARTIFACT or value.get("authority") != SOURCE_AUTHORITY:
        fail("source receipt artifact or authority posture is open")
    source = value.get("source")
    if not isinstance(source, dict): fail("source facts are missing")
    if source.get("repository") != REPOSITORY or source.get("commit") != COMMIT or source.get("tree") != TREE:
        fail("source receipt pin drifted")
    if source.get("clean_worktree") is not True or not isinstance(source.get("embedded_commit_signature"), bool):
        fail("source receipt cleanliness/signature presence is invalid")
    manifest = source.get("tree_manifest")
    if not isinstance(manifest, dict) or manifest.get("algorithm") != "sha256-framed-git-ls-tree-v1":
        fail("source tree manifest is invalid")
    validate_sha(manifest.get("sha256"), "tree manifest")
    for key in ("entry_count", "blob_count", "submodule_count", "symlink_count", "path_utf8_bytes"):
        if not isinstance(manifest.get(key), int) or isinstance(manifest[key], bool) or manifest[key] < 0:
            fail(f"tree manifest {key} is invalid")
    if manifest["entry_count"] < 1 or manifest["blob_count"] < 1:
        fail("source tree is empty")
    if manifest["entry_count"] != manifest["blob_count"] + manifest["submodule_count"]:
        fail("source tree counts are inconsistent")
    submodules = source.get("submodules")
    if not isinstance(submodules, list) or len(submodules) != manifest["submodule_count"]:
        fail("submodule inventory is inconsistent")
    license_value = source.get("license")
    if not isinstance(license_value, dict) or license_value.get("spdx_id") != LICENSE or license_value.get("path") != "LICENSE":
        fail("source license identity drifted")
    validate_sha(license_value.get("sha256"), "license digest")
    if not isinstance(license_value.get("bytes"), int) or license_value["bytes"] < 1:
        fail("license size is invalid")
    receipt_id(value, "servo-source-receipt:v1:", SOURCE_ID_DOMAIN)
    return value, raw, framed(SOURCE_FACTS_DOMAIN, canonical(source))


def compare_source(left_path: pathlib.Path, right_path: pathlib.Path, left_id: str, right_id: str, captured: str) -> dict[str, Any]:
    left_id = exact_id(left_id, "left fetch ID"); right_id = exact_id(right_id, "right fetch ID")
    if left_id == right_id: fail("independent fetch IDs must differ")
    left, left_raw, left_facts = validate_source(left_path)
    right, right_raw, right_facts = validate_source(right_path)
    if left_facts != right_facts or left["source"] != right["source"]:
        fail("independent source facts differ")
    source = left["source"]
    out: dict[str, Any] = {
        "schema": COMPARE_SCHEMA, "phase": "DEVELOPMENT",
        "claim_level": "INDEPENDENT_FETCH_SOURCE_FACTS_ONLY", "captured_at_utc": timestamp(captured),
        "source_binding": {
            "repository": REPOSITORY, "commit": COMMIT, "tree": TREE,
            "source_facts_sha256": left_facts,
            "tree_manifest_sha256": source["tree_manifest"]["sha256"],
            "license_file_sha256": source["license"]["sha256"],
            "embedded_commit_signature": source["embedded_commit_signature"],
            "signature_trust_verified": False,
        },
        "left": {"fetch_id": left_id, "receipt_id": left["receipt_id"], "receipt_sha256": hashlib.sha256(left_raw).hexdigest()},
        "right": {"fetch_id": right_id, "receipt_id": right["receipt_id"], "receipt_sha256": hashlib.sha256(right_raw).hexdigest()},
        "comparison": {
            "source_facts_equal": True, "tree_manifest_equal": True,
            "license_digest_equal": True, "signature_presence_equal": True,
            "signature_trust_inferred": False, "independent_fetch_ids": True,
        },
        "artifact": {"source_archive_created": False, "worker_artifact_built": False, "sbom_created": False},
        "authority": AUTHORITY,
        "decision": "INDEPENDENT_FETCH_FACTS_MATCH_BUILD_NOT_QUALIFIED",
    }
    out["receipt_id"] = "servo-source-comparison:v1:" + framed(COMPARE_ID_DOMAIN, canonical(out))
    return out


def git(checkout: pathlib.Path, *args: str) -> bytes:
    try:
        return subprocess.run(["git", "-C", os.fspath(checkout), *args], check=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE).stdout
    except (OSError, subprocess.CalledProcessError) as error:
        detail = getattr(error, "stderr", b"").decode(errors="replace").strip()
        raise Error(f"git {' '.join(args)} failed: {detail}") from error


def normalized_origin(raw: str) -> str:
    value = raw.strip().removesuffix("/").removesuffix(".git")
    for prefix in ("git@github.com:", "ssh://git@github.com/", "git://github.com/", "https://github.com/", "http://github.com/"):
        if value.startswith(prefix): value = value[len(prefix):]; break
    if value.strip("/").casefold() != "servo/servo": fail("checkout origin is not servo/servo")
    return REPOSITORY


def validate_checkout(checkout: pathlib.Path, source: dict[str, Any]) -> pathlib.Path:
    checkout = checkout.resolve(strict=True)
    if not checkout.is_dir() or git(checkout, "rev-parse", "--is-inside-work-tree").strip() != b"true":
        fail("checkout is not a Git worktree")
    if git(checkout, "rev-parse", "HEAD").decode().strip() != COMMIT or git(checkout, "rev-parse", "HEAD^{tree}").decode().strip() != TREE:
        fail("checkout commit/tree drifted")
    if git(checkout, "status", "--porcelain=v1", "--untracked-files=all"):
        fail("checkout is dirty or has untracked files")
    if normalized_origin(git(checkout, "config", "--get", "remote.origin.url").decode()) != source["source"]["repository"]:
        fail("checkout origin drifted")
    if source["source"]["tree_manifest"]["submodule_count"] != 0:
        fail("archive v1 rejects submodules until recursive policy is sealed")
    return checkout


def inspect_archive(path: pathlib.Path, prefix: str) -> dict[str, Any]:
    members: list[dict[str, Any]] = []; directories = 0; previous: bytes | None = None
    license_bytes: bytes | None = None
    try:
        with tarfile.open(path, "r:gz") as archive:
            for member in archive:
                encoded = member.name.encode("utf-8")
                if previous is not None and encoded <= previous: fail("archive members are not byte-sorted")
                previous = encoded
                if not member.name.startswith(prefix): fail("archive member escaped canonical prefix")
                relative = member.name[len(prefix):].rstrip("/")
                if not relative:
                    if not member.isdir(): fail("archive root must be a directory")
                    directories += 1; continue
                safe_path(relative)
                if member.isdir(): directories += 1; continue
                if member.issym():
                    target = member.linkname
                    if not target or pathlib.PurePosixPath(target).is_absolute(): fail("archive symlink target is unsafe")
                    resolved = pathlib.PurePosixPath(relative).parent.joinpath(target)
                    depth = 0
                    for part in resolved.parts:
                        if part == "..": depth -= 1
                        elif part not in {"", "."}: depth += 1
                        if depth < 0: fail("archive symlink escapes root")
                    raw = target.encode()
                    members.append({"path": relative, "type": "symlink", "mode": "120000", "bytes": len(raw), "sha256": hashlib.sha256(raw).hexdigest(), "link_target": target})
                elif member.isfile():
                    stream = archive.extractfile(member)
                    if stream is None: fail("archive regular file has no content")
                    raw = stream.read()
                    if relative == "LICENSE": license_bytes = raw
                    members.append({"path": relative, "type": "file", "mode": "100755" if member.mode & 0o111 else "100644", "bytes": len(raw), "sha256": hashlib.sha256(raw).hexdigest(), "link_target": None})
                else: fail("archive contains unsupported member type")
    except (OSError, tarfile.TarError) as error:
        raise Error(f"cannot inspect archive: {error}") from error
    if not members or license_bytes is None: fail("archive is empty or lacks LICENSE")
    return {
        "inventory_sha256": framed(ARCHIVE_INVENTORY_DOMAIN, canonical(members)),
        "entry_count": len(members), "regular_file_count": sum(x["type"] == "file" for x in members),
        "symlink_count": sum(x["type"] == "symlink" for x in members),
        "executable_file_count": sum(x["mode"] == "100755" for x in members),
        "directory_count": directories, "license_sha256": hashlib.sha256(license_bytes).hexdigest(),
        "license_bytes": len(license_bytes),
    }


def create_archive(checkout: pathlib.Path, source_receipt_path: pathlib.Path, archive_path: pathlib.Path, receipt_path: pathlib.Path, captured: str) -> dict[str, Any]:
    source, source_raw, source_facts = validate_source(source_receipt_path)
    checkout = validate_checkout(checkout, source)
    prefix = f"servo-{COMMIT}/"
    if archive_path.exists() or receipt_path.exists(): fail("archive output already exists")
    archive_path.parent.mkdir(parents=True, exist_ok=True)
    with tempfile.NamedTemporaryFile(dir=archive_path.parent, suffix=".tar") as raw_tar:
        try:
            subprocess.run(["git", "-C", os.fspath(checkout), "archive", "--format=tar", f"--prefix={prefix}", COMMIT], check=True, stdout=raw_tar, stderr=subprocess.PIPE)
        except (OSError, subprocess.CalledProcessError) as error:
            raise Error("git archive failed") from error
        raw_tar.flush(); os.fsync(raw_tar.fileno()); raw_tar.seek(0)
        tar_sha, tar_bytes = sha_file(pathlib.Path(raw_tar.name))
        fd = os.open(archive_path, os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o600)
        try:
            with os.fdopen(fd, "wb") as output:
                with gzip.GzipFile(filename="", mode="wb", fileobj=output, compresslevel=9, mtime=0) as gz:
                    shutil.copyfileobj(raw_tar, gz, 1024 * 1024)
                output.flush(); os.fsync(output.fileno())
        except Exception:
            archive_path.unlink(missing_ok=True); raise
    archive_sha, archive_bytes = sha_file(archive_path)
    facts = inspect_archive(archive_path, prefix)
    if facts["license_sha256"] != source["source"]["license"]["sha256"] or facts["license_bytes"] != source["source"]["license"]["bytes"]:
        archive_path.unlink(missing_ok=True); fail("archive LICENSE differs from source receipt")
    out: dict[str, Any] = {
        "schema": ARCHIVE_SCHEMA, "phase": "DEVELOPMENT", "claim_level": "DETERMINISTIC_SOURCE_ARCHIVE_ONLY",
        "captured_at_utc": timestamp(captured),
        "source_binding": {"repository": REPOSITORY, "commit": COMMIT, "tree": TREE, "source_receipt_id": source["receipt_id"], "source_receipt_sha256": hashlib.sha256(source_raw).hexdigest(), "source_facts_sha256": source_facts, "tree_manifest_sha256": source["source"]["tree_manifest"]["sha256"]},
        "archive": {"file_name": archive_path.name, "format": "tar.gz", "tar_format": "git-archive-tar", "compression": "gzip-level-9-mtime-zero-no-filename", "prefix": prefix, "tar_sha256": tar_sha, "tar_bytes": tar_bytes, "sha256": archive_sha, "bytes": archive_bytes, **{k:v for k,v in facts.items() if not k.startswith("license_")}},
        "policy": {"source_from_git_objects": True, "submodules_allowed": False, "absolute_paths_allowed": False, "path_traversal_allowed": False, "escaping_symlinks_allowed": False, "network_access_used": False},
        "artifact": {"source_archive_created": True, "worker_artifact_built": False, "sbom_created": False},
        "authority": AUTHORITY, "decision": "DETERMINISTIC_SOURCE_ARCHIVE_CREATED_BUILD_NOT_QUALIFIED",
    }
    out["receipt_id"] = "servo-source-archive:v1:" + framed(ARCHIVE_ID_DOMAIN, canonical(out))
    try: write_new(receipt_path, canonical(out))
    except Exception: archive_path.unlink(missing_ok=True); raise
    return out


def validate_archive_receipt(path: pathlib.Path, archive_path: pathlib.Path | None = None) -> tuple[dict[str, Any], bytes]:
    value, raw = load(path, "archive receipt")
    if value.get("schema") != ARCHIVE_SCHEMA or value.get("phase") != "DEVELOPMENT" or value.get("claim_level") != "DETERMINISTIC_SOURCE_ARCHIVE_ONLY": fail("archive receipt schema/claim is invalid")
    if value.get("authority") != AUTHORITY or value.get("decision") != "DETERMINISTIC_SOURCE_ARCHIVE_CREATED_BUILD_NOT_QUALIFIED": fail("archive receipt overclaims")
    timestamp(value.get("captured_at_utc")); receipt_id(value, "servo-source-archive:v1:", ARCHIVE_ID_DOMAIN)
    source = value.get("source_binding", {}); archive = value.get("archive", {})
    if source.get("repository") != REPOSITORY or source.get("commit") != COMMIT or source.get("tree") != TREE: fail("archive source binding drifted")
    for key in ("source_receipt_sha256", "source_facts_sha256", "tree_manifest_sha256"): validate_sha(source.get(key), key)
    if archive.get("format") != "tar.gz" or archive.get("tar_format") != "git-archive-tar" or archive.get("prefix") != f"servo-{COMMIT}/": fail("archive format/prefix drifted")
    for key in ("tar_sha256", "sha256", "inventory_sha256"): validate_sha(archive.get(key), key)
    for key in ("tar_bytes", "bytes", "entry_count", "regular_file_count", "symlink_count", "executable_file_count", "directory_count"):
        if not isinstance(archive.get(key), int) or archive[key] < 0: fail(f"archive {key} is invalid")
    if value.get("artifact") != {"source_archive_created": True, "worker_artifact_built": False, "sbom_created": False}: fail("archive artifact posture drifted")
    if archive_path is not None:
        digest, size = sha_file(archive_path)
        if digest != archive["sha256"] or size != archive["bytes"]: fail("archive bytes differ from receipt")
        facts = inspect_archive(archive_path, archive["prefix"])
        for key in ("inventory_sha256", "entry_count", "regular_file_count", "symlink_count", "executable_file_count", "directory_count"):
            if facts[key] != archive[key]: fail(f"archive recomputation differs for {key}")
    return value, raw


def compare_archives(left_receipt: pathlib.Path, right_receipt: pathlib.Path, left_archive: pathlib.Path, right_archive: pathlib.Path, left_id: str, right_id: str, captured: str) -> dict[str, Any]:
    left_id = exact_id(left_id, "left generation ID"); right_id = exact_id(right_id, "right generation ID")
    if left_id == right_id: fail("archive generation IDs must differ")
    left, left_raw = validate_archive_receipt(left_receipt, left_archive)
    right, right_raw = validate_archive_receipt(right_receipt, right_archive)
    stable_source = ("repository", "commit", "tree", "source_facts_sha256", "tree_manifest_sha256")
    if {k:left["source_binding"][k] for k in stable_source} != {k:right["source_binding"][k] for k in stable_source}: fail("archive source bindings differ")
    left_facts = dict(left["archive"]); right_facts = dict(right["archive"]); left_facts.pop("file_name"); right_facts.pop("file_name")
    if left_facts != right_facts: fail("archive receipt facts differ")
    if sha_file(left_archive) != sha_file(right_archive): fail("archive bytes differ")
    out: dict[str, Any] = {
        "schema": ARCHIVE_COMPARE_SCHEMA, "phase": "DEVELOPMENT", "claim_level": "INDEPENDENT_SOURCE_ARCHIVE_REPRODUCIBILITY_ONLY", "captured_at_utc": timestamp(captured),
        "source_binding": {k:left["source_binding"][k] for k in stable_source},
        "archive_binding": {"sha256": left["archive"]["sha256"], "bytes": left["archive"]["bytes"], "inventory_sha256": left["archive"]["inventory_sha256"], "entry_count": left["archive"]["entry_count"]},
        "left": {"generation_id": left_id, "receipt_id": left["receipt_id"], "receipt_sha256": hashlib.sha256(left_raw).hexdigest()},
        "right": {"generation_id": right_id, "receipt_id": right["receipt_id"], "receipt_sha256": hashlib.sha256(right_raw).hexdigest()},
        "comparison": {"archive_bytes_equal": True, "archive_digest_equal": True, "inventory_equal": True, "source_binding_equal": True, "independent_generation_ids": True},
        "artifact": {"source_archive_created": True, "worker_artifact_built": False, "sbom_created": False}, "authority": AUTHORITY,
        "decision": "INDEPENDENT_SOURCE_ARCHIVES_MATCH_BUILD_NOT_QUALIFIED",
    }
    out["receipt_id"] = "servo-source-archive-comparison:v1:" + framed(ARCHIVE_COMPARE_ID_DOMAIN, canonical(out))
    return out


def patch_inventory(governance_path: pathlib.Path, patch_root: pathlib.Path) -> dict[str, Any]:
    governance, _ = load(governance_path, "governance patch inventory", False)
    if governance.get("schema") != "hepta.browser.servo_patch_inventory.v1" or governance.get("servo_commit") != COMMIT or governance.get("servo_tree") != TREE: fail("patch governance pin/schema drifted")
    if governance.get("policy") != {"unrecorded_patch_allowed": False, "patch_without_test_allowed": False, "patch_without_deletion_condition_allowed": False, "production_authority": False}: fail("patch governance is not fail-closed")
    entries = governance.get("patches")
    if not isinstance(entries, list): fail("patch governance entries are invalid")
    files = sorted(p.name for p in patch_root.glob("*.patch"))
    if len(files) != len(entries): fail("patch files and governance entries differ")
    out_entries: list[dict[str, Any]] = []
    for entry in entries:
        if not isinstance(entry, dict): fail("patch governance entry is invalid")
        filename = entry.get("file"); patch_id = entry.get("id") or entry.get("patch_id")
        if filename not in files or not isinstance(patch_id, str): fail("patch governance entry references an unknown patch")
        path = patch_root / filename; meta = path.lstat()
        if stat.S_ISLNK(meta.st_mode) or not stat.S_ISREG(meta.st_mode) or getattr(meta, "st_nlink", 1) != 1: fail("patch file is not a private regular single-link file")
        digest, _ = sha_file(path)
        if digest != (entry.get("sha256") or entry.get("patch_sha256")): fail("patch digest drifted")
        reason = entry.get("reason") or entry.get("purpose"); upstream = entry.get("upstream_reference"); deletion = entry.get("deletion_condition"); tests = entry.get("tests")
        if not all(isinstance(x, str) and x for x in (reason, upstream, deletion)) or not isinstance(tests, list) or not tests: fail("patch governance metadata is incomplete")
        out_entries.append({"id": patch_id, "path": f"third_party/servo-patches/{filename}", "sha256": digest, "reason": reason, "upstream_reference": upstream, "deletion_condition": deletion, "tests": sorted(set(tests))})
    out_entries.sort(key=lambda x:x["id"])
    if [x["id"] for x in out_entries] != sorted(set(x["id"] for x in out_entries)): fail("patch IDs are not sorted and unique")
    return {"schema": PATCH_SCHEMA, "upstream_commit": COMMIT, "upstream_tree": TREE, "patches": out_entries}


def license_create(source_path: pathlib.Path, archive_path: pathlib.Path, archive_receipt_path: pathlib.Path, governance_path: pathlib.Path, patch_root: pathlib.Path, license_out: pathlib.Path, patch_out: pathlib.Path, distribution_out: pathlib.Path, captured: str) -> dict[str, Any]:
    source, source_raw, _ = validate_source(source_path)
    archive_receipt, archive_raw = validate_archive_receipt(archive_receipt_path, archive_path)
    if archive_receipt["source_binding"]["source_receipt_sha256"] != hashlib.sha256(source_raw).hexdigest(): fail("archive receipt does not bind source receipt")
    with tarfile.open(archive_path, "r:gz") as archive:
        member = archive.getmember(archive_receipt["archive"]["prefix"] + "LICENSE")
        stream = archive.extractfile(member)
        if stream is None: fail("archive LICENSE is unavailable")
        license_bytes = stream.read()
    if b"Mozilla Public License Version 2.0" not in license_bytes or hashlib.sha256(license_bytes).hexdigest() != source["source"]["license"]["sha256"]: fail("archive LICENSE differs from source receipt")
    patches = patch_inventory(governance_path, patch_root); patches_raw = canonical(patches)
    license_value = {
        "schema": LICENSE_SCHEMA, "upstream_repository": REPOSITORY, "upstream_commit": COMMIT,
        "primary_license": LICENSE, "license_file_sha256": hashlib.sha256(license_bytes).hexdigest(),
        "notices": sorted(["LICENSE", "MPL-2.0", f"PATCH_INVENTORY:{hashlib.sha256(patches_raw).hexdigest()}", f"SOURCE_ARCHIVE:{archive_receipt['archive']['sha256']}", f"SOURCE_RECEIPT:{source['receipt_id']}"]),
        "source_offer_required_by_project_policy": True, "legal_review_required_before_binary_distribution": True,
        "binary_distribution_authorized": False,
    }
    license_raw = canonical(license_value)
    distribution: dict[str, Any] = {
        "schema": DISTRIBUTION_SCHEMA, "phase": "DEVELOPMENT", "claim_level": "SOURCE_DISTRIBUTION_PACKET_ONLY", "captured_at_utc": timestamp(captured),
        "source_binding": {"repository": REPOSITORY, "commit": COMMIT, "tree": TREE, "source_receipt_id": source["receipt_id"], "source_receipt_sha256": hashlib.sha256(source_raw).hexdigest(), "source_archive_receipt_id": archive_receipt["receipt_id"], "source_archive_receipt_sha256": hashlib.sha256(archive_raw).hexdigest(), "source_archive_sha256": archive_receipt["archive"]["sha256"]},
        "license_binding": {"primary_license": LICENSE, "license_file_sha256": license_value["license_file_sha256"], "license_packet_sha256": hashlib.sha256(license_raw).hexdigest(), "source_offer_required_by_project_policy": True, "source_archive_is_distribution_payload": True, "binary_distribution_authorized": False},
        "patch_binding": {"patch_inventory_sha256": hashlib.sha256(patches_raw).hexdigest(), "patch_count": len(patches["patches"]), "modified_source": bool(patches["patches"])},
        "distribution": {"archive_available": True, "license_in_archive": True, "notices_available": True, "network_access_used": False, "worker_artifact_built": False, "sbom_created": False},
        "authority": AUTHORITY, "decision": "MPL_PACKET_AND_PATCH_INVENTORY_SEALED_BUILD_NOT_QUALIFIED",
    }
    distribution["receipt_id"] = "servo-source-distribution:v1:" + framed(DISTRIBUTION_ID_DOMAIN, canonical(distribution))
    write_new(license_out, license_raw)
    try: write_new(patch_out, patches_raw); write_new(distribution_out, canonical(distribution))
    except Exception:
        for path in (license_out, patch_out, distribution_out): path.unlink(missing_ok=True)
        raise
    return distribution


def verify_bundle(left_source: pathlib.Path, right_source: pathlib.Path, source_compare: pathlib.Path, left_archive: pathlib.Path, right_archive: pathlib.Path, left_archive_receipt: pathlib.Path, right_archive_receipt: pathlib.Path, archive_compare: pathlib.Path, license_packet: pathlib.Path, patch_inventory_path: pathlib.Path, distribution_path: pathlib.Path, captured: str) -> dict[str, Any]:
    left, left_raw, left_facts = validate_source(left_source); right, right_raw, right_facts = validate_source(right_source)
    compare, compare_raw = load(source_compare, "source comparison")
    if compare.get("schema") != COMPARE_SCHEMA or left_facts != right_facts or compare.get("source_binding",{}).get("source_facts_sha256") != left_facts: fail("source comparison does not bind both receipts")
    larch, larch_raw = validate_archive_receipt(left_archive_receipt, left_archive); rarch, _ = validate_archive_receipt(right_archive_receipt, right_archive)
    acompare, acompare_raw = load(archive_compare, "archive comparison")
    if acompare.get("schema") != ARCHIVE_COMPARE_SCHEMA or larch["archive"]["sha256"] != rarch["archive"]["sha256"] or sha_file(left_archive) != sha_file(right_archive): fail("archive comparison does not bind identical archives")
    license_value, license_raw = load(license_packet, "license packet"); patches, patches_raw = load(patch_inventory_path, "patch inventory"); distribution, distribution_raw = load(distribution_path, "distribution receipt")
    if license_value.get("schema") != LICENSE_SCHEMA or license_value.get("binary_distribution_authorized") is not False: fail("license packet overclaims")
    if patches.get("schema") != PATCH_SCHEMA: fail("patch inventory schema drifted")
    if distribution.get("schema") != DISTRIBUTION_SCHEMA or distribution.get("authority") != AUTHORITY: fail("distribution receipt overclaims")
    receipt_id(distribution, "servo-source-distribution:v1:", DISTRIBUTION_ID_DOMAIN)
    if distribution["source_binding"]["source_receipt_sha256"] != hashlib.sha256(left_raw).hexdigest() or distribution["source_binding"]["source_archive_receipt_sha256"] != hashlib.sha256(larch_raw).hexdigest(): fail("distribution source binding drifted")
    if distribution["license_binding"]["license_packet_sha256"] != hashlib.sha256(license_raw).hexdigest() or distribution["patch_binding"]["patch_inventory_sha256"] != hashlib.sha256(patches_raw).hexdigest(): fail("distribution packet binding drifted")
    out: dict[str, Any] = {
        "schema": BUNDLE_SCHEMA, "phase": "DEVELOPMENT", "claim_level": "SOURCE_BUNDLE_RECOMPUTED_ONLY", "captured_at_utc": timestamp(captured),
        "source": {"repository": REPOSITORY, "commit": COMMIT, "tree": TREE, "source_facts_sha256": left_facts, "source_comparison_receipt_sha256": hashlib.sha256(compare_raw).hexdigest()},
        "archive": {"sha256": larch["archive"]["sha256"], "bytes": larch["archive"]["bytes"], "archive_comparison_receipt_sha256": hashlib.sha256(acompare_raw).hexdigest()},
        "distribution": {"license_packet_sha256": hashlib.sha256(license_raw).hexdigest(), "patch_inventory_sha256": hashlib.sha256(patches_raw).hexdigest(), "distribution_receipt_sha256": hashlib.sha256(distribution_raw).hexdigest(), "binary_distribution_authorized": False},
        "artifact": {"worker_artifact_built": False, "sbom_created": False, "runtime_qualified": False}, "authority": AUTHORITY,
        "decision": "SOURCE_BUNDLE_RECOMPUTED_BUILD_AND_RUNTIME_NOT_QUALIFIED",
    }
    out["receipt_id"] = "servo-source-bundle-verification:v1:" + framed(BUNDLE_ID_DOMAIN, canonical(out)); return out


def contract() -> dict[str, Any]:
    text = pathlib.Path(__file__).read_text()
    for forbidden in ("import socket", "import urllib", "import requests"):
        if forbidden in text: fail(f"forbidden network surface: {forbidden}")
    if any(AUTHORITY.values()) or any(SOURCE_AUTHORITY.values()): fail("authority posture is open")
    return {"schema":"hepta.servo.source_bundle_contract.v1","status":"PASS_FIXTURE_CONTRACT_ONLY","servo_commit":COMMIT,"servo_tree":TREE,"network_fetch_performed":False,"worker_artifact_built":False,"authority":AUTHORITY}


def fixture_receipt(repo: pathlib.Path, captured: str) -> dict[str, Any]:
    commit = git(repo,"rev-parse","HEAD").decode().strip(); tree = git(repo,"rev-parse","HEAD^{tree}").decode().strip()
    raw = git(repo,"ls-tree","-r","-z","--full-tree",commit); entries=[]
    for record in raw.split(b"\0"):
        if not record: continue
        meta,path = record.split(b"\t",1); mode,kind,obj = meta.decode().split(" ",2)
        entries.append({"mode":mode,"object_type":kind,"object_id":obj,"path":path.decode()})
    fields=[b"\0".join((e["mode"].encode(),e["object_type"].encode(),e["object_id"].encode(),e["path"].encode())) for e in entries]
    tree_digest=hashlib.sha256(); tree_digest.update(len(b"hepta.servo.git-tree-manifest.v1").to_bytes(8,"big")); tree_digest.update(b"hepta.servo.git-tree-manifest.v1")
    for field in fields: tree_digest.update(len(field).to_bytes(8,"big")); tree_digest.update(field)
    license_bytes=git(repo,"show",f"{commit}:LICENSE")
    source={"repository":REPOSITORY,"commit":commit,"tree":tree,"clean_worktree":True,"embedded_commit_signature":False,"tree_manifest":{"algorithm":"sha256-framed-git-ls-tree-v1","sha256":tree_digest.hexdigest(),"entry_count":len(entries),"blob_count":sum(e["object_type"]=="blob" for e in entries),"submodule_count":sum(e["object_type"]=="commit" for e in entries),"symlink_count":sum(e["mode"]=="120000" for e in entries),"path_utf8_bytes":sum(len(e["path"].encode()) for e in entries)},"submodules":[e for e in entries if e["object_type"]=="commit"],"license":{"spdx_id":LICENSE,"path":"LICENSE","bytes":len(license_bytes),"sha256":hashlib.sha256(license_bytes).hexdigest()}}
    out={"schema":SOURCE_SCHEMA,"phase":"DEVELOPMENT","claim_level":"SOURCE_PIN_AND_TREE_ONLY","captured_at_utc":captured,"source":source,"artifact":SOURCE_ARTIFACT,"authority":SOURCE_AUTHORITY,"decision":"SOURCE_PIN_VERIFIED_BUILD_NOT_QUALIFIED"}
    out["receipt_id"]="servo-source-receipt:v1:"+framed(SOURCE_ID_DOMAIN,canonical(out)); return out


def self_test() -> dict[str, Any]:
    global COMMIT,TREE
    original=(COMMIT,TREE); tests=[]
    with tempfile.TemporaryDirectory() as directory:
        root=pathlib.Path(directory); origin=root/"origin"; origin.mkdir(); git(origin,"init","-q"); git(origin,"config","user.name","Hepta"); git(origin,"config","user.email","hepta@example.invalid")
        (origin/"LICENSE").write_text("Mozilla Public License Version 2.0\nfixture\n"); (origin/"README.md").write_text("fixture\n"); (origin/"mach").write_text("#!/bin/sh\nexit 0\n"); (origin/"mach").chmod(0o755)
        git(origin,"add","."); git(origin,"commit","-q","-m","fixture"); git(origin,"remote","add","origin","https://github.com/servo/servo.git")
        COMMIT=git(origin,"rev-parse","HEAD").decode().strip(); TREE=git(origin,"rev-parse","HEAD^{tree}").decode().strip()
        left=root/"left"; right=root/"right"
        for checkout in (left,right):
            subprocess.run(["git","clone","-q",os.fspath(origin),os.fspath(checkout)],check=True); git(checkout,"remote","set-url","origin","https://github.com/servo/servo.git")
        lsrc=root/"left.json"; rsrc=root/"right.json"; write_new(lsrc,canonical(fixture_receipt(left,"2026-08-27T01:00:00Z"))); write_new(rsrc,canonical(fixture_receipt(right,"2026-08-27T01:00:01Z")))
        comparison=compare_source(lsrc,rsrc,"fetch-a","fetch-b","2026-08-27T01:01:00Z"); cpath=root/"compare.json"; write_new(cpath,canonical(comparison)); tests.append("independent_source_compare")
        try: compare_source(lsrc,rsrc,"same","same","2026-08-27T01:01:00Z"); fail("same fetch ID passed")
        except Error: tests.append("same_fetch_id_rejected")
        la=root/"left.tar.gz"; ra=root/"right.tar.gz"; lar=root/"left-archive.json"; rar=root/"right-archive.json"; create_archive(left,lsrc,la,lar,"2026-08-27T01:02:00Z"); create_archive(right,rsrc,ra,rar,"2026-08-27T01:02:01Z")
        if sha_file(la)!=sha_file(ra): fail("fixture archives differ")
        ac=compare_archives(lar,rar,la,ra,"archive-a","archive-b","2026-08-27T01:03:00Z"); acp=root/"archive-compare.json"; write_new(acp,canonical(ac)); tests.append("deterministic_archive_reproduced")
        (left/"dirty").write_text("x")
        try: validate_checkout(left,validate_source(lsrc)[0]); fail("dirty checkout passed")
        except Error: tests.append("dirty_checkout_rejected")
        (left/"dirty").unlink()
        patch_root=root/"patches"; patch_root.mkdir(); governance={"schema":"hepta.browser.servo_patch_inventory.v1","schema_version":1,"status":"EMPTY_NO_SOURCE_IMPORTED","servo_commit":COMMIT,"servo_tree":TREE,"maximum_initial_patches":8,"patches":[],"next_expected_patch":{"patch_id":"SERVO-HEPTA-0001","purpose":"fixture","status":"NOT_CREATED"},"policy":{"unrecorded_patch_allowed":False,"patch_without_test_allowed":False,"patch_without_deletion_condition_allowed":False,"production_authority":False}}
        gp=patch_root/"PATCH_INVENTORY.json"; gp.write_text(json.dumps(governance)); lp=root/"license.json"; pp=root/"patch.json"; dp=root/"distribution.json"; license_create(lsrc,la,lar,gp,patch_root,lp,pp,dp,"2026-08-27T01:04:00Z"); tests.append("license_packet_sealed")
        bad=dict(governance); bad["policy"]=dict(governance["policy"]); bad["policy"]["production_authority"]=True; bp=patch_root/"BAD.json"; bp.write_text(json.dumps(bad))
        try: patch_inventory(bp,patch_root); fail("open patch policy passed")
        except Error: tests.append("open_patch_policy_rejected")
        try: safe_path("../escape"); fail("traversal passed")
        except Error: tests.append("traversal_rejected")
        bundle=verify_bundle(lsrc,rsrc,cpath,la,ra,lar,rar,acp,lp,pp,dp,"2026-08-27T01:05:00Z")
        if bundle["authority"]!=AUTHORITY: fail("bundle authority drifted")
        tests.append("complete_bundle_recomputed")
    COMMIT,TREE=original
    if len(tests)!=7: fail("unexpected fixture test count")
    return {"schema":"hepta.servo.source_bundle_self_test.v1","status":"PASS_LOCAL_FIXTURE_ONLY","tests":tests,"test_count":7,"canonical_servo_fetch_performed":False,"worker_artifact_built":False,"authority":AUTHORITY}


def parser() -> argparse.ArgumentParser:
    p=argparse.ArgumentParser(description=__doc__); s=p.add_subparsers(dest="command",required=True)
    q=s.add_parser("compare-source"); q.add_argument("--left-receipt",type=pathlib.Path,required=True); q.add_argument("--right-receipt",type=pathlib.Path,required=True); q.add_argument("--left-fetch-id",required=True); q.add_argument("--right-fetch-id",required=True); q.add_argument("--captured-at"); q.add_argument("--output",type=pathlib.Path,required=True)
    q=s.add_parser("archive-create"); q.add_argument("--checkout",type=pathlib.Path,required=True); q.add_argument("--source-receipt",type=pathlib.Path,required=True); q.add_argument("--output-archive",type=pathlib.Path,required=True); q.add_argument("--output-receipt",type=pathlib.Path,required=True); q.add_argument("--captured-at")
    q=s.add_parser("archive-verify"); q.add_argument("--archive",type=pathlib.Path,required=True); q.add_argument("--receipt",type=pathlib.Path,required=True)
    q=s.add_parser("archive-compare"); q.add_argument("--left-archive",type=pathlib.Path,required=True); q.add_argument("--left-receipt",type=pathlib.Path,required=True); q.add_argument("--left-generation-id",required=True); q.add_argument("--right-archive",type=pathlib.Path,required=True); q.add_argument("--right-receipt",type=pathlib.Path,required=True); q.add_argument("--right-generation-id",required=True); q.add_argument("--captured-at"); q.add_argument("--output",type=pathlib.Path,required=True)
    q=s.add_parser("license-create"); q.add_argument("--source-receipt",type=pathlib.Path,required=True); q.add_argument("--source-archive",type=pathlib.Path,required=True); q.add_argument("--archive-receipt",type=pathlib.Path,required=True); q.add_argument("--governance-patch-inventory",type=pathlib.Path,required=True); q.add_argument("--patch-root",type=pathlib.Path,required=True); q.add_argument("--output-license-packet",type=pathlib.Path,required=True); q.add_argument("--output-patch-inventory",type=pathlib.Path,required=True); q.add_argument("--output-distribution-receipt",type=pathlib.Path,required=True); q.add_argument("--captured-at")
    q=s.add_parser("license-verify"); q.add_argument("--license-packet",type=pathlib.Path,required=True); q.add_argument("--patch-inventory",type=pathlib.Path,required=True); q.add_argument("--distribution-receipt",type=pathlib.Path,required=True)
    q=s.add_parser("verify-bundle")
    for name in ("left-source-receipt","right-source-receipt","source-comparison","left-archive","right-archive","left-archive-receipt","right-archive-receipt","archive-comparison","license-packet","patch-inventory","distribution-receipt","output"): q.add_argument("--"+name,type=pathlib.Path,required=True)
    q.add_argument("--captured-at"); s.add_parser("contract"); s.add_parser("self-test"); return p


def main(argv: list[str] | None=None) -> int:
    a=parser().parse_args(argv)
    try:
        if a.command=="compare-source": out=compare_source(a.left_receipt,a.right_receipt,a.left_fetch_id,a.right_fetch_id,a.captured_at or timestamp(None)); write_new(a.output,canonical(out)); result={"status":out["decision"],"receipt_id":out["receipt_id"]}
        elif a.command=="archive-create": out=create_archive(a.checkout,a.source_receipt,a.output_archive,a.output_receipt,a.captured_at or timestamp(None)); result={"status":out["decision"],"receipt_id":out["receipt_id"],"archive_sha256":out["archive"]["sha256"]}
        elif a.command=="archive-verify": out,raw=validate_archive_receipt(a.receipt,a.archive); result={"status":"SOURCE_ARCHIVE_VERIFIED_BUILD_NOT_QUALIFIED","receipt_id":out["receipt_id"],"receipt_sha256":hashlib.sha256(raw).hexdigest()}
        elif a.command=="archive-compare": out=compare_archives(a.left_receipt,a.right_receipt,a.left_archive,a.right_archive,a.left_generation_id,a.right_generation_id,a.captured_at or timestamp(None)); write_new(a.output,canonical(out)); result={"status":out["decision"],"receipt_id":out["receipt_id"]}
        elif a.command=="license-create": out=license_create(a.source_receipt,a.source_archive,a.archive_receipt,a.governance_patch_inventory,a.patch_root,a.output_license_packet,a.output_patch_inventory,a.output_distribution_receipt,a.captured_at or timestamp(None)); result={"status":out["decision"],"receipt_id":out["receipt_id"]}
        elif a.command=="license-verify":
            lv,lr=load(a.license_packet,"license packet"); pv,pr=load(a.patch_inventory,"patch inventory"); dv,dr=load(a.distribution_receipt,"distribution receipt")
            if lv.get("schema")!=LICENSE_SCHEMA or lv.get("binary_distribution_authorized") is not False or pv.get("schema")!=PATCH_SCHEMA or dv.get("schema")!=DISTRIBUTION_SCHEMA or dv.get("authority")!=AUTHORITY: fail("license packet verification failed")
            receipt_id(dv,"servo-source-distribution:v1:",DISTRIBUTION_ID_DOMAIN); result={"status":"SOURCE_DISTRIBUTION_PACKET_VERIFIED_BUILD_NOT_QUALIFIED","receipt_id":dv["receipt_id"],"license_sha256":hashlib.sha256(lr).hexdigest(),"patch_sha256":hashlib.sha256(pr).hexdigest(),"distribution_sha256":hashlib.sha256(dr).hexdigest()}
        elif a.command=="verify-bundle": out=verify_bundle(a.left_source_receipt,a.right_source_receipt,a.source_comparison,a.left_archive,a.right_archive,a.left_archive_receipt,a.right_archive_receipt,a.archive_comparison,a.license_packet,a.patch_inventory,a.distribution_receipt,a.captured_at or timestamp(None)); write_new(a.output,canonical(out)); result={"status":out["decision"],"receipt_id":out["receipt_id"]}
        elif a.command=="contract": result=contract()
        else: result=self_test()
    except (Error,OSError,subprocess.SubprocessError) as error:
        print(json.dumps({"status":"FAIL_CLOSED","error":str(error)},sort_keys=True)); return 1
    print(json.dumps(result,sort_keys=True)); return 0

if __name__=="__main__": sys.exit(main())
