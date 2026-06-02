# Hepta Core Activation Long-Soak Operator Approval Packet Gate

Status: gated, read-only, non-activation.

Gate:

`scripts/hepta-core-activation-long-soak-operator-approval-packet-gate.sh`

## Purpose

This gate defines the packet shape required before any Hepta Core activation can move from observed readiness into an operator-approved live mutation path. It aggregates the existing activation readiness summary, evidence freshness policy, evidence binding manifest, evidence completeness scoreboard, and public GA operator approval packet.

The gate is intentionally not an activation command. It records no approval, persists no packet, writes no evidence ledger, invokes no provider, sends no channel message, restarts no service, and writes no release artifact.

## Current Verdict

The expected verdict is:

`blocked_until_operator_approval_and_fresh_24_sample_evidence_records_exist`

The gate can report `status=ready` because the schema and source evidence gates are ready. It still reports `activation_allowed=false` because all trusted activation records remain intentionally absent.

## Required Packet Fields

The packet shape requires 16 fields before activation can be considered:

- `activation_request_id`
- `operator_approval_id`
- `operator_identity_hash`
- `single_surface_activation_scope`
- `active_binary_sha256`
- `live_dependency_isolation_evidence_id`
- `watchdog_evidence_id`
- `browser_smoke_evidence_id`
- `long_soak_evidence_id`
- `rollback_plan_id`
- `release_publication_denial_evidence_id`
- `memory_intelligence_boundary_evidence_id`
- `final_release_governance_audit_id`
- `no_public_claim_decision`
- `no_release_artifact_write_decision`
- `post_activation_watchdog_soak_plan_id`

Current recorded packet field count is `0`.

## Required Evidence

- Required source gate count: `5`
- Ready source gate count: `5`
- Activation-blocking source gate count: `5`
- Required evidence count: `8`
- Missing evidence count: `8`
- Fresh evidence count: `0`
- Required binding record count: `8`
- Missing binding record count: `8`
- Recorded binding record count: `0`
- Required trusted record count: `8`
- Accepted trusted record count: `0`
- Fresh trusted record count: `0`
- Minimum long-soak sample count: `24`

The short live soak used by watchdog regression remains operational evidence only. If the source reports a known operator-security attention soak failure, the packet records that classification as blocked observation, not as a passed soak. It does not authorize activation or memory mutation.

## Denied Actions

The gate keeps these actions denied by default:

- Public release claim
- Public GA claim
- Public distribution publication
- Release artifact write
- Memory store mutation
- Provider or model invocation
- Channel delivery
- Install, launchd mutation, service restart, or active binary mutation
- Upstream fetch or merge
- Approval packet persistence
- Long-soak evidence persistence
- Trusted record persistence

## Preflight Wiring

`scripts/hepta-preflight.sh` runs this gate immediately after:

`scripts/hepta-core-activation-readiness-summary-gate.sh`

This placement makes the operator approval packet the next explicit boundary after core readiness is summarized.
