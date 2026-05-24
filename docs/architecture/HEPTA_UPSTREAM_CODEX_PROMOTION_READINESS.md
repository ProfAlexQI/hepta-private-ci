# Hepta Upstream Codex Promotion Readiness

Readiness id: `upstream-codex-promotion-readiness`

This packet is the decision layer after
`upstream-codex-absorption-replay-readiness`. The previous gate proves that the
selected upstream Codex buckets have absorption contracts and translation or
replay evidence. This gate answers the next question: whether any selected
bucket is allowed to move from report-only intake into active Hepta behavior.

## Current Decision

- Assessed buckets: `4 / 4`
- Absorption/replay source readiness: `4 / 4`
- Required surface promotion packets: `4`
- Completed surface promotion packets: `0`
- Promotable buckets: `0`
- Promotion-blocked buckets: `4`
- Active promotion ready: `false`

The decision packet is ready, but active promotion is not open.

## Blocked Buckets

- `product-doc-release-governance`: release claims still require a dedicated
  operator-approved claim packet.
- `legacy-cli-tui-compatibility`: legacy CLI/TUI behavior needs a Hepta-native
  parity packet before use.
- `provider-credential-sandbox-security`: provider, credential, sandbox, and
  network deltas need a P0 security promotion packet.
- `runtime-session-tool-mcp-appserver`: runtime, session, tool, MCP, and
  app-server deltas need route/event promotion evidence.

## Required Promotion Packets

- `release-governance-claim-promotion-packet`
- `hepta-cli-tui-parity-promotion-packet`
- `provider-security-policy-promotion-packet`
- `runtime-appserver-route-event-promotion-packet`

## Boundaries

- No active Codex engine dependency
- No active runtime code wiring
- No active runtime dependency
- No active upstream auto-rebase
- No credential or secret read
- No provider invocation
- No channel delivery
- No gateway RPC
- No public release claim

Active behavior changes require per-surface promotion packets plus active
dependency isolation, watchdog, operator approval, and soak evidence.
