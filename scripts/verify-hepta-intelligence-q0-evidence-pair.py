#!/usr/bin/env python3
"""Verify a same-run, dual-architecture Q0 evidence pair and its artifacts."""

from __future__ import annotations

import argparse
import copy
import hashlib
import json
from pathlib import Path
import re
import tempfile
from typing import Any

SCHEMA = "hepta.intelligence.q0.executable_evidence.v2"
PAIR_SCHEMA = "hepta.intelligence.q0.evidence_pair.v2"
WORKFLOW_PATH = ".github/workflows/hepta-intelligence-q0-paired-candidate-v10.yml"
REPOSITORY = {
    "full_name": "ProfHepta/hepta-private-ci",
    "repository_id": 1320694176,
    "owner_id": 102159240,
}
HEX40 = re.compile(r"[0-9a-f]{40}")
NEGATIVE_AUTHORITY_KEYS = (
    "runtime_wired",
    "external_effects",
    "kg_write_authority",
    "model_authority",
    "provider_effects",
    "fleet_authority",
    "production_authority",
    "operator_acceptance",
    "promotion",
    "release_authority",
    "callers_ratchet",
)
RECEIPT_NAMES = {
    "e1-qualification-receipt.json",
    "e2-qualification-receipt.json",
    "q0-evidence-pair-receipt.json",
}


def canonical(value: Any) -> bytes:
    return json.dumps(value, sort_keys=True, separators=(",", ":")).encode()


def sha256_bytes(value: bytes) -> str:
    return hashlib.sha256(value).hexdigest()


def sha256_file(path: Path) -> str:
    return sha256_bytes(path.read_bytes())


def load(path: Path) -> tuple[dict[str, Any], bytes]:
    raw = path.read_bytes()
    value = json.loads(raw)
    if not isinstance(value, dict):
        raise ValueError(f"{path}: receipt must be an object")
    return value, raw


def add(failures: list[str], condition: bool, label: str) -> None:
    if not condition:
        failures.append(label)


def false_authority(value: Any) -> bool:
    return (
        isinstance(value, dict)
        and set(value) == set(NEGATIVE_AUTHORITY_KEYS)
        and all(value.get(key) is False for key in NEGATIVE_AUTHORITY_KEYS)
    )


def validate_artifact(
    root: Path, receipt: dict[str, Any], label: str, failures: list[str]
) -> None:
    manifest = receipt.get("artifact_manifest")
    if not isinstance(manifest, list) or not manifest:
        failures.append(f"{label}.artifact_manifest")
        return
    add(
        failures,
        receipt.get("artifact_manifest_sha256") == sha256_bytes(canonical(manifest)),
        f"{label}.artifact_manifest_digest",
    )
    listed: set[str] = set()
    for index, entry in enumerate(manifest):
        if not isinstance(entry, dict):
            failures.append(f"{label}.artifact_entry[{index}]")
            continue
        path_value = entry.get("path")
        if (
            not isinstance(path_value, str)
            or path_value.startswith("/")
            or ".." in Path(path_value).parts
        ):
            failures.append(f"{label}.artifact_path[{index}]")
            continue
        listed.add(path_value)
        path = root / path_value
        add(failures, path.is_file(), f"{label}.artifact_missing:{path_value}")
        if path.is_file():
            add(
                failures,
                entry.get("sha256") == sha256_file(path),
                f"{label}.artifact_sha:{path_value}",
            )
            add(
                failures,
                entry.get("size") == path.stat().st_size,
                f"{label}.artifact_size:{path_value}",
            )
    actual = {
        path.relative_to(root).as_posix()
        for path in root.rglob("*")
        if path.is_file() and path.relative_to(root).as_posix() not in RECEIPT_NAMES
    }
    add(failures, listed == actual, f"{label}.artifact_surface")


