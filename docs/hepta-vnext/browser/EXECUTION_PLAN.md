# Hepta Browser WEB-D Execution Plan

Plan ID: `HEPTA-BROWSER-WEB-D`  
Revision: `1.0`  
Phase: `DEVELOPMENT`  
Authority: `qualification-only`  
Canonical pointer: `docs/hepta-vnext/browser/CURRENT.yaml`

## 1. Decision and present state

Hepta will build one typed browser subsystem around Servo. Servo supplies standards-based rendering and page execution; Hepta owns authority, session lifecycle, semantic projection, secrets, egress, evidence, recovery, product integration and release qualification.

The first implemented slice is deliberately bounded:

- **WEB-C0** typed contracts and negative-authority defaults;
- **WEB-C2 fixture** bounded semantic page observations;
- **WEB-C3 fixture** single-owner actor, four-way fencing, human takeover and request idempotency;
- deterministic qualification engine with external network and raw-cookie export denied.

This is useful executable development, but it is not WEB-C1. No Servo source or binary is imported, no listener is started, no production caller exists, and no release authority is granted.

## 2. Product goals

1. Let an Agent and a human safely share one visible page without concurrent mutation races.
2. Expose a stable, bounded semantic page model rather than unrestricted DOM or WebDriver access.
3. Preserve authenticated sessions without exposing raw credentials to the Agent or model.
4. Make every command, observation and outcome traceable to one session generation, owner epoch and page revision.
5. Deny unknown network, file, secret and browser-profile access by default.
6. Produce redacted, digest-bound evidence suitable for qualification and incident analysis.
7. Support deterministic crash recovery without repeating an indeterminate external effect.
8. Qualify one exact source/artifact set across Linux, macOS and Windows before product promotion.

## 3. Non-goals

- Browser automation is not a generic remote-control or scraping service.
- Servo/WebDriver types are not Hepta public contracts.
- Page text cannot mint authority, override policy or request credentials.
- The browser subsystem does not become a workflow scheduler, wallet, fleet authority, model runtime or secret store.
- Qualification receipts do not imply operator acceptance or production promotion.
- C0/C2/C3 fixture tests do not qualify real Servo behavior.

## 4. Architecture

```text
Codex / Hepta product caller (future C6)
                 |
       typed BrowserRequest/Response
                 |
      Browser Actor — sole session owner
        |       |       |       |
    policy   secrets  journal  evidence
        |       |       |       |
     Servo adapter / private transport (C1)
                 |
        pinned Servo runtime + sandbox
                 |
          hostile web content/network
```

The Browser Actor is the only mutation owner. Read-only observations may be served concurrently only from an immutable snapshot bound to a page revision. The Servo adapter cannot grant authority; it reports typed facts and outcomes.

## 5. Core contracts

A mutating request contains at least:

```text
schema_version
request_id
session_id
generation
owner_epoch
page_revision
actor_kind
command
policy_digest
issued_at/deadline
```

The canonical request digest is computed over the complete typed request. The actor caches a bounded mapping from request ID to digest and response. Repeating the same ID and digest returns the same response; reusing the ID with another digest fails closed.

A semantic reference contains at least:

```text
session_id
page_revision
node_id
role/name/value/state digest
```

It is invalid after revision change. The actor does not silently relocate a stale reference.

## 6. Control modes

### Agent-controlled

Agent mutations are accepted only when session, generation, epoch and revision match the current actor state and no human lease is active.

### Human-controlled

Acquiring a human lease increments the owner epoch. Agent mutations are immediately denied. The lease is bounded and visible through status. Expiry or explicit release creates a new current state; old Agent requests remain stale.

### Fenced or draining

Supervisor generation changes, teardown, crash, integrity failure or policy revocation fence the session. New mutations fail. Recovery creates a new generation/epoch and never reuses stale in-memory authority.

## 7. Semantic model

The initial semantic projection is a bounded tree/list of interactive and informative nodes with stable IDs only within one page revision. Fields are size-limited and redacted. Role, accessible name, value/state class, frame/origin classification and action affordances are explicit. Hidden secrets, password values, authorization material and unrestricted script objects are excluded.

C2 fixture implementation proves contract behavior only. Real C2 qualification requires Servo-backed extraction, cross-origin frame separation, dynamic DOM invalidation, accessibility mapping, shadow DOM behavior, form state and malicious-page tests.

## 8. Servo C1 integration plan

The source candidate is pinned in `SERVO_UPSTREAM_PIN.json`. Before import:

1. Record source/tree, license, notices, toolchain, lockfiles, patches and SBOM.
2. Choose one minimal integration seam through an ADR. Preferred order:
   - private in-process/embedder API with a Hepta facade;
   - private UDS/named-pipe sidecar;
   - loopback-only ephemeral WebDriver bridge with explicit authentication and an allowlisted command subset.
