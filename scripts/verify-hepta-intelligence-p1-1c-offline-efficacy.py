#!/usr/bin/env python3
"""Fail-closed source gate for Hepta Intelligence P1.1c."""

from __future__ import annotations

import csv
import json
import subprocess
import sys
import tomllib
from collections import Counter
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
CRATE = ROOT / "codex-rs/hepta-memory-p1-1c-qualification"
CORPUS = CRATE / "fixtures/p1_1c_multilingual_seed.tsv"
PLAN = ROOT / "plans/hepta-intelligence/P1-1C_MULTILINGUAL_OFFLINE_EFFICACY_PLAN.md"
STATUS = ROOT / "plans/hepta-intelligence/P1-1C_EXECUTION_STATUS.json"
P1B_ROOT = ROOT / "codex-rs/hepta-memory-p1-1b-qualification"
PPM = 1_000_000
EXPECTED_HEADER = [
    "case_id",
    "locale",
    "query",
    "candidate_id",
    "relevance_grade",
    "lexical_ppm",
    "vector_ppm",
    "citation_supported",
    "latency_micros",
    "token_cost",
    "start_node",
    "middle_node",
    "goal_node",
    "edge1_truth_ppm",
    "edge1_contradiction_ppm",
    "edge2_truth_ppm",
    "edge2_contradiction_ppm",
]


def fail(message: str) -> None:
    raise SystemExit(message)


def read(path: Path) -> str:
    if not path.is_file():
        fail(f"missing required path: {path.relative_to(ROOT)}")
    return path.read_text(encoding="utf-8")


def require_markers(path: Path, markers: list[str]) -> None:
    text = read(path)
    missing = [marker for marker in markers if marker not in text]
    if missing:
        fail(
            json.dumps(
                {
                    "path": str(path.relative_to(ROOT)),
                    "missing_markers": missing,
                },
                indent=2,
                sort_keys=True,
            )
        )


def verify_p1b_prerequisite() -> None:
    require_markers(
        P1B_ROOT / "src/index/build.rs",
        [
            "read_bounded_index_bytes",
            "MAX_INDEX_FILE_BYTES.saturating_add(1)",
            "changed while being read or exceeded the bounded limit",
        ],
    )
    require_markers(
        P1B_ROOT / "src/index/impl.rs",
        ["MAX_EMBEDDING_DIMENSIONS", "bucket_count > item_count"],
    )
    require_markers(
        P1B_ROOT / "src/index/tests_module.rs",
        [
            "decode_rejects_oversized_dimensions_before_vector_allocation",
            "bounded_reader_rejects_growth_after_metadata_without_unbounded_read",
        ],
    )


def verify_crate_isolation() -> None:
    cargo = tomllib.loads(read(CRATE / "Cargo.toml"))
    if cargo.get("dependencies"):
        fail("P1.1c qualification crate must remain dependency-free")
    package = cargo.get("package", {})
    if package.get("name") != "hepta-memory-p1-1c-qualification":
        fail("unexpected P1.1c package name")
    workspace = read(ROOT / "codex-rs/Cargo.toml")
    if "hepta-memory-p1-1c-qualification" in workspace:
        fail("P1.1c qualification crate must not join the product workspace")

    forbidden = [
        "reqwest",
        "ureq",
        "hyper::",
        "tokio::net",
        "TcpStream",
        "UdpSocket",
        "Command::new",
        "http://",
        "https://",
        "model_download: true",
        "network_access: true",
        "runtime_wired: true",
        "production_authority: true",
    ]
    violations: dict[str, list[str]] = {}
    for path in sorted(CRATE.rglob("*")):
        if not path.is_file() or path.suffix not in {".rs", ".toml", ".tsv"}:
            continue
        text = path.read_text(encoding="utf-8")
        found = [token for token in forbidden if token in text]
        if found:
            violations[str(path.relative_to(ROOT))] = found
    if violations:
        fail(json.dumps({"forbidden_tokens": violations}, indent=2, sort_keys=True))


