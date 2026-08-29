#!/usr/bin/env python3
"""Fail-closed source verifier for Hepta P0.6 runtime authority closure."""

from __future__ import annotations

import json
import pathlib
import subprocess
import sys
from typing import Any, NoReturn

ROOT = pathlib.Path(__file__).resolve().parents[1]
STATUS = ROOT / "docs/architecture/HEPTA_QUALIFICATION_STATUS_V2.json"
LEDGER = ROOT / "docs/architecture/HEPTA_ARCHITECTURE_GAP_LEDGER_V2.json"
WORKFLOW = ROOT / ".github/workflows/hepta-gap-closure-p0-6.yml"


def fail(message: str) -> NoReturn:
    raise SystemExit(f"FAIL_HEPTA_P0_6_RUNTIME_AUTHORITY: {message}")


def read(relative: str) -> str:
    path = ROOT / relative
    try:
        return path.read_text(encoding="utf-8")
    except (OSError, UnicodeError) as error:
        fail(f"cannot read {relative}: {error}")


def load_json(path: pathlib.Path) -> dict[str, Any]:
    def pairs_hook(pairs: list[tuple[str, Any]]) -> dict[str, Any]:
        value: dict[str, Any] = {}
        for key, item in pairs:
            if key in value:
                fail(f"duplicate JSON key {key!r} in {path.relative_to(ROOT)}")
            value[key] = item
        return value

    try:
        value = json.loads(path.read_text(encoding="utf-8"), object_pairs_hook=pairs_hook)
    except (OSError, UnicodeError, json.JSONDecodeError) as error:
        fail(f"cannot parse {path.relative_to(ROOT)}: {error}")
    if not isinstance(value, dict):
        fail(f"{path.relative_to(ROOT)} must contain one JSON object")
    return value


def compact_source(value: str) -> str:
    """Return a formatting-insensitive representation for source contracts."""
    return "".join(value.split())


def contains_marker(source: str, marker: str) -> bool:
    return marker in source or compact_source(marker) in compact_source(source)


def require(relative: str, markers: tuple[str, ...]) -> str:
    source = read(relative)
    for marker in markers:
        if not contains_marker(source, marker):
            fail(f"{relative} is missing {marker!r}")
    return source


def require_absent(relative: str, markers: tuple[str, ...]) -> None:
    source = read(relative)
    for marker in markers:
        if contains_marker(source, marker):
            fail(f"{relative} contains forbidden {marker!r}")


def verify_base_closure() -> None:
    result = subprocess.run(
        [sys.executable, "scripts/verify-hepta-p0-5-gap-closure.py"],
        cwd=ROOT,
        text=True,
        check=False,
    )
    if result.returncode != 0:
        fail("P0.5 normative architecture closure no longer verifies")


def verify_contracts() -> None:
    require(
        "codex-rs/hepta-contracts/src/runtime_authority.rs",
        (
            "pub struct RuntimeAuthorityContext",
            "pub trait CapabilityUseVerifier",
            "pub fn verify_capability_use",
            "runtime authority context does not match the local grant",
            "verifier.verify_use(&CapabilityUseVerificationRequest {",
            "external_capability_is_reverified_on_every_use_and_revocation_fails_closed",
            "changed_epoch_fence_and_expiry_are_rejected_before_use_verifier",
        ),
    )
    require(
        "codex-rs/hepta-contracts/src/runtime_instance.rs",
        (
            "pub struct RuntimeInstanceGraph",
            "pub enum RuntimeServiceRequirement",
            "pub enum RuntimeServiceState",
            "pub fn mark_ready",
            "local_instance_distinguishes_optional_degradation_from_readiness",
            "qualification_instance_requires_memory",
        ),
    )
    require(
        "codex-rs/hepta-contracts/src/checked_provider_operation.rs",
        (
            "pub struct ProviderOperationCoordinator<A, V>",
            "V: CapabilityUseVerifier",
            "self.verify_now(observed_at_unix_seconds)?;",
            "verify_capability_use(",
        ),
    )
    lib = require(
        "codex-rs/hepta-contracts/src/lib.rs",
        (
            "mod checked_provider_operation;",
            "mod runtime_authority;",
            "mod runtime_instance;",
            "pub use checked_provider_operation::ProviderOperationCoordinator;",
            "pub use runtime_authority::CapabilityUseVerifier;",
            "pub use runtime_instance::RuntimeInstanceGraph;",
        ),
    )
    if "pub use provider_operation::ProviderOperationCoordinator;" in lib:
        fail("unchecked provider coordinator remains publicly exported")


