# Hepta inference INF-0 reference contracts

This standalone qualification crate implements the backend-neutral, no-network
reference contracts for `HEPTA-INFERENCE-RUNTIME-V2`.

It is deliberately outside the product Cargo workspace until exact-head
qualification passes. It performs no model loading, filesystem access, network
access, process spawning, inference effect, Memory/KG write, routing change or
production action.

Run:

```sh
cargo fmt --manifest-path tools/hepta-inference-inf0/Cargo.toml -- --check
cargo test --manifest-path tools/hepta-inference-inf0/Cargo.toml --locked
cargo clippy --manifest-path tools/hepta-inference-inf0/Cargo.toml \
  --all-targets --locked -- -D warnings
```

A source PASS is qualification-only and grants no runtime or promotion
authority.
