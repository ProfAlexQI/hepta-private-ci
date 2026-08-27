# Hepta Browser WEB-D Execution Plan

Plan ID: `HEPTA-BROWSER-WEB-D`  
Revision: `1.2`  
Phase: `DEVELOPMENT`  
Claim level: `L1_QUALIFICATION_ONLY`  
Authority: all runtime, effect, caller, writer, network, credential, operator,
promotion and release flags are false  
Canonical pointer: `docs/hepta-vnext/browser/CURRENT.yaml`

## 1. Decision

Hepta will build one typed browser subsystem around one exact Servo runtime per
browser session. Servo supplies standards-based page execution and rendering.
Hepta owns all authority, admission, session lifecycle, process ownership,
semantic projection, human/Agent arbitration, secrets, egress, durability,
evidence, recovery, product integration and release qualification.

The architecture rejects:

- a public or loopback TCP/WebSocket WebDriver service;
- raw WebDriver, Servo or DOM types as Hepta contracts;
- more than one page mutation owner;
- a hybrid Servo/Obscura live page;
- automatic external-network access;
- raw cookie, profile, credential or authorization export;
- qualification evidence as production authority.

## 2. Current implementation state

### Implemented, not yet independently qualified

- **WEB-C0:** typed request, response, command, action, semantic snapshot,
  semantic reference and receipt contracts with negative authority defaults;
- **WEB-C2 fixture:** bounded deterministic semantic observations and redacted
  extraction behavior;
- **WEB-C3 fixture:** single-owner Actor, session/generation/owner-epoch/page-
  revision fences, bounded human takeover and canonical request idempotency;
- exact Servo commit/tree/license/MSRV/source-file topology;
- isolated Servo source-checkout and worker-artifact boundary outside the
  `codex-rs` Cargo workspace;
- empty governed Servo patch queue;
- offline deterministic Servo source-receipt generator and strict receipt
  schema;
- private worker protocol v1 with canonical length-bounded framing, one-use
  startup capability, monotonic sequence spaces and closed-authority checks;
- portable child-pipe qualification harness;
- pathless inherited Unix `socketpair` qualification harness with spawned-PID
  binding;
- reusable Browser, Hepta-vNext and Blocking-CI workflow call chain with a
  dependency-free runner preflight.

### Not implemented or not qualified

- a generated receipt from the exact Servo checkout;
- complete Cargo/native SBOM, license inventory, toolchain receipt and offline
  source bundle;
- any imported, patched, built or linked Servo source;
- a real Servo WebView, renderer, semantic adapter or headed UI;
- Unix peer-credential, parent-death, artifact-identity, sandbox or platform
  qualification;
- a Windows SID-restricted named pipe or Job Object lifecycle;
- external browser network access;
- SecretRef injection, upload/download or browser-profile handling;
- durable browser journal and crash recovery;
- an App Server/product caller;
- operator acceptance, promotion or release.

GitHub Actions currently records the pure `Runner preflight` job as failed
before any step exists and skips all dependent jobs. That is a runner, billing
or repository-policy blocker. It is neither a passing nor a failing result for
the code in this branch. No current file may represent the branch as qualified
until the exact head executes the required commands and produces evidence.

## 3. Product goals

1. Let an Agent and a human safely share one visible page without concurrent
   mutation races.
2. Expose a stable, bounded semantic page model rather than unrestricted DOM,
   script or WebDriver access.
3. Preserve authenticated sessions without exposing raw credentials to the
   Agent, model, normal logs or ordinary receipts.
4. Bind every command, observation and outcome to one session, worker
   generation, owner epoch, page revision, request identity and policy digest.
5. Deny unknown network, file, secret, clipboard, device and browser-profile
   access by default.
6. Produce redacted, digest-bound evidence suitable for qualification,
   reconciliation and incident analysis.
7. Recover deterministically without repeating an indeterminate external
   effect.
8. Qualify one exact Hepta source, Servo source, patch, toolchain, artifact and
   platform set before a product caller or release transition.

## 4. Non-goals

- Browser automation is not a generic remote-control, scraping, proxy or
  credential-broker service.
- Page content cannot mint authority, change policy, request secrets or approve
  an effect.
- Servo is not the workflow scheduler, fleet authority, secret store, model
  runtime, evidence authority or recovery decision maker.