def verify_agentd_wiring() -> None:
    require(
        "codex-rs/hepta-agentd/src/composition.rs",
        (
            "RuntimeProfileContract::for_authority(&authority)",
            "runtime_authority_context(&record, &identity, &authority)",
            "let authority_epoch = record",
            ".release_state\n        .generation",
            "let owner_epoch = record.lifecycle.generation;",
            "runtime_fencing_token(record, identity, authority)",
            "RuntimeInstanceGraph::agent_composed(",
            "&runtime_authority,",
        ),
    )
    require(
        "codex-rs/hepta-agentd/src/memory_service.rs",
        (
            "runtime_authority: &RuntimeAuthorityContext",
            "runtime_authority.validate_grant(authority)",
        ),
    )
    require(
        "codex-rs/hepta-agentd/src/automation_service.rs",
        (
            "runtime_authority: &RuntimeAuthorityContext",
            "runtime_authority.authority_epoch()",
            "runtime_authority.owner_epoch()",
            "runtime_authority.generation()",
            "runtime_authority.fencing_token_sha256().clone()",
            "run_automation_scheduler_with_context(",
        ),
    )
    require(
        "codex-rs/hepta-agentd/src/runtime.rs",
        (
            "Arc<Mutex<RuntimeInstanceGraph>>",
            "monitor_runtime_with_instance(",
            ".mark_ready(ProductComponentId::AppServer)",
            "if !instance.ready()",
        ),
    )
    require(
        "codex-rs/hepta-agentd/src/production_writer_host.rs",
        (
            "effect_runtime_authority: Option<RuntimeAuthorityContext>",
            "pub async fn dispatch<V>(",
            "V: CapabilityUseVerifier + ?Sized",
            "verify_capability_use(",
            "production external-effect use rejected",
            "effect_binding.fencing_token_sha256() != cognitive_binding.fencing_token_sha256()",
        ),
    )
    require_absent(
        "codex-rs/hepta-agentd/src/automation_service.rs",
        (
            "u64::from(authority.schema_version())",
            "identity.spawn_generation,\n            identity.spawn_generation,\n            authority.digest()",
        ),
    )


def verify_status_and_ledger() -> None:
    status = load_json(STATUS)
    if status.get("schema") != "hepta.qualification-status.v2":
        fail("qualification status schema drifted")
    subject = status.get("subject")
    if not isinstance(subject, dict) or subject.get("sourceBranch") != (
        "codex/hepta-gap-closure-p0-6-20260829"
    ):
        fail("qualification status is not bound to the P0.6 source branch")
    if status.get("claimLevel") != "source_present_unqualified":
        fail("source-controlled status overclaims executable qualification")
    qualification = status.get("qualification")
    if not isinstance(qualification, dict) or not qualification:
        fail("qualification map is missing")
    for gate, record in qualification.items():
        if not isinstance(record, dict) or record.get("state") != "not_run":
            fail(f"source-controlled gate {gate} must remain not_run")
    decisions = status.get("externalDecisions")
    if not isinstance(decisions, dict) or any(
        not isinstance(record, dict) or record.get("state") != "not_issued"
        for record in decisions.values()
    ):
        fail("external decisions must remain not_issued")
    authority = status.get("authorityBoundary")
    if not isinstance(authority, dict) or any(authority.values()):
        fail("qualification status widened runtime authority")

    ledger = load_json(LEDGER)
    if ledger.get("schema") != "hepta.architecture-gap-ledger.v2":
        fail("gap ledger schema drifted")
    subject = ledger.get("subject")
    if not isinstance(subject, dict) or subject.get("branch") != (
        "codex/hepta-gap-closure-p0-6-20260829"
    ):
        fail("gap ledger is not bound to the P0.6 source branch")
    closed = ledger.get("closedSourceGaps")
    if not isinstance(closed, dict):
        fail("closedSourceGaps is missing")
    for key in (
        "runtimeAuthorityContext",
        "lifecycleEpochAndFenceBinding",
        "runtimeInstanceReadinessGraph",
        "perUseExternalEffectVerification",
        "repositoryIdentityAndGovernanceContract",
    ):
        if closed.get(key, {}).get("state") != "source_implemented":
            fail(f"P0.6 source gap is not closed in ledger: {key}")


def verify_workflow() -> None:
    workflow = read(".github/workflows/hepta-gap-closure-p0-6.yml")
    for marker in (
        "name: Hepta gap closure P0.6",
        "permissions:\n  contents: read",
        "codex/hepta-gap-closure-p0-6-20260829",
        "Exact source-head P0.6 closure",
        "Merge-candidate P0.6 closure",
        "Hepta P0.6 gap closure required",
        "python3 scripts/verify-hepta-p0-6-runtime-authority.py",
        "cargo test --locked -p codex-hepta-contracts runtime_authority::tests",
        "cargo test --locked -p codex-hepta-contracts runtime_instance::tests",
        "cargo test --locked -p codex-hepta-agentd composition::tests",
        "cargo clippy --locked --all-targets -p codex-hepta-contracts -p codex-hepta-agentd -- -D warnings",
    ):
        if marker not in workflow:
            fail(f"P0.6 workflow is missing {marker!r}")
    for forbidden in (
        "contents: write",
        "persist-credentials: true",
        "git push",
        "git commit",
        "git update-ref",
    ):
        if forbidden in workflow:
            fail(f"P0.6 qualification workflow contains mutation path: {forbidden}")


def main() -> int:
    required = (
        STATUS,
        LEDGER,
        WORKFLOW,
        ROOT / "codex-rs/hepta-contracts/src/runtime_authority.rs",
        ROOT / "codex-rs/hepta-contracts/src/runtime_instance.rs",
        ROOT / "codex-rs/hepta-contracts/src/checked_provider_operation.rs",
    )
    missing = [str(path.relative_to(ROOT)) for path in required if not path.is_file()]
    if missing:
        fail(f"required P0.6 files are absent: {missing}")
    verify_base_closure()
    verify_contracts()
    verify_agentd_wiring()
    verify_status_and_ledger()
    verify_workflow()
    print("PASS_HEPTA_P0_6_RUNTIME_AUTHORITY_SOURCE_ONLY")
    return 0


if __name__ == "__main__":
    sys.exit(main())
