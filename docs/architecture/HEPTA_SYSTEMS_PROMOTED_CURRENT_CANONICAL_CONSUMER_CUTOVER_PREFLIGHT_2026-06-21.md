# Hepta Systems Promoted Current Canonical Consumer Cutover Preflight - 2026-06-21

This note records the local-only Promoted Current Canonical Consumer Cutover
Preflight. It checks whether the promoted successor consumer can replace the
existing current canonical consumer in place.

The answer is no: direct replacement would create a dependency cycle because the
promoted successor still depends on the existing current consumer through the
promoted summary and promoted closure index.

The preflight does not invoke `scripts/hepta-systems-canonical-gate.sh` and does
not invoke `scripts/hepta-systems-current-canonical-wrapper-gate.sh`.

## Current Checkout Reality

The promoted current canonical consumer is ready as a successor surface. It is
not yet safe to replace the existing current consumer in place. A terminal
successor cutover packet can describe a future manual cutover without changing
the active consumer or invoking live paths.

Current report facts:

- `cutover_preflight_ready=true`
- `direct_current_consumer_replacement_allowed=false`
- `direct_current_consumer_replacement_blocked=true`
- `dependency_cycle_detected=true`
- `terminal_successor_consumer_cutover_packet_required=true`
- `terminal_successor_consumer_cutover_packet_allowed=true`
- `current_canonical_consumer_replaced_in_place=false`
- `current_canonical_consumer_mutated=false`
- `promoted_current_canonical_consumer_mutated=false`
- `canonical_summary_mutated=false`
- `historical_canonical_gate_mutated=false`
- `execution_enabled_count=0`
- `public_ga_enabled_count=0`
- `canonical_gate_wrapper_invoked=false`
- `wrapper_target_invoked=false`
- `tool_execution_live_cutover_allowed=false`
- `tool_execution_public_ga_allowed=false`

## Preflight Rules

- The promoted current canonical consumer must be ready.
- The current canonical consumer must be ready.
- Direct replacement must stay blocked while it would create a dependency cycle.
- A terminal successor cutover packet may be created as report-only evidence.
- The historical alias must not be invoked.
- The target wrapper must not be invoked.
- Live cutover and Public GA remain disabled.

## Guardrails

- No historical patch replay.
- No patch body emission.
- No plugin fixture fabrication.
- No canonical summary mutation.
- No promoted post-canonical summary mutation.
- No current canonical consumer mutation.
- No promoted current canonical consumer mutation.
- No current canonical wrapper mutation.
- No promoted current canonical wrapper mutation.
- No current canonical closure mutation.
- No promoted current canonical closure mutation.
- No promoted current canonical closure index mutation.
- No historical canonical gate mutation.
- No strict-missing consumer mutation.
- No historical snapshot evidence write.
- No wrapper body emission by the report.
- No canonical gate invocation.
- No wrapper target invocation.
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
  `scripts/hepta-systems-promoted-current-canonical-consumer-cutover-preflight-report.sh`
- Gate:
  `scripts/hepta-systems-promoted-current-canonical-consumer-cutover-preflight-gate.sh`
- Sources:
  `scripts/hepta-systems-promoted-current-canonical-consumer-report.sh`
  and `scripts/hepta-systems-current-canonical-consumer-report.sh`

## Next Move

Create a terminal successor canonical consumer cutover packet without invoking
the restored alias, invoking the target wrapper, opening live URL paths,
starting long-soak paths, or promoting Public GA.