- The fixture engine is not a browser and cannot qualify Servo behavior.
- The stdio and Unix socketpair harnesses are not production-qualified worker
  transports.
- A source receipt is not a binary artifact receipt or complete SBOM.
- C1 completion does not enable external websites, credentials or effects.

## 5. Architecture

```text
Codex / Hepta product caller (future C6)
                 |
       typed BrowserRequest/Response
                 |
      Browser Actor — sole session owner
        |       |       |       |
    policy   secrets  journal  evidence
        |       |       |       |
  Hepta worker protocol / private inherited transport
                 |
      hepta-servo-worker — one process/session
                 |
   exact pinned Servo + one WebView + sandbox/profile
                 |
          hostile page content and network
```

The Browser Actor is the only mutation owner. Read-only observations may be
served concurrently only from immutable snapshots bound to one page revision.
The worker can report typed facts and outcomes but cannot grant authority or
select a broader transport, policy or endpoint.

## 6. Source and supply-chain topology

Servo remains a separate exact source checkout and a separately built worker
artifact. It is not a dependency in the Codex Cargo workspace. This prevents a
large MPL-2.0/native graph from silently changing the main lockfile, features,
license boundary or failure domain.

The source topology freezes:

- repository, commit and tree;
- workspace version, edition and MSRV;
- MPL-2.0 license;
- reviewed upstream source paths and Git blob IDs;
- allowed and forbidden initial Servo surfaces;
- source-bundle and artifact boundaries;
- an ordered patch queue with reason, tests, upstream reference and deletion
  condition for every patch.

The offline provenance generator rejects:

- a noncanonical, missing or symlinked checkout;
- the wrong `HEAD` or `HEAD^{tree}`;
- staged, tracked, untracked or other dirty state;
- a changed reviewed blob;
- missing required source files or unexpected license text;
- unregistered, missing or digest-mismatched patches;
- output paths with symlink/traversal ambiguity.

Its compact canonical source receipt deliberately omits the machine-local
checkout path and keeps every authority flag false. C1-002 remains incomplete
until that generator is run against the exact source and the same tree has a
Cargo/native SBOM, complete license inventory, toolchain receipt and offline
bundle checksum set.

## 7. Worker protocol v1

Every frame contains exactly:

```text
schema_version
session_id
generation
sequence
payload
```

The frame is compact recursively sorted JSON preceded by a four-byte unsigned
big-endian length. The payload is bounded to `1..=65536` bytes. Unknown fields,
malformed or noncanonical JSON, duplicate/stale/skipped sequences, wrong
session/generation/transport/capability, unexpected payloads and open authority
close the worker channel generation.

Handshake sequence zero binds a one-use 32-byte startup capability. After
readiness, parent-to-worker and worker-to-parent sequence spaces each begin at
one and increase exactly. There is no wraparound or resynchronization.

Payloads are limited to:

```text
ParentHello
WorkerReady
BrowserRequest
BrowserResponse
Shutdown
ShutdownAck
ProtocolError
```

Raw WebDriver commands, Servo preferences, arbitrary JavaScript, cookie jars,
profiles, files, sockets, native pointers and browser internals have no wire
variant. Adding one requires a successor schema, threat-model review and new
negative tests.

## 8. Platform-private transport plan

### Portable qualification pipes

Inherited stdin/stdout pipes qualify framing, process startup, bounded timeout,
PID binding, Actor request/response and explicit shutdown. They are not a
production transport.

### Unix inherited socketpair

The current Unix scaffold creates one anonymous full-duplex `socketpair`, keeps
one endpoint in the parent and duplicates the child endpoint into file
descriptors zero and one. It creates no socket path and no listener. The parent
requires the PID in `WorkerReady` to match the process it spawned.

Before production qualification, Unix still requires:

- executable/artifact digest binding;
- peer credential evidence where the platform supports it;
- parent-death and process-group behavior;
- private runtime/profile roots;
- sandbox, resource and core-dump policy;
- listener inventory and external-egress denial;
- Linux and macOS receipts.

### Windows named pipe

Windows must use a unique nonpersistent named pipe with a DACL restricted to the
expected service/user SID, an unpredictable instance identity and single-client
mode. The parent must bind the connected process identity, place the worker in a
Job Object and close/fence on impersonation, additional clients, reconnect,
sequence drift or parent exit.

