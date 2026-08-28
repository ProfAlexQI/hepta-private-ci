from __future__ import annotations

import json
import os
from pathlib import Path


def main() -> None:
    path = Path(os.environ["RECEIPT"])
    data = json.loads(path.read_text(encoding="utf-8"))
    data["source_candidate_commit"] = os.environ["SOURCE_SHA"]
    data["source_candidate_tree"] = os.environ["SOURCE_TREE"]

    components = data["source_candidate_components"]
    components["inf0c_explicit_format_macro_bindings"] = True
    components["inf0c_package_scoped_rustfmt_repair"] = True

    validation = data["validation"]
    validation["local_python_syntax"] = "PASS"
    validation["local_receipt_json_parse"] = "PASS"
    validation["exact_candidate_source_gate"] = "NOT_RUN_FOR_NEW_RECEIPT_HEAD"
    validation["rust_fmt"] = "PASS_RUST_1_95_MACOS_PACKAGE_AND_FRAGMENT_SCOPE"
    validation["rust_check"] = "PASS_RUST_1_95_MACOS_FOUR_PACKAGE_SCOPE"
    validation["rust_test"] = "PASS_RUST_1_95_MACOS_FOUR_PACKAGE_SCOPE"
    validation["rust_clippy"] = "PASS_RUST_1_95_MACOS_FOUR_PACKAGE_SCOPE_D_WARNINGS"

    run_id = int(os.environ["GITHUB_RUN_ID"])
    history = [entry for entry in data.get("ci_history", []) if entry.get("run_id") != run_id]
    history.append(
        {
            "run_id": run_id,
            "head": os.environ["GITHUB_SHA"],
            "runner_assigned": True,
            "runner_os": os.environ.get("RUNNER_OS", "macOS"),
            "runner_arch": os.environ.get("RUNNER_ARCH", "unknown"),
            "exact_input_head": os.environ["EXPECTED_HEAD"],
            "source_gate_full_history_diagnostic": "PASS_RUN_33174647316",
            "reference_format": "PASS",
            "fragment_format": "PASS",
            "package_format": "PASS_AFTER_REPAIR",
            "package_check": "PASS_AFTER_REPAIR",
            "package_test": "PASS_AFTER_REPAIR",
            "package_clippy_d_warnings": "PASS_AFTER_REPAIR",
            "classification": "EXECUTED_BOUNDED_SOURCE_REPAIR",
        }
    )
    data["ci_history"] = history
    data["current_ci"] = {
        "status": "NOT_RUN_FOR_RECEIPT_HEAD",
        "runner_assigned": False,
        "source_gate": "NOT_RUN_ON_HOSTED_RUNNER_FOR_RECEIPT_HEAD",
        "reference_contract": "NOT_RUN_ON_HOSTED_RUNNER_FOR_RECEIPT_HEAD",
        "compatibility_readiness": "NOT_RUN_ON_HOSTED_RUNNER_FOR_RECEIPT_HEAD",
        "real_software_e2e": "NOT_RUN",
    }
    data["claim"] = "SOURCE_PRESENT_NOT_RUN"
    data["qualified"] = False
    data["known_gaps"] = [
        "the new exact receipt head must execute the canonical source, package formatting, test, check and Clippy matrix on its hosted qualification workflow",
        "the canonical workflow still needs its formatting responsibility narrowed from unrelated whole-workspace drift to the owned INF-0C package and fragment scope",
        "the real-software E2E harness has not executed against fixed pre-installed Ollama and LM Studio tuples",
        "semantic, tool-call, disconnect, backend cancellation acknowledgement and controlled-restart evidence remain in stacked qualification lanes",
        "hepta-inferd, the UDS protocol, native workers and hardware performance receipts are absent",
        "INF-1 remains inactive and all production, effect, model-NPU, remote-inference and promotion authority remains closed",
    ]
    path.write_text(json.dumps(data, indent=2) + "\n", encoding="utf-8")


if __name__ == "__main__":
    main()
