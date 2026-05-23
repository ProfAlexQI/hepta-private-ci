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
- `hepta-cli --bin hepta` is now the active gateway release package. `codex-cli
  --bin hepta` remains a compatibility test surface and still owns direct
  references to `codex-core`, `codex-exec`, `codex-tui`, `codex-state`,
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

- `/api/hepta-engine-adapter-boundary` is now the canonical Hepta-named
  adapter-boundary route; `/api/hepta-codex-engine-adapter-boundary` is
  retained as a compatibility alias.
- `scripts/hepta-*.sh` wrappers now front the transition
  `scripts/hepta-codex-*.sh` release gate family.
- `/api/hepta-name-repository-closure` now marks the engine-adapter route and
  release-gate script family as `alias_active`; remaining Phase 4 blockers are
  the runtime report strings, core-fusion document title, and workspace
  repository directory name.
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
  and those gates are part of the promoted adapter parity evidence chain.
- The live watchdog now fails if the adapter boundary route loses typed
  envelopes, typed parity gates, no-live-mutation status, or forbidden
  side-effect guardrails.
- The adapter boundary route now also reports promotion criteria and blockers
  for `adapter_parity_complete=true`; current state has no remaining adapter
  parity promotion blockers because behavior evidence and shadow replay cover
  all six surfaces.
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
  keeping full fusion separate from adapter parity promotion.
- The adapter boundary now also exposes a promotion-only completion gate:
  `adapter_behavior_equivalence_to_parity_completion_gate`. It is ready as a
  guardrail, its status is
  `ready_adapter_parity_promoted_full_fusion_pending_binary_package_inversion`,
  and `adapter_parity_completion_gate_allows_promotion=true`.
- Stronger shadow replay coverage is now present for all six adapter surfaces:
  `model_provider_execution`, `session_thread_store`, `tool_invocation`,
  `sandbox_exec`, `mcp_app_server`, and `legacy_tui_cli`. Each replay compares
  the threading plan with the typed request/response envelope, preserves
  compatibility dispatch, and proves provider invocation, credential reads,
  session-store writes, external reads, and live mutation remain blocked. The
  boundary reports `adapter_shadow_replay_covered_surface_count=6`, but
  promotion is now accepted by the dedicated adapter parity decision slice.
- The adapter boundary now reports `adapter_parity_complete=true`,
  `adapter_parity_promotion_ready=true`, and an empty
  `adapter_parity_promotion_blockers` set. It still reports
  `full_fusion_complete=false` because binary/package inversion is a separate
  phase.
- These plans are side-effect-free and preserve current Codex provider/session,
  tool, sandbox, MCP/app-server, and legacy TUI semantics; they do not invoke
  providers, read credentials, mutate session stores, perform external reads,
  publish public release state, or cross `task_publish` real mutation.

### Phase 3 - Binary / Package Inversion

Goal: move from `codex-cli --bin hepta` to a first-class Hepta binary crate.

First patches:

- Report and gate the current binary/package ownership state before moving
  code: active package `codex-cli`, active target `hepta`, intended package
  `hepta-cli`, intended target `hepta`.
- Add or promote a `hepta-cli` binary crate.
- Make the old `codex-cli` entrypoint a compatibility shell.
- Keep the launchd label stable during migration, then move the active service
  binary path to the first-class Hepta install once live parity gates pass.

Acceptance:

- The installed binary still serves `127.0.0.1:7373`.
- The live service can be rebuilt and restarted from the Hepta binary crate.
- Legacy Codex commands still resolve through compatibility shims.

Current landed state:

- `/api/hepta-core-fusion-readiness` now reports
  `phase=phase_3_binary_package_inversion`.
- Phase 3 package ownership is now complete for the active gateway release:
  `phase_3_binary_package_inversion_ready=true`.
- The live route reports a dedicated
  `hepta_first_class_binary_package_inversion_gate`, with status
  `ready_hepta_cli_release_package_ownership_active`.
- The active release package is `hepta-cli` with target `hepta`; `codex-cli`
  remains a compatibility test surface rather than the package that carries the
  installed gateway release.
- The active service binary path is now expected to be
  `/Users/qianqi/.local/opt/hepta/bin/hepta`; the old
  `/Users/qianqi/.local/opt/hepta-codex/bin/hepta-codex` path is retained as a
  transition rollback anchor, not the primary launch target.
- Watchdog now checks this Phase 3 state in addition to adapter parity, so full
  fusion cannot regress to Codex package ownership silently.
- A first-class `hepta-cli` package now exists in the workspace and builds a
  `hepta` binary for the native gateway `--serve-ui` entrypoint. Non-gateway
  legacy CLI shell coverage is still tracked separately and does not block the
  active gateway release package.

### Phase 4 - Name and Repository Closure

Goal: make `hepta-codex` a historical transition name, not the architectural
identity.

First patches:

- Runtime reports and control UI copy now identify the active runtime as
  `hepta`.
- Keep internal `codex-engine` naming only for adapter diagnostics.
- Keep this dated transition doc as rollback evidence while the canonical
  Hepta-named route note lives at `docs/architecture/HEPTA_CORE_FUSION_ROUTE.md`.

Acceptance:

- Product, binary, service, docs, and route reports consistently say Hepta is
  the root runtime owner.
- Codex is visible only as a contained engine/provider compatibility layer.

Current landed state:

- `/api/hepta-name-repository-closure` is now the Phase 4 inventory gate. It is
  side-effect-free and does not rename files, mutate launchd, publish release
  state, read credentials, or invoke models.
- The closure report marks the active binary package transition as closed:
  `codex-cli --bin hepta` has been replaced by `hepta-cli --bin hepta` for the
  active service artifact.
- The same report keeps full fusion blocked only for the remaining workspace
  repository directory cutover. Runtime report strings now expose `hepta`, this
  document has a Hepta-named canonical successor, and the engine-adapter route
  slug plus release gate script family have active Hepta-named aliases.
- `/api/hepta-core-fusion-readiness` now includes the Phase 4 closure gate and
  blocker list while still reporting `phase_4_name_repository_closure_ready=false`
  and `full_fusion_complete=false`.

## Immediate Safe Next Slice

Advance after Phase 3:

1. Finish the workspace repository directory cutover from
   `/Users/qianqi/.openclaw/workspace/hepta-codex` to a verified Hepta-owned
   active checkout path, without overwriting the existing
   `/Users/qianqi/.openclaw/workspace/Hepta` repository.
2. Keep direct Codex dependencies explicit as internal engine-adapter
   compatibility surfaces until repository/name closure is complete.
3. Port or retire non-gateway legacy CLI shell compatibility that still routes
   through `codex-cli`.
4. Run focused checks, full preflight, release build, live install, and live
   gates before claiming full fusion.
