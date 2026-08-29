# AuthBus P1.2 durable replay-ledger qualification

This nested Rust 1.95 workspace implements the **qualification-only** durable
state boundary required after AuthBus P1.1 signature verification.

It persists only already-verified public metadata, canonical bindings and
SHA-256 evidence digests in a private SQLite database configured for WAL and
`FULL` synchronous durability. It deliberately cannot represent private keys,
raw signatures, credentials, provider response bodies, secret values, or an
authority grant.

## Implemented durable ledgers

- purpose-inclusive verification-key identity across SQLite primary/foreign
  keys, Rust lookup/revocation APIs, durable receipts, epoch rotation and revocation;
- cross-purpose reuse of the same issuer/key/epoch without namespace collision;
- launch-nonce replay claims that survive process reopen;
- immutable operation bindings and provider-status revision history;
- independent operator/manual evidence revision history;
- terminal tombstones that survive detail compaction;
- generation-fenced writers, bounded capacity and CAS garbage collection;
- row-digest, reference and SQLite `quick_check` verification;
- deterministic pre-commit failure injection for crash-window tests.

## Qualification commands

```bash
cargo fmt --manifest-path Cargo.toml --all -- --check
cargo test --manifest-path Cargo.toml --locked --no-default-features --lib
cargo test --manifest-path Cargo.toml --locked --features p1-2-qualification --tests
cargo check --manifest-path Cargo.toml --locked --features p1-2-qualification --all-targets
cargo clippy --manifest-path Cargo.toml --locked --features p1-2-qualification --all-targets -- -D warnings
```

The lightweight source-contract job may use `ubuntu-slim`; inherited P1.1 and
P1.2 Rust builds use `ubuntu-24.04` because cold-cache all-target check and
Clippy are not valid within the slim runner's hard execution cap.

The crate is not a member of the product workspace and must not be wired into a
listener, provider call, OpenBao path, product writer, or production caller on
the strength of qualification evidence alone.