def validate_source(receipt: dict[str, Any], label: str, failures: list[str]) -> None:
    source = receipt.get("source_overlay")
    if not isinstance(source, dict):
        failures.append(f"{label}.source_overlay")
        return
    files = source.get("files")
    add(failures, isinstance(files, list) and bool(files), f"{label}.source_files")
    if isinstance(files, list):
        paths = [entry.get("path") for entry in files if isinstance(entry, dict)]
        add(
            failures,
            len(paths) == len(files) and paths == sorted(set(paths)),
            f"{label}.source_file_surface",
        )
        for index, entry in enumerate(files):
            add(
                failures,
                isinstance(entry, dict)
                and isinstance(entry.get("path"), str)
                and re.fullmatch(r"[0-9a-f]{64}", str(entry.get("sha256", "")))
                is not None
                and isinstance(entry.get("size"), int)
                and entry.get("size", -1) >= 0,
                f"{label}.source_file[{index}]",
            )
    base = {key: value for key, value in source.items() if key != "manifest_sha256"}
    add(
        failures,
        source.get("manifest_sha256") == sha256_bytes(canonical(base)),
        f"{label}.source_manifest_digest",
    )
    for key in (
        "changed_files_sha256",
        "repair_patch_sha256",
        "repair_stat_sha256",
        "manifest_sha256",
    ):
        add(
            failures,
            re.fullmatch(r"[0-9a-f]{64}", str(source.get(key, ""))) is not None,
            f"{label}.{key}",
        )


def validate_one(
    receipt: dict[str, Any],
    receipt_raw: bytes,
    artifact_root: Path,
    evidence_class: str,
    expected_job: str,
    expected_arches: set[str],
) -> list[str]:
    label = evidence_class
    failures: list[str] = []
    add(failures, receipt.get("schema") == SCHEMA, f"{label}.schema")
    add(failures, receipt.get("evidence_class") == evidence_class, f"{label}.class")
    expected_status = {
        "E1_LOCAL_EXECUTABLE": "PASS_Q0_E1_LOCAL_EXECUTABLE",
        "E2_INDEPENDENT_RUNNER": "PASS_Q0_E2_INDEPENDENT_RUNNER",
    }[evidence_class]
    add(failures, receipt.get("status") == expected_status, f"{label}.status")
    add(failures, receipt.get("repository") == REPOSITORY, f"{label}.repository")

    candidate = receipt.get("candidate")
    add(
        failures,
        isinstance(candidate, dict)
        and set(candidate) == {"head", "tree", "parent"}
        and all(
            isinstance(candidate.get(key), str) and HEX40.fullmatch(candidate[key])
            for key in candidate
        ),
        f"{label}.candidate",
    )
    workflow = receipt.get("workflow")
    add(failures, isinstance(workflow, dict), f"{label}.workflow")
    if isinstance(workflow, dict):
        add(failures, workflow.get("path") == WORKFLOW_PATH, f"{label}.workflow_path")
        add(
            failures,
            isinstance(candidate, dict)
            and workflow.get("sha") == candidate.get("head"),
            f"{label}.workflow_sha",
        )
        add(
            failures,
            isinstance(workflow.get("run_id"), int) and workflow["run_id"] > 0,
            f"{label}.run_id",
        )
        add(
            failures,
            isinstance(workflow.get("run_attempt"), int)
            and workflow["run_attempt"] > 0,
            f"{label}.run_attempt",
        )
        add(failures, workflow.get("job") == expected_job, f"{label}.job")

    runner = receipt.get("runner")
    add(failures, isinstance(runner, dict), f"{label}.runner")
    if isinstance(runner, dict):
        add(failures, bool(runner.get("name")), f"{label}.runner_name")
        add(
            failures, str(runner.get("os", "")).upper() == "LINUX", f"{label}.runner_os"
        )
        add(
            failures,
            str(runner.get("arch", "")).upper() in expected_arches,
            f"{label}.runner_arch",
        )

    blocking = receipt.get("blocking_results")
    add(
        failures,
        isinstance(blocking, dict) and bool(blocking),
        f"{label}.blocking_results",
    )
    if isinstance(blocking, dict):
        add(
            failures,
            all(type(value) is int and value == 0 for value in blocking.values()),
            f"{label}.blocking_zero",
        )
        add(
            failures,
            receipt.get("blocking_results_sha256") == sha256_bytes(canonical(blocking)),
            f"{label}.blocking_digest",
        )
    diagnostics = receipt.get("diagnostics")
    add(failures, isinstance(diagnostics, dict), f"{label}.diagnostics")
    if isinstance(diagnostics, dict):
        add(
            failures,
            all(type(value) is int for value in diagnostics.values()),
            f"{label}.diagnostic_values",
        )
        add(
            failures,
            receipt.get("diagnostics_sha256") == sha256_bytes(canonical(diagnostics)),
            f"{label}.diagnostics_digest",
        )
    add(failures, receipt.get("all_gates_zero") is True, f"{label}.all_gates_zero")
    add(failures, receipt.get("source_writeback") is False, f"{label}.source_writeback")
    add(
        failures,
        receipt.get("qualified_candidate") is False,
        f"{label}.premature_qualification",
    )
    add(failures, false_authority(receipt.get("authority")), f"{label}.authority")

    binding = dict(receipt)
    observed_binding = binding.pop("receipt_binding_sha256", None)
    add(
        failures,
        observed_binding == sha256_bytes(canonical(binding)),
        f"{label}.receipt_binding",
    )
    validate_source(receipt, label, failures)
    validate_artifact(artifact_root, receipt, label, failures)
    add(failures, bool(receipt_raw), f"{label}.receipt_raw")
    return failures


