# Hepta Upstream Codex R7 Semantic Absorption Record

## Source

- Intake:
  `docs/architecture/HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_2026-07-24_R7.json`
- Frozen range:
  `6c729ef1c1dcfbcbe1bd9d0c2dddde24377ae899..f201c30c52a35f819262865a53df94b6f4ea7a50`
- Integration mode: selective semantic evaluation across unrelated roots. No
  merge, rebase, or cherry-pick was performed.

## Existing Narrow Equivalence

R7 fixes an upstream `McpRuntime` path where an explicit refresh could reuse an
existing connection. Hepta does not have that path. Its explicit refresh:

1. queues a complete refresh configuration;
2. consumes it at the next turn boundary;
3. constructs a new `McpConnectionManager` with no previous-manager reuse
   input;
4. atomically swaps the new manager into the session; and
5. cancels and shuts down the old manager.

Commit `c3d57839b0c1` adds executable evidence for that existing behavior:

- consecutive explicit refresh requests each replace the manager startup
  generation;
- a real Apps Streamable HTTP transport receives two distinct `initialize`
  requests across initial startup and an explicit same-configuration refresh.

This is recorded as `existing_semantic_equivalent`, not as a production-code
transplant.

## Fail-Closed Deferral

The full R7 commit also relies on the R6 published-runtime architecture and
preserves a force-reconnect request if replacement is cancelled. Hepta does not
yet have:

- immutable published runtime snapshots;
- serialized desired/applied refresh generations;
- recoverable refresh intent across cancellation or startup failure;
- authoritative authentication generation;
- same-generation binding between the exposed catalog and executing client; or
- an immediate ready/publication signal for idle-thread refresh.

Copying the upstream `AtomicBool` alone would not close those races and would
put ownership in the wrong layer. That broader scope remains
`deferred_prerequisite_unavailable`.

## Verification

- `cargo test -p codex-core explicit_refresh_rebuilds_mcp_connections_on_each_next_turn`
  — 1 passed.
- `cargo test -p codex-core --test all explicit_refresh_reinitializes_the_same_apps_transport`
  — 1 passed.
- `just fix -p codex-core` — passed.
- `just fmt` — passed; stable Rust emitted only the existing
  `imports_granularity` warnings.

## Non-Claims

- R7 is not fully absorbed.
- R6 background prewarm and authentication-rotation semantics remain deferred.
- Refresh is not immediate for an idle thread.
- Cancellation-safe intent restoration and cross-generation credential safety
  are not claimed.
- No deployment, restart, publication, signing, or live enablement occurred.
