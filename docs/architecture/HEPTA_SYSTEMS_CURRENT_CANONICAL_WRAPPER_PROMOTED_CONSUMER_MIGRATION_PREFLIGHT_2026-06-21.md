# Hepta Systems Current Canonical Wrapper Promoted Consumer Migration Preflight - 2026-06-21

This note records the local-only Promoted Consumer Migration Preflight for the
current canonical wrapper. The preflight checks whether
`scripts/hepta-systems-current-canonical-wrapper-report.sh` can directly consume
the promoted current canonical consumer.

The answer is no: direct replacement would create a dependency cycle. The
current canonical consumer already depends on the current wrapper through the
closure, alias readback index, and post-canonical closure summary chain.

The preflight does not invoke `scripts/hepta-systems-canonical-gate.sh` and does
not invoke `scripts/hepta-systems-current-canonical-wrapper-gate.sh`.

## Current Checkout Reality

The post-canonical closure summary is the promoted current canonical consumer,
but the existing current wrapper is still part of that consumer's evidence
chain. Replacing the wrapper source in place would make the wrapper depend on a
consumer that depends on the wrapper.

Current report facts:

- `migration_preflight_ready=true`
- `direct_current_wrapper_source_replacement_allowed=false`
- `direct_current_wrapper_source_replacement_blocked=true`
- `dependency_cycle_detected=true`
- `successor_wrapper_surface_required=true`
- `successor_wrapper_surface_allowed=true`
- `successor_wrapper_surface=promoted_current_canonical_wrapper`
- `current_canonical_wrapper_mutated=false`
- `canonical_summary_mutated=false`
- `historical_canonical_gate_mutated=false`
- `canonical_gate_wrapper_invoked=false`
- `wrapper_target_invoked=false`
- `tool_execution_live_cutover_allowed=false`
- `tool_execution_public_ga_allowed=false`

## Migration Rules

- The current canonical wrapper must be ready.
- The current canonical consumer must be ready.
- Direct replacement of the current wrapper source must stay blocked when it
  would create a dependency cycle.
- A successor wrapper surface may consume the promoted consumer without replacing
  the source that the promoted consumer already depends on.
- The historical alias must not be invoked.
- The target wrapper must not be invoked.
- Live cutover and Public GA remain disabled.

## Guardrails

- No historical patch replay.
- No patch body emission.
- No plugin fixture fabrication.
- No canonical summary mutation.
- No current canonical wrapper mutation.
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
  `scripts/hepta-systems-current-canonical-wrapper-promoted-consumer-migration-preflight-report.sh`
- Gate:
  `scripts/hepta-systems-current-canonical-wrapper-promoted-consumer-migration-preflight-gate.sh`
- Sources:
  `scripts/hepta-systems-current-canonical-wrapper-report.sh`
  and `scripts/hepta-systems-current-canonical-consumer-report.sh`

## Next Move

Create a non-circular promoted current canonical wrapper successor without
invoking the restored alias, invoking the target wrapper, opening live URL paths,
starting long-soak paths, or promoting Public GA.