def verify_source_contracts() -> None:
    require_markers(
        CRATE / "src/corpus.rs",
        [
            "EXPECTED_COLUMNS: usize = 17",
            "SyntheticSeed",
            "ReviewedHuman",
            "reviewed flag must agree with corpus provenance",
            "MAX_CANDIDATES_PER_CASE",
        ],
    )
    require_markers(
        CRATE / "src/kg.rs",
        [
            "MAX_KG_HOPS: u8 = 2",
            "MAX_GRAPH_NODES",
            "MAX_GRAPH_EDGES",
            "bounded_two_hop",
            "path_sha256",
        ],
    )
    require_markers(
        CRATE / "src/reranker.rs",
        [
            "AblationLane",
            "LexicalVector",
            "LexicalKg",
            "VectorKg",
            "Full",
            "contradiction_penalty_ppm",
            "production_calibrated: false",
        ],
    )
    require_markers(
        CRATE / "src/metrics.rs",
        [
            "recall_at_4_ppm",
            "ndcg_at_4_ppm",
            "citation_precision_ppm",
            "p50_latency_micros",
            "p95_latency_micros",
            "mean_token_cost",
        ],
    )
    require_markers(
        CRATE / "src/evaluation.rs",
        [
            "network_access: false",
            "model_download: false",
            "product_workspace_member: false",
            "runtime_wired: false",
            "efficacy_validation: false",
            "efficacy_claim: false",
            "production_authority: false",
        ],
    )
    require_markers(
        CRATE / "tests/p1_1c.rs",
        [
            "seed_corpus_has_eight_locales_and_forty_eight_candidates",
            "evaluation_is_byte_deterministic",
            "all_seven_ablation_lanes_are_emitted_once",
            "full_lane_outperforms_lexical_and_vector_seed_baselines",
            "receipt_redacts_query_and_kg_node_text",
            "seed_receipt_keeps_all_runtime_and_authority_flags_false",
        ],
    )


def parse_metadata(lines: list[str]) -> dict[str, str]:
    result: dict[str, str] = {}
    for line in lines:
        if not line.startswith("#") or "=" not in line:
            continue
        key, value = line.removeprefix("#").strip().split("=", 1)
        if key in result:
            fail(f"duplicate corpus metadata key: {key}")
        result[key] = value
    return result


def verify_corpus() -> dict[str, object]:
    lines = read(CORPUS).splitlines()
    metadata = parse_metadata(lines)
    expected_metadata = {
        "schema": "hepta.intelligence.p1_1c.offline_efficacy.v1",
        "corpus_id": "p1-1c-multilingual-seed-v1",
        "version": "1",
        "provenance": "synthetic_seed",
        "reviewed": "false",
    }
    if metadata != expected_metadata:
        fail(
            json.dumps(
                {"expected_metadata": expected_metadata, "actual_metadata": metadata},
                indent=2,
                sort_keys=True,
            )
        )
    data_lines = [line for line in lines if line and not line.startswith("#")]
    if not data_lines:
        fail("seed corpus contains no tabular data")
    rows = list(csv.DictReader(data_lines, delimiter="\t"))
    if rows and list(rows[0]) != EXPECTED_HEADER:
        fail("seed corpus header does not match governed 17-column schema")
    if len(rows) != 48:
        fail(f"expected 48 seed candidates, observed {len(rows)}")

    cases = Counter(row["case_id"] for row in rows)
    locales = {row["locale"] for row in rows}
    candidate_ids = [row["candidate_id"] for row in rows]
    if len(cases) != 8 or set(cases.values()) != {6}:
        fail(f"expected 8 cases with 6 candidates each, observed {dict(cases)}")
    if len(locales) != 8:
        fail(f"expected 8 locales, observed {sorted(locales)}")
    if len(candidate_ids) != len(set(candidate_ids)):
        fail("seed corpus candidate IDs must be globally unique")

    ppm_fields = [
        "lexical_ppm",
        "vector_ppm",
        "edge1_truth_ppm",
        "edge1_contradiction_ppm",
        "edge2_truth_ppm",
        "edge2_contradiction_ppm",
    ]
    relevant_cases: set[str] = set()
    for row in rows:
        grade = int(row["relevance_grade"])
        if grade not in range(4):
            fail(f"invalid relevance grade for {row['candidate_id']}")
        if grade > 0:
            relevant_cases.add(row["case_id"])
        for field in ppm_fields:
            value = int(row[field])
            if not 0 <= value <= PPM:
                fail(f"{row['candidate_id']} {field} exceeds bounded ppm")
        if row["citation_supported"] not in {"true", "false"}:
            fail(f"invalid citation flag for {row['candidate_id']}")
        if int(row["latency_micros"]) <= 0 or int(row["token_cost"]) < 0:
            fail(f"invalid cost receipt for {row['candidate_id']}")
        if row["middle_node"] == "-" and (
            int(row["edge2_truth_ppm"]) != 0
            or int(row["edge2_contradiction_ppm"]) != 0
        ):
            fail(f"one-hop candidate {row['candidate_id']} has second-edge values")
    if relevant_cases != set(cases):
        fail("every seed case must contain at least one relevant candidate")
    return {
        "case_count": len(cases),
        "candidate_count": len(rows),
        "locale_count": len(locales),
        "locales": sorted(locales),
    }


