# Hepta Upstream Codex Provider Security Replay

This packet translates the selected upstream Codex P0
`provider-credential-sandbox-security` bucket into local Hepta replay contracts.
It follows `scripts/hepta-upstream-codex-provider-security-absorption.sh`.

Selected changed paths: `104`

The replay scope is intentionally report-only. It does not promote upstream
provider, auth, sandbox, exec, or network behavior into the active Hepta
runtime.

## Replay Surfaces

- Redacted provider contracts: provider catalog, endpoint, model-family, and
  request-shape deltas become Hepta report fields with no secret values.
- Auth and credential redaction: login, config, permission, token, and profile
  deltas must replay through fixtures that prove credential redaction.
- approval-policy replay: allow/deny/escalation decisions remain dry-run
  evidence until an operator packet approves live mutation.
- sandbox and exec replay: command, sandbox, and Windows/Linux policy deltas
  must replay without spawning unbounded local processes.
- Network-proxy replay: network-proxy and live-network deltas default to deny
  until explicit Hepta policy and operator approval exist.
- Side-effect boundary replay: the gate proves no provider invocation, no
  credential read, no channel delivery, no gateway RPC, and no public release.

## Promotion Boundary

The current packet is a P0 replay and redaction gate, not a runtime integration.

- No credential value read.
- No secret file read.
- No provider invocation.
- No active provider promotion.
- No active security-policy promotion.
- No active runtime code wiring.
- No active Codex engine dependency.
- No public release claim.

Promotion remains blocked until Hepta has redacted provider contracts bound to
runtime report fields, sandbox/exec replay evidence, network-policy replay
evidence, active dependency isolation, operator approval, watchdog evidence, and
long soak evidence.
