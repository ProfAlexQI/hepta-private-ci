# Hepta / Codex Core Fusion Route - 2026-05-23

## Verdict

Core fusion is now feasible, but it should be executed as ownership inversion,
not as a one-shot deletion of Codex.

The target is:

```text
hepta product runtime
  -> hepta-kernel / hepta-runtime / hepta-gateway
  -> codex-engine adapter
  -> Codex model/session/tool/sandbox/thread subsystems
```

In that shape, Hepta is the product and root runtime owner. Codex remains a
powerful internal engine and compatibility layer until each engine surface has a
Hepta-owned trait boundary and replacement path.

## Current Facts

The current crate graph already supports the first phase:

- `hepta-core` is payload-light and depends only on `serde` / `serde_json`.
- `hepta-kernel` depends on `hepta-core`, `hepta-intelligence`,
  `hepta-memory`, and `hepta-plugins`.
- `hepta-runtime` depends on the Hepta crates and owns most runtime projection
  wrappers.
- `hepta-gateway` depends on `hepta-runtime` and owns the external gateway
  surface.
- `codex-cli --bin hepta` remains the active binary entrypoint and still owns
  direct references to `codex-core`, `codex-exec`, `codex-tui`, `codex-state`,
  app-server, MCP, sandboxing, and other Codex engine crates.

This means the already-finished kernel migrations are directionally correct:
Hepta has been absorbing pure policy, report, readiness, Telegram, and
native-post contracts into Hepta-owned crates. The remaining base-dependency is
mostly entrypoint and engine ownership, not the Hepta domain model.

## Non-Negotiable Boundaries

- Do not silently cross the public-release claim boundary.
- Do not silently cross the real `task_publish` mutation boundary.
- Keep gateway file I/O, network calls, Bot API sends, cursor writes, launchd
  mutation, and package install side effects outside `hepta-kernel`.
- Keep Codex subsystems working through compatibility routes until a Hepta
  trait boundary proves parity.
- Keep live gates green after each slice: preflight, release build, watchdog,
  soak, GA readiness, operator packet, packaging, legacy closure, and browser
  visual smoke.

## Fusion Phases

### Phase 1 - Root Ownership Inversion

Goal: make Hepta the conceptual and code-level entrypoint owner while still
using Codex as an internal engine.

First patches:

- Add a Hepta-owned product-runtime facade in `hepta-runtime`.
- Route `codex-rs/cli/src/main.rs` through that facade before falling into the
  Codex subcommand and TUI machinery.
- Add a machine-readable fusion readiness report that distinguishes:
  - Hepta-owned root surfaces,
  - Codex engine adapter surfaces,
  - still-direct Codex base dependencies,
  - forbidden real side effects.

Acceptance:

- Existing `hepta` CLI behavior remains compatible.
- The gateway reports Hepta as root owner and Codex as internal engine.
- No public release claim or real native-post mutation occurs.

### Phase 2 - Engine Adapter Boundary

Goal: stop letting the Hepta entrypoint scatter direct Codex calls through the
binary layer.

First patches:

- Introduce `CodexEngineAdapter`-style traits/types behind Hepta runtime.
- Wrap model/session/tool/sandbox/thread-store calls behind Hepta-owned adapter
  functions.
- Preserve old Codex commands as compatibility dispatch until the adapter proves
  parity.

Acceptance:

- `codex-cli` direct dependency count remains high, but live execution routes
  go through a Hepta-owned adapter boundary.
- Tests can assert that root routing uses Hepta-owned dispatch before invoking
  Codex engine functions.

Current landed state:

- `/api/hepta-codex-engine-adapter-boundary` enumerates the adapter surfaces.
- All six adapter surfaces are now marked
  `adapter_threaded_compatibility_dispatch`: model-provider execution,
  session/thread-store, tool invocation, sandbox/exec, MCP/app-server, and
  legacy TUI/CLI.
- The `hepta` CLI entrypoint, `exec`, `review`, resume/fork interactive flows,
  sandbox commands, MCP/app-server commands, debug model/state/app-server paths,
  apply, stdio bridge, and exec-server dispatch call Hepta-owned adapter
  threading plans before entering Codex compatibility dispatch.
- All six adapter surfaces now also carry typed adapter request/response
  envelopes before compatibility dispatch.
- All six adapter surfaces now expose reportable typed adapter parity gates
  while still keeping `adapter_parity_complete=false` until real parity
  criteria are promoted.
- The live watchdog now fails if the adapter boundary route loses typed
  envelopes, typed parity gates, no-live-mutation status, or forbidden
  side-effect guardrails.
- The adapter boundary route now also reports promotion criteria and blockers
  for `adapter_parity_complete=true`; current state is intentionally
  `adapter_parity_promotion_ready=false` until per-surface compatibility
  evidence goes beyond typed envelope/gate presence.
- Per-surface compatibility evidence records are now reported for all six
  surfaces and enforced by the watchdog, covering typed envelope readiness,
  typed parity gate readiness, compatibility dispatch checks, live-mutation
  blocking, and forbidden side-effect blocking.
- Those evidence records now include behavior-equivalence checks for each
  surface: provider selection/invocation policy, session persistence intent,
  tool approval/side-effect classification, sandbox policy, MCP/app-server
  route shape, and legacy CLI command classification. The watchdog enforces
  that these checks are present and marked as preserving observable behavior.
- The preflight matrix now has a dedicated adapter behavior-equivalence gate:
  it runs targeted runtime and native-gateway tests before the broader gateway
  and UI/native app checks, requiring exact per-surface behavior evidence while
  keeping `adapter_parity_complete=false`.
- These plans are side-effect-free and preserve current Codex provider/session,
  tool, sandbox, MCP/app-server, and legacy TUI semantics; they do not invoke
  providers, read credentials, mutate session stores, perform external reads,
  publish public release state, or cross `task_publish` real mutation.

### Phase 3 - Binary / Package Inversion

Goal: move from `codex-cli --bin hepta` to a first-class Hepta binary crate.

First patches:

- Add or promote a `hepta-cli` binary crate.
- Make the old `codex-cli` entrypoint a compatibility shell.
- Keep install paths and launchd labels stable during migration, then flip
  package naming only after live parity gates pass.

Acceptance:

- The installed binary still serves `127.0.0.1:7373`.
- The live service can be rebuilt and restarted from the Hepta binary crate.
- Legacy Codex commands still resolve through compatibility shims.

### Phase 4 - Name and Repository Closure

Goal: make `hepta-codex` a historical transition name, not the architectural
identity.

First patches:

- Rename operator-facing runtime strings from `hepta-codex` to `hepta` after
  compatibility gates are stable.
- Keep internal `codex-engine` naming only for adapter diagnostics.
- Archive transition docs and keep rollback evidence.

Acceptance:

- Product, binary, service, docs, and route reports consistently say Hepta is
  the root runtime owner.
- Codex is visible only as a contained engine/provider compatibility layer.

## Immediate Safe Next Slice

Continue Phase 2:

1. Add a promotion-only adapter parity completion gate that consumes the
   behavior-equivalence preflight evidence and still refuses promotion until
   live shadow replay or equivalent stronger coverage exists.
2. Keep direct Codex dependencies explicit until adapter parity has enforced
   evidence beyond typed envelope presence.
3. Run focused checks, full preflight, release build, live install, and live
   gates before advancing toward binary/package inversion.
