# Hepta Systems Tool Execution Canonical Summary Attachment Index - 2026-06-21

This note records the local-only Tool Execution Canonical Summary Attachment
Index. This is a without invocation recovery slice. It consumes the terminal
governance bridge and the session patch queue to make the next canonical
attachment blocker machine-readable.

The index intentionally does not replay historical patches.

## Current Checkout Reality

The tool execution closure is already visible through the terminal governance
bridge. The old compact canonical or capability summary entrypoints were absent
when the recovery snapshot was captured. The index now consumes snapshot
evidence for those historical missing paths instead of probing the live
filesystem, so later wrapper creation will not make the evidence drift.

Current report facts:

- `source_terminal_governance_bridge_ready=true`
- `source_manual_operator_live_cutover_approval_required=true`
- `source_live_cutover_allowed=false`
- `source_public_ga_allowed=false`
- `source_terminal_live_gates_invoked=false`
- `source_long_soak_required=false`
- `source_patch_queue_ready=true`
- `source_patch_replay_applied=false`
- `source_patch_replay_enabled=false`
- `source_patch_body_emitted=false`
- `source_snapshot_ready=true`
- `canonical_summary_probe_basis=historical_snapshot_evidence`
- `canonical_summary_current_filesystem_probe_used=false`
- `canonical_summary_probe_count=8`
- `canonical_summary_present_count=0`
- `current_checkout_missing_canonical_summary=true`
- `historical_snapshot_missing_canonical_summary=true`
- `historical_snapshot_evidence_consumable_after_wrapper_creation=true`
- `canonical_summary_available=false`
- `historical_compact_capability_matrix_patch_call_count>=1`
- `historical_compact_capability_matrix_missing_path_count>=1`
- `attach_to_existing_canonical_summary_allowed=false`
- `canonical_summary_mutation_allowed=false`
- `tool_execution_canonical_summary_attachment_index_ready=true`
- `tool_execution_live_cutover_allowed=false`
- `tool_execution_public_ga_allowed=false`

The selected reconstruction candidate is evidence only. It points at the first
historical compact capability/canonical patch touching:

- `scripts/hepta-systems-canonical-gate.sh`
- `scripts/hepta-systems-p0-local-gate.sh`
- `docs/architecture/HEPTA_SYSTEMS_CANONICAL_GATE_MATRIX_2026-06-12.md`
- `docs/architecture/HEPTA_SYSTEMS_PLUGINS_TOOLS_WORKFLOW_PLAN_2026-06-12.md`
- `plugins/hepta-system/skills/hepta-system-status/SKILL.md`

## Attachment Rules

- The source terminal governance bridge must be ready.
- Manual operator live cutover approval must still be required.
- Tool execution live cutover must remain disallowed.
- Public GA must remain disallowed.
- Terminal live gates must not be invoked by this index.
- The session patch queue must be report-only, with replay disabled and no
  patch bodies emitted by this index.
- Historical canonical/capability summary entrypoints must be treated as missing
  according to snapshot evidence until restored through a manual apply check.
- The attachment index must not use live filesystem absence as the canonical
  missing-path evidence source.
- Attaching tool execution to an existing canonical summary remains blocked
  while no current summary exists.

## Guardrails

- No historical patch replay.
- No patch body emission.
- No canonical summary mutation.
- No canonical gate invocation.
- No capability matrix gate invocation.
- No terminal live gate invocation.
- No live URL contact.
- No long soak start.
- No ToolRegistry registration.
- No execution adapter dispatch.
- No tool invocation.
- No ledger write.
- No ApprovalBroker request.
- No approval request send.
- No operator cutover acceptance record.
- No live cutover start.
- No rollback execution.
- No rollback receipt write.
- No result receipt write.
- No MCP server or app connector startup.
- No workflow event log, SQLite, local storage, WorkGraph, provider/model,
  gateway/auth, Native POST, channel send, package, release, Public GA, or
  external live action.

## Files

- Report:
  `scripts/hepta-systems-tool-execution-canonical-summary-attachment-index-report.sh`
- Gate:
  `scripts/hepta-systems-tool-execution-canonical-summary-attachment-index-gate.sh`
- Source bridge:
  `scripts/hepta-systems-tool-execution-terminal-governance-bridge-gate.sh`
- Source queue:
  `scripts/hepta-systems-session-patch-queue-gate.sh`

## Next Move

Restore the compact capability/canonical summary from selected patch evidence
only after a manual apply check against the current checkout. The restored
summary can then consume this attachment index as the tool execution closure
source without opening approval, dispatch, invocation, ledger, receipt, rollback,
live cutover, or Public GA paths.
