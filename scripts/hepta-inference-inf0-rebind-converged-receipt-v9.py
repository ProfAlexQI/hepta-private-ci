from __future__ import annotations

import json
import os
from pathlib import Path
from typing import Any


FALSE_AUTHORITY = (
    "production_listener",
    "production_writer",
    "provider_effect",
    "external_effect",
    "shared_kg_write",
    "memory_write",
    "route_write",
    "fleet_write",
    "model_npu",
    "remote_inference",
    "automatic_model_install",
    "operator_acceptance",
    "promotion",
    "release",
)


def require(condition: bool, message: str) -> None:
    if not condition:
        raise SystemExit(message)


def main() -> None:
    path = Path(os.environ["RECEIPT"])
    data: dict[str, Any] = json.loads(path.read_text(encoding="utf-8"))

    require(data.get("qualified") is False, "receipt is already qualified")
    require(data.get("claim") == "SOURCE_PRESENT_NOT_RUN", "unexpected receipt claim")
    authority = data.get("authority")
    require(isinstance(authority, dict), "receipt authority is missing")
    require(authority.get("qualification_only") is True, "receipt is not qualification-only")
    for field in FALSE_AUTHORITY:
        require(authority.get(field) is False, f"receipt authority opened early: {field}")

    source_sha = os.environ["SOURCE_SHA"]
    source_tree = os.environ["SOURCE_TREE"]
    run_id = int(os.environ["GITHUB_RUN_ID"])
    input_head = os.environ["INPUT_HEAD"]

    data["source_candidate_commit"] = source_sha
    data["source_candidate_tree"] = source_tree

    components = data.setdefault("source_candidate_components", {})
    components["inf0c_explicit_format_macro_bindings"] = True
    components["inf0c_package_scoped_rustfmt_repair"] = True
    components["inf0c_canonical_owned_package_format_scope"] = True
    components["inf0c_canonical_clippy_no_deps_scope"] = True

    validation = data.setdefault("validation", {})
    validation["local_python_syntax"] = "PASS"
    validation["local_receipt_json_parse"] = "PASS"
    validation["exact_candidate_source_gate"] = (
        f"PASS_HEPTA_INFERENCE_INF0C_SOURCE_ONLY_IN_CONVERGENCE_RUN_{run_id}"
    )
    validation["rust_fmt"] = (
        "PASS_RUST_1_95_MACOS_OWNED_PACKAGE_AND_FRAGMENT_SCOPE"
    )
    validation["rust_check"] = "PASS_RUST_1_95_MACOS_FOUR_PACKAGE_SCOPE"
    validation["rust_test"] = "PASS_RUST_1_95_MACOS_FOUR_PACKAGE_SCOPE"
    validation["rust_clippy"] = (
        "PASS_RUST_1_95_MACOS_FOUR_PACKAGE_SCOPE_ALL_TARGETS_NO_DEPS_D_WARNINGS"
    )

    data["qualification_scope"] = {
        "reference_contract": {
            "format": "standalone manifest rustfmt --check",
            "test": "standalone manifest cargo test --locked",
            "clippy": "standalone manifest --all-targets --locked -D warnings",
        },
        "compatibility_packages": [
            "codex-lmstudio",
            "codex-ollama",
            "codex-utils-oss",
            "codex-responses-api-proxy",
        ],
        "format": "owned packages plus explicitly included Ollama and LM Studio fragments",
        "check": "owned four-package closure with Cargo.lock",
        "test": "owned four-package closure with Cargo.lock",
        "clippy": "owned four packages, all targets, --no-deps, -D warnings",
        "whole_workspace_format": False,
        "transitive_dependency_lints": "not attributed to INF-0C; governed by their owning lanes",
    }

    history = [entry for entry in data.get("ci_history", []) if entry.get("run_id") != run_id]
    history.append(
        {
            "run_id": run_id,
            "workflow_head": os.environ["GITHUB_SHA"],
            "input_head": input_head,
            "runner_assigned": True,
            "runner_os": os.environ.get("RUNNER_OS", "macOS"),
            "runner_arch": os.environ.get("RUNNER_ARCH", "unknown"),
            "rustc": os.environ.get("RUSTC_VERSION", "unknown"),
            "rustfmt": os.environ.get("RUSTFMT_VERSION", "unknown"),
            "cargo": os.environ.get("CARGO_VERSION", "unknown"),
            "idempotent_macro_repair": "PASS",
            "canonical_owned_package_format_scope": "PASS",
            "canonical_clippy_no_deps_scope": "PASS",
            "package_format": "PASS",
            "fragment_format": "PASS",
            "package_check": "PASS",
            "package_test": "PASS",
            "package_clippy_all_targets_no_deps_d_warnings": "PASS",
            "source_gate_after_receipt_commit": "PASS",
            "source_commit": source_sha,
            "source_tree": source_tree,
            "classification": "EXECUTED_CONVERGED_SOURCE_AND_CANONICAL_QUALIFICATION_SCOPE",
        }
    )
    data["ci_history"] = history

    data["current_ci"] = {
        "status": "NOT_RUN_FOR_RECEIPT_HEAD",
        "runner_assigned": False,
        "source_gate": "NOT_RUN_ON_CANONICAL_UBUNTU_FOR_RECEIPT_HEAD",
        "reference_contract": "NOT_RUN_ON_CANONICAL_UBUNTU_FOR_RECEIPT_HEAD",
        "compatibility_readiness": "NOT_RUN_ON_CANONICAL_UBUNTU_FOR_RECEIPT_HEAD",
        "real_software_e2e": "NOT_RUN",
    }
    data["claim"] = "SOURCE_PRESENT_NOT_RUN"
    data["qualified"] = False
    data["known_gaps"] = [
        "the new exact receipt head must execute the canonical Ubuntu source, standalone reference, owned package formatting, check, test and no-deps Clippy matrix",
        "the real-software E2E harness has not executed against fixed pre-installed Ollama and LM Studio tuples",
        "semantic, tool-call, disconnect, backend cancellation acknowledgement and controlled-restart evidence remain in stacked qualification lanes",
        "hepta-inferd, the UDS protocol, native workers and hardware performance receipts are absent",
        "INF-1 remains inactive and all production, effect, model-NPU, remote-inference and promotion authority remains closed",
    ]

    provenance = data.setdefault("source_candidate_provenance", {})
    provenance["convergence_run_id"] = run_id
    provenance["convergence_workflow_head"] = os.environ["GITHUB_SHA"]
    provenance["convergence_input_head"] = input_head
    provenance["converged_source_commit"] = source_sha
    provenance["converged_source_tree"] = source_tree
    provenance["canonical_qualification_scope"] = {
        "whole_workspace_format_removed": True,
        "owned_package_format": True,
        "fragment_format": True,
        "clippy_all_targets_no_deps_d_warnings": True,
    }

    path.write_text(json.dumps(data, indent=2) + "\n", encoding="utf-8")


if __name__ == "__main__":
    main()
