# Hepta Browser C1 private worker protocol

This standalone, dependency-free Rust crate qualifies the first process boundary required by
`WEB-C1`. It is deliberately outside the production workspace while the Servo source/import and
artifact qualification gates remain open.

## What it implements

- a versioned, length-prefixed binary protocol with a 64 KiB hard frame bound;
- exact `BrowserSessionId + generation + owner_epoch` process fencing;
- exact pinned Servo commit/tree fields;
- a redacted 256-bit startup capability;
- a second 256-bit host nonce and mutual three-message handshake;
- a fixed qualification-only authority bitset;
- local-fixture-only navigation and bounded semantic commands;
- stale identity rejection before any frame is written;
- Unix `socketpair` qualification tests with no listener.

## What it does not implement

- Servo source import, Servo compilation, rendering, or a real WebView;
- TCP, HTTP, WebDriver, CDP, public UDS, network discovery, or remote control;
- external HTTP(S) navigation or DNS;
- cookies, profile bytes, credentials, unrestricted JavaScript, file access, upload, or download;
- production caller/writer, effect authority, G5, operator acceptance, promotion, or release.

The protocol is an implementation input for the future pinned Servo worker. A passing test run is
not `WEB-C1` completion and is not a release receipt.

## Local qualification

```sh
cargo fmt --manifest-path tools/hepta-browser-c1-protocol/Cargo.toml -- --check
cargo test --locked --manifest-path tools/hepta-browser-c1-protocol/Cargo.toml
cargo clippy --locked --manifest-path tools/hepta-browser-c1-protocol/Cargo.toml \
  --all-targets -- -D warnings
python3 scripts/verify-hepta-browser-c1-protocol.py
```
