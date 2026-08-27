# Hepta Browser implementation plan — WEB-E

## 0. Executive decision

Hepta will build a headed, human-visible, Servo-only browser subsystem. The browser is not a tool
wrapper around a hidden WebDriver session. One `BrowserSession` owns one process, one
`BrowserActor`, one Servo runtime, one `WebView`, one page state, one storage namespace, and one
ordered event stream. Human and Agent interaction share that exact page.

This plan converts the historical WEB-D direction into an executable repository-native program.
It preserves the stage identifiers `WEB-C0` through `WEB-C7`; no A/B/D stage is active.

The current implementation tranche completes portable C0 contracts and a qualification-only
C2/C3 fixture. It does not claim that Servo has been imported or that C1 has passed.

## 1. Goals

1. Let an Agent navigate, observe, act, wait, and extract through typed semantic operations.
2. Let a human see and control the same live page without migration or mirroring.
3. Make every mutation ordered, fenced, bounded, idempotent, and receipted.
4. Keep cookies, passwords, tokens, raw profile state, and unrestricted JavaScript out of model
   context.
5. Make web content observation-only with respect to capability and effect authority.
6. Support a later real-world corpus without weakening SSRF, redirect, tenant, or secret controls.
7. Produce machine-verifiable plans, schemas, tests, CI, evidence, and release boundaries.

## 2. Non-goals

The current development lane does not:

- expose production HTTP(S) navigation;
- import or build Servo;
- expose raw WebDriver/CDP;
- implement arbitrary JavaScript evaluation;
- perform login, upload, download, payment, message send, account mutation, or external form
  submission;
- create a second DOM, JavaScript engine, layout engine, browser profile, or page owner;
- authorize TaskFlow effects, G5, operator acceptance, promotion, or release;
- guarantee public-site compatibility, accessibility-tree fidelity, or anti-bot bypass.

## 3. Source and dependency binding

### 3.1 Hepta baseline

```text
repository: ProfAlexQI/hepta-private-ci
default_branch: integration/vnext-main-20260811
implementation_parent: fe0889ecd46a5fc89de7b1ff3f28158c133a3502
implementation_branch: codex/hepta-vnext-plan-browser-c0-c3-20260827
historical_code_parent: a85612afb43af722c61b54efe73570b25e9e4031
```

The implementation commit is bound by a successor current-state update after the first atomic
code/plan commit. Self-referential hashes are not embedded inside the commit they name.

### 3.2 Servo research candidate

```text
repository: servo/servo
commit: 0a48e298482659817eb50097df23841f2b8e3044
tree: b04d2f75b3217374d079d579c270177b57fa1389
license: MPL-2.0
status: research_pin_not_imported
```

The pin was selected as a reproducible research candidate observed on 2026-08-27. It is not a
promise to track upstream `main`, and it is not accepted into the Hepta build until C1 produces:

- source archive/submodule digest;
- Cargo.lock and Rust toolchain binding;
- native dependency inventory;
- MPL notices and SBOM;
- patch inventory with upstream issue/commit and deletion condition;
- Mac/Linux/Windows build commands and results;
- no-public-listener proof;
- headed WebView smoke evidence.

### 3.3 Mandatory Servo patch seam

At the research pin, Servo's WebDriver server starts an HTTP server on `0.0.0.0`. Hepta cannot
ship or qualify that behavior. C1 must use one of these options, in priority order:

1. call the WebDriver handler/command path in-process without an HTTP server;
2. add a small upstreamable handler facade with no network listener;
3. use an authenticated owner-only local IPC transport with peer identity, generation, session,
   nonce, request bounds, and no routable bind.

A plain loopback TCP port is insufficient because another same-host process may race or reuse it.

## 4. Target architecture

```text
Codex / TaskFlow planner
        │ typed BrowserRequest
        ▼
agentd / fleet generation fence
        │
        ▼
hepta-browserd (one process per BrowserSession)
  ├─ BrowserActor                  sole ordered owner
  ├─ PolicyGate                    URL/action/secret/effect decision
  ├─ SemanticProjector             bounded Servo facts → SemanticSnapshot
  ├─ ReceiptSealer                 canonical Activity/WebEvidence receipts
  ├─ HumanLease                    takeover/release/expiry
  ├─ ProfileManager                private per-session storage
  ├─ NetworkBroker                 C5 URL/DNS/redirect/SSRF enforcement
  ├─ DownloadQuarantine            C5/C7 only
  └─ ServoAdapter
       └─ one Servo runtime + one headed WebView
               ▲
               └── native human window/input for the same page
```

