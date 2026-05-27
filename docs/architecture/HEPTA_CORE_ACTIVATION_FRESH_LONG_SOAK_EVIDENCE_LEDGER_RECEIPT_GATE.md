# Hepta Core Activation Fresh Long-Soak Evidence Ledger Receipt Gate

Status: gated, read-only, non-activation.

Gate:

`scripts/hepta-core-activation-fresh-long-soak-evidence-ledger-receipt-gate.sh`

## Purpose

This gate defines the ledger and receipt shape required for fresh 24-sample long-soak evidence before any Hepta Core activation can be considered. It sits after the long-soak operator approval packet gate and keeps the evidence path schema-only.

The gate does not run the 24-sample soak, persist a ledger record, materialize a receipt, write a filesystem artifact, record an approval packet, invoke a provider, send a channel message, restart a service, or publish a release.

## Current Verdict

The expected decision is:

`blocked_until_fresh_24_sample_long_soak_evidence_receipt_and_operator_approval_records_exist`

The gate can report `status=ready` because the schema, source reports, redaction rules, and side-effect denials are in place. It still reports `activation_allowed=false` because the fresh long-soak record, trusted records, operator approval, and receipt persistence approval are intentionally absent.

## Source Gates

This gate consumes four source reports:

- `scripts/hepta-core-activation-long-soak-operator-approval-packet-gate.sh`
- `scripts/hepta-live-mutation-pre-activation-soak-evidence-gate.sh`
- `scripts/hepta-upstream-codex-activation-evidence-ledger.sh`
- `scripts/hepta-upstream-codex-activation-evidence-receipt-filesystem-persistence-approval-packet.sh`

All four source reports must be ready and activation-blocking.

## Required Ledger Record Fields

The ledger record schema requires 20 fields:

- `long_soak_evidence_id`
- `activation_request_id`
- `operator_approval_id`
- `operator_identity_hash`
- `single_surface_activation_scope`
- `soak_command_hash`
- `sample_count`
- `ok_count`
- `fail_count`
- `started_at_unix_ms`
- `finished_at_unix_ms`
- `active_binary_sha256`
- `watchdog_evidence_id`
- `dependency_isolation_evidence_id`
- `browser_smoke_evidence_id`
- `receipt_payload_hash`
- `redaction_policy_id`
- `no_secret_review_id`
- `rollback_plan_id`
- `public_claim_and_artifact_decision`

Current recorded ledger record field count is `0`.

## Required Receipt Fields

The receipt schema requires 20 fields:

- `receipt_id`
- `ledger_record_id`
- `activation_request_id`
- `operator_approval_id`
- `operator_identity_hash`
- `long_soak_evidence_id`
- `sample_set_hash`
- `redacted_soak_summary_hash`
- `source_approval_packet_report_sha256`
- `source_pre_activation_soak_report_sha256`
- `source_activation_ledger_report_sha256`
- `source_receipt_persistence_packet_report_sha256`
- `active_binary_sha256`
- `route_status_hash`
- `no_secret_payload_review_id`
- `materialization_plan_id`
- `filesystem_persistence_approval_id`
- `rollback_plan_id`
- `post_activation_watchdog_plan_id`
- `post_activation_long_soak_plan_id`

Current recorded receipt field count is `0`.

## Redaction And Audit Rules

The gate requires 10 redaction and audit rules before any future receipt can be accepted:

- Raw soak sample payloads must not be persisted.
- Credential values must be absent.
- Secret file paths must be absent or redacted.
- Channel payloads must be hash-only.
- Provider prompts and outputs must be absent.
- Operator identity must be hash-only.
- Timestamps must be bounded evidence metadata only.
- Filesystem paths must be redacted or allowlist-bound.
- Public claim and artifact decisions must remain false.
- Receipt hash-chain writes require a separate specific approval.

## Denied Actions

The gate keeps these actions denied by default:

- Running long soak from this gate
- Recording or persisting long-soak evidence
- Accepting trusted evidence records
- Recording an activation request or operator approval
- Recording or persisting a ledger record
- Materializing or persisting a receipt
- Recording an audit trail or hash chain
- Persisting raw soak sample payloads
- Public release or public GA claims
- Release artifact writes
- Provider or model invocation
- Channel or Telegram delivery
- Install, launchd mutation, service restart, or active binary mutation
- Upstream fetch or merge

## Preflight Wiring

`scripts/hepta-preflight.sh` runs this gate immediately after:

`scripts/hepta-core-activation-long-soak-operator-approval-packet-gate.sh`

This placement makes the fresh long-soak evidence ledger and receipt schema the next boundary after the operator approval packet shape.
