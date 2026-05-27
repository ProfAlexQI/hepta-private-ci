# Hepta Core Activation Evidence Receipt Acceptance Denial Gate

Status: gated, read-only, non-activation.

Gate:

`scripts/hepta-core-activation-evidence-receipt-acceptance-denial-gate.sh`

## Purpose

This gate joins the Hepta Core filesystem persistence denial gate with the
upstream filesystem persistence approval packet, receipt persistence command
contract, and receipt persistence invocation dry-run.

It draws the next boundary after receipt materialization and filesystem
persistence denial: a receipt-shaped report is still not an accepted Hepta Core
activation input. It does not become operator approval, activation authority, a
ledger record, an index record, a delivery record, a completion acknowledgement,
or a public release decision.

## Current Verdict

The expected decision is:

`blocked_until_operator_approval_filesystem_persistence_approval_receipt_persistence_and_fresh_long_soak_evidence_acceptance_exist`

The gate can report `status=ready` because all four source reports are present
and activation-blocking. It still reports `receipt_accepted=false` and
`activation_allowed=false` because operator approval, filesystem persistence
approval, receipt persistence, fresh evidence acceptance, ledger persistence,
delivery recording, and completion acknowledgement are not recorded.

## Source Gates

This gate consumes four source reports:

- `scripts/hepta-core-activation-evidence-receipt-filesystem-persistence-denial-gate.sh`
- `scripts/hepta-upstream-codex-activation-evidence-receipt-filesystem-persistence-approval-packet.sh`
- `scripts/hepta-upstream-codex-activation-evidence-receipt-persistence-command-contract.sh`
- `scripts/hepta-upstream-codex-activation-evidence-receipt-persistence-invocation-dry-run.sh`

All four source reports must be ready and activation-blocking.

## Required Acceptance Fields

The receipt acceptance schema requires 20 fields:

- `receipt_acceptance_request_id`
- `receipt_id`
- `receipt_payload_hash`
- `receipt_persistence_command_id`
- `receipt_persistence_approval_id`
- `filesystem_persistence_approval_id`
- `operator_approval_id`
- `operator_identity_hash`
- `fresh_long_soak_evidence_id`
- `trusted_evidence_record_id`
- `active_binary_sha256`
- `source_filesystem_persistence_denial_report_sha256`
- `source_filesystem_persistence_approval_packet_report_sha256`
- `source_persistence_command_contract_report_sha256`
- `source_persistence_invocation_dry_run_report_sha256`
- `no_secret_payload_review_id`
- `ledger_record_id`
- `index_record_id`
- `delivery_record_id`
- `completion_ack_id`

Current recorded receipt acceptance field count is `0`.

## Current Boundary

- Minimum required long-soak samples: `24`
- Required source count: `4`
- Ready source count: `4`
- Activation-blocking source count: `4`
- Required receipt acceptance field count: `20`
- Recorded receipt acceptance field count: `0`
- Required receipt acceptance fixture count: `4`
- Receipt acceptance request count: `4`
- Blocked receipt acceptance fixture count: `4`
- Allowed receipt acceptance fixture count: `0`
- Receipt accepted count: `0`
- Receipt acceptance performed count: `0`
- Receipt acceptance recorded count: `0`
- Receipt acceptance persisted count: `0`
- Receipt acceptance materialized count: `0`
- Receipt acceptance filesystem written count: `0`
- Operator approval recorded: `false`
- Operator authority accepted: `false`
- Activation request recorded: `false`
- Filesystem persistence approval recorded: `false`
- Receipt persistence command enabled by default: `false`
- Receipt persistence command invoked: `false`
- Filesystem persistence allowed: `false`
- Filesystem persistence execution performed: `false`
- Workspace write performed: `false`
- Evidence receipt persisted: `false`
- Completion acknowledgement recorded: `false`
- Completion acknowledgement accepted: `false`
- Ledger recorded: `false`
- Ledger persisted: `false`
- Index recorded: `false`
- Delivery recorded: `false`
- Activation allowed: `false`

## Denied Acceptance Fixtures

The gate models four blocked receipt acceptance attempts:

- `receipt-persistence-denied-attempt`
- `operator-authority-unrecorded-attempt`
- `ledger-index-delivery-ack-attempt`
- `public-claim-artifact-acceptance-attempt`

Every fixture is requested, blocked, not accepted, and not activation-authorizing.

## Denied Actions

The gate keeps these actions denied by default:

- Receipt persistence and receipt acceptance
- Receipt acceptance recording, materialization, persistence, or filesystem write
- Operator approval acceptance or operator authority grant
- Activation request recording
- Filesystem persistence approval recording
- Filesystem persistence execution
- Workspace writes
- Ledger, index, delivery, or completion acknowledgement recording
- Public release or public GA claims
- Release artifact writes
- Install, launchd mutation, service restart, or active binary mutation
- Provider or model invocation
- Channel or Telegram delivery
- Upstream fetch or merge

## Preflight Wiring

`scripts/hepta-preflight.sh` runs this gate immediately after:

`scripts/hepta-core-activation-evidence-receipt-filesystem-persistence-denial-gate.sh`

This placement makes receipt acceptance denial the next Hepta Core boundary
after filesystem persistence denial, before JSON report capture and latest
upstream safety gates.
