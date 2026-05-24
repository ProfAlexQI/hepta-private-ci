# Hepta Upstream Codex Runtime Appserver Replay

This packet translates the selected upstream Codex P0
`runtime-session-tool-mcp-appserver` bucket into local Hepta replay contracts.
It follows `scripts/hepta-upstream-codex-runtime-appserver-absorption.sh`.

Selected changed paths: `462`

The replay scope is report-only. It does not promote upstream app-server,
session, tool, MCP, exec, hook, or runtime event-loop behavior into the active
Hepta service.

## Replay Surfaces

- app-server protocol replay: schema, request, notification, and route-event
  deltas become Hepta report contracts before any protocol promotion.
- app-server daemon and transport replay: daemon lifecycle and transport
  changes stay behind route/event contracts and side-effect checks.
- session thread-store replay: session, thread-store, lifecycle, resume,
  rollback, and archive deltas must replay against bounded fixtures.
- tool invocation replay: tool-policy, permission, request-envelope, and result
  deltas must replay before any active invocation promotion.
- MCP client server replay: MCP client, server, and request-envelope deltas
  remain local replay evidence until operator-approved wiring exists.
- exec-server hook replay: exec-server, hook, and runtime event-loop deltas
  must replay without spawning live runtime work.
- Side-effect boundary replay: the gate proves no credential read, no provider
  invocation, no channel delivery, no gateway RPC, no active dependency
  regression, and no public release.

## Promotion Boundary

The current packet is a P0 runtime replay gate, not a runtime integration.

- No active runtime promotion.
- No active app-server promotion.
- No active tool/MCP promotion.
- No active runtime code wiring.
- No active Codex engine dependency.
- No provider invocation.
- No channel delivery.
- No gateway RPC.
- No public release claim.

Promotion remains blocked until Hepta has route/event adapter contracts,
session/thread-store replay evidence, tool/MCP replay evidence, app-server
protocol replay evidence, exec/hook replay evidence, active dependency
isolation, operator approval, watchdog evidence, and long soak evidence.