def verify_governance() -> None:
    require_markers(
        PLAN,
        [
            "P1.1c",
            "synthetic_seed",
            "efficacy_validation=false",
            "runtime_wired=false",
            "P1.1c.1",
        ],
    )
    status = json.loads(read(STATUS))
    false_paths = [
        ("qualification", "source_qualified"),
        ("qualification", "efficacy_validation"),
        ("authority", "runtime_wired"),
        ("authority", "network_access"),
        ("authority", "production_authority"),
        ("authority", "promotion"),
    ]
    for group, key in false_paths:
        if status[group][key] is not False:
            fail(f"status field {group}.{key} must remain false before qualification")
    if status["corpus"]["provenance"] != "synthetic_seed":
        fail("status corpus provenance must remain synthetic_seed")


def git_head() -> str:
    try:
        return subprocess.check_output(
            ["git", "rev-parse", "HEAD"], cwd=ROOT, text=True
        ).strip()
    except (OSError, subprocess.CalledProcessError):
        return "unavailable"


def main() -> None:
    verify_p1b_prerequisite()
    verify_crate_isolation()
    verify_source_contracts()
    corpus = verify_corpus()
    verify_governance()
    receipt = {
        "schema": "hepta.intelligence.p1_1c.source_gate.v1",
        "status": "PASS_P1_1C_SOURCE_GATE",
        "source_commit": git_head(),
        "crate": str(CRATE.relative_to(ROOT)),
        "corpus": {
            **corpus,
            "provenance": "synthetic_seed",
            "reviewed": False,
        },
        "contracts": {
            "bounded_two_hop_kg": True,
            "fixed_point_scores": True,
            "ablation_lanes": 7,
            "receipt_redaction": True,
            "p1b_bounded_decode_prerequisite": True,
        },
        "source_qualified": False,
        "efficacy_validation": False,
        "efficacy_claim": False,
        "product_workspace_member": False,
        "product_module_registered": False,
        "runtime_wired": False,
        "network_access": False,
        "model_download": False,
        "external_effects": False,
        "production_authority": False,
        "operator_acceptance": False,
        "promotion": False,
        "callers_ratchet": False,
    }
    print(json.dumps(receipt, indent=2, sort_keys=True))


if __name__ == "__main__":
    try:
        main()
    except Exception as error:  # fail closed with a readable message
        print(f"P1.1c source gate failed: {error}", file=sys.stderr)
        raise
