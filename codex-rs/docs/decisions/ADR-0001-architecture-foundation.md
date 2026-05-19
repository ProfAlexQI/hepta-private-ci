# ADR-0001: Hepta architecture foundation

- Status: Accepted
- Date: 2026-04-21
- Deciders: 齐教授, 发发

## Context

Hepta is intended to become a long-lived agent platform, not a one-off tool-calling shell.

The project should combine durable multi-surface execution, Rust-first runtime boundaries, persistent memory, product completeness, workflow packaging, and governance from day one.

Early decisions need to prevent three common failure modes:
1. a monolithic runtime with unclear boundaries,
2. memory and policy being bolted on too late,
3. plugin and workflow systems becoming tightly coupled to the main runtime.

## Decision

### 1. Core implementation language

Hepta uses **Rust** as the core implementation language.

Rust is the primary language for:
- kernel abstractions,
- runtime orchestration,
- scheduling,
- policy and approval boundaries,
- session and memory core,
- gateway core,
- plugin runtime,
- observability and eventing.

Python and TypeScript are allowed only as supporting layers:
- TypeScript for web UI and IDE-facing integrations,
- Python for experimental workflows, research tooling, and evaluation sidecars.

### 2. Initial repository topology

In `hepta-codex`, Hepta starts inside the existing `codex-rs` Rust workspace with 6 Hepta crates and the `codex-cli --bin hepta` entrypoint.

Initial workspace members:
- `codex-rs/hepta-core`
- `codex-rs/hepta-runtime`
- `codex-rs/hepta-memory`
- `codex-rs/hepta-gateway`
- `codex-rs/hepta-intelligence`
- `codex-rs/hepta-plugins`
- `codex-rs/cli` as the `hepta` binary entrypoint

### 3. Crate responsibilities

#### `hepta-core`
Stable abstractions only:
- model/provider traits,
- tool traits,
- channel/message traits,
- session and memory traits,
- policy traits,
- scheduler and plugin traits,
- shared runtime IDs/events/types,
- core error and config types.

`hepta-core` must not depend on any business crate.

#### `hepta-runtime`
Owns the runtime brain:
- agent loop,
- model routing,
- tool dispatch,
- approval flow,
- context assembly,
- background tasks,
- subagent orchestration,
- event emission.

#### `hepta-memory`
Owns state and long-term memory:
- session persistence,
- memory storage,
- search,
- compaction,
- memory adapters.

#### `hepta-gateway`
Owns external interaction surfaces:
- messaging adapters,
- webhook ingress,
- outbound delivery,
- message normalization,
- platform session binding.

#### `hepta-cli`
Owns local operator UX:
- interactive CLI,
- command registry,
- diagnostics,
- local session UX,
- model switching UX.

#### `hepta-plugins`
Owns extensibility:
- plugin loading,
- hooks,
- workflow package model,
- policy plugins,
- future marketplace alignment.

### 4. Initial core traits

The first architecture boundary is organized around these traits:
- `ModelProvider`
- `Tool`
- `Channel`
- `SessionStore`
- `MemoryStore`
- `PolicyEngine`
- `Scheduler`
- `Plugin`

### 5. Hard boundary rules

The following do **not** belong in `hepta-core`:
- concrete provider implementations,
- concrete tool implementations,
- Telegram/Discord/API adapter code,
- prompt templates and persona logic,
- workflow/skill content,
- database-specific connection logic.

### 6. Product stance

Hepta is positioned as:

> A modular agent operating system for persistent memory, governed tool use, multi-surface interaction, and reusable workflows.

## Consequences

### Positive
- strong long-term boundaries,
- memory and policy become first-class from day one,
- plugin system can evolve without being runtime-private,
- better fit for long-running, multi-session, multi-agent execution,
- easier future governance and observability.

### Tradeoffs
- slower early iteration than a Python-first monolith,
- more up-front design discipline required,
- plugin and workflow authoring will need careful ergonomics.

## Follow-up

1. Define the first public API surface for `hepta-core`.
2. Build a minimal vertical slice:
   - CLI input,
   - model call,
   - one tool,
   - one session store,
   - `/model` switching.
3. Add diagnostics (`doctor`) early.
4. Keep crate count stable until the first real runtime slice exists.
