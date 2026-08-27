# WEB-C1 private protocol bootstrap-secret lifecycle

Status: **implementation hardened; exact-head compile/test/Clippy evidence pending**

## Change

The protocol bootstrap values are now treated as secret-bearing values rather than ordinary copyable structs:

- `StartupCapability` is `Clone` but no longer `Copy`;
- capability bytes are overwritten in `Drop`;
- `HostExpectedWorker`, `HostAck`, and `WorkerConfirm` use explicit `Debug` implementations;
- startup capability and host nonce are rendered only as `<redacted>`;
- nonce arrays are overwritten in `Drop`;
- `worker_hello()` performs an explicit capability clone, making duplication visible in source review;
- integration tests verify known capability/nonce byte patterns do not appear in Debug output.

## Boundary

This hardening reduces accidental logging and ordinary Rust-value lifetime exposure. It is not a complete memory-erasure proof. The following remain open:

- compiler optimization may remove or transform plain zero-fill stores;
- allocator, serializer, kernel pipe, core-dump, debugger, swap, VM snapshot and process-inspection copies are not proven absent;
- the qualification protocol still carries the raw capability once over an inherited private channel;
- OS-level peer identity, artifact-handle binding and platform crash-dump policy require separate qualification.

No new authority is introduced. The change does not link Servo, start a listener, permit external network, export credentials, create a production caller, or qualify the runtime.

## Required evidence

The C1 protocol workflow must run on the exact head and pass:

```text
cargo fmt --check
cargo test --locked
cargo run process trial
cargo clippy --all-targets -- -D warnings
```

Until that evidence exists, the implementation status is `IMPLEMENTED_UNQUALIFIED` and merge remains unauthorized.