No other process may claim to be the authoritative page owner. Screenshots, accessibility trees,
semantic snapshots, or cached DOMs are projections, not alternate page states.

## 5. Portable contracts

### 5.1 BrowserSessionId

A session ID is a versioned domain-separated digest. Deserialization validates its canonical
prefix and lowercase SHA-256 shape. It is not an arbitrary caller string.

### 5.2 BrowserRequest

Every request binds:

```text
schema_version
request_id
session_id
actor (agent | human)
generation
owner_epoch
expected_page_revision
command
```

`request_id` is nonzero. Canonical request bytes bind idempotency. A reused ID with different
bytes is denied before command validation or execution.

### 5.3 BrowserCommand

The portable command set is deliberately small:

```text
Navigate { url }
Observe { max_nodes }
Act { semantic_ref, action }
Wait { condition }
Extract { allowlisted query, max_bytes }
HumanTakeControl { lease_ms }
HumanReleaseControl
HumanInput { semantic_ref, action }
```

There is no raw script-eval command, raw cookie command, profile-file command, network-header
command, or generic WebDriver pass-through.

### 5.4 SemanticSnapshot and SemanticRef

A semantic snapshot contains bounded URL/title and bounded nodes. Each node has a role, name,
redacted value, interactivity flag, and `SemanticRef` derived from:

```text
session_id + page_revision + engine node key
```

Any DOM-affecting mutation increments `page_revision` and clears active refs. Reusing an old ref
is a typed denial.

Servo C2 must build these nodes from engine-owned facts, preferably accessibility/layout-backed
facts. It must not execute untrusted page JavaScript to synthesize the semantic tree.

### 5.5 Outcomes

Outcomes are explicit:

```text
Applied
Observed
Extracted
WaitSatisfied
ControlTransferred
Denied
Challenge
Indeterminate
```

A timeout, renderer crash, unknown external mutation result, challenge, or anti-bot response is
not silently converted to success or blind retry.

## 6. Single-owner state machine

### 6.1 State

```text
session_id
generation
owner_epoch >= 1
page_revision >= 0
mode = AgentTurn | HumanTurn | ChallengePaused
human_lease_expires_at_ms?
active semantic refs
bounded request-response cache
engine handle
```

### 6.2 Transitions

```text
AgentTurn --HumanTakeControl--> HumanTurn
HumanTurn --HumanReleaseControl--> AgentTurn
HumanTurn --lease expiry--> AgentTurn
any --challenge--> ChallengePaused (C1/C4 integration)
ChallengePaused --human resolution + fresh observation--> AgentTurn/HumanTurn
```

Takeover increments owner epoch. Release and lease expiry increment owner epoch and page revision.
Mutations increment page revision. Read-only observation may occur during HumanTurn, but Agent
mutation is denied.

### 6.3 Exactly-one logical owner

The actor is the only component allowed to call mutating engine methods. UI events from the
headed window are translated into `HumanInput` under an active human lease and enter the same
queue. Servo callbacks that represent page changes must be serialized back through the actor
before a new semantic snapshot is published.

## 7. Privacy and data minimization

### 7.1 Engine privacy contract

Engine snapshots and extract results carry explicit markers:

```text
raw_secret_bytes_present
cross_tenant_data_present
```

The actor rejects marked results before returning model-facing content. Production C2 must derive
these markers from a real redaction/policy pass, not trust arbitrary page labels.

### 7.2 Never model-facing

- Cookie values and raw cookie jars
- Password fields and credential manager data
- Authorization headers, bearer tokens, refresh tokens, private keys
- Full browser profile files
- Cross-tenant storage
- Unbounded DOM/HTML/source
- Raw downloads
- Browser-internal object pointers or WebDriver element IDs

### 7.3 Allowed fixture extraction

The deterministic fixture allows safe values such as title, URL, typed fixture value, click count,
history length, and a non-secret storage revision number. Attempting to extract `document.cookie`
is explicitly marked sensitive and denied.