def verify_pair(
    e1_path: Path, e1_root: Path, e2_path: Path, e2_root: Path
) -> tuple[dict[str, Any], list[str]]:
    e1, e1_raw = load(e1_path)
    e2, e2_raw = load(e2_path)
    failures = validate_one(
        e1,
        e1_raw,
        e1_root,
        "E1_LOCAL_EXECUTABLE",
        "prove-primary",
        {"X64", "X86_64", "AMD64"},
    )
    failures.extend(
        validate_one(
            e2,
            e2_raw,
            e2_root,
            "E2_INDEPENDENT_RUNNER",
            "prove-independent",
            {"ARM64", "AARCH64"},
        )
    )

    add(failures, e1.get("candidate") == e2.get("candidate"), "pair.candidate_identity")
    add(
        failures,
        e1.get("repository") == e2.get("repository") == REPOSITORY,
        "pair.repository_identity",
    )
    e1_workflow = e1.get("workflow", {})
    e2_workflow = e2.get("workflow", {})
    add(
        failures,
        e1_workflow.get("path") == e2_workflow.get("path") == WORKFLOW_PATH,
        "pair.workflow_path",
    )
    add(failures, e1_workflow.get("sha") == e2_workflow.get("sha"), "pair.workflow_sha")
    add(
        failures,
        e1_workflow.get("run_id") == e2_workflow.get("run_id"),
        "pair.same_run",
    )
    add(
        failures,
        e1_workflow.get("run_attempt") == e2_workflow.get("run_attempt"),
        "pair.same_attempt",
    )
    add(
        failures, e1_workflow.get("job") != e2_workflow.get("job"), "pair.distinct_jobs"
    )
    add(
        failures,
        e1.get("source_overlay") == e2.get("source_overlay"),
        "pair.source_overlay",
    )
    add(
        failures,
        set(e1.get("blocking_results", {})) == set(e2.get("blocking_results", {})),
        "pair.blocking_surface",
    )
    add(
        failures,
        set(e1.get("diagnostics", {})) == set(e2.get("diagnostics", {})),
        "pair.diagnostic_surface",
    )
    failures = sorted(set(failures))

    candidate = e1.get("candidate") if not failures else None
    source_overlay = e1.get("source_overlay") if not failures else None
    binding_payload = {
        "repository": REPOSITORY,
        "candidate": candidate,
        "workflow_sha": e1_workflow.get("sha"),
        "run_id": e1_workflow.get("run_id"),
        "run_attempt": e1_workflow.get("run_attempt"),
        "e1_receipt_sha256": sha256_bytes(e1_raw),
        "e2_receipt_sha256": sha256_bytes(e2_raw),
        "source_overlay_manifest_sha256": source_overlay.get("manifest_sha256")
        if isinstance(source_overlay, dict)
        else None,
    }
    output = {
        "schema": PAIR_SCHEMA,
        "status": "PASS_Q0_E1_E2_EVIDENCE_PAIR"
        if not failures
        else "FAIL_Q0_E1_E2_EVIDENCE_PAIR",
        **binding_payload,
        "evidence_pair_binding_sha256": sha256_bytes(canonical(binding_payload)),
        "independent_jobs": not failures,
        "independent_architectures": not failures,
        "q0_executable_qualified": not failures,
        "qualified_candidate": not failures,
        "runtime_wired": False,
        "external_effects": False,
        "production_authority": False,
        "operator_acceptance": False,
        "promotion": False,
        "release_authority": False,
        "callers_ratchet": False,
        "failures": failures,
    }
    return output, failures


