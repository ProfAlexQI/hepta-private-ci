# Hepta

Hepta is a local agent runtime derived from an upstream Codex Rust workspace
snapshot. It is not maintained by directly rebasing or merging upstream
history. Security, protocol, sandbox, app-server, and provider changes are
selected and adapted behind Hepta's own compatibility and side-effect gates.

The workspace still keeps compatibility crates and packages such as
`codex-cli`, while the active user-facing binary and package are owned by
`hepta-cli` and the Hepta runtime.

## Quickstart

Build the first-class Hepta CLI directly:

```shell
cargo build -p hepta-cli --bin hepta --manifest-path codex-rs/Cargo.toml
./codex-rs/target/debug/hepta --help
```

Set `HEPTA_HOME` to override the runtime home. Without it, Hepta uses
`~/.hepta`. Legacy `CODEX_HOME` remains a compatibility fallback while the
fork is being migrated.

Local Hepta release artifacts are built and checked by the
backend+Native+release preflight. The active local service path is
`~/.local/opt/hepta/bin/hepta`. A green local artifact is not a public release:
code signing, notarization, publication, installation, restart, and public
release claims remain separate operator-controlled steps.

The Rust workspace keeps the upstream-compatible internal version `0.0.0`.
It is not a Hepta public release version. Public Hepta semantic versioning will
start only with a signed release candidate.

## Upstream synchronization

Use the `upstream` remote only for read-only comparison and selective
absorption. Do not bulk rebase or merge upstream Codex into Hepta. The policy,
baseline, and validation requirements are recorded in
[`HEPTA_UPSTREAM_CODEX_SYNC_LANE.md`](./docs/architecture/HEPTA_UPSTREAM_CODEX_SYNC_LANE.md).

## Local Docs

- [Hepta fork notes](./HEPTA_FORK.md)
- [Rebrand residual audit](./HEPTA_REBRAND_RESIDUAL_AUDIT_2026-05-17.md)
- [Rust workspace notes](./AGENTS.md)
- [App-server API docs](./codex-rs/app-server/README.md)

This repository is licensed under the [Apache-2.0 License](LICENSE).
