# Hepta Systems Current Canonical Wrapper - 2026-06-21

This note records the local-only Current Canonical Wrapper. The wrapper consumes
the current compact capability summary and makes it a stable high-level
entrypoint without live invocation.

This wrapper now records the historical canonical gate thin wrapper claim:
`scripts/hepta-systems-canonical-gate.sh` exists as a local exec wrapper around
`scripts/hepta-systems-current-canonical-wrapper-gate.sh`. The wrapper is shape
evidence only here and is not invoked by this report or gate.

## Current Checkout Reality

The old `scripts/hepta-systems-canonical-gate.sh` path has been reintroduced as
a local thin wrapper after the missing-path evidence was decoupled into a
historical snapshot. The snapshot remains the source of historical absence
truth; the current filesystem presence of the wrapper is now a post-creation
state, not evidence replay.

Current report facts:

- `source_compact_capability_summary_ready=true`
- `source_local_surface_count=5`
- `source_local_surface_ready_count=5`
- `source_execution_enabled_count=0`
- `source_public_ga_enabled_count=0`
- `current_canonical_wrapper_ready=true`
- `wrapper_plan_step_count=3`
- `historical_canonical_gate_name_claimed=true`
- `historical_canonical_gate_created=true`
- `historical_canonical_gate_executable=true`
- `historical_canonical_gate_wrapper_kind=thin_local_exec_wrapper`
- `historical_canonical_gate_wrapper_target=scripts/hepta-systems-current-canonical-wrapper-gate.sh`
- `historical_canonical_gate_wrapper_exec_count=1`
- `historical_canonical_gate_mutated=true`
- `historical_canonical_gate_mutated_by_report=false`
- `canonical_gate_wrapper_invoked=false`
- `capability_matrix_gate_invoked=false`
- `terminal_live_gate_invoked=false`
- `live_url_required=false`
- `long_soak_required=false`
- `manual_operator_live_cutover_approval_required=true`
- `tool_execution_live_cutover_allowed=false`
- `tool_execution_public_ga_allowed=false`

## Wrapper Rules

- The current compact capability summary must be ready.
- The wrapper report must not invoke the summary, restore preflight, or
  attachment gates; the wrapper gate handles validation.
- Execution and Public GA must remain disabled.
- The historical canonical filename may be claimed only as the local thin
  wrapper shape recorded above.
- The wrapper target must not be invoked by this report or gate.
- No terminal live gate, live URL, long soak, or release path is required.

## Guardrails

- No historical patch replay.
- No patch body emission.
- No plugin fixture fabrication.
- No canonical summary mutation.
- No historical canonical gate mutation by the report or gate.
- No canonical gate invocation from the report.
- No capability matrix gate invocation from the report.
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
  `scripts/hepta-systems-current-canonical-wrapper-report.sh`
- Gate:
  `scripts/hepta-systems-current-canonical-wrapper-gate.sh`
- Source:
  `scripts/hepta-systems-current-compact-capability-summary-gate.sh`

## Next Move

Validate the historical canonical gate thin wrapper without invoking it or its
target. The validation should inspect wrapper shape, permissions, and target
reference only.
