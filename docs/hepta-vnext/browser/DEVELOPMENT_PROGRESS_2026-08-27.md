# Hepta Browser WEB-E development progress — 2026-08-27

Status: `DRAFT_PR / DEVELOPMENT / QUALIFICATION_ONLY / EXACT_HEAD_CHECKS_REQUIRED`  
Branch: `codex/hepta-vnext-plan-browser-c0-c3-20260827`  
Pull request: `#1`  
Merge authorization: `false`

## Implemented in this branch

### Repository-native planning and governance

- single active plan family `WEB-PLAN-2026-08-27E`;
- only `WEB-C0` through `WEB-C7` are active stage identifiers;
- repository-native current state, stage/traceability matrices, threat model, ADRs, state
  machines, security review, recovery/fuzz/platform plans, status records, and receipt schemas;
- Hepta CODEOWNERS and focused merge-blocking reusable workflows;
- separation of focused PR qualification from full integration-branch qualification.

### WEB-C0 / C2 / C3 deterministic qualification slice

- typed browser session/request/command/outcome/ref/snapshot contracts;
- deterministic local fixture engine with external HTTP(S) disabled;
- single-owner BrowserActor with exact session/generation/owner-epoch/page-revision fences;
- bounded human takeover and stale-ref rejection;
- exact request replay/conflict handling and sensitive-output denial;
- qualification-only activity/evidence receipt shapes and focused regression tests.

### WEB-C1.3 private worker protocol candidate

- standalone Rust crate with zero third-party dependencies;
- no Servo dependency, TCP/UDP/HTTP/WebDriver/CDP/public UDS/network/credential surface;
- 64 KiB maximum length-prefixed canonical binary frames;
- exact session/generation/owner-epoch/source-pin binding;
- redacted 32-byte startup capability and 32-byte host nonce;
- WorkerHello → HostAck → WorkerConfirm mutual startup handshake;
- local-fixture-only command vocabulary;
- traversal, oversized frame, malformed input, unknown authority, stale fence, wrong capability,
  and post-handshake injection rejection;
- thread-level Unix socketpair integration test.

### WEB-C1.3B real parent-child qualification trial

- real child process started from the qualification binary;
- anonymous inherited stdin/stdout pipes;
- bootstrap secret material sent through inherited pipe rather than argv/environment/files;
- exact handshake, Ping, Shutdown, outcome matching, channel close, and child reap;
- no Servo, WebView, network, sandbox claim, product caller, or runtime authority.

### C1-004A source receipt tooling

- exact fixed Servo repository/commit/tree/license expectation;
- clean checkout and untracked-file check;
- recursive Git tree inventory and domain-separated manifest digest;
- submodule/symlink counts, license byte digest, and embedded-signature-presence fact;
- compact canonical create-only receipt with duplicate-key and tamper rejection;
- local Git fixture tests only; no canonical Servo checkout has been receipted.

### C1-004B artifact-binding tooling

- source/build/artifact/patch/license/SBOM binding contract;
- build target/profile/toolchain/feature/command/environment hash fields;
- nonexecuting ELF/Mach-O/PE architecture inspection;
- symlink/hardlink/writable-artifact and target mismatch rejection;
- synthetic executable-header fixture tests only; no real Servo artifact exists.

## Evidence state

All focused workflows are configured as dependencies of `CI required`:

```text
hepta-browser-c0-c3.yml
hepta-browser-c1-protocol.yml
hepta-servo-source-contract.yml
hepta-servo-artifact-contract.yml
```

A PASS is valid only when the workflow and uploaded receipt are bound to the exact current PR
head/tree. Earlier failures or successes on older heads are immutable history and do not determine
the current candidate.

This file does not assert that the current head is green. Until exact-head results are observed and
sealed, all status records remain `EVIDENCE_PENDING`, the PR remains Draft, and
`merge_authorized=false`.

## Explicitly not implemented or qualified

- canonical Servo checkout source receipt;
- independently repeated Servo fetch/source comparison;
- deterministic source archive;
- actual applied-patch source tree;
- real Servo toolchain/native build;
- real worker executable, symbols, SBOM, license packet, or artifact receipt;
- executable/source correspondence and reproducible build comparison;
- real Servo worker behind the inherited protocol;
- one real WebView, renderer, storage/profile, or semantic adapter;
- OS-level listener/egress proof;
- Linux/macOS sandbox, Windows named-pipe SID ACL, resource limits, or descendant cleanup;
- bounded handshake/read/write/command/teardown deadlines and forced-kill qualification;
- parser fuzzing and allocation instrumentation;
- durable request tombstones, crash recovery, or checkpoint restore;
- browser product caller, App Server UI, persistent credentials, external browsing, downloads,
  upload, effect handling, operator acceptance, promotion, or release.

## Authority posture

The following remain false throughout the branch:

```text
machine_authority
runtime_authority
production_caller
production_writer
effect_authority
external_effect
external_network_allowed
credential_export_allowed
operator_acceptance
g5_allowed
execute_allowed
promotion
release_qualified
```

## Next executable sequence

1. obtain two independent exact Servo checkouts and run the source receipt tool;
2. compare complete Git tree manifests and independently validate signature trust;
3. create deterministic source archive, license/source-distribution packet, and empty patch
   inventory;
4. freeze build command/environment/toolchain/native dependency inputs;
5. build the first Linux local-fixture-only worker with network disabled during the build phase;
6. generate complete SPDX SBOM and artifact binding receipt;
7. independently repeat the build and explain binary differences;
8. replace the process-trial child with the artifact-bound Servo worker;
9. prove no listener/no egress and create exactly one local-fixture WebView;
10. bind real semantic observe/click/type behavior to BrowserActor revision fencing;
11. repeat on macOS and implement the Windows named-pipe/SID-ACL equivalent;
12. only after these gates consider C4 network policy or any product caller.