def write_fake_artifact(root: Path, evidence_class: str, job: str, arch: str) -> Path:
    (root / "gates").mkdir(parents=True, exist_ok=True)
    (root / "rust").mkdir(parents=True, exist_ok=True)
    (root / "gates/source.exit").write_text("0\n")
    (root / "rust/test.exit").write_text("0\n")
    (root / "diagnostic.txt").write_text("diagnostic\n")
    files = []
    for path in sorted(item for item in root.rglob("*") if item.is_file()):
        rel = path.relative_to(root).as_posix()
        if rel in RECEIPT_NAMES:
            continue
        files.append(
            {"path": rel, "sha256": sha256_file(path), "size": path.stat().st_size}
        )
    source_base = {
        "changed_files_sha256": "1" * 64,
        "repair_patch_sha256": "2" * 64,
        "repair_patch_size": 10,
        "repair_stat_sha256": "3" * 64,
        "files": [{"path": "codex-rs/example.rs", "sha256": "4" * 64, "size": 9}],
    }
    source = {**source_base, "manifest_sha256": sha256_bytes(canonical(source_base))}
    blocking = {"gates/source": 0, "rust/test": 0}
    diagnostics: dict[str, int] = {}
    receipt: dict[str, Any] = {
        "schema": SCHEMA,
        "status": "PASS_Q0_E1_LOCAL_EXECUTABLE"
        if evidence_class.startswith("E1")
        else "PASS_Q0_E2_INDEPENDENT_RUNNER",
        "evidence_class": evidence_class,
        "repository": REPOSITORY,
        "candidate": {"head": "a" * 40, "tree": "b" * 40, "parent": "c" * 40},
        "workflow": {
            "name": "Hepta Intelligence Q0 paired candidate v10",
            "path": WORKFLOW_PATH,
            "ref": "x",
            "sha": "a" * 40,
            "run_id": 42,
            "run_attempt": 1,
            "job": job,
        },
        "runner": {
            "name": f"runner-{arch}",
            "os": "Linux",
            "arch": arch,
            "host": "host",
        },
        "blocking_results": blocking,
        "blocking_results_sha256": sha256_bytes(canonical(blocking)),
        "diagnostics": diagnostics,
        "diagnostics_sha256": sha256_bytes(canonical(diagnostics)),
        "all_gates_zero": True,
        "source_overlay": source,
        "artifact_manifest": files,
        "artifact_manifest_sha256": sha256_bytes(canonical(files)),
        "source_writeback": False,
        "qualified_candidate": False,
        "authority": {key: False for key in NEGATIVE_AUTHORITY_KEYS},
    }
    receipt["receipt_binding_sha256"] = sha256_bytes(canonical(receipt))
    name = (
        "e1-qualification-receipt.json"
        if evidence_class.startswith("E1")
        else "e2-qualification-receipt.json"
    )
    path = root / name
    path.write_text(json.dumps(receipt, indent=2, sort_keys=True) + "\n")
    return path


