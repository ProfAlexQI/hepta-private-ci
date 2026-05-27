# Hepta Core Activation Evidence Receipt Materialization Dry-Run Gate

Status: gated, read-only, non-activation.

Gate:

`scripts/hepta-core-activation-evidence-receipt-materialization-dry-run-gate.sh`

## Purpose

This gate joins the Hepta Core fresh long-soak ledger/receipt schema with the upstream Codex receipt materialization dry-run, no-write sink adapter contract, filesystem output path allowlist, and output path evidence binding gates.

It answers one narrow question: if activation evidence receipt materialization is eventually approved, can Hepta describe the receipt plan, no-write sink, allowed output path family, and evidence binding without performing a write today?

The answer is yes for schema readiness and no for activation. The gate does not select an output path, materialize a receipt, persist a receipt, run the long soak, invoke a provider, send a channel message, restart a service, fetch upstream, or publish a release.

## Current Verdict

The expected decision is:

`blocked_until_fresh_long_soak_receipt_operator_approval_and_filesystem_persistence_approval_exist`

The gate can report `status=ready` because all source contracts are ready and side-effect-free. It still reports `activation_allowed=false` because the fresh long-soak evidence, operator approval, activation request, ledger record, filesystem persistence approval, output path selection, and receipt materialization record are intentionally absent.

## Source Gates

This gate consumes five source reports:

- `scripts/hepta-core-activation-fresh-long-soak-evidence-ledger-receipt-gate.sh`
- `scripts/hepta-upstream-codex-activation-evidence-receipt-no-write-sink-adapter-contract.sh`
- `scripts/hepta-upstream-codex-activation-evidence-receipt-materialization-dry-run.sh`
- `scripts/hepta-upstream-codex-activation-evidence-receipt-filesystem-output-path-allowlist.sh`
- `scripts/hepta-upstream-codex-activation-evidence-receipt-filesystem-output-path-evidence-binding.sh`

All five source reports must be ready and activation-blocking.

## Required Materialization Fields

The materialization schema requires 20 fields:

- `receipt_id`
- `ledger_record_id`
- `materialization_plan_id`
- `no_write_sink_adapter_id`
- `redacted_payload_hash`
- `redacted_output_path`
- `output_path_allowlist_id`
- `output_path_evidence_binding_id`
- `active_binary_sha256`
- `source_ledger_receipt_report_sha256`
- `source_no_write_sink_report_sha256`
- `source_materialization_report_sha256`
- `source_output_path_allowlist_report_sha256`
- `source_output_path_binding_report_sha256`
- `no_secret_payload_review_id`
- `operator_approval_id`
- `fresh_long_soak_evidence_id`
- `filesystem_persistence_approval_id`
- `rollback_plan_id`
- `public_claim_and_artifact_decision`

Current recorded materialization field count is `0`.

## Current Boundary

- Minimum required long-soak samples: `24`
- Source required evidence count: `8`
- Source recorded evidence count: `0`
- Source fresh evidence count: `0`
- Source required ledger record field count: `20`
- Source required receipt field count: `20`
- Required no-write sink surface count: `6`
- Required materialization fixture count: `3`
- Required output path allowlist entry count: `6`
- Required output path binding count: `8`
- Recorded output path binding count: `0`
- Filesystem persistence allowed: `false`
- Filesystem persistence execution performed: `false`
- Workspace write performed: `false`
- Receipt materialized: `false`
- Receipt persisted: `false`

## Denied Actions

The gate keeps these actions denied by default:

- Running long soak from this gate
- Recording fresh long-soak evidence
- Recording operator approval or activation request
- Recording a ledger record
- Recording a receipt materialization plan
- Materializing or persisting a receipt
- Recording filesystem persistence approval
- Selecting an output path
- Binding an output path to fresh evidence
- Persisting raw soak sample payloads
- Writing the workspace or filesystem
- Public release or public GA claims
- Release artifact writes
- Provider or model invocation
- Channel or Telegram delivery
- Install, launchd mutation, service restart, or active binary mutation
- Upstream fetch or merge

## Preflight Wiring

`scripts/hepta-preflight.sh` runs this gate immediately after:

`scripts/hepta-core-activation-fresh-long-soak-evidence-ledger-receipt-gate.sh`

This placement makes receipt materialization dry-run the next boundary after Hepta Core has defined the fresh long-soak ledger and receipt schema.
