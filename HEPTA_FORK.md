# Hepta Codex Fork

This repository is the upstream-Codex-derived Hepta fork. It is not a bridge
layer around the older Hepta runtime; the imported Rust source is the substrate
being modified into the new Hepta.

Current first-cut identity changes:

- Cargo builds a `hepta` binary from the imported CLI entrypoint.
- CLI help and key user-facing runtime messages identify as Hepta.
- Runtime home resolution uses `HEPTA_HOME` and defaults to `~/.hepta`.
- Login browser pages, entitlement errors, MCP/plugin marketplace help, and
  debug-build update errors identify as Hepta.
- Auth keyring storage now uses a Hepta service name for new credentials.
- Runtime defaults now recognize `HEPTA_DEFAULT_MODEL_PROVIDER` and
  `HEPTA_DEFAULT_MODEL` as Hepta-owned code-default policy inputs, with legacy
  `CODEX_*` names kept only as fallbacks.
- Internal crate names still use `codex-*` for the first migration step, so the
  fork stays buildable while behavior is moved over deliberately.

Next migration slices should replace behavior in place rather than add a
Hepta-to-upstream bridge:

1. Continue replacing remaining user-visible upstream product strings in TUI/app-server
   surfaces where they are not protocol compatibility names.
2. Continue moving provider/profile/session defaults from upstream assumptions
   to Hepta policy. The first provider/model env-policy slice has landed; the
   next work should inspect session persistence, profile-v2 naming, and hosted
   runtime defaults.
3. Port Hepta memory/runtime/plugin semantics into the core crates directly.
4. Only after the fork is stable, rename internal crates where the churn is
   worth it.
