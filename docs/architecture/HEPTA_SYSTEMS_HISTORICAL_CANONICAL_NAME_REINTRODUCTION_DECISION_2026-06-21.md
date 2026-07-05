# Hepta Systems Historical Canonical Name Reintroduction Decision - 2026-06-21

This note records the local-only Historical Canonical Name Reintroduction
Decision. It consumes the current canonical wrapper and decides whether it is
safe to reclaim the old `scripts/hepta-systems-canonical-gate.sh` filename.

The decision is now a claimed thin wrapper: the historical filename is restored
only as a local exec wrapper around the current canonical wrapper gate, without
invoking the wrapper or any live path.

## Current Checkout Reality

The current wrapper is stable and the earlier live absence probes have been
converted to snapshot evidence. Those phase-aware surfaces preserve historical
missing-path truth while allowing the current checkout to contain the restored
local wrapper:

- `scripts/hepta-systems-tool-execution-canonical-summary-attachment-index-gate.sh`
- `scripts/hepta-systems-compact-capability-matrix-restore-preflight-gate.sh`

Reintroducing `scripts/hepta-systems-canonical-gate.sh` no longer invalidates
those evidence surfaces because they read the historical snapshot instead of the
live filesystem.

Current report facts:

- `source_current_canonical_wrapper_ready=true`
- `source_historical_canonical_gate_name_claimed=true`
- `source_historical_canonical_gate_created=true`
- `source_historical_canonical_gate_executable=true`
- `source_historical_canonical_gate_wrapper_kind=thin_local_exec_wrapper`
- `source_historical_canonical_gate_wrapper_target=scripts/hepta-systems-current-canonical-wrapper-gate.sh`
- `source_historical_canonical_gate_wrapper_exec_count=1`
- `source_historical_canonical_gate_mutated=true`
- `source_historical_canonical_gate_mutated_by_report=false`
- `source_canonical_gate_wrapper_invoked=false`
- `proposed_historical_canonical_gate_path=scripts/hepta-systems-canonical-gate.sh`
- `historical_canonical_gate_name_reintroduction_decision=claimed_thin_wrapper`
- `historical_canonical_gate_name_reintroduction_allowed=true`
- `historical_canonical_gate_name_claimed=true`
- `historical_canonical_gate_created=true`
- `historical_canonical_gate_executable=true`
- `phase_split_required_before_name_claim=false`
- `phase_split_completed_before_name_claim=true`
- `decision_check_count=3`
- `execution_enabled_count=0`
- `public_ga_enabled_count=0`
- `manual_operator_live_cutover_approval_required=true`
- `tool_execution_live_cutover_allowed=false`
- `tool_execution_public_ga_allowed=false`

## Decision Rules

- The current wrapper must stay ready.
- The historical filename may stay claimed only as the current local thin
  wrapper.
- Attachment/preflight surfaces must preserve historical missing-path evidence
  through snapshot evidence rather than live filesystem absence.
- The wrapper and wrapper target must not be invoked by this decision surface.
- Live cutover and Public GA must stay disabled.

## Guardrails

- No historical patch replay.
- No patch body emission.
- No plugin fixture fabrication.
- No canonical summary mutation.
- No historical canonical gate mutation by the report or gate.
- No additional historical canonical name claim beyond the local thin wrapper.
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
  `scripts/hepta-systems-historical-canonical-name-reintroduction-decision-report.sh`
- Gate:
  `scripts/hepta-systems-historical-canonical-name-reintroduction-decision-gate.sh`
- Source:
  `scripts/hepta-systems-current-canonical-wrapper-gate.sh`

## Next Move

Validate the historical canonical gate thin wrapper without invoking it or its
target. Validation should remain a shape/readback check only.