## 8. Receipts and evidence

### 8.1 BrowserActivityReceipt v2

Each processed request receives a receipt binding:

```text
actual actor session_id and generation
request id and canonical request digest
actor and command kind
outcome digest
owner epoch before/after
page revision before/after
closed authority object
receipt digest
```

For wrong-session or stale-generation requests, the receipt binds the actual server actor identity,
not the untrusted identity supplied in the request.

### 8.2 WebEvidenceReceipt v2

Evidence binds a sanitized semantic snapshot:

```text
session/generation/owner epoch/page revision
URL digest
snapshot digest
node count
raw_secret_bytes_present = false
cross_tenant_data_present = false
external_effect = false
qualification_only = true
receipt digest
```

Denied, challenged, or indeterminate commands do not receive a state evidence receipt, preventing
a rejected cross-session request from learning current page metadata.

## 9. Resource bounds

The initial fixture uses conservative limits:

| Resource | Bound |
|---|---:|
| Engine nodes | 1,024 |
| Published observe nodes | 256 |
| Extracted bytes | 16 KiB |
| Typed text | 4 KiB |
| URL | 2 KiB |
| Title | 512 B |
| Node role/name/value | 128 B / 512 B / 4 KiB |
| Cached request responses | 1,024 |
| Human lease | 300 s |

C1/C4/C5 must add process RSS, CPU, GPU, network bytes, redirect count, frame/worker count,
download bytes, wall-clock duration, and profile size limits.

## 10. Stage plan

### WEB-C0 — Authority and contract closure

**Deliverables**

- repository-native active plan;
- current-state document;
- threat model;
- stage matrix;
- traceability matrix;
- receipt schema;
- dependency-free verifier;
- CI/default-branch trigger repair;
- Hepta CODEOWNERS.

**Exit**

- all active stage IDs are C0-C7 only;
- every authority flag is closed;
- documents parse and cross-reference existing code/tests;
- historical snapshot is explicitly non-authoritative.

### WEB-C1 — Pinned headed Servo foundation

**Deliverables**

- immutable Servo source receipt and patch inventory;
- dedicated `hepta-browser-contracts` and `hepta-browserd` crates;
- one headed native WebView on Mac and Linux first, Windows next;
- private per-session profile;
- in-process or authenticated local control path;
- no public/loopback unauthenticated control listener;
- crash and generation-fence shutdown;
- minimal `fixture://`/local content loader.

**Exit**

- exact Servo commit/tree/toolchain/native dependencies build reproducibly;
- one process creates exactly one headed WebView;
- human UI is visible and input reaches the page;
- no external URL or production effect is enabled;
- source/SBOM/license/build receipts are sealed.

### WEB-C2 — Semantic projection

**Deliverables**

- move portable types from the qualification crate to `hepta-browser-contracts`;
- Servo semantic adapter using engine-owned accessibility/layout facts;
- redaction and privacy markers;
- stable ref derivation and invalidation;
- typed challenges and unsupported-capability outcomes;
- golden semantic fixtures.

**Exit**

- snapshots are deterministic for frozen fixtures;
- no page script is required to construct the semantic tree;
- cookie/password/token fixtures are denied/redacted;
- stale refs fail after every relevant mutation.

### WEB-C3 — Shared human/Agent actor

**Deliverables**

- bind the portable actor to the single Servo WebView;
- translate native UI input into ordered human events;
- visible takeover/release state;
- owner epoch and page revision fences;
- idempotency cache or durable journal;
- crash/restart policy;
- vertical-slice test with one live page.

**Exit**

- Agent changes are visible to the human;
- human changes are visible to the Agent without migration;
- no hidden second page/process is used;
- stale generation/epoch/revision/ref requests fail;
- same request replay is stable and conflict is denied.

### WEB-C4 — Read-only public-web qualification

**Deliverables**

- curated local corpus plus bounded public read-only corpus;
- navigation/redirect/frame/worker challenge taxonomy;
- compatibility metrics and known-gap registry;
- no authentication, upload, download, or external mutation.

**Exit**

