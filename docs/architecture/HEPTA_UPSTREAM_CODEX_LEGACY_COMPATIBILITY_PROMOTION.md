# Hepta Upstream Codex Legacy Compatibility Promotion Packet

Promotion id: `hepta-cli-tui-parity-promotion-packet`

This packet is the P1 per-surface promotion decision for the
`legacy-cli-tui-compatibility` bucket. It consumes
`upstream-codex-legacy-compatibility-replay-packet` and proves that the local
promotion prerequisites are documented without enabling live CLI, TUI,
code-mode, channel, or gateway side effects.

## Source

- Selected bucket: `legacy-cli-tui-compatibility`
- Selected changed paths: `128`
- Source replay gate: `scripts/hepta-upstream-codex-legacy-compatibility-replay.sh`
- Promotion gate: `scripts/hepta-upstream-codex-legacy-compatibility-promotion.sh`

## Promotion Conditions

- CLI command contract parity ready
- TUI presentation parity ready
- code-mode callback boundary ready
- Terminal helper contract ready
- Adapter shadow replay ready
- Operator approval model ready
- Side-effect boundary ready

Ready promotion conditions: `7 / 7`
Promotion packet ready: `true`

## Active Promotion Decision

Active CLI/TUI promotion allowed: `false`
Active TUI presentation promotion allowed: `false`
Active code-mode promotion allowed: `false`

This packet closes the Hepta CLI/TUI parity promotion packet prerequisite, but
it does not promote retained upstream CLI, TUI, or code-mode behavior into the
active Hepta service.

## Remaining Blockers

- Active CLI/TUI command promotion is not part of this packet.
- Live TUI presentation promotion remains forbidden.
- code-mode callback promotion remains forbidden.
- Gateway RPC and channel delivery remain forbidden.

## Boundaries

- No credential value read
- No secret file read
- No provider invocation
- No channel delivery
- No gateway RPC
- No active Codex engine dependency
- No public release claim

The next promotion step requires active Hepta-native CLI/TUI parity evidence
and explicit operator approval before any live command, presentation, or
code-mode behavior.
