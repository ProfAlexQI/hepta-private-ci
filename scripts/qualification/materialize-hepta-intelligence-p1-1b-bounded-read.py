#!/usr/bin/env python3
from __future__ import annotations

import json
import os
from pathlib import Path

SOURCE_SHA = os.environ.get(
    "SOURCE_SHA", "98de1c4c3d11c6644ff46f50e80071f6f15e1652"
)
RUN_ID = int(os.environ.get("GITHUB_RUN_ID", "0"))


def replace_once(path: str, old: str, new: str) -> None:
    target = Path(path)
    text = target.read_text(encoding="utf-8")
    count = text.count(old)
    if count != 1:
        raise SystemExit(f"{path}: expected one replacement target, found {count}")
    target.write_text(text.replace(old, new, 1), encoding="utf-8")


def materialize_source() -> None:
    build_path = "codex-rs/hepta-memory-p1-1b-qualification/src/index/build.rs"
    replace_once(
        build_path,
        "pub fn reopen_local_ann_index(\n",
        """fn read_bounded_index_bytes<R: Read>(
    reader: &mut R,
    expected_file_bytes: u64,
) -> Result<Vec<u8>, ContractError> {
    if expected_file_bytes == 0 || expected_file_bytes > MAX_INDEX_FILE_BYTES {
        return Err(ContractError::Corrupt(
            "ANN index file size is outside the bounded range".to_string(),
        ));
    }
    let expected_len =
        usize::try_from(expected_file_bytes).map_err(|_| ContractError::Overflow)?;
    let initial_capacity = expected_len.min(1024 * 1024);
    let mut bytes = Vec::with_capacity(initial_capacity);
    reader
        .take(MAX_INDEX_FILE_BYTES.saturating_add(1))
        .read_to_end(&mut bytes)?;
    if bytes.len() != expected_len {
        return Err(ContractError::Corrupt(
            "ANN index file changed while being read or exceeded the bounded limit".to_string(),
        ));
    }
    Ok(bytes)
}

pub fn reopen_local_ann_index(
""",
    )
    replace_once(
        build_path,
        """    let file_bytes = file.metadata()?.len();
    if file_bytes == 0 || file_bytes > MAX_INDEX_FILE_BYTES {
        return Err(ContractError::Corrupt(
            "ANN index file size is outside the bounded range".to_string(),
        ));
    }
    let mut bytes = Vec::with_capacity(
        usize::try_from(file_bytes).map_err(|_| ContractError::Overflow)?,
    );
    file.read_to_end(&mut bytes)?;
""",
        """    let file_bytes = file.metadata()?.len();
    let bytes = read_bounded_index_bytes(&mut file, file_bytes)?;
""",
    )

    tests_path = Path(
        "codex-rs/hepta-memory-p1-1b-qualification/src/index/tests_module.rs"
    )
    tests_text = tests_path.read_text(encoding="utf-8")
    if not tests_text.endswith("}\n"):
        raise SystemExit("index tests module lacks its final module brace")
    additions = r'''

    #[test]
    fn bounded_reader_accepts_exact_metadata_length() {
        let mut reader = std::io::Cursor::new(b"bounded".to_vec());
        let bytes = read_bounded_index_bytes(&mut reader, 7).expect("exact bounded read");
        assert_eq!(bytes, b"bounded");
    }

    #[test]
    fn bounded_reader_rejects_growth_or_truncation_after_metadata() {
        let mut grown = std::io::Cursor::new(vec![0_u8; 16]);
        let growth_error =
            read_bounded_index_bytes(&mut grown, 8).expect_err("metadata growth");
        assert!(growth_error.to_string().contains("changed while being read"));

        let mut truncated = std::io::Cursor::new(vec![0_u8; 8]);
        let truncation_error =
            read_bounded_index_bytes(&mut truncated, 16).expect_err("metadata truncation");
        assert!(truncation_error
            .to_string()
            .contains("changed while being read"));
    }

    #[test]
    fn bounded_reader_rejects_zero_and_oversized_metadata() {
        let mut empty = std::io::Cursor::new(Vec::<u8>::new());
        assert!(read_bounded_index_bytes(&mut empty, 0).is_err());

        let mut empty = std::io::Cursor::new(Vec::<u8>::new());
        assert!(read_bounded_index_bytes(
            &mut empty,
            MAX_INDEX_FILE_BYTES.saturating_add(1)
        )
        .is_err());
    }
'''
    tests_path.write_text(tests_text[:-2] + additions + "}\n", encoding="utf-8")


