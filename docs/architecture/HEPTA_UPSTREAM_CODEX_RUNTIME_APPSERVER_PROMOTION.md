# Hepta Upstream Codex Runtime/App-Server Promotion Packet

Promotion id: `runtime-appserver-route-event-promotion-packet`

This packet is the P0 per-surface promotion decision for the
`runtime-session-tool-mcp-appserver` bucket. It consumes
`upstream-codex-runtime-appserver-replay-packet` and proves that the local
promotion prerequisites are documented without enabling live runtime,
app-server, tool, MCP, channel, or gateway side effects.

## Source

- Selected bucket: `runtime-session-tool-mcp-appserver`
- Selected changed paths: `462`
- Source replay gate: `scripts/hepta-upstream-codex-runtime-appserver-replay.sh`
- Promotion gate: `scripts/hepta-upstream-codex-runtime-appserver-promotion.sh`

## Promotion Conditions

- App-server route and event contract ready
- Session thread lifecycle contract ready
- Tool/MCP request envelope ready
- Exec hook event-loop replay ready
- Adapter shadow replay ready
- Operator approval model ready
- Side-effect boundary ready

Ready promotion conditions: `7 / 7`
Promotion packet ready: `true`

## Active Promotion Decision

Active runtime promotion allowed: `false`
Active app-server promotion allowed: `false`
Active tool/MCP promotion allowed: `false`

This packet closes the runtime/app-server route-event promotion packet
prerequisite, but it does not wire upstream runtime behavior into the active
Hepta service.

## Remaining Blockers

- Active runtime route/event wiring is not part of this packet.
- Live app-server promotion remains forbidden.
- Live tool and MCP invocation promotion remains forbidden.
- Gateway RPC and channel delivery remain forbidden.

## Boundaries

- No credential value read
- No secret file read
- No provider invocation
- No channel delivery
- No gateway RPC
- No active Codex engine dependency
- No public release claim

The next promotion step requires active route/event adapter parity evidence and
explicit operator approval before any live app-server, tool, or MCP behavior.
