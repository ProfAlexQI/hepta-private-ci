#!/usr/bin/env python3
"""Version-controlled runner qualification and recovery contracts."""

from __future__ import annotations

from .common import *  # noqa: F401,F403

def verify_policy(path: pathlib.Path = POLICY_PATH) -> dict[str, Any]:
    if not path.is_file():
        fail(f"missing runner qualification policy: {path.relative_to(ROOT)}")
    raw = path.read_bytes()
    policy = load_json(path)
    if not isinstance(policy, dict):
        fail("runner qualification policy must be an object")
    if raw != canonical(policy):
        fail("runner qualification policy is not compact canonical JSON")
    if policy.get("schema") != "hepta.browser.runner_qualification_policy.v2":
        fail("runner qualification policy schema drifted")
    if policy.get("schema_version") != 2:
        fail("runner qualification policy version drifted")
    if policy.get("phase") != "DEVELOPMENT":
        fail("runner qualification policy must remain DEVELOPMENT")
    if policy.get("claim_level") != "L1_QUALIFICATION_ONLY":
        fail("runner qualification policy must remain qualification-only")
    if policy.get("scope") != "exact_head_ci_execution_only":
        fail("runner qualification scope drifted")
    if policy.get("supported_observation_mode") != "github_actions_api_snapshot":
        fail("runner qualification observation mode drifted")
    if policy.get("required_workflows") != EXPECTED_WORKFLOWS:
        fail("runner qualification workflow set or ordering drifted")
    if policy.get("recovery_contract") != (
        "docs/hepta-vnext/browser/RUNNER_RECOVERY_CONTRACT_V1.json"
    ):
        fail("runner qualification recovery pointer drifted")
    if policy.get("supersedes") != (
        "docs/hepta-vnext/browser/RUNNER_QUALIFICATION_POLICY_V1.json"
    ):
        fail("runner qualification supersession pointer drifted")
    expected_contract = {
        "api_snapshots_are_inputs_not_authority": True,
        "completed_success_required": True,
        "evidence_digest": "sha256_of_canonical_unsigned_evidence",
        "evidence_output_create_only": True,
        "evidence_output_fsync": True,
        "evidence_output_mode": "0600",
        "exact_head_required": True,
        "expected_repository_full_name": EXPECTED_REPOSITORY,
        "job_list_required": True,
        "jobs_payload_total_count_must_match": True,
        "jobs_url_must_match_repository_and_run": True,
        "minimum_runner_id": 1,
        "minimum_successful_required_job_steps": 1,
        "required_job_steps_must_be_terminal_nonfailure": True,
        "run_and_required_job_conclusions": "success",
        "run_attempt_required": True,
        "steps_required": True,
        "unused_jobs_snapshots_rejected": True,
        "zero_jobs_disposition": "environment_blocker_not_pass",
        "zero_runner_disposition": "environment_blocker_not_pass",
        "zero_steps_disposition": "environment_blocker_not_pass",
    }
    if policy.get("evidence_contract") != expected_contract:
        fail("runner qualification evidence contract drifted")
    require_all_false(policy.get("authority"))
    return policy