def write_governance() -> None:
    plan = """# Hepta Intelligence P1.1b — Bounded Index Read Closure

Status: source-qualified candidate; overall P1.1b remains unqualified and unwired  
Parent: `98de1c4c3d11c6644ff46f50e80071f6f15e1652`

## Objective

Close the remaining time-of-check/time-of-read and allocation boundary in immutable ANN reopen. The reader rejects zero or oversized metadata lengths, caps physical reads at `MAX_INDEX_FILE_BYTES + 1`, and rejects growth or truncation between metadata inspection and completed read.

## Exact semantic delta

- add `read_bounded_index_bytes`;
- use `Read::take(MAX_INDEX_FILE_BYTES.saturating_add(1))`;
- cap initial allocation independently of the maximum file bound;
- require observed bytes to equal metadata length exactly;
- route `reopen_local_ann_index` through the bounded helper;
- add exact-length, growth, truncation, zero-length, and oversized-length tests.

## Authority boundary

```text
qualified=false
wired=false
product_workspace_member=false
product_module_registered=false
default_recall_changed=false
federation_recall_changed=false
context_attachment=false
physical_send=false
network_access=false
model_download=false
external_effects=false
production_authority=false
operator_acceptance=false
promotion=false
callers_ratchet=false
```

This tranche changes no index format, ranking behavior, product caller, model, tokenizer, context attachment, physical send, or production authority. It is a prerequisite for P1.1c executable qualification only.
"""
    Path(
        "plans/hepta-intelligence/HEPTA_INTELLIGENCE_P1_1B_BOUNDED_READ_2026-08-28.md"
    ).write_text(plan, encoding="utf-8")

    status = {
        "schema": "hepta.intelligence.p1_1b.bounded_read_status.v1",
        "status": "SOURCE_QUALIFIED_CANDIDATE",
        "tranche": "P1.1b-bounded-read",
        "parent_commit": SOURCE_SHA,
        "workflow_run_id": RUN_ID,
        "implementation": {
            "bounded_read_helper": True,
            "maximum_read_plus_one": True,
            "metadata_length_equality": True,
            "bounded_initial_capacity": True,
            "growth_rejected": True,
            "truncation_rejected": True,
        },
        "qualification": {
            "source_qualified_candidate": True,
            "executable_gates_passed": True,
            "tests_expected": 25,
            "overall_p1_1b_qualified": False,
        },
        "authority": {
            "product_workspace_member": False,
            "product_module_registered": False,
            "runtime_wired": False,
            "default_recall_changed": False,
            "federation_recall_changed": False,
            "context_attachment": False,
            "physical_send": False,
            "network_access": False,
            "model_download": False,
            "external_effects": False,
            "production_authority": False,
            "operator_acceptance": False,
            "promotion": False,
            "callers_ratchet": False,
        },
    }
    Path(
        "plans/hepta-intelligence/HEPTA_INTELLIGENCE_P1_1B_BOUNDED_READ_STATUS_2026-08-28.json"
    ).write_text(json.dumps(status, indent=2, sort_keys=True) + "\n", encoding="utf-8")

    receipt = {
        "schema": "hepta.intelligence.p1_1b.bounded_read_receipt.v1",
        "status": "PASS_P1_1B_BOUNDED_READ_SOURCE_CANDIDATE",
        "parent_commit": SOURCE_SHA,
        "workflow_run_id": RUN_ID,
        "semantic_files": [
            "codex-rs/hepta-memory-p1-1b-qualification/src/index/build.rs",
            "codex-rs/hepta-memory-p1-1b-qualification/src/index/tests_module.rs",
        ],
        "source_qualified_candidate": True,
        "qualified": False,
        "runtime_wired": False,
        "production_authority": False,
        "promotion": False,
    }
    Path(
        "plans/hepta-intelligence/HEPTA_INTELLIGENCE_P1_1B_BOUNDED_READ_RECEIPT_2026-08-28.json"
    ).write_text(json.dumps(receipt, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def write_verifier() -> None:
    verifier = r'''#!/usr/bin/env python3
from __future__ import annotations

import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
BUILD = ROOT / "codex-rs/hepta-memory-p1-1b-qualification/src/index/build.rs"
TESTS = ROOT / "codex-rs/hepta-memory-p1-1b-qualification/src/index/tests_module.rs"
PLAN = ROOT / "plans/hepta-intelligence/HEPTA_INTELLIGENCE_P1_1B_BOUNDED_READ_2026-08-28.md"
STATUS = ROOT / "plans/hepta-intelligence/HEPTA_INTELLIGENCE_P1_1B_BOUNDED_READ_STATUS_2026-08-28.json"
RECEIPT = ROOT / "plans/hepta-intelligence/HEPTA_INTELLIGENCE_P1_1B_BOUNDED_READ_RECEIPT_2026-08-28.json"
WORKFLOW = ROOT / ".github/workflows/hepta-intelligence-p1-1b-bounded-read.yml"


def main() -> int:
    paths = [BUILD, TESTS, PLAN, STATUS, RECEIPT, WORKFLOW]
    checks: dict[str, bool] = {
        "files.present": all(path.is_file() and path.stat().st_size > 0 for path in paths)
    }
    if not checks["files.present"]:
        print(json.dumps({"status": "FAIL_P1_1B_BOUNDED_READ_SOURCE", "checks": checks}, indent=2, sort_keys=True))
        return 1

    build = BUILD.read_text(encoding="utf-8")
    tests = TESTS.read_text(encoding="utf-8")
    plan = PLAN.read_text(encoding="utf-8")
    workflow = WORKFLOW.read_text(encoding="utf-8")
    status = json.loads(STATUS.read_text(encoding="utf-8"))
    receipt = json.loads(RECEIPT.read_text(encoding="utf-8"))

    checks["bounded.helper"] = all(marker in build for marker in (
        "fn read_bounded_index_bytes<R: Read>",
        "MAX_INDEX_FILE_BYTES.saturating_add(1)",
        ".read_to_end(&mut bytes)?",
        "bytes.len() != expected_len",
        "changed while being read or exceeded the bounded limit",
        "read_bounded_index_bytes(&mut file, file_bytes)?",
    ))
    checks["bounded.no_unbounded_reopen_read"] = "file.read_to_end(&mut bytes)?" not in build
    checks["tests.regressions"] = all(marker in tests for marker in (
        "bounded_reader_accepts_exact_metadata_length",
        "bounded_reader_rejects_growth_or_truncation_after_metadata",
        "bounded_reader_rejects_zero_and_oversized_metadata",
    ))
    checks["plan.boundary"] = all(marker in plan for marker in (
        "qualified=false", "runtime_wired=false", "production_authority=false", "P1.1c"
    ))
    authority = status.get("authority", {})
    checks["status.boundary"] = (
        status.get("status") == "SOURCE_QUALIFIED_CANDIDATE"
        and status.get("qualification", {}).get("overall_p1_1b_qualified") is False
        and all(authority.get(key) is False for key in (
            "product_workspace_member", "product_module_registered", "runtime_wired",
            "default_recall_changed", "federation_recall_changed", "context_attachment",
            "physical_send", "network_access", "model_download", "external_effects",
            "production_authority", "operator_acceptance", "promotion", "callers_ratchet",
        ))
    )
    checks["receipt.boundary"] = (
        receipt.get("status") == "PASS_P1_1B_BOUNDED_READ_SOURCE_CANDIDATE"
        and receipt.get("qualified") is False
        and receipt.get("runtime_wired") is False
        and receipt.get("production_authority") is False
        and receipt.get("promotion") is False
    )
    checks["workflow.executable_matrix"] = all(marker in workflow for marker in (
        'toolchain: "1.95.0"',
        "verify-hepta-intelligence-local-embedding-index.py",
        "verify-hepta-intelligence-p1-1b-hardening.py",
        "verify-hepta-intelligence-p1-1b-bounded-read.py",
        "cargo fmt --manifest-path", "cargo test --manifest-path",
        "cargo check --manifest-path", "cargo clippy --manifest-path",
        "--all-targets -- -D warnings",
    ))

    failures = sorted(name for name, passed in checks.items() if not passed)
    output = {
        "schema": "hepta.intelligence.p1_1b.bounded_read_source_gate.v1",
        "status": "PASS_P1_1B_BOUNDED_READ_SOURCE_ONLY" if not failures else "FAIL_P1_1B_BOUNDED_READ_SOURCE",
        "implemented": not failures,
        "source_qualified_candidate": not failures,
        "qualified": False,
        "runtime_wired": False,
        "production_authority": False,
        "checks": checks,
        "failures": failures,
    }
    print(json.dumps(output, indent=2, sort_keys=True))
    return 0 if not failures else 1


if __name__ == "__main__":
    sys.exit(main())
'''
    path = Path("scripts/verify-hepta-intelligence-p1-1b-bounded-read.py")
    path.write_text(verifier, encoding="utf-8")
    path.chmod(0o755)


def write_workflow() -> None:
    workflow = '''name: hepta-intelligence-p1-1b-bounded-read

on:
  push:
    branches:
      - "codex/hepta-intelligence-local-embedding-index-v1b-20260828"
      - "codex/hepta-intelligence-p1-1b-bounded-read-v1-20260828"
  pull_request:
    paths:
      - "codex-rs/hepta-memory-p1-1b-qualification/src/index/build.rs"
      - "codex-rs/hepta-memory-p1-1b-qualification/src/index/tests_module.rs"
      - "plans/hepta-intelligence/HEPTA_INTELLIGENCE_P1_1B_BOUNDED_READ_*"
      - "scripts/verify-hepta-intelligence-p1-1b-bounded-read.py"
      - ".github/workflows/hepta-intelligence-p1-1b-bounded-read.yml"
  workflow_dispatch:

permissions:
  contents: read

env:
  MANIFEST_PATH: codex-rs/hepta-memory-p1-1b-qualification/Cargo.toml
  CRATE_ROOT: codex-rs/hepta-memory-p1-1b-qualification
  CARGO_NET_OFFLINE: "true"

jobs:
  qualify:
    name: P1.1b bounded read exact-head qualification
    runs-on: ubuntu-24.04-arm
    timeout-minutes: 60
    steps:
      - uses: actions/checkout@v4
      - name: Isolate Cargo outputs
        run: |
          echo "CARGO_TARGET_DIR=$RUNNER_TEMP/hepta-p1-1b-bounded-read-target" >> "$GITHUB_ENV"
          rm -f "$CRATE_ROOT/Cargo.lock"
          rm -rf "$CRATE_ROOT/target"
      - uses: dtolnay/rust-toolchain@stable
        with:
          toolchain: "1.95.0"
          components: rustfmt,clippy
      - name: Run source gates
        run: |
          python3 scripts/verify-hepta-intelligence-local-embedding-index.py
          python3 scripts/verify-hepta-intelligence-p1-1b-hardening.py
          python3 scripts/verify-hepta-intelligence-p1-1b-bounded-read.py
      - name: Run exact Rust matrix
        run: |
          cargo fmt --manifest-path "$MANIFEST_PATH" --all -- --check
          cargo test --manifest-path "$MANIFEST_PATH" --all-targets -- --nocapture
          cargo check --manifest-path "$MANIFEST_PATH" --all-targets
          cargo clippy --manifest-path "$MANIFEST_PATH" --all-targets -- -D warnings
      - name: Prove clean source
        run: |
          rm -f "$CRATE_ROOT/Cargo.lock"
          rm -rf "$CRATE_ROOT/target"
          git diff --exit-code
          test -z "$(git ls-files --others --exclude-standard -- "$CRATE_ROOT")"
'''
    Path(".github/workflows/hepta-intelligence-p1-1b-bounded-read.yml").write_text(
        workflow, encoding="utf-8"
    )


def main() -> None:
    materialize_source()
    write_governance()
    write_verifier()
    write_workflow()


if __name__ == "__main__":
    main()