No Windows transport is implemented in the current branch. A portable pipe is
not an acceptable substitute for the Windows production transport.

## 9. Process and artifact identity

A future production launch contract must bind:

```text
Hepta source commit/tree
Servo source commit/tree
patch inventory digest
Cargo.lock/toolchain/target/features
worker artifact SHA-256
sandbox/profile policy digest
session ID and generation
expected process identity
startup capability digest
```

The current harness validates an existing non-symlink worker executable and
binds the spawned PID to `WorkerReady`, but it does not yet prove the bytes
mapped into the running process. Artifact hashing, signed build manifest,
platform code-signing evidence where applicable and replacement-race tests are
release blockers.

## 10. Core browser contracts

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

The canonical request digest covers the complete typed request. Repeating the
same ID and digest returns the same response. Reusing an ID with another digest
fails closed. Response-body cache eviction must not make a previously used
request ID reusable; C5 must preserve a durable tombstone or monotonic request
frontier.

A semantic reference is bound to session and page revision. Navigation,
rerender, human input or other state-changing action invalidates older refs. The
Actor never silently relocates a stale reference by role, name or selector.

## 11. Control modes

### Agent-controlled

Agent mutations are accepted only when session, generation, epoch and revision
match and no human lease is active.

### Human-controlled

Taking control increments the owner epoch before success is returned. Queued
Agent mutations under the old epoch become stale. Human control is bounded by a
lease and is visible through typed status. Expiry or release creates another
current state; it never revives old Agent requests.

### Fenced or draining

Supervisor generation change, teardown, worker crash, integrity failure,
policy revocation or protocol ambiguity fences the session. Recovery creates a
new generation/epoch and does not reuse stale in-memory authority or worker
sequence state.

## 12. Semantic model—WEB-C2

The initial semantic projection is a bounded set/tree of interactive and
informative nodes with IDs stable only within one page revision. Role,
accessible name, redacted value/state class, frame/origin class and allowed
actions are explicit and size-limited.

Real Servo-backed C2 must qualify:

- accessibility and DOM mapping;
- dynamic invalidation and asynchronous page settlement;
- cross-origin frames and sandboxed documents;
- shadow DOM and form state;
- popups, prompts and navigation replacement;
- malicious text, huge trees, Unicode and mutation races;
- screenshot/evidence redaction.

The deterministic engine proves contracts only.

## 13. Policy, secrets and egress—WEB-C4

C4 freezes a `BrowserPolicySnapshot` at session and command admission. It binds:

- origin and destination classes;
- DNS, proxy, redirect, TLS and private-address rules;
- allowed methods, headers, body and resource limits;
- upload/download capability and staging roots;
- script/evaluation and extension capabilities;
- clipboard, camera, microphone, geolocation and notification policy;
- SecretRef audience, purpose, expiry and origin;
- evidence and redaction policy.

Unknown or changed policy fails closed. Credentials are resolved inside the
worker boundary through opaque references and are never returned as page data,
logs or ordinary receipts. Consent, CAPTCHA, anti-bot, 429 and unknown external
outcomes are typed challenges or indeterminate states, not automatic fallback
or bypass triggers.

## 14. Durability and recovery—WEB-C5

The journal records intent before crossing the browser/network boundary and an
outcome afterward:

```text
Prepared -> Dispatched -> Completed
                    \-> Failed
                    \-> Indeterminate -> Reconcile
```

An indeterminate action is never blindly replayed. Recovery checks generation,
policy, page revision, request/idempotency identity and any externally
observable state. It proves completion, proves nonexecution, quarantines the
session or requires human review.

C5 must also make request-ID history durable enough that cache eviction cannot
reopen an old ID with a different payload. Snapshots, journals and receipts use
canonical bytes, bounded inventories, private ownership, atomic replacement and
platform-appropriate file and directory durability.

## 15. Product caller—WEB-C6

C6 exposes only typed commands through the existing Codex/Hepta path. The UI
shows:

- current origin classification;
- Agent/human/fenced/draining control mode;
- generation, epoch and revision diagnostics;
- worker source/artifact identity and degradation;
- policy/network/evidence status;
- pending takeover and reconciliation requirements.

There is no silent fallback to a system browser, public WebDriver, hosted
browser or unrestricted automation service. A missing or degraded worker is a
typed unavailable state.

