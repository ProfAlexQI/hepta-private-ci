# Hepta Browser WEB-E development progress — 2026-08-28

Status: `DRAFT_PR / DEVELOPMENT / QUALIFICATION_ONLY / EXACT_HEAD_EVIDENCE_PENDING`  
Branch: `codex/hepta-vnext-plan-browser-c0-c3-20260827`  
Pull request: `#1`  
Merge authorization: `false`

## Delta from the 2026-08-27 progress record

### C1-004B-1 build-input freezer

The canonical standard-library, offline, create-only build-input binding tool freezes exact source/source-bundle, patch, MPL, SPDX SBOM, toolchain, target/profile/features, canonical build-command, and allowlisted environment inputs. Raw environment values are not emitted. Verification independently recomputes both the input packet and the worker build-manifest shape.

The strict canonical entrypoint now additionally requires:

```text
cargo build | cargo rustc
--locked
--offline
unique feature list
no registry/acquisition operation
no NUL/CR/LF ambiguity
no secret-bearing environment key
CARGO_NET_OFFLINE=true
```

Local fixture evidence:

```text
Python syntax: PASS
engine fixture tests: 7/7 PASS
strict policy fixture tests: 8/8 PASS
combined build-input fixture tests: 15/15 PASS
static contract verifier: updated; exact-head evidence pending
canonical Servo source bundle: absent
real worker artifact: absent
runtime qualification: false
```

A simplified parallel build-manifest implementation created during development review was removed after the pre-existing source-bundle-aware engine was identified as the stronger authority. `scripts/hepta-servo-worker-build-inputs.py` is the sole strict entrypoint and delegates to `scripts/hepta-servo-worker-build-manifest.py` as its serialization/recomputation engine.

### C1-004C-1 artifact-bound launch gate scaffold

A separate zero-third-party-dependency Rust crate binds the child executable hash, embedded build-manifest hash, embedded source-receipt hash, reported child PID, and a private challenge before normal protocol traffic. It uses inherited Unix socketpairs, bounded read/write deadlines, normal shutdown/reap, and an intentional hung-child forced-kill/reap trial.

Local evidence is limited to static source verification because the execution container has no Rust toolchain. Compile, test, trial, and strict Clippy remain exact-head CI requirements. The crate does not link Servo and its embedded documents are qualification fixtures rather than real source/build receipts.

### Process-trial and workflow wiring repairs

`tools/hepta-browser-c1-protocol/Cargo.toml` declared `autobins = false` but had not explicitly declared `hepta-browser-c1-process-trial`; the process integration test therefore could not receive its `CARGO_BIN_EXE_*` binding. The binary is now explicit. The C1 private-protocol workflow now exposes `workflow_call` and can be consumed by the blocking aggregator.

### Merge-gate correction

The blocking workflow calls every reusable focused gate directly rather than relying on separately triggered workflows or prose claims. The required aggregation set now includes:

```text
C0-C3 deterministic fixture
C1 private protocol
C1 artifact-bound launch gate
Servo source receipt
Servo source bundle
Servo worker build-input packet
Servo worker artifact binding
full Hepta vNext qualification
existing Bazel/cargo-deny/codespell/repository/Rust/SDK gates
```

A dependency-free verifier checks reusable-workflow declarations, aggregator job/`needs` wiring, absence of superseded gate names, C1 binary declaration, and CODEOWNERS coverage.

## Current environmental evidence limitation

On the observed PR heads, independent workflows finish as failures without recorded steps, and job-log retrieval returns no log blob. This is treated as a runner, billing, policy, or Actions allocation failure—not as a PASS or a code-test failure. The PR remains Draft and all exact-head statuses remain pending.

## Still not implemented or qualified

- two successful independent canonical Servo checkouts and sealed source receipts;
- deterministic canonical Servo archive and complete distribution packet evidence;
- real Linux worker build input packet and manifest;
- real Servo worker executable, symbols, complete SPDX SBOM, and artifact receipt;
- independent reproducibility build;
- artifact-bound real Servo worker process;
- one real local-fixture WebView and semantic observe/click/type integration;
- sandbox, no-listener/no-egress OS evidence, profile isolation, resource limits, parent-death and descendant cleanup;
- macOS receipt and Windows SID-restricted named-pipe/Job-Object equivalence;
- C4 policy/secrets/network, C5 durability, C6 product caller, C7 operator acceptance/promotion/release.

## Immediate execution order

1. restore an Actions runner capable of entering the first step;
2. rerun all exact-head fixture contracts and Rust gates;
3. manually dispatch the acknowledged two-fetch source qualification;
4. seal the canonical source bundle;
5. capture and seal the real Linux build-input packet/manifest through the strict entrypoint;
6. build the first local-fixture-only worker and complete SBOM;
7. generate and independently verify its artifact receipt;
8. repeat the build for reproducibility;
9. replace the launch-gate fixtures with the exact real receipts and executable;
10. create one local-fixture WebView, bind semantic actions to BrowserActor, and qualify teardown/sandbox/listener/egress/platform behavior.

All machine, runtime, product, effect, network, credential, operator, execute, promotion, and release authority remains false.