def verify_recovery_contract(
    path: pathlib.Path = RECOVERY_PATH,
    policy: dict[str, Any] | None = None,
) -> dict[str, Any]:
    policy = policy or verify_policy()
    if not path.is_file():
        fail(f"missing runner recovery contract: {path.relative_to(ROOT)}")
    raw = path.read_bytes()
    contract = load_json(path)
    if not isinstance(contract, dict):
        fail("runner recovery contract must be an object")
    if raw != canonical(contract):
        fail("runner recovery contract is not compact canonical JSON")
    if contract.get("schema") != "hepta.browser.runner_recovery_contract.v1":
        fail("runner recovery contract schema drifted")
    if contract.get("schema_version") != 1:
        fail("runner recovery contract version drifted")
    if contract.get("phase") != "DEVELOPMENT":
        fail("runner recovery contract must remain DEVELOPMENT")
    if contract.get("claim_level") != "L1_QUALIFICATION_ONLY":
        fail("runner recovery contract must remain qualification-only")
    if contract.get("scope") != "runner_allocation_recovery_only":
        fail("runner recovery scope drifted")
    if contract.get("repository_full_name") != EXPECTED_REPOSITORY:
        fail("runner recovery repository binding drifted")
    if contract.get("policy") != str(POLICY_PATH.relative_to(ROOT)):
        fail("runner recovery policy pointer drifted")
    if contract.get("required_workflows") != policy["required_workflows"]:
        fail("runner recovery workflow set or ordering drifted")
    if contract.get("environment_blocker_disposition") != BLOCKER_DISPOSITION:
        fail("runner recovery blocker disposition drifted")
    expected_closure = {
        "all_required_workflows_completed_success": True,
        "evidence_digest_verified": True,
        "exact_head_must_match": True,
        "required_job_steps_verified": True,
        "required_jobs_have_positive_runner_id": True,
        "required_workflow_count": 3,
    }
    if contract.get("closure_criteria") != expected_closure:
        fail("runner recovery closure criteria drifted")
    expected_shortcuts = [
        "treat queued or pending as pass",
        "treat runner_id zero as pass",
        "treat empty steps as pass",
        "reuse evidence from another head SHA",
        "cancel a run created after the queue-hygiene observation started",
        "dispatch exact-source qualification before exact-head required graphs are executable",
        "grant Browser Servo build runtime operator promotion or release authority",
    ]
    if contract.get("forbidden_shortcuts") != expected_shortcuts:
        fail("runner recovery forbidden-shortcut set drifted")
    expected_checks = [
        {
            "id": "actions-policy",
            "owner": "repository_or_organization_admin",
            "verification": (
                "GitHub Actions is enabled for the repository and required actions "
                "are allowed"
            ),
        },
        {
            "id": "billing-and-spending",
            "owner": "repository_or_organization_billing_manager",
            "verification": (
                "Actions billing spending limit and payment state permit "
                "hosted-runner allocation"
            ),
        },
        {
            "id": "runner-allocation",
            "owner": "repository_or_organization_admin",
            "verification": (
                "an exact-head required job records runner_id greater than zero "
                "and at least one step"
            ),
        },
        {
            "id": "queue-hygiene",
            "owner": "repository_or_organization_admin",
            "verification": (
                "only obsolete queued runs observed before cleanup starts are "
                "cancelled; the cleanup excludes its exact head and every run created "
                "after its observation timestamp"
            ),
        },
        {
            "id": "exact-head-rerun",
            "owner": "repository_maintainer",
            "verification": (
                "all three canonical required workflows execute on the same current "
                "PR head SHA"
            ),
        },
    ]
    if contract.get("required_external_checks") != expected_checks:
        fail("runner recovery external check contract drifted")
    require_all_false(contract.get("authority"))
    return contract



