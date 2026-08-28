# AuthBus P1.1 signed identity qualification

This nested workspace implements the first executable AuthBus P1 boundary:

- strict Ed25519 verification over the existing canonical `IdentityBinding`;
- issuer, audience, service identity, policy, process-peer, epoch and fence binding;
- current-key-epoch and revocation enforcement;
- bounded launch-nonce replay protection that fails closed at capacity;
- signed provider-status evidence with revision/time anti-replay;
- independent operator-signed manual evidence that can only resume lookup-only,
  keep manual review, or quarantine.

It is qualification-only. The default feature set compiles only negative
authority constants. The opt-in `p1-qualification` feature does not open a
listener, call a provider or OpenBao, load a private key, join the parent
workspace, or grant execution authority.

## Qualification

```bash
python3 scripts/verify-authbus-p1-1.py

cargo fmt \
  --manifest-path codex-rs/hepta-authbus-p1-qualification/Cargo.toml \
  --package codex-hepta-authbus-p1-qualification -- --check

cargo test --locked \
  --manifest-path codex-rs/hepta-authbus-p1-qualification/Cargo.toml \
  --no-default-features --lib -- --nocapture

cargo test --locked \
  --manifest-path codex-rs/hepta-authbus-p1-qualification/Cargo.toml \
  --features p1-qualification --tests -- --nocapture

cargo check --locked \
  --manifest-path codex-rs/hepta-authbus-p1-qualification/Cargo.toml \
  --features p1-qualification --all-targets

cargo clippy --locked \
  --manifest-path codex-rs/hepta-authbus-p1-qualification/Cargo.toml \
  --features p1-qualification --all-targets -- -D warnings
```
