# Hepta Browser WEB-E development progress — 2026-08-28

Status: `DRAFT_PR / DEVELOPMENT / QUALIFICATION_ONLY / EXACT_HEAD_EVIDENCE_PENDING`  
Branch: `codex/hepta-vnext-plan-browser-c0-c3-20260827`  
Pull request: `#1`  
Merge authorization: `false`

## Delta from the 2026-08-27 progress record

### C1-004B-1 build-input freezer

Added a standard-library, offline, create-only build-input binding tool. It freezes exact source/source-bundle, patch, MPL, SPDX SBOM, toolchain, target/profile/features, canonical build-command, and allowlisted environment inputs. Raw environment values are not emitted. Verification independently recomputes both the input packet and the existing worker build-manifest shape.

Local fixture evidence:

```text
Python syntax: PASS
fixture tests: 7/7 PASS
static contract verifier: PASS_FIXTURE_CONTRACT_ONLY
canonical Servo source bundle: absent
real worker artifact: absent
runtime qualification: false
```

### C1-004C-1 artifact-bound launch gate scaffold

Added a separate zero-third-party-dependency Rust crate that binds the child executable hash, embedded build-manifest hash, embedded source-receipt hash, reported child PID, and a private challenge before normal protocol traffic. It uses inherited Unix socketpairs, bounded read/write deadlines, normal shutdown/reap, and an intentional hung-child forced-kill/reap trial.

Local evidence is limited to static source verification because the execution container has no Rust toolchain. Compile, test, trial, and strict Clippy remain exact-head CI requirements. The crate does not link Servo and its embedded documents are qualification fixtures rather than real source/build receipts.

### Merge-gate correction

The blocking workflow is updated to call each reusable focused gate directly rather than relying on separately triggered workflows or prose claims. The required aggregation set includes C0-C3, C1 protocol, source receipt, source bundle, build-input manifest, artifact binding, and artifact launch gate in addition to the existing repository-wide checks.

## Current environmental evidence limitation

On the current PR head, all independent workflows finish as failures without any recorded steps, and job-log retrieval returns no log blob. This is treated as a runner, billing, policy, or Actions allocation failure—not as a PASS or a code-test failure. The PR remains Draft and all exact-head statuses remain pending.

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
5. capture and seal the real Linux build-input packet/manifest;
6. build the first local-fixture-only worker and complete SBOM;
7. generate and independently verify its artifact receipt;
8. repeat the build for reproducibility;
9. replace the launch-gate fixtures with the exact real receipts and executable;
10. create one local-fixture WebView, bind semantic actions to BrowserActor, and qualify teardown/sandbox/listener/egress/platform behavior.

All machine, runtime, product, effect, network, credential, operator, execute, promotion, and release authority remains false.
