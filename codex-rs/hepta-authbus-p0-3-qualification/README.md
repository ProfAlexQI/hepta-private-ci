# AuthBus B4 P0.3 qualification

This isolated crate implements the P0.3 scheduler safety tranche without changing the product workspace or granting runtime authority.

## Implemented

- canonical six-dimensional quota vector:
  `request_count`, `rpm`, `tpm`, `concurrency`, `day_budget`, `context`;
- exact idempotency replay that returns the original reservation snapshot;
- changed-payload conflict and changed-binding conflict without a second hold;
- durable-style reservation lifecycle snapshots;
- explicit dispatch-started and outcome-unknown phases;
- monotonic owner/generation rebind while old permits remain held;
- owner-authorized old-fence reconcile:
  - verified consumed -> complete and account bounded actual usage;
  - verified no effect -> release;
  - unknown -> retain the full hold;
- exact reconcile receipt replay and conflicting terminal evidence rejection;
- expiry scanning that releases only pre-dispatch reservations;
- post-dispatch and unknown reservations are never auto-released;
- invariant verification for held quota, active permits, idempotency records, and authority posture.

## Not implemented or authorized

This crate is not a production scheduler, listener, daemon, provider adapter, OpenBao client, credential reader, operator-accepted component, promotion candidate, or effect authority.

## Qualification

```bash
cargo fmt --manifest-path codex-rs/hepta-authbus-p0-3-qualification/Cargo.toml --all -- --check
cargo test --manifest-path codex-rs/hepta-authbus-p0-3-qualification/Cargo.toml --no-default-features --lib
cargo test --manifest-path codex-rs/hepta-authbus-p0-3-qualification/Cargo.toml --features p0-3-qualification --tests -- --nocapture
cargo check --manifest-path codex-rs/hepta-authbus-p0-3-qualification/Cargo.toml --features p0-3-qualification --all-targets
cargo clippy --manifest-path codex-rs/hepta-authbus-p0-3-qualification/Cargo.toml --features p0-3-qualification --all-targets -- -D warnings
```
