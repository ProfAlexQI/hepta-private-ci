# Hepta Systems Gate Recursion Cost Boundary Readback

This note closes the local readback step after the recovery-receipt gate proved
that new gates can validate source report invariants without recursively
invoking the full upstream source-gate chain.

## Scope

- Surface: `hepta_systems_gate_recursion_cost_boundary_readback`.
- Source facts:
  - recovery-receipt gates are bounded to source report invariants;
  - recovery-window and older source gates can still invoke upstream gate chains;
  - current matrix and dashboard reports still require a full matrix render;
  - heavy Hepta systems gates are serialized by the `hepta-systems` lane lock.
- Projection: 4 cost-boundary entries are exposed as a queryable local readback.

## Boundary

This is a readback-only gate recursion cost boundary. It measures and classifies
the current cost shape without writing a matrix cache, persisting compact cache
state, changing source report semantics, executing workflow or replay, or
opening live paths.

The closed boundary is explicit: no matrix cache write, compact cache persistence, source report semantic change, full upstream gate-chain invocation, workflow execution, replay execution, event-log write, SQLite write, provider invocation, model invocation, Gateway/Auth mutation, Native POST mutation, Telegram transport mutation, package, release, Public GA promotion, or live execution.

## Current Next Gate

After the gate-recursion cost boundary is represented in the matrix, the next
local gate is
`hepta_systems_matrix_report_single_render_cache_boundary_readback`.

That gate should make a single-render matrix summary queryable without
persisting cache state, changing matrix semantics, writing event-log/SQLite,
running workflow/replay, invoking providers or models, mutating transports,
releasing, canarying, or opening live execution.
