# Hepta Codex Fork

This repository is the Codex-derived Hepta fork. It is not a bridge layer around
the older Hepta runtime; Codex Rust source is the substrate being modified into
the new Hepta.

Current first-cut identity changes:

- Cargo builds a `hepta` binary from the former Codex CLI entrypoint.
- CLI help and key user-facing runtime messages identify as Hepta.
- Runtime home resolution uses `HEPTA_HOME` and defaults to `~/.hepta`.
- Internal crate names still use `codex-*` for the first migration step, so the
  fork stays buildable while behavior is moved over deliberately.

Next migration slices should replace behavior in place rather than add a
Hepta-to-Codex bridge:

1. Rebase install/update/doctor output and release packaging around Hepta.
2. Move provider/profile/session defaults from Codex assumptions to Hepta
   policy.
3. Port Hepta memory/runtime/plugin semantics into Codex core crates directly.
4. Only after the fork is stable, rename internal crates where the churn is
   worth it.
