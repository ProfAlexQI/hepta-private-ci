# Hepta Upstream Codex R8 Semantic Absorption Record

## Source

- Intake:
  `docs/architecture/HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_2026-07-24_R8.json`
- Manifest SHA-256:
  `60833a3504bca61ed33f527d6bc9315193540b839650e92280eeedb6cf10dba3`
- Upstream range:
  `f201c30c52a35f819262865a53df94b6f4ea7a50..c8957bbf0f79fa29c5e08b8c0b942c12ea3893f2`
- Intake commit:
  `5fe7506f63e4bf10ed351b203a48639b217f2002`

No merge, rebase, or cherry-pick was performed.

## Existing Semantic Equivalence

`c8957bbf0f79` moves pending invalidation, serialized refresh publication, and
cancellation recovery into upstream's `McpRefresh` abstraction.

Hepta commit `16364d4f2599` already implements the active-session behavior with
a stronger generation-bound model:

- one session-owned lock serializes replacement publication;
- desired and applied generations remain distinct;
- cancellation or invalid configuration leaves the exact desired generation
  pending;
- a superseded generation cannot publish;
- catalog exposure and dispatch bind to the same manager generation.

Focused cancellation, rapid-refresh, connection-rebuild, and stale-catalog
tests passed. This is a bounded semantic equivalence, not a source import.

## Deferred Scope

The following R8 structure remains intentionally deferred:

- explicit closeable refresh-gate behavior during shutdown;
- moving the generation state and serialization gate behind one coordinator
  module/API.

The local coordinator currently uses adjacent `Session` fields and task
teardown rather than a closeable semaphore. Structural consolidation should
follow the R6 authentication-generation contract so it does not churn or
weaken the security-critical publication boundary.

## Non-Claims

- R6 authentication refresh and background prewarm are not closed by R8.
- No byte-for-byte or type-level equivalence with upstream `McpRefresh` is
  claimed.
- Upstream is not fully consumed.
- No deployment, restart, publication, signing, push, or live enablement was
  performed.
