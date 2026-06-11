# Hepta Memory/Intelligence/KG Runtime Provider-Router Shadow Context Activation Execution Controlled Route

This route exposes the controlled shadow context activation execution contract as native Hepta gateway evidence.

## Boundary

- It is a report route, not an activation command.
- It depends on the shadow execution readiness route being ready.
- It records that the source gate runs `execute_memory_context_activation_shadow` only against an isolated fixture.
- It does not invoke runtime execution from the live report route.
- It does not mutate live 7373 router state, live Memory, live KG, active binaries, launchd, providers, credentials, channels, or release artifacts.

## Native Route

Endpoint:

`/api/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-approved-shadow-context-activation-execution-controlled`

Source command:

`/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-approved-shadow-context-activation-execution-controlled --json`

Compatibility mode:

`native_runtime_provider_router_shadow_context_activation_execution_controlled_route_source_only`

## Required Preconditions

- Full runtime readiness is ready.
- The shadow execution readiness route is ready.
- Native gateway route/source-command counts are synchronized.
- Live mutation lanes remain zero.
- Runtime-owned shadow execution surface exists.
- Release gate, operator release approval, canary telemetry, rollback kill switch, post-activation watchdog/soak plan, idempotency, and 0ppm traffic remain required.

## Denied Effects

- Live report-route shadow execution invocation.
- Live activation command exposure.
- Provider/model invocation.
- Auth secret or credential reads.
- External network calls.
- Live KG writes.
- Live Memory writes.
- Channel or Telegram delivery.
- Service restart or active-binary mutation.
- Public release or public status claim.

## Gate

`scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-approved-shadow-context-activation-execution-controlled-route-gate.sh`

The gate:

- checks live runtime readiness for ready/0-lane/no-effect status;
- verifies native gateway source patterns for the controlled endpoint and denied effects;
- runs the native gateway endpoint test;
- runs the isolated hepta-runtime shadow activation execution fixture test;
- emits JSON evidence that distinguishes isolated source-gate execution from live report-route execution.

## Next Slice

After this gate and full preflight pass, a separate controlled live catch-up may expose the read-only controlled route on live 7373. That catch-up may replace/restart the active binary, but still must not enable live Memory/KG writes, invoke providers/models, read credentials, send channels, publish releases, or expose an activation command.