- corpus coverage and failure classes are reproducible;
- unknown behavior is `Challenge`/`Indeterminate`, not fallback to another engine;
- all external requests pass the C5 policy broker even in read-only mode.

### WEB-C5 — Security and isolation

**Deliverables**

- network broker and SSRF defenses;
- DNS/redirect/private-IP tests;
- process sandbox and filesystem policy;
- download quarantine;
- profile encryption/permissions and tenant separation;
- same-UID peer authentication;
- fuzzing and malformed-engine-output tests.

**Exit**

- security corpus passes on all supported platforms;
- no control listener is routable or unauthenticated;
- no secret/cross-tenant bytes reach model-facing outputs or logs;
- engine crash/OOM is contained.

### WEB-C6 — Native product integration

**Deliverables**

- Mac/Linux/Windows headed UI integration;
- focus, IME, clipboard, file picker, permissions, window show/hide;
- observability and runbooks;
- packaging and update path;
- accessibility qualification.

**Exit**

- native input and Agent control remain the same page;
- platform-specific known gaps are explicit;
- no permission surface silently widens authority.

### WEB-C7 — Authentication and controlled effects

**Deliverables**

- AuthBus-backed credentials and virtual secret refs;
- operator-mediated login and MFA/challenge handling;
- TaskFlow effect intents, idempotency, and reconcile;
- upload/download/form-submit policies;
- final release qualification and rollback.

**Exit**

- no raw credential is model-facing;
- potentially applied mutations reconcile rather than blind retry;
- operator acceptance binds exact artifact and evidence;
- production flags change only through a separate promotion receipt.

## 11. Test strategy

### 11.1 Current portable tests

- shared Agent→Human→Agent state on one fixture actor;
- semantic ref invalidation;
- exact idempotent replay;
- request-ID conflict;
- human lease expiry;
- external navigation denial;
- wrong-session receipt identity;
- canonical ID deserialization;
- sensitive cookie extraction denial;
- closed authority assertions.

### 11.2 C1 integration tests

- headed window starts with no listener;
- one WebView identity remains stable;
- process generation fence kills stale actor;
- crash restart creates a new session/generation rather than reusing refs;
- fixture navigation, observe, input, click, and extract through Servo;
- native human input is observed by the same semantic projector.

### 11.3 C4-C7 tests

- Web Platform Tests subset and compatibility corpus;
- malformed HTML/JS/CSS/media and renderer crash corpus;
- SSRF/DNS rebinding/redirect/proxy tests;
- frame/worker/service-worker/WebSocket policy;
- login/MFA/challenge fixtures;
- unknown mutation/reconcile fixtures;
- long-running resource/soak tests;
- platform packaging and rollback.

## 12. CI and merge policy

The focused browser CI must run on pull requests that touch Hepta code, browser documents, the
verifier, or the workflow itself. It must also run on pushes to the actual default branch.

Required focused checks:

```text
plan verifier
browser Rust tests
browser library check
browser clippy -D warnings
workspace cargo fmt --check
clean worktree
```

C1 will add Servo source/build/license jobs rather than overloading the portable fixture job.

A green fixture job means `C0/C2/C3 fixture qualified for that commit`; it does not set C1 or
integrated C3 to complete.

## 13. Rollback

The current tranche is additive and qualification-only. Rollback is:

1. revert the implementation commit;
2. remove the active repo-native pointer or mark it superseded;
3. retain historical receipts and CI logs;
4. do not rewrite a failed/indeterminate outcome into success;
5. keep all production flags false.

Future Servo rollback must preserve the last known source/build/SBOM receipts and remove the
candidate from the product dependency graph. A browser profile from one candidate must not be
opened by another candidate without an explicit migration gate.

## 14. Immediate next queue after this tranche

1. Seal the implementation commit binding and focused CI result.
2. Create an isolated C1 source-import branch from the merged exact head.
3. Generate Servo source/license/SBOM/patch receipts for the research pin.
4. Prototype an in-process command facade; do not start upstream WebDriver HTTP.
5. Bring up one headed `fixture://shared-form` WebView on macOS and Linux.
6. Bind the existing BrowserActor trait to that WebView without changing the portable contract.
7. Run the same shared-page tests against Servo and keep external navigation disabled.
8. Only after integrated C3 passes, start C4/C5 read-only network qualification.
