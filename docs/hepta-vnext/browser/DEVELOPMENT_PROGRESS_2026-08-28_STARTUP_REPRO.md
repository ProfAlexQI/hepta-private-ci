# Hepta Browser C1 development progress — startup bridge and reproducibility

Date: 2026-08-28  
Plan: `WEB-PLAN-2026-08-27E`  
Phase: `DEVELOPMENT`  
Authority: qualification-only, all runtime/product/effect/network/operator/promotion/release flags false

## Implemented in this slice

### Artifact-to-browser startup bridge

A new standalone Rust qualification crate now composes the two previously separate C1 gates in the required order:

```text
exact executable/build/source/PID binding
  -> one-process artifact challenge
  -> BrowserSessionId/generation/owner-epoch/source-pin handshake
  -> bounded Ping/Shutdown
  -> deadline/kill/reap
```

Files:

```text
tools/hepta-browser-c1-startup-bridge/
scripts/verify-hepta-browser-c1-startup-bridge.py
docs/hepta-vnext/browser/C1_STARTUP_BRIDGE.md
docs/hepta-vnext/browser/C1_STARTUP_BRIDGE_STATUS.json
docs/hepta-vnext/browser/C1_STARTUP_BRIDGE_TRACEABILITY.json
docs/hepta-vnext/browser/hepta.browser.c1_startup_bridge_receipt.v1.schema.json
.github/workflows/hepta-browser-c1-startup-bridge.yml
```

The implementation uses anonymous inherited Unix socketpairs, exact spawned-PID validation, independent executable binding, one-use artifact challenge, private browser capability/nonce, exact Servo commit/tree, and bounded force-kill/reap handling. It explicitly keeps `servo_linked=false` and uses embedded qualification fixture receipts, so it is not a real Servo startup result.

### Strict worker-build reproducibility contract

A standard-library comparator now accepts two distinct build replica roots and one compact-canonical manifest. It produces a receipt only when every declared binary, canonical JSON, and text output is byte-for-byte identical.

Files:

```text
scripts/hepta-servo-worker-reproducibility.py
scripts/tests/test_hepta_servo_worker_reproducibility.py
scripts/verify-hepta-servo-worker-reproducibility.py
docs/hepta-vnext/browser/C1_REPRODUCIBILITY.md
docs/hepta-vnext/browser/C1_REPRODUCIBILITY_STATUS.json
docs/hepta-vnext/browser/NEXT_WORK_QUEUE_C1_REPRODUCIBILITY.json
docs/hepta-vnext/browser/hepta.servo.worker_reproducibility_manifest.v1.schema.json
docs/hepta-vnext/browser/hepta.servo.worker_reproducibility_receipt.v1.schema.json
.github/workflows/hepta-servo-worker-reproducibility-contract.yml
```

The v1 comparison policy is deliberately strict:

```text
require_byte_identical=true
allow_missing_optional=false
allow_explained_differences=false
```

Local fixture validation covers exact recomputation, create-only receipts, binary drift, positive runtime claim rejection, and unsorted manifest rejection. No real worker build has been compared.

### Protocol bootstrap-secret lifecycle

`StartupCapability` is no longer `Copy`, its bytes are zero-filled on drop, and host nonce structures now have explicit redacted `Debug` implementations and zero-fill on drop. A new regression test checks known capability and nonce byte patterns never appear in Debug output.

Files:

```text
tools/hepta-browser-c1-protocol/src/protocol.rs
tools/hepta-browser-c1-protocol/tests/secret_redaction.rs
docs/hepta-vnext/browser/C1_PROTOCOL_SECRET_LIFECYCLE.md
```

This is defense-in-depth rather than a formal memory-erasure proof.

## Evidence state

The Python reproducibility comparator was locally syntax-checked and its fixture contract/self-test/unit suite was executed before submission. The new Rust startup bridge and the protocol lifecycle changes have not received exact-head rustfmt/compile/test/Clippy evidence in GitHub Actions because repository jobs continue to fail before steps.

Accordingly:

```text
startup bridge implementation = IMPLEMENTED_UNQUALIFIED
real worker reproducibility comparisons = 0
real Servo artifact = unavailable
real Servo runtime = not integrated
merge_authorized = false
operator_acceptance = false
promotion = false
release_qualified = false
```

## Immediate next executable boundary

1. restore the GitHub runner/account/policy path and run exact-head focused checks;
2. seal two independent canonical Servo source receipts and one source-bundle receipt;
3. produce a real worker build manifest and complete SPDX/native/license packet;
4. build two independent worker replicas;
5. seal both artifact receipts and the byte-identical reproducibility receipt;
6. replace startup-bridge fixture receipts with those immutable real receipts;
7. only then start one real local-fixture Servo WebView.

Any mismatch or absent evidence keeps C1 blocked and authority false.
