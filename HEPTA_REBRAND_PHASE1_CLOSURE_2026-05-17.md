# Hepta Rebrand Phase 1 Closure - 2026-05-17

This note closes the first direct Hepta rebrand pass over the upstream-Codex
source fork. The goal of this phase was not to rename every Rust crate or wire
schema. The goal was to make the active local product present itself as Hepta
while preserving compatibility with upstream package, API, hosted-service, and
stored-data contracts.

## Current baseline

- Repo: `/Users/qianqi/.openclaw/workspace/hepta-codex`
- Branch: `main`
- Latest phase-1 commit: `6ca5e25 refactor: rebase analytics local types to hepta`
- Remote: none configured
- Working tree at closure time: clean

## Landed identity surface

- CLI binary, top-level help, login, doctor, MCP/plugin help, app-server,
  TUI, config/home defaults, key user-facing docs, prompts, model/runtime
  defaults, and local debug/test helper surfaces now present Hepta as the active
  runtime.
- Local runtime homes now prefer `HEPTA_HOME`, `HEPTA_SQLITE_HOME`,
  `HEPTA_OSS_*`, `HEPTA_DEFAULT_MODEL_PROVIDER`, and `HEPTA_DEFAULT_MODEL`,
  while keeping legacy `CODEX_*` inputs as fallbacks where needed.
- Upstream self-update/download behavior is fail-closed for this source fork
  unless a Hepta-owned update channel is provided.
- Analytics sending is explicit opt-in for the Hepta fork instead of defaulting
  to upstream Codex analytics.
- Product policy now resolves normal local sessions as `Product::Hepta` while
  matching Codex-restricted marketplace assets for compatibility.

## Compatibility boundary

Keep these names unless there is a dedicated migration plan:

- Crate, package, module, and Bazel identifiers such as `codex-core`,
  `codex-tui`, `codex_protocol`, and `codex_app_server_protocol`.
- Wire/API fields and generated schema names such as `codexHome`,
  `codexErrorInfo`, `codexStreamlinedLogin`, and `codex_cli_simplified_flow`.
- Hosted service paths and event schemas such as `/codex/device`,
  `/codex/analytics-events/events`, `codex_*_event`, and
  `codex_rs_version`.
- Compatibility type names such as `CodexAuth`, `CodexErr`,
  `CodexErrorInfo`, plus the public analytics aliases
  `CodexCompactionEvent` and `CodexTurnSteerEvent`.
- Legacy storage/input paths such as `.codex` and `CODEX_HOME` when they are
  documented fallbacks, project metadata conventions, or stored-data migration
  surfaces.
- Model ids and hosted URLs that literally include Codex, such as
  `gpt-5.1-codex` or ChatGPT Codex billing/usage endpoints.

## Evidence

- Targeted residual scan for daemon pid fixtures now leaves only crate/import
  compatibility in `codex-rs/app-server-daemon`.
- Targeted analytics residual scan for
  `CodexRuntimeMetadata|CodexCompactionEvent|CodexTurnSteerEvent` leaves only
  the deliberate public aliases for compaction and turn-steer events.
- Broad residual scan still finds many Codex strings, but the dominant classes
  are crate/module names, wire schema, hosted service identifiers, compatibility
  fallbacks, and tests for those contracts. That is expected at the end of phase
  1.

## Verification used during closure

- `cargo fmt --all --check --manifest-path codex-rs/Cargo.toml`
- `cargo test --offline -q -p codex-app-server-daemon --manifest-path codex-rs/Cargo.toml`
- `cargo test --offline -p codex-analytics --manifest-path codex-rs/Cargo.toml -- --list`
- Targeted analytics tests:
  - `compaction_event_serializes_expected_shape`
  - `compaction_event_ingests_custom_fact`
  - `accepted_turn_steer_emits_expected_event`
- `cargo check --offline -q -p codex-analytics --manifest-path codex-rs/Cargo.toml`
- `cargo build --offline -q -p codex-cli --bin hepta --manifest-path codex-rs/Cargo.toml`
- `git diff --check`

## Next phase candidates

The next phase should be chosen deliberately rather than mechanically:

1. Add Hepta-owned schema aliases only where external clients can consume them
   without breaking existing Codex-compatible clients.
2. Rename internal Rust crates/modules in batches only after deciding whether
   `codex-rs` remains the source-root compatibility name.
3. Stage Hepta-owned hosted endpoints before changing ChatGPT Codex URLs,
   analytics event names, billing/usage links, or device auth paths.
4. Migrate stored `.codex` project metadata only with a backward-compatible
   read/write plan and rollback path.