3. Prohibit Servo's current wildcard WebDriver bind.
4. Define process ownership, sandbox, filesystem roots, egress and resource limits.
5. Define startup, health, cancellation, crash, drain and teardown state machines.
6. Add source parity and update qualification. Upstream changes never auto-promote.

C1 definition of done is a real pinned Servo artifact that renders a local fixture, produces a typed observation and executes a bounded action while no wildcard listener, external egress, credential export or production caller is enabled.

## 9. Policy and secret boundary (C4)

C4 introduces a typed `BrowserPolicySnapshot` frozen at session/command admission. It binds:

- allowed origins and destination classes;
- redirect, DNS, proxy and TLS rules;
- request methods and body limits;
- upload/download capabilities and staging roots;
- script/evaluation capability;
- clipboard, camera, microphone, geolocation and notification policy;
- SecretRef audience, purpose, expiry and origin;
- evidence and redaction policy.

Unknown or changed policy fails closed. Credentials are injected inside the process boundary through opaque references. The Agent receives only success/failure classes and redacted evidence.

## 10. Durability and recovery (C5)

The durable journal records intent before crossing the Servo/network boundary and records an outcome afterward. States include:

```text
Prepared -> Dispatched -> Completed
                    \-> Failed
                    \-> Indeterminate -> Reconcile
```

An indeterminate action is never blindly replayed. Recovery checks session generation, policy, page revision, provider/browser state and any observable idempotency key. Reconciliation either proves completion, proves non-execution, quarantines the session, or requires human review.

Snapshots and receipts use canonical bytes, bounded inventories, SHA-256 digests, private ownership and atomic replace/fsync rules appropriate to each platform.

## 11. Product caller (C6)

C6 exposes only typed operations through the existing Codex/Hepta product path. The UI shows:

- current URL/origin classification;
- Agent/human/fenced control mode;
- generation, owner epoch and page revision diagnostics;
- degraded policy, network or evidence state;
- pending human takeover and reconciliation requirements.

Product integration must not silently fall back to an unrestricted external browser or WebDriver service.

## 12. Qualification and release (C7)

Each receipt conforms to `hepta.browser.qualification_receipt.v1.schema.json` and binds:

- exact Hepta commit/tree;
- exact Servo commit/tree when applicable;
- target, toolchain, build profile and features;
- artifact/SBOM/patch digests;
- test commands, results and logs;
- malicious-site, crash, resource and privacy results;
- known gaps;
- explicit negative authority flags.

Required qualification classes:

1. contract/property/golden tests;
2. real Servo local-fixture tests;
3. malicious site and prompt-injection corpus;
4. egress/DNS/redirect/loopback tests;
5. secret and evidence byte scans;
6. crash-point and recovery matrix;
7. CPU/RSS/disk/startup/latency limits;
8. Linux/macOS/Windows build and smoke tests;
9. supply-chain and license verification;
10. exact product caller tests.

Operator acceptance and promotion are separate ceremonies after all receipts bind one release candidate. Neither is produced by this plan or the current implementation.

## 13. Performance and capacity budgets

Initial development ceilings, to be replaced by measured platform profiles:

- one actor per session;
- bounded command queue and idempotency cache;
- semantic snapshot node count and serialized bytes bounded;
- typed text, extracted text, title, URL and node fields bounded;
- navigation/action deadline mandatory;
- browser process RSS, CPU and disk quota explicit;
- per-Agent and per-workspace session quotas explicit;
- evidence retention and total bytes explicit.

Exceeding a bound produces a typed denial or fenced/degraded state, never silent truncation of an authority-relevant value.

## 14. Work breakdown

### Immediate—complete current PR

- compile and test C0/C2/C3 fixture slice;
- validate canonical plan bundle in CI;
- ensure current default branch and PRs trigger Hepta qualification;
- retain all authority flags false.

### Next PR—C1 bootstrap

- add Servo source/artifact provenance tooling;
- write private transport ADR;
- introduce a minimal Servo adapter trait implementation and local fixture smoke test;
- add listener exposure and egress scans;
- no product caller.

### Following PRs

- real semantic projection (C2);
- real actor/Servo integration and takeover UI seam (C3);
- policy/SecretRef/egress (C4);
- journal/recovery/evidence (C5);
- product caller (C6);
- platform/release qualification (C7).

## 15. Stop conditions

Development stops and the candidate is marked `BLOCKED` when:

- source, tree, toolchain or policy binding is unknown;
- Servo starts a wildcard or unauthenticated listener;
- raw secret/cookie/profile bytes cross the boundary;
- more than one mutation owner exists;
- stale ref, generation or request conflict is accepted;
- external egress occurs without a bound policy;
- an indeterminate external effect is replayed blindly;
- evidence cannot be reproduced from the exact candidate;
- a qualification-only result is represented as production authority.
