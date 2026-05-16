# Hepta Codex Fork

Hepta is a local agent runtime being rebased directly onto the upstream Codex
Rust workspace. The fork still keeps crate and package names such as
`codex-cli` while the user-facing binary, runtime home, login surfaces, and
app-server behavior move to Hepta in place.

## Quickstart

Build the forked Rust CLI directly:

```shell
cargo build -p codex-cli --bin hepta --manifest-path codex-rs/Cargo.toml
./codex-rs/target/debug/hepta --help
```

Set `HEPTA_HOME` to override the runtime home. Without it, Hepta uses
`~/.hepta`. Legacy `CODEX_HOME` remains a compatibility fallback while the
fork is being migrated.

Packaged Hepta release artifacts are not staged yet. Do not install upstream
Codex release archives as a substitute for this fork unless you intentionally
want the upstream product.

## Local Docs

- [Hepta fork notes](./HEPTA_FORK.md)
- [Rebrand residual audit](./HEPTA_REBRAND_RESIDUAL_AUDIT_2026-05-17.md)
- [Rust workspace notes](./AGENTS.md)
- [App-server API docs](./codex-rs/app-server/README.md)

This repository is licensed under the [Apache-2.0 License](LICENSE).
