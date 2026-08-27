# Hepta Browser C1 artifact launch gate — qualification only

This zero-dependency crate qualifies a fail-closed Unix parent/child launch gate that binds:

- the child executable SHA-256;
- embedded qualification build-manifest SHA-256;
- embedded qualification source-receipt SHA-256;
- the exact spawned child PID;
- a private 256-bit challenge;
- qualification-only negative authority.

It runs a normal authenticated ping/shutdown/reap trial and a hung-child deadline/kill/reap trial. It does **not** link Servo, create a WebView, use a real worker artifact, open a listener, access the network, export credentials, or grant production/effect/operator/promotion/release authority.

The embedded JSON files under `fixtures/` are deliberate qualification fixtures. Replacing them with real receipts is blocked until the canonical Servo source bundle, build-input manifest, worker artifact receipt, and reproducibility evidence exist.

```sh
cargo fmt -- --check
cargo test --locked
cargo run --locked --bin hepta-browser-c1-artifact-bound-trial
cargo run --locked --bin hepta-browser-c1-artifact-bound-trial -- --force-kill-trial
cargo clippy --locked --all-targets -- -D warnings
```