## 16. Qualification and release—WEB-C7

Each qualification receipt binds:

- exact Hepta and Servo commit/tree;
- exact patch inventory;
- target, toolchain, lockfile, build profile and features;
- executable/artifact/SBOM/license digests;
- transport, sandbox and policy digest;
- test commands, results and logs;
- malicious-site, crash, resource, privacy and listener results;
- known gaps;
- explicit negative authority.

Required qualification classes include:

1. contract/property/golden tests;
2. worker protocol and transport replay/identity tests;
3. real Servo local-fixture tests;
4. malicious page and prompt-injection corpus;
5. egress, DNS, redirect and local-network tests;
6. secret/evidence byte scans;
7. crash-point and recovery matrix;
8. CPU/RSS/disk/startup/latency budgets;
9. Linux/macOS/Windows build and smoke tests;
10. source, artifact, patch, SBOM and license verification;
11. exact product caller tests.

Operator acceptance and promotion are separate ceremonies after all evidence
binds one release candidate. Neither is produced by development CI.

## 17. CI and evidence policy

The canonical call chain is:

```text
Blocking CI
  -> Hepta vNext reusable qualification
       -> Hepta Browser reusable qualification
```

The browser workflow runs the repository verifier, provenance-generator tests,
contract/Actor/protocol unit tests, portable child process test, Unix socketpair
process test, all-target Cargo check and strict Clippy. Evidence records that
Servo is not imported or qualified and all authority/network flags remain
false.

A dependency-free runner preflight precedes all jobs. If it cannot record even
its first shell step, the result is `ENVIRONMENT_BLOCKED`, not `TEST_FAILED`.
Dependent jobs remain skipped and no PASS receipt may be created. Default-
branch pushes enter the qualification stack through Blocking CI only; the
standalone Hepta workflow is retained for manual, `vnext-main` and explicit
full-CI integration branches to avoid duplicate runs.

## 18. Performance and capacity budgets

Development bounds must exist before real Servo integration:

- one worker and one Actor per session;
- bounded command queue, frame and request-history capacity;
- bounded semantic node count and serialized bytes;
- bounded text, title, URL, node fields and protocol diagnostics;
- mandatory startup, command, navigation and shutdown deadlines;
- explicit RSS, CPU, GPU, disk, process and file-descriptor limits;
- per-Agent/workspace session quotas;
- evidence retention and total byte limits.

Exceeding a bound produces a typed denial, degraded state or fence. An
authority-relevant value is never silently truncated or evicted.

## 19. Exact next development sequence

1. **Run and seal C1-002 source receipt:** prepare the exact clean Servo
   checkout, run the offline generator twice, compare canonical bytes and add
   independent schema/source verification.
2. **Complete C1-002 supply-chain evidence:** Cargo/native SBOM, license notices,
   toolchain compatibility, source bundle and SHA256SUMS.
3. **Qualify C1-003 scaffolds:** restore Actions or use an independently
   controlled equivalent runner; run fmt, all-target check, tests, Clippy and
   process tests against the exact Draft head.
4. **Harden Unix transport:** artifact identity, peer credentials,
   parent-death/process-group, sandbox and platform evidence.
5. **Implement Windows transport:** SID DACL, single-client named pipe, process
   binding and Job Object.
6. **Create governed Servo patch 0001:** private worker entrypoint that reuses
   the headed Servo event loop without calling the WebDriver HTTP server.
7. **Build C1-004 local-fixture worker:** one real WebView, no external network,
   typed observation/action and complete teardown.
8. Continue C1-005 through C1-008 only after preceding evidence is sealed.

## 20. Stop conditions

Development stops and the candidate remains blocked when:

- source, tree, patch, toolchain, artifact or policy binding is unknown;
- the Actions/environment blocker is represented as a code PASS or FAIL;
- Servo starts any TCP, WebSocket or unauthenticated listener;
- the worker executable or process identity is ambiguous;
- raw secret, cookie, profile or authorization bytes cross the boundary;
- more than one mutation owner exists;
- stale session, generation, epoch, revision, ref, sequence or request conflict
  is accepted;
- external egress occurs without a bound policy;
- an indeterminate external effect is replayed blindly;
- evidence cannot be reproduced from the exact candidate;
- qualification-only output is represented as runtime, operator, promotion or
  release authority.