def verify_plan_bindings() -> dict[str, Any]:
    current_raw = CURRENT_PATH.read_bytes() if CURRENT_PATH.is_file() else None
    if current_raw is None:
        fail(f"missing C1 current pointer: {CURRENT_PATH.relative_to(ROOT)}")
    current = load_json(CURRENT_PATH)
    if current_raw != canonical(current):
        fail("C1_CURRENT_V7.json is not compact canonical JSON")
    if current.get("schema") != "hepta.browser.c1_current.v7":
        fail("C1 current schema drifted")
    if current.get("schema_version") != 1:
        fail("C1 current version drifted")
    if current.get("phase") != "DEVELOPMENT" or current.get("stage") != "WEB_C1":
        fail("C1 current phase/stage drifted")
    if current.get("plan_id") != "HEPTA-BROWSER-WEB-D":
        fail("C1 current plan id drifted")
    expected_current_pointers = {
        "canonical_aggregate_workflow": (
            ".github/workflows/hepta-browser-next-required-v9.yml"
        ),
        "canonical_execution_delta": (
            "docs/hepta-vnext/browser/C1_EXECUTION_DELTA_2026-08-28_V2.json"
        ),
        "canonical_next_work_queue": (
            "docs/hepta-vnext/browser/NEXT_WORK_QUEUE_C1_V2.json"
        ),
        "canonical_runner_qualification_policy": str(POLICY_PATH.relative_to(ROOT)),
        "canonical_runner_recovery_contract": str(RECOVERY_PATH.relative_to(ROOT)),
        "canonical_runner_recovery_runbook": str(RUNBOOK_PATH.relative_to(ROOT)),
    }
    for key, expected in expected_current_pointers.items():
        if current.get(key) != expected:
            fail(f"C1 current pointer drifted for {key}")
    require_false_mapping(
        current.get("authority"),
        CURRENT_AUTHORITY_KEYS,
        "C1 current authority",
    )
    claims = require_object(current.get("claims"), "C1 current claims")
    enabled_claims = sorted(key for key, value in claims.items() if value is not False)
    if enabled_claims:
        fail(f"C1 current attempted to enable claims: {enabled_claims}")
    if claims.get("exact_head_ci_qualified") is not False:
        fail("C1 current exact-head CI claim must remain false")
    hosted_runner = require_object(
        current.get("hosted_runner"),
        "C1 current hosted_runner",
    )
    expected_hosted_runner = {
        "classification": "ACTIONS_ALLOCATION_LAYER_ZERO_STEPS",
        "closure_disposition": PASS_DISPOSITION,
        "evidence_schema": "hepta.browser.runner_qualification_evidence.v2",
        "exact_head_qualification_available": False,
        "historical_snapshots_are_not_current_head_authority": True,
        "labels_tested": ["ubuntu-22.04", "ubuntu-24.04", "ubuntu-latest"],
        "qualification_policy": str(POLICY_PATH.relative_to(ROOT)),
        "recovery_contract": str(RECOVERY_PATH.relative_to(ROOT)),
        "repository_test_result_available": False,
    }
    if hosted_runner != expected_hosted_runner:
        fail("C1 current hosted-runner binding drifted")

    if not QUEUE_PATH.is_file():
        fail(f"missing C1 work queue: {QUEUE_PATH.relative_to(ROOT)}")
    queue_raw = QUEUE_PATH.read_bytes()
    queue = load_json(QUEUE_PATH)
    if queue_raw != canonical(queue):
        fail("NEXT_WORK_QUEUE_C1_V2.json is not compact canonical JSON")
    if queue.get("schema") != "hepta.browser.next_work_queue.c1.v2":
        fail("C1 work queue schema drifted")
    if queue.get("schema_version") != 2:
        fail("C1 work queue version drifted")
    if queue.get("phase") != "DEVELOPMENT":
        fail("C1 work queue phase drifted")
    if queue.get("plan") != "HEPTA-BROWSER-WEB-D":
        fail("C1 work queue plan drifted")
    if queue.get("supersedes") != (
        "docs/hepta-vnext/browser/NEXT_WORK_QUEUE_C1.json"
    ):
        fail("C1 work queue supersession pointer drifted")
    expected_queue_pointers = {
        "canonical_runner_policy": str(POLICY_PATH.relative_to(ROOT)),
        "canonical_runner_recovery_contract": str(RECOVERY_PATH.relative_to(ROOT)),
        "canonical_runner_recovery_runbook": str(RUNBOOK_PATH.relative_to(ROOT)),
    }
    for key, expected in expected_queue_pointers.items():
        if queue.get(key) != expected:
            fail(f"C1 work queue pointer drifted for {key}")
    require_false_mapping(
        queue.get("authority"),
        QUEUE_AUTHORITY_KEYS,
        "C1 work queue authority",
    )
    items = require_list(queue.get("items"), "C1 work queue items")
    expected_item_ids = [
        "C1-RUNNER-0",
        "C1-004A-1",
        "C1-004A-2",
        "C1-004A-3",
        "C1-004A-4",
        "C1-SOURCE-ACCEPTANCE",
        "C1-TOPOLOGY-ACCEPTANCE",
        "C1-004B-1",
        "C1-004B-2",
        "C1-004B-3",
        "C1-004B-4",
        "C1-004C-1",
        "C1-004C-2",
        "C1-004C-3",
        "C1-004C-4",
    ]
    actual_item_ids = [
        require_object(item, "C1 work queue item").get("id") for item in items
    ]
    if actual_item_ids != expected_item_ids:
        fail("C1 work queue item ordering or set drifted")
    if items[0].get("state") != (
        "RECOVERY_CONTRACT_IMPLEMENTED_HOSTED_EVIDENCE_ABSENT"
    ):
        fail("C1 runner item state drifted")
    environment = require_object(
        queue.get("environment_blocker"),
        "C1 work queue environment blocker",
    )
    if environment.get("status") != "ACTIONS_ALLOCATION_LAYER_ZERO_STEPS":
        fail("C1 work queue blocker classification drifted")
    if environment.get("dynamic_head_rule") != (
        "resolve pull request head SHA at evidence capture time"
    ):
        fail("C1 work queue dynamic-head rule drifted")
    if environment.get("historical_snapshots_are_authority") is not False:
        fail("historical runner snapshots must not become authority")
    if environment.get("closure") != (
        "exact current head produces a digest-valid V2 receipt with "
        "PASS_CI_EXECUTION_ONLY"
    ):
        fail("C1 work queue closure rule drifted")
    historical = require_object(
        environment.get("latest_historical_observation"),
        "C1 historical runner observation",
    )
    parent_head = historical.get("observed_parent_head")
    if not isinstance(parent_head, str) or not HEX40.fullmatch(parent_head):
        fail("C1 historical runner observation head is invalid")
    runs = require_list(historical.get("runs"), "C1 historical runner runs")
    if [item.get("workflow") for item in runs] != [
        item["workflow_name"] for item in EXPECTED_WORKFLOWS
    ]:
        fail("C1 historical runner workflow set or ordering drifted")
    if queue.get("next_authorized_step") != (
        "complete external runner recovery checks and capture exact-head V2 "
        "evidence; only after PASS_CI_EXECUTION_ONLY dispatch exact-source "
        "qualification v3"
    ):
        fail("C1 work queue next authorized step drifted")

    if not DELTA_PATH.is_file():
        fail(f"missing C1 execution delta: {DELTA_PATH.relative_to(ROOT)}")
    delta = load_json(DELTA_PATH)
    if delta.get("schema") != "hepta.browser.c1_execution_delta.v2":
        fail("C1 execution delta schema drifted")
    if delta.get("schema_version") != 1:
        fail("C1 execution delta version drifted")
    if delta.get("phase") != "DEVELOPMENT" or delta.get("stage") != "WEB_C1":
        fail("C1 execution delta phase/stage drifted")
    expected_delta_pointers = {
        "canonical_current": str(CURRENT_PATH.relative_to(ROOT)),
        "canonical_aggregate": (
            ".github/workflows/hepta-browser-next-required-v9.yml"
        ),
        "canonical_next_work_queue": str(QUEUE_PATH.relative_to(ROOT)),
        "canonical_runner_qualification_policy": str(POLICY_PATH.relative_to(ROOT)),
        "canonical_runner_recovery_contract": str(RECOVERY_PATH.relative_to(ROOT)),
        "canonical_runner_recovery_runbook": str(RUNBOOK_PATH.relative_to(ROOT)),
    }
    for key, expected in expected_delta_pointers.items():
        if delta.get(key) != expected:
            fail(f"C1 execution delta pointer drifted for {key}")
    delta_claims = require_object(delta.get("claims"), "C1 execution delta claims")
    enabled_delta_claims = sorted(
        key for key, value in delta_claims.items() if value is not False
    )
    if enabled_delta_claims:
        fail(f"C1 execution delta attempted to enable claims: {enabled_delta_claims}")
    require_false_mapping(
        delta.get("authority"),
        CURRENT_AUTHORITY_KEYS,
        "C1 execution delta authority",
    )
    blocker = require_object(
        delta.get("runner_blocker"),
        "C1 execution delta runner blocker",
    )
    if blocker.get("classification") != "ACTIONS_ALLOCATION_LAYER_ZERO_STEPS":
        fail("C1 execution delta runner classification drifted")
    if blocker.get("closure_disposition") != PASS_DISPOSITION:
        fail("C1 execution delta runner closure disposition drifted")
    if blocker.get("current_head_evidence_required") is not True:
        fail("C1 execution delta must require current-head evidence")
    historical_delta = require_object(
        blocker.get("historical_observation"),
        "C1 execution delta historical observation",
    )
    if historical_delta.get("observation_is_current_head_authority") is not False:
        fail("C1 execution delta historical observation became authority")

    workflow_trigger_requirements = {
        BROWSER_REQUIRED_WORKFLOW_PATH: [
            "name: hepta-browser next required v9",
            '      - "scripts/verify-hepta-browser-*.py"',
            '      - "scripts/hepta_browser_runner_evidence/**"',
            '      - "scripts/tests/test_hepta_browser_*.py"',
        ],
        VNEXT_REQUIRED_WORKFLOW_PATH: [
            "name: hepta-vnext qualification",
            '      - "scripts/verify-hepta-browser-runner-evidence.py"',
            '      - "scripts/hepta_browser_runner_evidence/**"',
            '      - "scripts/tests/test_hepta_browser_runner_evidence.py"',
        ],
    }
    for workflow_path, fragments in workflow_trigger_requirements.items():
        if not workflow_path.is_file():
            fail(f"missing required workflow: {workflow_path.relative_to(ROOT)}")
        workflow_text = workflow_path.read_text(encoding="utf-8")
        missing_workflow_fragments = [
            fragment for fragment in fragments if fragment not in workflow_text
        ]
        if missing_workflow_fragments:
            fail(
                f"required workflow trigger binding drifted for "
                f"{workflow_path.relative_to(ROOT)}: "
                f"{missing_workflow_fragments}"
            )

    if not RUNBOOK_PATH.is_file():
        fail(f"missing runner recovery runbook: {RUNBOOK_PATH.relative_to(ROOT)}")
    runbook = RUNBOOK_PATH.read_text(encoding="utf-8")
    required_runbook_fragments = [
        "DEVELOPMENT / L1_QUALIFICATION_ONLY",
        "PASS_CI_EXECUTION_ONLY",
        "runner_id > 0",
        "--verify-evidence",
        "only after PASS_CI_EXECUTION_ONLY dispatch exact-source qualification v3",
    ]
    missing_fragments = [
        fragment for fragment in required_runbook_fragments if fragment not in runbook
    ]
    if missing_fragments:
        fail(f"runner recovery runbook is incomplete: {missing_fragments}")

    return {
        "current_schema": current["schema"],
        "delta_schema": delta["schema"],
        "queue_schema": queue["schema"],
        "status": "PASS_C1_RUNNER_PLAN_BINDINGS_V2",
    }


