# AuthBus P0.2 SQLite WAL qualification

This crate is an **opt-in, qualification-only** durable coordinator for AuthBus B3/B4/B5 safety properties.

It is deliberately a nested Cargo workspace and is not part of the product workspace. The default feature set exposes only negative-authority constants. The SQLite implementation is compiled only with `sqlite-qualification`.

## What it models

- atomic admission, quota hold, and token-family claim;
- an fsync-backed dispatch-attempt witness before any provider boundary may be crossed;
- accepted, terminal, verified-no-effect, and unknown response markers;
- lookup-only recovery after any durable dispatch witness;
- monotonic status observations;
- terminal quota settlement and claim release;
- stable outbox delivery with cursor compare-and-swap;
- writer boot/generation fencing and row-digest corruption checks.

## What it does not do

It opens no listener, calls no provider, resolves no SecretRef, reads no raw credential, and grants no production, effect, operator, promotion, G5, or execution authority.

## Qualification commands

```bash
cargo fmt --manifest-path codex-rs/hepta-authbus-qualification/Cargo.toml --all -- --check
cargo test --manifest-path codex-rs/hepta-authbus-qualification/Cargo.toml --no-default-features --lib
cargo test --manifest-path codex-rs/hepta-authbus-qualification/Cargo.toml --features sqlite-qualification --tests -- --nocapture
cargo check --manifest-path codex-rs/hepta-authbus-qualification/Cargo.toml --features sqlite-qualification --all-targets
cargo clippy --manifest-path codex-rs/hepta-authbus-qualification/Cargo.toml --features sqlite-qualification --all-targets -- -D warnings
```

A source commit, a successful local command, or a GitHub job with `runner_id=0` / `steps=[]` is not production evidence.