def self_test() -> int:
    def refresh(path: Path, value: dict[str, Any]) -> None:
        value = copy.deepcopy(value)
        value.pop("receipt_binding_sha256", None)
        value["receipt_binding_sha256"] = sha256_bytes(canonical(value))
        path.write_text(json.dumps(value, indent=2, sort_keys=True) + "\n")

    with tempfile.TemporaryDirectory() as temp:
        base = Path(temp)
        e1_root, e2_root = base / "e1", base / "e2"
        e1_path = write_fake_artifact(
            e1_root, "E1_LOCAL_EXECUTABLE", "prove-primary", "X64"
        )
        e2_path = write_fake_artifact(
            e2_root, "E2_INDEPENDENT_RUNNER", "prove-independent", "ARM64"
        )
        output, failures = verify_pair(e1_path, e1_root, e2_path, e2_root)
        assert not failures and output["q0_executable_qualified"] is True

        # Artifact tamper.
        tampered = e2_root / "diagnostic.txt"
        original = tampered.read_bytes()
        tampered.write_bytes(b"tampered\n")
        assert (
            "E2_INDEPENDENT_RUNNER.artifact_sha:diagnostic.txt"
            in verify_pair(e1_path, e1_root, e2_path, e2_root)[1]
        )
        tampered.write_bytes(original)

        # Source-overlay divergence with a valid receipt binding.
        e2 = json.loads(e2_path.read_text())
        e2["source_overlay"]["repair_patch_sha256"] = "9" * 64
        base_source = {
            key: value
            for key, value in e2["source_overlay"].items()
            if key != "manifest_sha256"
        }
        e2["source_overlay"]["manifest_sha256"] = sha256_bytes(canonical(base_source))
        refresh(e2_path, e2)
        assert (
            "pair.source_overlay" in verify_pair(e1_path, e1_root, e2_path, e2_root)[1]
        )
        e2_path = write_fake_artifact(
            e2_root, "E2_INDEPENDENT_RUNNER", "prove-independent", "ARM64"
        )

        # Same-job fake independence.
        e2 = json.loads(e2_path.read_text())
        e2["workflow"]["job"] = "prove-primary"
        refresh(e2_path, e2)
        failures = verify_pair(e1_path, e1_root, e2_path, e2_root)[1]
        assert (
            "E2_INDEPENDENT_RUNNER.job" in failures and "pair.distinct_jobs" in failures
        )
        e2_path = write_fake_artifact(
            e2_root, "E2_INDEPENDENT_RUNNER", "prove-independent", "ARM64"
        )

        # Repository identity drift.
        e2 = json.loads(e2_path.read_text())
        e2["repository"]["owner_id"] += 1
        refresh(e2_path, e2)
        failures = verify_pair(e1_path, e1_root, e2_path, e2_root)[1]
        assert (
            "E2_INDEPENDENT_RUNNER.repository" in failures
            and "pair.repository_identity" in failures
        )

    print("PASS_Q0_EVIDENCE_PAIR_SELFTEST")
    return 0


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--self-test", action="store_true")
    parser.add_argument("--e1", type=Path)
    parser.add_argument("--e1-root", type=Path)
    parser.add_argument("--e2", type=Path)
    parser.add_argument("--e2-root", type=Path)
    parser.add_argument("--output", type=Path)
    args = parser.parse_args()
    if args.self_test:
        return self_test()
    if not all((args.e1, args.e1_root, args.e2, args.e2_root)):
        parser.error("--e1, --e1-root, --e2 and --e2-root are required")
    output, failures = verify_pair(args.e1, args.e1_root, args.e2, args.e2_root)
    encoded = json.dumps(output, indent=2, sort_keys=True) + "\n"
    if args.output:
        args.output.parent.mkdir(parents=True, exist_ok=True)
        args.output.write_text(encoded, encoding="utf-8")
    print(encoded, end="")
    return 0 if not failures else 1


if __name__ == "__main__":
    raise SystemExit(main())
