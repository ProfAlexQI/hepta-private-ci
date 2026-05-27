# Hepta Core Activation Evidence Receipt Filesystem Persistence Denial Gate

Status: gated, read-only, non-activation.

Gate:

`scripts/hepta-core-activation-evidence-receipt-filesystem-persistence-denial-gate.sh`

## Purpose

This gate joins the Hepta Core receipt materialization dry-run with the upstream Codex sink write preview and filesystem persistence execution denial matrix.

It draws a hard boundary between preview evidence and write authority. Hepta can preview redacted output paths and deterministic payload hashes, but those previews do not authorize filesystem persistence, workspace writes, public artifact writes, receipt persistence, activation, install, restart, or public release.

## Current Verdict

The expected decision is:

`blocked_until_explicit_filesystem_persistence_approval_fresh_evidence_and_operator_approval_exist`

The gate can report `status=ready` because materialization planning, sink preview, and execution-denial fixtures are all present. It still reports `activation_allowed=false` and `filesystem_persistence_execution_performed=false` because explicit filesystem persistence approval, fresh live evidence acceptance, and operator approval authority are not recorded.

## Source Gates

This gate consumes three source reports:

- `scripts/hepta-core-activation-evidence-receipt-materialization-dry-run-gate.sh`
- `scripts/hepta-upstream-codex-activation-evidence-receipt-filesystem-sink-write-preview.sh`
- `scripts/hepta-upstream-codex-activation-evidence-receipt-filesystem-persistence-execution-denial-matrix.sh`

All three source reports must be ready and activation-blocking.

## Required Persistence Execution Fields

The filesystem persistence execution schema requires 20 fields:

- `filesystem_persistence_execution_request_id`
- `receipt_id`
- `materialization_plan_id`
- `future_persistence_approval_id`
- `redacted_payload_hash`
- `redacted_output_path`
- `output_path_root_id`
- `output_path_evidence_binding_id`
- `fresh_live_evidence_id`
- `active_binary_sha256`
- `operator_approval_id`
- `trusted_source_binding_id`
- `source_materialization_gate_report_sha256`
- `source_sink_write_preview_report_sha256`
- `source_execution_denial_report_sha256`
- `no_secret_payload_review_id`
- `workspace_path_denial_id`
- `public_artifact_denial_id`
- `rollback_plan_id`
- `public_claim_and_artifact_decision`

Current recorded filesystem persistence execution field count is `0`.

## Current Boundary

- Minimum required long-soak samples: `24`
- Required materialization field count: `20`
- Recorded materialization field count: `0`
- Required preview fixture count: `3`
- Preview fixture count: `3`
- Previewed output path count: `3`
- Deterministic payload hash count: `3`
- Redacted output path preview count: `3`
- Blocked preview fixture count: `3`
- Allowed preview fixture count: `0`
- Required denial fixture count: `4`
- Execution requested fixture count: `4`
- Future persistence approval slot count: `4`
- Explicit persistence approval id present count: `3`
- Explicit persistence approval id missing count: `1`
- Stale or missing fresh evidence fixture count: `1`
- Workspace path attempt fixture count: `1`
- Public claim attempt fixture count: `1`
- Release artifact write attempt fixture count: `1`
- Blocked execution fixture count: `4`
- Allowed execution fixture count: `0`
- Filesystem persistence allowed: `false`
- Filesystem persistence execution performed: `false`
- Workspace write performed: `false`
- Evidence receipt persisted: `false`

## Denied Actions

The gate keeps these actions denied by default:

- Filesystem persistence approval recording
- Filesystem persistence execution
- Workspace path execution
- Public artifact execution
- Public release or public GA claims
- Release artifact writes
- Evidence receipt persistence
- Receipt materialization execution
- Running long soak from this gate
- Operator authority acceptance
- Activation request recording
- Install, launchd mutation, service restart, or active binary mutation
- Provider or model invocation
- Channel or Telegram delivery
- Upstream fetch or merge

## Preflight Wiring

`scripts/hepta-preflight.sh` runs this gate immediately after:

`scripts/hepta-core-activation-evidence-receipt-materialization-dry-run-gate.sh`

This placement makes filesystem persistence denial the next Hepta Core boundary after materialization dry-run, before JSON report capture and latest upstream safety gates.
